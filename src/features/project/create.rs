pub struct ProjectCreateFeature;

impl ProjectCreateFeature {
    pub async fn execute<'p>(
        params: crate::params::feature::ProjectCreateParams<'p>,
        database_connection: std::sync::Arc<sqlx::PgPool>,
        authios_client: std::sync::Arc<authios_sdk::AuthiosClient>
    ) -> Result<(), crate::errors::feature::ProjectCreateError> {
        use crate::errors::feature::ProjectCreateError as Error;
        use authios_sdk::requests::LoggedUserCheckServicePermissionRequest as AuthRequest;
        use authios_sdk::responses::LoggedUserCheckServicePermissionResponse as AuthResponse;

        let mut database_connection = database_connection.acquire()
            .await
            .unwrap();

        let auth_response = authios_client.query()
            .user()
            .logged(params.token.clone())
            .permissions()
            .service()
            .check(AuthRequest {
                service_id: String::from("taskios")
            })
            .await;

        match auth_response {
            AuthResponse::Ok { has_permission } => if !has_permission {
                return Err(Error::Unauthorized)
            },
            AuthResponse::InvalidToken => return Err(Error::InvalidToken),
            AuthResponse::ServerNotAuthios => panic!("AUTH SERVER ERROR: auth server returns invalid responses"),
            AuthResponse::ServerUnavailable => panic!("AUTH SERVER ERROR: auth server shut down"),
            AuthResponse::PermissionNotFound => panic!("AUTH SERVER ERROR: auth server wasn't inited - it's lacking crucial permissions to run this software")
        };
        
        let sql = "INSERT INTO projects (name) VALUES ($1);";
        let _ = sqlx::query(sql)
            .bind(params.name)
            .execute(&mut *database_connection)
            .await;

        Ok(())
    }
    
    pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
        use actix_web::web;

        cfg.service(
            web::resource(Self::path()).route(web::post().to(Self::controller))
        );
    }

    fn path() -> &'static str { "/projects" }
    
    async fn controller(
        body: actix_web::web::Json<Json>,
        token: crate::extractors::TokenExtractor,
        database_connection: actix_web::web::Data<sqlx::PgPool>,
        authios_client: actix_web::web::Data<authios_sdk::AuthiosClient>
    ) -> actix_web::HttpResponse {
        use serde_json::json;
        use actix_web::HttpResponse;
        use crate::errors::feature::ProjectCreateError as Error;

        let result = Self::execute(
            crate::params::feature::ProjectCreateParams {
                name: &body.name,
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
                    .json(json!({ "code": "invalid_token" }))
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct Json {
    pub name: String
}
