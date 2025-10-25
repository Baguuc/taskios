pub struct ProjectUpdateFeature;

impl ProjectUpdateFeature {
    pub async fn execute<'p, A: sqlx::Acquire<'p, Database = sqlx::Postgres>>(
        params: crate::params::feature::ProjectUpdateParams<'p>,
        database_connection: A,
        authios_client: authios_sdk::AuthiosClient
    ) -> Result<(), crate::errors::feature::ProjectUpdateError> {
        use crate::errors::feature::ProjectUpdateError as Error;
        use authios_sdk::requests::LoggedUserCheckServicePermissionRequest as AuthRequest;
        use authios_sdk::responses::LoggedUserCheckServicePermissionResponse as AuthResponse;
        
        if params.new_name.is_none() {
            return Ok(());
        }

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
        
        let sql = "UPDATE projects SET name = $1 WHERE id = $2;";
        let result = sqlx::query(sql)
            .bind(params.new_name.clone().unwrap())
            .execute(&mut *database_connection)
            .await
            .unwrap();

        if result.rows_affected() > 0 { 
            return Ok(()); 
        } else { 
            return Err(Error::ProjectNotFound);
        }
    }
}
