use sellershut_core::{
    google,
    users::{
        query_users_server::QueryUsers, QueryUserByIdRequest, QueryUserByIdResponse,
        QueryUserByNameRequest, QueryUserByNameResponse, QueryUsersResponse,
    },
};
use tonic::{Request, Response, Status};
use tracing::instrument;

use super::AppState;

#[tonic::async_trait]
impl QueryUsers for AppState {
    #[instrument(skip(self), err(Debug))]
    async fn query_users(
        &self,
        _request: Request<google::protobuf::Empty>,
    ) -> Result<Response<QueryUsersResponse>, Status> {
        todo!()
    }

    #[instrument(skip(self), err(Debug))]
    async fn query_user_by_name(
        &self,
        _request: Request<QueryUserByNameRequest>,
    ) -> Result<Response<QueryUserByNameResponse>, Status> {
        todo!()
    }

    #[instrument(skip(self), err(Debug))]
    async fn query_user_by_id(
        &self,
        _request: Request<QueryUserByIdRequest>,
    ) -> Result<Response<QueryUserByIdResponse>, Status> {
        todo!()
    }
}
