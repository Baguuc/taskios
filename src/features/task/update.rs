pub struct TaskUpdateFeature;

impl TaskUpdateFeature {
    pub async fn execute<'p>(
        params: crate::params::feature::TasksUpdateParams<'p>,
        database_connection: std::sync::Arc<sqlx::PgPool>,
        authios_client: std::sync::Arc<authios_sdk::AuthiosClient>
    ) -> Result<crate::models::Task, crate::errors::feature::ProjectUpdateTaskError> {
        use crate::utils::panic::UtilPanics;
        use crate::errors::feature::ProjectUpdateTaskError as Error;
        use authios_sdk::requests::{
            LoggedUserCheckServicePermissionRequest as ServicePermissionRequest,
            LoggedUserCheckResourcePermissionRequest as ResourcePermissionRequest
        };
        use authios_sdk::responses::{
            LoggedUserCheckServicePermissionResponse as ServicePermissionResponse,
            LoggedUserCheckResourcePermissionResponse as ResourcePermissionResponse
        };

        let mut database_connection = database_connection.acquire()
            .await
            .unwrap();

        let service_permission_response = authios_client.query()
            .user()
            .logged(params.token.clone())
            .permissions()
            .service()
            .check(ServicePermissionRequest {
                service_id: String::from("taskios")
            })
            .await;

        match service_permission_response {
            ServicePermissionResponse::Ok { has_permission } => if !has_permission {
                return Err(Error::Unauthorized);
            },
            ServicePermissionResponse::InvalidToken => return Err(Error::InvalidToken),
            ServicePermissionResponse::ServerNotAuthios => UtilPanics::server_not_authios(),
            ServicePermissionResponse::ServerUnavailable => UtilPanics::authios_unavailable(),
            ServicePermissionResponse::PermissionNotFound => UtilPanics::authios_not_inited(),
        };

        let resource_permission_response = authios_client.query()
            .user()
            .logged(params.token.clone())
            .permissions()
            .resource()
            .check(ResourcePermissionRequest {
                service_id: String::from("taskios"),
                resource_type: String::from("project"),
                resource_id: params.task_id.to_string(),
                permission_name: String::from("write")
            })
            .await;

        match resource_permission_response {
            ResourcePermissionResponse::Ok { has_permission } => if !has_permission {
                return Err(Error::Unauthorized);
            },
            ResourcePermissionResponse::InvalidToken => return Err(Error::InvalidToken),
            ResourcePermissionResponse::ServerNotAuthios => UtilPanics::server_not_authios(),
            ResourcePermissionResponse::ServerUnavailable => UtilPanics::authios_unavailable(),
            ResourcePermissionResponse::PermissionNotFound => UtilPanics::authios_not_inited(),
        };

        let result = crate::repositories::TaskRepository::update(&mut *database_connection, params.task_id, params.new_data).await;

        result.map_err(|_| Error::TaskNotFound)
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
        body: actix_web::web::Json<crate::models::PartialTask>,
        token: crate::extractors::TokenExtractor,
        database_connection: actix_web::web::Data<sqlx::PgPool>,
        authios_client: actix_web::web::Data<authios_sdk::AuthiosClient>
    ) -> actix_web::HttpResponse {
        use serde_json::json;
        use actix_web::HttpResponse;
        use crate::errors::feature::ProjectUpdateTaskError as Error;

        let result = Self::execute(
            crate::params::feature::TasksUpdateParams {
                task_id: &path.id,
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
                Error::TaskNotFound => HttpResponse::NotFound()
                    .json(json!({ "code": "task_not_found" }))
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct Path {
    id: i32
}