use sellershut_core::users::{
    mutate_users_server::MutateUsers, CreateUserRequest, CreateUserResponse, DeleteUserRequest,
    DeleteUserResponse, FollowUserRequest, FollowUserResponse, UpsertUserRequest,
    UpsertUserResponse,
};
use tonic::{Request, Response, Status};
use tracing::instrument;

use super::AppState;

#[tonic::async_trait]
impl MutateUsers for AppState {
    #[instrument(skip(self), err(Debug))]
    async fn create_user(
        &self,
        _request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        todo!()
    }

    #[instrument(skip(self), err(Debug))]
    async fn upsert_user(
        &self,
        _request: Request<UpsertUserRequest>,
    ) -> Result<Response<UpsertUserResponse>, Status> {
        todo!()
    }

    #[instrument(skip(self), err(Debug))]
    async fn delete_user(
        &self,
        _request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, tonic::Status> {
        todo!()
    }

    #[instrument(skip(self), err(Debug))]
    async fn follow_user(
        &self,
        _request: Request<FollowUserRequest>,
    ) -> Result<Response<FollowUserResponse>, tonic::Status> {
        todo!()
    }
}
