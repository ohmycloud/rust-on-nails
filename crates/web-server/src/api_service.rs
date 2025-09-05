use db::{Pool, queries};
use grpc_api::api::{GetUsersRequest, GetUsersResponse, User};
use tonic::{Request, Response, Status};

use crate::errors::CustomError;

#[derive(Debug, Clone)]
pub struct UsersService {
    pub pool: Pool,
}

#[tonic::async_trait]
impl grpc_api::api::users_server::Users for UsersService {
    async fn get_users(
        &self,
        _request: Request<GetUsersRequest>,
    ) -> Result<Response<GetUsersResponse>, Status> {
        // Get a client from our database pool
        let client = self
            .pool
            .get()
            .await
            .map_err(|e| CustomError::Database(e.to_string()))
            .unwrap();

        // Get the users from the database
        let users: Vec<db::User> = queries::users::get_users()
            .bind(&client)
            .all()
            .await
            .unwrap();

        let users = users
            .into_iter()
            .map(|user| User {
                id: user.id as u32,
                email: user.email,
            })
            .collect();
        let users = GetUsersResponse { users };

        Ok(Response::new(users))
    }
}
