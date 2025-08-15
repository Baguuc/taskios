#[actix_web::patch("/tasks/{id}/completion")]
pub async fn controller(
    req: actix_web::HttpRequest,
    body: actix_web::web::Json<RequestBody>,
    path: actix_web::web::Path<PathData>,
    _authios_sdk: actix_web::web::Data<authios_sdk::Sdk>,
    database_client: actix_web::web::Data<sqlx::postgres::PgPool>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use taskios_application::TasksUseCase;
    use taskios_domain::{
        errors::use_cases::tasks::update::Error,
        params::use_cases::task::update::{Params,NewData},
    };
    
    let user_token = match req.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(str) => str.to_string(),
            Err(_) => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
        },
        None => return HttpResponse::Unauthorized().body("NO_TOKEN")
    };
    
    let mut database_client = match database_client.acquire().await {
        Ok(client) => client,
        Err(_) => return HttpResponse::InternalServerError().body("DATABASE_CONNECTION")
    };
    
    let params = Params {
        id: path.id,
        user_token,
        new: NewData {
            title: body.title.clone(),
            description: body.description.clone()
        }
    };
    return match TasksUseCase::update(&params, _authios_sdk.into_inner(), &mut *database_client).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
            Error::NotExist => HttpResponse::NotFound().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string()),
        } 
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    title: Option<String>,
    description: Option<String>
}

#[derive(serde::Deserialize)]
pub struct PathData {
    id: i32
}
