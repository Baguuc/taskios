pub struct ProjectDeleteFeature;

impl ProjectDeleteFeature {
    pub async fn execute<'p>(
        params: crate::params::feature::ProjectDeleteParams<'p>,
        database_connection: std::sync::Arc<sqlx::PgPool>,
        authios_client: std::sync::Arc<authios_sdk::AuthiosClient>
    ) -> Result<(), crate::errors::feature::ProjectDeleteError> {
        use crate::errors::feature::ProjectDeleteError as Error;
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
            ServicePermissionResponse::ServerNotAuthios => panic!("AUTH SERVER ERROR: auth server returns invalid responses"),
            ServicePermissionResponse::ServerUnavailable => panic!("AUTH SERVER ERROR: auth server shut down"),
            ServicePermissionResponse::PermissionNotFound => panic!("AUTH SERVER ERROR: auth server wasn't inited - it's lacking crucial permissions to run this software")
        };
        
        let auth_response = authios_client.query()
            .user()
            .logged(params.token.clone())
            .permissions()
            .resource()
            .check(ResourcePermissionRequest {
                service_id: String::from("taskios"),
                resource_type: String::from("project"),
                resource_id: params.id.to_string(),
                permission_name: String::from("manage")
            })
            .await;
        
        match auth_response {
            ResourcePermissionResponse::Ok { has_permission } => if !has_permission {
                return Err(Error::Unauthorized);
            },
            ResourcePermissionResponse::InvalidToken => return Err(Error::InvalidToken),
            ResourcePermissionResponse::ServerNotAuthios => panic!("AUTH SERVER ERROR: auth server returns invalid responses"),
            ResourcePermissionResponse::ServerUnavailable => panic!("AUTH SERVER ERROR: auth server shut down"),
            ResourcePermissionResponse::PermissionNotFound => panic!("AUTH SERVER ERROR: auth server wasn't inited - it's lacking crucial permissions to run this software")
        };
        
        let sql = "DELETE FROM projects WHERE id = $1;";
        let _ = sqlx::query(sql)
            .bind(params.id)
            .execute(&mut *database_connection)
            .await;

        Ok(())
    }

    pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
        use actix_web::web;

        cfg.service(
            web::resource(Self::path()).route(web::delete().to(Self::controller))
        );
    }

    fn path() -> &'static str { "/projects/{id}" }
    
    async fn controller(
        path: actix_web::web::Path<Path>,
        token: crate::extractors::TokenExtractor,
        database_connection: actix_web::web::Data<sqlx::PgPool>,
        authios_client: actix_web::web::Data<authios_sdk::AuthiosClient>
    ) -> actix_web::HttpResponse {
        use serde_json::json;
        use actix_web::HttpResponse;
        use crate::errors::feature::ProjectDeleteError as Error;

        let result = Self::execute(
            crate::params::feature::ProjectDeleteParams {
                id: &path.id,
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

#[derive(serde::Deserialize)]
struct Path {
    pub id: i32
}
