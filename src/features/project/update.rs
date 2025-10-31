pub struct ProjectUpdateFeature;

impl ProjectUpdateFeature {
    pub async fn execute<'p>(
        params: crate::params::feature::ProjectUpdateParams<'p>,
        database_connection: std::sync::Arc<sqlx::PgPool>,
        authios_client: std::sync::Arc<authios_sdk::AuthiosClient>
    ) -> Result<(), crate::errors::feature::ProjectUpdateError> {
        use crate::utils::panic::UtilPanics;
        use crate::errors::feature::ProjectUpdateError as Error;
        use authios_sdk::requests::{
            LoggedUserCheckServicePermissionRequest as ServicePermissionRequest,
            LoggedUserCheckResourcePermissionRequest as ResourcePermissionRequest
        };
        use authios_sdk::responses::{
            LoggedUserCheckServicePermissionResponse as ServicePermissionResponse,
            LoggedUserCheckResourcePermissionResponse as ResourcePermissionResponse
        };
        
        if params.new_name.is_none() {
            return Ok(());
        }

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
                return Err(Error::Unauthorized)
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
                permission_name: String::from("manage"),
            })
            .await;

        match resource_permission_response {
            ResourcePermissionResponse::Ok { has_permission } => if !has_permission {
                return Err(Error::Unauthorized)
            },
            ResourcePermissionResponse::InvalidToken => return Err(Error::InvalidToken),
            ResourcePermissionResponse::ServerNotAuthios => UtilPanics::server_not_authios(),
            ResourcePermissionResponse::ServerUnavailable => UtilPanics::authios_unavailable(),
            ResourcePermissionResponse::PermissionNotFound => UtilPanics::authios_not_inited(),
        };
        
        let sql = "UPDATE projects SET name = $1 WHERE id = $2;";
        let result = sqlx::query(sql)
            .bind(params.new_name.clone().unwrap())
            .bind(params.id)
            .execute(&mut *database_connection)
            .await
            .unwrap();

        if result.rows_affected() > 0 { 
            return Ok(()); 
        } else { 
            return Err(Error::ProjectNotFound);
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
        body: actix_web::web::Json<Json>,
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
                new_name: &body.new_name
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

#[derive(serde::Deserialize)]
struct Json {
    new_name: Option<String>
}
