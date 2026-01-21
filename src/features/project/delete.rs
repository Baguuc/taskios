pub struct ProjectDeleteFeature;

impl ProjectDeleteFeature {
    pub async fn execute<'p>(
        params: crate::params::feature::ProjectDeleteParams<'p>,
        database_connection: std::sync::Arc<sqlx::PgPool>,
        authios_client: std::sync::Arc<authios_sdk::AuthiosClient>,
        config: std::sync::Arc<crate::config::Config>,
    ) -> Result<(), crate::errors::feature::ProjectDeleteError> {
        use crate::errors::{
            feature::ProjectDeleteError as Error,
            utils::auth::{ProjectPermissionCheckError, ServicePermissionCheckError},
        };
        use crate::utils::auth::{check_user_project_permission, check_user_service_permission};

        match check_user_service_permission(params.token.clone(), authios_client.clone()).await {
            Ok(false) => return Err(Error::Unauthorized),
            Err(ServicePermissionCheckError::InvalidToken) => return Err(Error::InvalidToken),
            _ => (),
        };

        match check_user_project_permission(
            params.token.clone(),
            *params.id,
            String::from("manage"),
            authios_client.clone(),
        )
        .await
        {
            Ok(false) => return Err(Error::Unauthorized),
            Err(ProjectPermissionCheckError::InvalidToken) => return Err(Error::InvalidToken),
            _ => (),
        };

        let mut database_connection = database_connection.acquire().await.unwrap();

        match crate::repositories::ProjectRepository::delete(&mut *database_connection, params.id)
            .await
        {
            Ok(_) => (),
            // only this possibility can be covered
            Err(_) => return Err(Error::ProjectNotFound),
        };

        // this won't error as we reject invalid token during the permission check
        let _ = crate::utils::auth::bulk_revoke_project_permissions(
            params.token.clone(),
            *params.id,
            vec![
                String::from("read"),
                String::from("write"),
                String::from("manage"),
            ],
            authios_client,
            config.auth.root.password.clone(),
        )
        .await;

        Ok(())
    }

    pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
        use actix_web::web;

        cfg.service(web::resource(Self::path()).route(web::delete().to(Self::controller)));
    }

    fn path() -> &'static str {
        "/projects/{id}"
    }

    async fn controller(
        path: actix_web::web::Path<Path>,
        token: crate::extractors::TokenExtractor,
        database_connection: actix_web::web::Data<sqlx::PgPool>,
        authios_client: actix_web::web::Data<authios_sdk::AuthiosClient>,
        config: actix_web::web::Data<crate::config::Config>,
    ) -> actix_web::HttpResponse {
        use crate::errors::feature::ProjectDeleteError as Error;
        use actix_web::HttpResponse;
        use serde_json::json;

        let result = Self::execute(
            crate::params::feature::ProjectDeleteParams {
                id: &path.id,
                token: &token.0,
            },
            database_connection.into_inner(),
            authios_client.into_inner(),
            config.into_inner(),
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
                Error::ProjectNotFound => {
                    HttpResponse::NotFound().json(json!({ "code": "project_not_found" }))
                }
            },
        }
    }
}

#[derive(serde::Deserialize)]
struct Path {
    pub id: i32,
}
