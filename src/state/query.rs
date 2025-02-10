use sellershut_core::{
    google,
    users::{
        query_users_server::QueryUsers, QueryUserByIdRequest, QueryUserByIdResponse,
        QueryUserByNameRequest, QueryUserByNameResponse, QueryUsersResponse,
    },
};
use tonic::{Request, Response, Status};
use tracing::instrument;

use crate::entity;

use super::AppState;

#[tonic::async_trait]
impl QueryUsers for AppState {
    #[instrument(skip(self), err(Debug))]
    async fn query_users(
        &self,
        _request: Request<google::protobuf::Empty>,
    ) -> Result<Response<QueryUsersResponse>, Status> {
        let user = sqlx::query_as!(
            entity::User,
            "select * from \"user\" where local = $1",
            true
        )
        .fetch_all(&self.services.postgres)
        .await
        .map_err(|e| tonic::Status::unavailable(e.to_string()))?;

        let resp = QueryUsersResponse {
            users: user.into_iter().map(Into::into).collect(),
        };

        Ok(tonic::Response::new(resp))
    }

    #[instrument(skip(self), err(Debug))]
    async fn query_user_by_name(
        &self,
        request: Request<QueryUserByNameRequest>,
    ) -> Result<Response<QueryUserByNameResponse>, Status> {
        let username = request.into_inner().username;

        let user = sqlx::query_as!(
            entity::User,
            "select * from \"user\" where username = $1 and local = $2",
            username,
            true
        )
        .fetch_optional(&self.services.postgres)
        .await
        .map_err(|e| tonic::Status::unavailable(e.to_string()))?;

        let resp = QueryUserByNameResponse {
            user: user.map(Into::into),
        };

        Ok(tonic::Response::new(resp))
    }

    #[instrument(skip(self), err(Debug))]
    async fn query_user_by_id(
        &self,
        request: Request<QueryUserByIdRequest>,
    ) -> Result<Response<QueryUserByIdResponse>, Status> {
        let id = request.into_inner().id;

        let user = sqlx::query_as!(entity::User, "select * from \"user\" where ap_id = $1", id)
            .fetch_optional(&self.services.postgres)
            .await
            .map_err(|e| tonic::Status::unavailable(e.to_string()))?;

        let resp = QueryUserByIdResponse {
            user: user.map(Into::into),
        };

        Ok(tonic::Response::new(resp))
    }
}
