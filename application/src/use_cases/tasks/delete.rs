impl crate::TasksUseCase {
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &taskios_domain::params::use_cases::task::DeleteParams,
        _authios_sdk: std::sync::Arc<authios_sdk::Sdk>,
        client: A
    ) -> Result<(), Error> {
        use taskios_domain::params::repositories::task::DeleteParams;
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


        let delete_params = DeleteParams {
            id: params.id
        };
        let result = crate::TasksRepository::delete(&delete_params, &mut *client)
            .await
            .map_err(|_| Error::NotAddedYet)?;

        if result.rows_affected() == 0 {
            return Err(Error::NotAddedYet); 
        }
        
        return Ok(());
    }
}

type Error = taskios_domain::errors::use_cases::tasks::delete::Error;
