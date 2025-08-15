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
        errors::use_cases::tasks::change_completion::Error,
        params::use_cases::task::change_completion::Params,
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
        completion: body.completion.to_string(),
        user_token
    };
    return match TasksUseCase::change_task_completion(&params, _authios_sdk.into_inner(), &mut *database_client).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(error) => match error {
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
            Error::ProjectNotExist => HttpResponse::NotFound().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string()),
        } 
    };
}

#[derive(serde::Deserialize)]
struct RequestBody {
    completion: Completion
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum Completion {
    Completed,
    Uncompleted
}

impl ToString for Completion {
    fn to_string(self: &Self) -> String {
        return match self {
            Self::Completed => "completed",
            Self::Uncompleted => "uncompleted"
        }
        .to_string();
    }
}

#[derive(serde::Deserialize)]
pub struct PathData {
    id: i32
}
