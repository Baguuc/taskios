pub struct ProjectCreateFeature;

impl ProjectCreateFeature {
    pub async fn execute<'p>(
        params: crate::params::feature::ProjectCreateParams<'p>,
        database_connection: std::sync::Arc<sqlx::PgPool>,
        authios_client: std::sync::Arc<authios_sdk::AuthiosClient>,
        config: std::sync::Arc<crate::config::Config>
    ) -> Result<crate::models::Project, crate::errors::feature::ProjectCreateError> {
        use crate::utils::auth::check_user_service_permission;
        use crate::errors::{
            feature::ProjectCreateError as Error,
            utils::auth::ServicePermissionCheckError
        };

        match check_user_service_permission(params.token.clone(), authios_client.clone()).await {
            Ok(false) => return Err(Error::Unauthorized),
            Err(ServicePermissionCheckError::InvalidToken) => return Err(Error::InvalidToken),
            _ => ()
        };

        let mut database_connection = database_connection.acquire()
            .await
            .unwrap();
        let project = match crate::repositories::ProjectRepository::create(&mut *database_connection, params.project).await {
            Ok(project) => project,
            // this shouldn't happen
            _ => panic!()
        };

        // this won't error as we reject invalid token during the permission check
        let _ = crate::utils::auth::bulk_grant_project_permissions(
            params.token.clone(),
            project.id,
            vec![
                String::from("read"),
                String::from("write"),
                String::from("manage")
            ],
            authios_client,
            config.auth.root.password.clone()
        ).await;

        Ok(project)
    }
    
    pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
        use actix_web::web;

        cfg.service(
            web::resource(Self::path()).route(web::post().to(Self::controller))
        );
    }

    fn path() -> &'static str { "/projects" }
    
    async fn controller(
        body: actix_web::web::Json<crate::models::ProjectWithoutId>,
        token: crate::extractors::TokenExtractor,
        database_connection: actix_web::web::Data<sqlx::PgPool>,
        authios_client: actix_web::web::Data<authios_sdk::AuthiosClient>,
        config: actix_web::web::Data<crate::config::Config>
    ) -> actix_web::HttpResponse {
        use serde_json::json;
        use actix_web::HttpResponse;
        use crate::errors::feature::ProjectCreateError as Error;

        let result = Self::execute(
            crate::params::feature::ProjectCreateParams {
                project: &body.into_inner(),
                token: &token.0
            },
            database_connection.into_inner(),
            authios_client.into_inner(),
            config.into_inner()
        ).await;

        match result {
            Ok(_) => HttpResponse::Ok().into(),
            Err(error) => match error {
                Error::Unauthorized => HttpResponse::Forbidden()
                    .json(json!({ "code": "forbidden" })),
                Error::InvalidToken => HttpResponse::Unauthorized()
                    .json(json!({ "code": "invalid_token" }))
            }
        }
    }
}