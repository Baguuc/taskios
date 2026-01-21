pub struct TaskUpdateFeature;

impl TaskUpdateFeature {
    /// A helper function to register the feature in the service configuration.
    pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
        use actix_web::web;

        cfg.service(web::resource(Self::path()).route(web::patch().to(Self::controller)));
    }

    /// The logic of the feature - database interaction, authorization
    async fn execute<'p>(
        params: crate::params::feature::TasksUpdateParams<'p>,
        database_connection: std::sync::Arc<sqlx::PgPool>,
        authios_client: std::sync::Arc<authios_sdk::AuthiosClient>,
    ) -> Result<crate::models::Task, crate::errors::feature::TaskUpdateError> {
        use crate::errors::{
            feature::TaskUpdateError as Error,
            utils::auth::{ProjectPermissionCheckError, ServicePermissionCheckError},
        };
        use crate::repositories::TaskRepository;
        use crate::utils::auth::{check_user_project_permission, check_user_service_permission};

        match check_user_service_permission(params.token.clone(), authios_client.clone()).await {
            Ok(false) => return Err(Error::Unauthorized),
            Err(ServicePermissionCheckError::InvalidToken) => return Err(Error::InvalidToken),
            _ => (),
        };

        let mut database_connection = database_connection.acquire().await.unwrap();

        let task = match TaskRepository::retrieve(&mut *database_connection, params.task_id).await {
            Some(task) => task,
            None => return Err(Error::TaskNotFound),
        };

        match check_user_project_permission(
            params.token.clone(),
            task.project_id,
            String::from("write"),
            authios_client.clone(),
        )
        .await
        {
            Ok(false) => return Err(Error::Unauthorized),
            Err(ProjectPermissionCheckError::InvalidToken) => return Err(Error::InvalidToken),
            _ => (),
        };

        let result =
            TaskRepository::update(&mut *database_connection, params.task_id, params.new_data)
                .await;

        result.map_err(|_| Error::TaskNotFound)
    }

    /// A helper function to store the feature's url in one place.
    const fn path() -> &'static str {
        "/projects/{id}"
    }

    /// The controller for the feature.
    /// Recieves HTTP request's extractors as parameters and bridges the data to the business logic layer.
    async fn controller(
        path: actix_web::web::Path<Path>,
        body: actix_web::web::Json<crate::models::PartialTask>,
        token: crate::extractors::TokenExtractor,
        database_connection: actix_web::web::Data<sqlx::PgPool>,
        authios_client: actix_web::web::Data<authios_sdk::AuthiosClient>,
    ) -> actix_web::HttpResponse {
        use crate::errors::feature::TaskUpdateError as Error;
        use actix_web::HttpResponse;
        use serde_json::json;

        let result = Self::execute(
            crate::params::feature::TasksUpdateParams {
                task_id: &path.id,
                token: &token.0,
                new_data: &body.into_inner(),
            },
            database_connection.into_inner(),
            authios_client.into_inner(),
        )
        .await;

        match result {
            Ok(_) => HttpResponse::Ok().into(),
            Err(error) => match error {
                Error::Unauthorized => {
                    HttpResponse::Forbidden().json(json!({ "code": "forbidden" }))
                }
                Error::InvalidToken => {
                    HttpResponse::Unauthorized().json(json!({ "code": "invalid_token" }))
                }
                Error::TaskNotFound => {
                    HttpResponse::NotFound().json(json!({ "code": "task_not_found" }))
                }
            },
        }
    }
}

#[derive(serde::Deserialize)]
struct Path {
    id: i32,
}
