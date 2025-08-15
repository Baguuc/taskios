#[actix_web::get("/projects")]
pub async fn controller(
    req: actix_web::HttpRequest,
    _authios_sdk: actix_web::web::Data<authios_sdk::Sdk>,
    database_client: actix_web::web::Data<sqlx::postgres::PgPool>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use taskios_application::ProjectsUseCase;
    use taskios_domain::{
        errors::use_cases::projects::list::Error,
        params::use_cases::project::list::Params,
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
        user_token
    };
    return match ProjectsUseCase::list(&params, _authios_sdk.into_inner(), &mut *database_client).await {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(error) => match error {
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string()),
        } 
    };
}
