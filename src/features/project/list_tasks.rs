pub struct ProjectListTasksFeature;

impl ProjectListTasksFeature {
    pub async fn execute<'p>(
        params: crate::params::feature::ProjectListTasksParams<'p>,
        database_connection: std::sync::Arc<sqlx::PgPool>,
        authios_client: std::sync::Arc<authios_sdk::AuthiosClient>
    ) -> Result<Option<Vec<crate::models::Task>>, crate::errors::feature::ProjectListTasksError> {
        use crate::utils::panic::UtilPanics;
        use crate::utils::project::project_exists;
        use crate::errors::feature::ProjectListTasksError as Error;
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
                resource_id: params.id.to_string(),
                permission_name: String::from("read")
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

        if !project_exists(params.id, &mut *database_connection).await {
            return Err(Error::ProjectNotFound);
        }

        let sql = "SELECT t.id, t.name, t.description, t.done FROM tasks t WHERE t.project_id = $1;";
        let result = sqlx::query_as(sql)
            .bind(params.id)
            .fetch_all(&mut *database_connection)
            .await
            .unwrap();

        Ok(Some(result))
    }
    
    pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
        use actix_web::web;

        cfg.service(
            web::resource(Self::path()).route(web::get().to(Self::controller))
        );
    }

    fn path() -> &'static str { "/projects/{id}/tasks" }
    
    async fn controller(
        path: actix_web::web::Path<Path>,
        query: actix_web::web::Query<Query>,
        token: crate::extractors::TokenExtractor,
        database_connection: actix_web::web::Data<sqlx::PgPool>,
        authios_client: actix_web::web::Data<authios_sdk::AuthiosClient>
    ) -> actix_web::HttpResponse {
        use serde_json::json;
        use actix_web::HttpResponse;
        use crate::errors::feature::ProjectListTasksError as Error;

        let result = Self::execute(
            crate::params::feature::ProjectListTasksParams {
                id: &path.id,
                token: &token.0,
                page_number: &query.page_number.unwrap_or(0)
            },
            database_connection.into_inner(),
            authios_client.into_inner()
        ).await;

        match result {
            Ok(data) if data.is_some() => HttpResponse::Ok().json(json!({
                "code": "ok",
                "page": data.unwrap().iter().map(|row| {
                    DataRow {
                        id: if query.get_id.unwrap_or(true)
                            { Some(row.id.clone()) } else { None },
                        title: if query.get_title.unwrap_or(true)
                            { Some(row.title.clone()) } else { None },
                        description: if query.get_description.unwrap_or(true)
                            { Some(row.description.clone()) } else { None },
                        done: if query.get_done.unwrap_or(true)
                            { Some(row.done.clone()) } else { None }
                    }
                }).collect::<Vec<DataRow>>()
            })),
            Ok(_) => HttpResponse::Ok().json(json!({
                "code": "ok",
                "page": null
            })),
            Err(error) => match error {
                Error::Unauthorized => HttpResponse::Forbidden()
                    .json(json!({ "code": "forbidden" })),
                Error::InvalidToken => HttpResponse::Unauthorized()
                    .json(json!({ "code": "invalid_token" })),
                Error::ProjectNotFound => HttpResponse::NotFound()
                    .json(json!({ "code": "project_not_found" })),
            }
        }
    }
}

#[derive(serde::Serialize)]
#[serde_with::skip_serializing_none]
struct DataRow {
    id: Option<i32>,
    title: Option<String>,
    description: Option<String>,
    done: Option<bool>
}

#[derive(serde::Deserialize)]
struct Path {
    id: i32
}

#[derive(serde::Deserialize)]
struct Query {
    page_number: Option<u32>,
    get_id: Option<bool>,
    get_title: Option<bool>,
    get_description: Option<bool>,
    get_done: Option<bool>
}
