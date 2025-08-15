impl crate::TasksUseCase { 
    pub async fn change_task_completion<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &taskios_domain::params::use_cases::task::ChangeCompletionParams,
        _authios_sdk: std::sync::Arc<authios_sdk::Sdk>,
        client: A
    ) -> Result<(), Error> {
        use taskios_domain::params::repositories::task::{UpdateParams,UpdateNewData};
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


        let update_params = UpdateParams {
            id: params.id.clone(),
            new: UpdateNewData {    
                title: None, 
                description: None, 
                completion: Some(params.completion.clone())
            }
        };
        let result = crate::TasksRepository::update(&update_params, &mut *client)
            .await
            .map_err(|_| Error::ProjectNotExist)?;

        if result.rows_affected() == 0 {
            return Err(Error::ProjectNotExist); 
        }
        
        return Ok(());
    }
}

type Error = taskios_domain::errors::use_cases::tasks::change_completion::Error;
