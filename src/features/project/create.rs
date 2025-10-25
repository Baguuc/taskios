pub struct ProjectCreateFeature;

impl ProjectCreateFeature {
    pub async fn execute<'p, A: sqlx::Acquire<'p, Database = sqlx::Postgres>>(
        params: crate::params::feature::ProjectCreateParams<'p>,
        database_connection: A,
        authios_client: authios_sdk::AuthiosClient
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
}
