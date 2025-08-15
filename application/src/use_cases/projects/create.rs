impl crate::ProjectsUseCase {
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &taskios_domain::params::use_cases::project::CreateParams,
        _authios_sdk: std::sync::Arc<authios_sdk::Sdk>,
        client: A
    ) -> Result<(), Error> {
        use taskios_domain::params::repositories::project::InsertParams;
        use authios_sdk::user::authorize::AuthorizeParams;
 
        let authorize_params = AuthorizeParams { 
            token: params.user_token.clone(), 
            permission: format!("taskios:projects:create")
        };
        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;


        let insert_params = InsertParams { 
            name: params.name.clone()
        };
        let _ = crate::ProjectsRepository::insert(&insert_params, &mut *client)
            .await
            .map_err(|_| Error::AlreadyExist)?;
        
        return Ok(());
    }
}

type Error = taskios_domain::errors::use_cases::projects::create::Error;
