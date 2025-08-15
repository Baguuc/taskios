impl crate::ProjectsUseCase {
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &taskios_domain::params::use_cases::project::RetrieveParams,
        _authios_sdk: std::sync::Arc<authios_sdk::Sdk>,
        client: A
    ) -> Result<taskios_domain::Project, Error> {
        use taskios_domain::params::repositories::project::RetrieveParams;
        use authios_sdk::user::authorize::AuthorizeParams;
 
        let authorize_params = AuthorizeParams { 
            token: params.user_token.clone(), 
            permission: format!("taskios:projects:read")
        };
        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let retrieve_params = RetrieveParams {
            id: params.id
        };
        let data = crate::ProjectsRepository::retrieve(&retrieve_params, &mut *client)
            .await
            .map_err(|_| Error::NotExist)?;
        
        return Ok(data);
    }
}

type Error = taskios_domain::errors::use_cases::projects::retrieve::Error;
