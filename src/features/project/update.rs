pub struct ProjectUpdateFeature;

impl ProjectUpdateFeature {
    pub async fn execute<'p>(
        params: crate::params::feature::ProjectUpdateParams<'p>,
        database_connection: std::sync::Arc<sqlx::PgPool>,
        authios_client: std::sync::Arc<authios_sdk::AuthiosClient>
    ) -> Result<crate::models::Project, crate::errors::feature::ProjectUpdateError> {
        use crate::utils::auth::{
            check_user_service_permission,
            check_user_project_permission
        };
        use crate::errors::{
            feature::ProjectUpdateError as Error,
            utils::auth::{
                ServicePermissionCheckError,
                ProjectPermissionCheckError
            }
        };

        match check_user_service_permission(params.token.clone(), authios_client.clone()).await {
            Ok(false) => return Err(Error::Unauthorized),
            Err(ServicePermissionCheckError::InvalidToken) => return Err(Error::InvalidToken),
            _ => ()
        };

        match check_user_project_permission(params.token.clone(), params.id.clone(), String::from("manage"), authios_client.clone()).await {
            Ok(false) => return Err(Error::Unauthorized),
            Err(ProjectPermissionCheckError::InvalidToken) => return Err(Error::InvalidToken),
            _ => ()
        };

        let mut database_connection = database_connection.acquire()
            .await
            .unwrap();
        let result = crate::repositories::ProjectRepository::update(&mut *database_connection, params.id, params.new_data).await;

        match result {
            Ok(r) => Ok(r),
            Err(_) => Err(Error::ProjectNotFound)
        }
    }

    pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
        use actix_web::web;

        cfg.service(
            web::resource(Self::path()).route(web::patch().to(Self::controller))
        );
    }

    fn path() -> &'static str { "/projects/{id}" }
    
    async fn controller(
        path: actix_web::web::Path<Path>,
        body: actix_web::web::Json<crate::models::PartialProject>,
        token: crate::extractors::TokenExtractor,
        database_connection: actix_web::web::Data<sqlx::PgPool>,
        authios_client: actix_web::web::Data<authios_sdk::AuthiosClient>
    ) -> actix_web::HttpResponse {
        use serde_json::json;
        use actix_web::HttpResponse;
        use crate::errors::feature::ProjectUpdateError as Error;

        let result = Self::execute(
            crate::params::feature::ProjectUpdateParams {
                id: &path.id,
                token: &token.0,
                new_data: &body.into_inner()
            },
            database_connection.into_inner(),
            authios_client.into_inner()
        ).await;

        match result {
            Ok(_) => HttpResponse::Ok().into(),
            Err(error) => match error {
                Error::Unauthorized => HttpResponse::Forbidden()
                    .json(json!({ "code": "forbidden" })),
                Error::InvalidToken => HttpResponse::Unauthorized()
                    .json(json!({ "code": "invalid_token" })),
                Error::ProjectNotFound => HttpResponse::NotFound()
                    .json(json!({ "code": "project_not_found" }))
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct Path {
    id: i32
}
