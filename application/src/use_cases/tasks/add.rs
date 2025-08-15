impl crate::TasksUseCase { 
    pub async fn add<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &taskios_domain::params::use_cases::task::AddParams,
        _authios_sdk: std::sync::Arc<authios_sdk::Sdk>,
        client: A
    ) -> Result<(), Error> {
        use taskios_domain::params::repositories::task::InsertParams;
        use authios_sdk::user::authorize::AuthorizeParams;
 
        let authorize_params = AuthorizeParams { 
            token: params.user_token.clone(), 
            permission: format!("taskios:projects:edit")
        };
        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;


        let insert_params = InsertParams { 
            title: params.title.clone(), 
            description: params.description.clone(), 
            project_id: params.project_id.clone()
        };
        let _ = crate::TasksRepository::insert(&insert_params, &mut *client)
            .await
            .map_err(|_| Error::ProjectNotExist)?;
        
        return Ok(());
    }
}

type Error = taskios_domain::errors::use_cases::tasks::add::Error;
