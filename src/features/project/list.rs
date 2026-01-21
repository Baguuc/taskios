/// Lists the projects that the user has access to.
pub struct ProjectListFeature;

impl ProjectListFeature {
    /// A helper function to register the feature in the service configuration
    pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
        use actix_web::web;

        cfg.service(web::resource(Self::path()).route(web::get().to(Self::controller)));
    }

    /// The logic of the feature - database interaction, authorization
    async fn execute<'p>(
        params: crate::params::feature::ProjectListParams<'p>,
        database_connection: std::sync::Arc<sqlx::PgPool>,
        authios_client: std::sync::Arc<authios_sdk::AuthiosClient>,
    ) -> Result<Option<Vec<crate::models::UserProject>>, crate::errors::feature::ProjectListError>
    {
        use crate::errors::{
            feature::ProjectListError as Error, utils::auth::ServicePermissionCheckError,
        };
        use crate::models::UserProject;
        use crate::utils::{auth::check_user_service_permission, panic::UtilPanics};
        use authios_sdk::requests::LoggedUserListResourcePermissionsRequest as ResourcePermissionRequest;
        use authios_sdk::responses::LoggedUserListResourcePermissionsResponse as ResourcePermissionResponse;

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
            .list(ResourcePermissionRequest {
                service_id: String::from("taskios"),
                resource_type: String::from("project"),
                page_number: *params.page_number,
                get_service_id: false,
                get_resource_type: false,
                get_resource_id: true,
                get_permission_names: true,
                get_page_number: true,
            })
            .await;

        let permissions_page = match resource_permission_response {
            ResourcePermissionResponse::Ok { page } => page,
            ResourcePermissionResponse::InvalidToken => return Err(Error::InvalidToken),
            ResourcePermissionResponse::ServerNotAuthios => {
                UtilPanics::server_not_authios();
                // compilation error otherwise
                panic!();
            }
            ResourcePermissionResponse::ServerUnavailable => {
                UtilPanics::authios_unavailable();
                // compilation error otherwise
                panic!();
            }
        };

        let permissions = match permissions_page.permissions {
            Some(permissions) => permissions,
            None => return Ok(None),
        };

        let mut user_projects = vec![];

        // the page has only five elements
        for permission in permissions.iter() {
            let id = match permission.resource_id.clone().unwrap().parse() {
                Ok(id) => id,
                _ => continue,
            };

            let project = match crate::repositories::ProjectRepository::retrieve(
                &mut *database_connection,
                &id,
            )
            .await
            {
                Some(project) => project,
                None => continue,
            };

            let permissions = permission.permissions.clone().unwrap();

            let user_project = UserProject {
                id: project.id,
                name: project.name,
                permissions,
            };

            user_projects.push(user_project);
        }

        Ok(Some(user_projects))
    }

    /// A helper function to store the feature's url in one place.
    const fn path() -> &'static str {
        "/projects/my"
    }

    /// The controller for the feature.
    /// Recieves HTTP request's extractors as parameters and bridges the data to the business logic layer.
    async fn controller(
        query: actix_web::web::Query<Query>,
        token: crate::extractors::TokenExtractor,
        database_connection: actix_web::web::Data<sqlx::PgPool>,
        authios_client: actix_web::web::Data<authios_sdk::AuthiosClient>,
    ) -> actix_web::HttpResponse {
        use crate::errors::feature::ProjectListError as Error;
        use actix_web::HttpResponse;
        use serde_json::json;

        let result = Self::execute(
            crate::params::feature::ProjectListParams {
                token: &token.0,
                page_number: &query.page_number.unwrap_or(0),
            },
            database_connection.into_inner(),
            authios_client.into_inner(),
        )
        .await;

        match result {
            Ok(data) if data.is_some() => HttpResponse::Ok().json(json!({
                "code": "ok",
                "page": data.unwrap().iter().map(|row| {
                    DataRow {
                        id: if query.get_id.unwrap_or(true)
                            { Some(row.id) } else { None },
                        name: if query.get_name.unwrap_or(true)
                            { Some(row.name.clone()) } else { None },
                        permissions: if query.get_permissions.unwrap_or(true)
                            { Some(row.permissions.clone()) } else { None }
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
            },
        }
    }
}

#[derive(serde::Serialize)]
#[serde_with::skip_serializing_none]
struct DataRow {
    id: Option<i32>,
    name: Option<String>,
    permissions: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
struct Query {
    page_number: Option<u32>,
    get_id: Option<bool>,
    get_name: Option<bool>,
    get_permissions: Option<bool>,
}
