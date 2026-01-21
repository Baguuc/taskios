pub struct TaskListFeature;

impl TaskListFeature {
    /// A helper function to register the feature in the service configuration
    pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
        use actix_web::web;

        cfg.service(web::resource(Self::path()).route(web::get().to(Self::controller)));
    }

    /// The logic of the feature - database interaction, authorization
    pub async fn execute<'p>(
        params: crate::params::feature::TasksListParams<'p>,
        database_connection: std::sync::Arc<sqlx::PgPool>,
        authios_client: std::sync::Arc<authios_sdk::AuthiosClient>,
    ) -> Result<Option<Vec<crate::models::Task>>, crate::errors::feature::TasksListError>
    {
        use crate::errors::{
            feature::TasksListError as Error, utils::auth::ServicePermissionCheckError,
        };
        use crate::utils::{
            auth::check_user_service_permission, panic::UtilPanics, project::project_exists,
        };
        use authios_sdk::requests::LoggedUserCheckResourcePermissionRequest as ResourcePermissionRequest;
        use authios_sdk::responses::LoggedUserCheckResourcePermissionResponse as ResourcePermissionResponse;

        let mut database_connection = database_connection.acquire().await.unwrap();

        match check_user_service_permission(params.token.clone(), authios_client.clone()).await {
            Ok(false) => return Err(Error::Unauthorized),
            Err(ServicePermissionCheckError::InvalidToken) => return Err(Error::InvalidToken),
            _ => (),
        };

        let resource_permission_response = authios_client
            .query()
            .user()
            .logged(params.token.clone())
            .permissions()
            .resource()
            .check(ResourcePermissionRequest {
                service_id: String::from("taskios"),
                resource_type: String::from("project"),
                resource_id: params.project_id.to_string(),
                permission_name: String::from("read"),
            })
            .await;

        match resource_permission_response {
            ResourcePermissionResponse::Ok { has_permission } => {
                if !has_permission {
                    return Err(Error::Unauthorized);
                }
            }
            ResourcePermissionResponse::InvalidToken => return Err(Error::InvalidToken),
            ResourcePermissionResponse::ServerNotAuthios => UtilPanics::server_not_authios(),
            ResourcePermissionResponse::ServerUnavailable => UtilPanics::authios_unavailable(),
            ResourcePermissionResponse::PermissionNotFound => UtilPanics::authios_not_inited(),
        };

        if !project_exists(params.project_id, &mut *database_connection).await {
            return Err(Error::ProjectNotFound);
        }

        let tasks =
            crate::repositories::TaskRepository::list(&mut *database_connection, params.project_id)
                .await;
        Ok(Some(tasks))
    }

    /// A helper function to store the feature's url in one place.
    const fn path() -> &'static str {
        "/projects/{id}/tasks"
    }

    /// The controller for the feature.
    /// Recieves HTTP request's extractors as parameters and bridges the data to the business logic layer.
    async fn controller(
        path: actix_web::web::Path<Path>,
        query: actix_web::web::Query<Query>,
        token: crate::extractors::TokenExtractor,
        database_connection: actix_web::web::Data<sqlx::PgPool>,
        authios_client: actix_web::web::Data<authios_sdk::AuthiosClient>,
    ) -> actix_web::HttpResponse {
        use crate::errors::feature::TasksListError as Error;
        use actix_web::HttpResponse;
        use serde_json::json;

        let result = Self::execute(
            crate::params::feature::TasksListParams {
                project_id: &path.id,
                token: &token.0,
            },
            database_connection.into_inner(),
            authios_client.into_inner(),
        )
        .await;

        match result {
            Ok(data) if data.is_some() => HttpResponse::Ok().json(json!({
                "code": "ok",
                "tasks": data.unwrap().iter().map(|row| {
                    DataRow {
                        id: if query.get_id.unwrap_or(true)
                            { Some(row.id) } else { None },
                        title: if query.get_title.unwrap_or(true)
                            { Some(row.title.clone()) } else { None },
                        description: if query.get_description.unwrap_or(true)
                            { Some(row.description.clone()) } else { None },
                        done: if query.get_done.unwrap_or(true)
                            { Some(row.done) } else { None }
                    }
                }).collect::<Vec<DataRow>>()
            })),
            Ok(_) => HttpResponse::Ok().json(json!({
                "code": "ok",
                "page": null
            })),
            Err(error) => match error {
                Error::Unauthorized => {
                    HttpResponse::Forbidden().json(json!({ "code": "forbidden" }))
                }
                Error::InvalidToken => {
                    HttpResponse::Unauthorized().json(json!({ "code": "invalid_token" }))
                }
                Error::ProjectNotFound => {
                    HttpResponse::NotFound().json(json!({ "code": "project_not_found" }))
                }
            },
        }
    }
}

#[derive(serde::Serialize)]
#[serde_with::skip_serializing_none]
struct DataRow {
    id: Option<i32>,
    title: Option<String>,
    description: Option<String>,
    done: Option<bool>,
}

#[derive(serde::Deserialize)]
struct Path {
    id: i32,
}

#[derive(serde::Deserialize)]
struct Query {
    get_id: Option<bool>,
    get_title: Option<bool>,
    get_description: Option<bool>,
    get_done: Option<bool>,
}
