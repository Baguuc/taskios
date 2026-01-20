pub struct TaskCreateFeature;

impl TaskCreateFeature {
    pub async fn execute<'p>(
        params: crate::params::feature::TaskCreateParams<'p>,
        database_connection: std::sync::Arc<sqlx::PgPool>,
        authios_client: std::sync::Arc<authios_sdk::AuthiosClient>
    ) -> Result<crate::models::Task, crate::errors::feature::ProjectCreateTaskError> {
        use crate::utils::auth::{
            check_user_service_permission,
            check_user_project_permission
        };
        use crate::errors::{
            feature::ProjectCreateTaskError as Error,
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

        match check_user_project_permission(params.token.clone(), params.task.project_id.clone(), String::from("write"), authios_client.clone()).await {
            Ok(false) => return Err(Error::Unauthorized),
            Err(ProjectPermissionCheckError::InvalidToken) => return Err(Error::InvalidToken),
            _ => ()
        };

        let mut database_connection = database_connection.acquire()
            .await
            .unwrap();
        let result = crate::repositories::TaskRepository::create(&mut *database_connection, params.task).await;

        match result {
            Ok(task) => Ok(task),
            Err(err) => match err {
                crate::errors::repository::TaskCreateError::ProjectNotFound => Err(Error::ProjectNotFound)
            }
        }
    }

    pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
        use actix_web::web;

        cfg.service(
            web::resource(Self::path()).route(web::post().to(Self::controller))
        );
    }

    fn path() -> &'static str { "/tasks" }

    async fn controller(
        body: actix_web::web::Json<crate::models::TaskWithoutId>,
        token: crate::extractors::TokenExtractor,
        database_connection: actix_web::web::Data<sqlx::PgPool>,
        authios_client: actix_web::web::Data<authios_sdk::AuthiosClient>
    ) -> actix_web::HttpResponse {
        use serde_json::json;
        use actix_web::HttpResponse;
        use crate::errors::feature::ProjectCreateTaskError as Error;

        let result = Self::execute(
            crate::params::feature::TaskCreateParams {
                task: &body.into_inner(),
                token: &token.0
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