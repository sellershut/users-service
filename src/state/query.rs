use sellershut_core::{
    google,
    users::{
        QueryUserByApIdRequest, QueryUserByApIdResponse, QueryUserByNameRequest,
        QueryUserByNameResponse, QueryUsersFollowingRequest, QueryUsersFollowingResponse,
        QueryUsersResponse, query_users_server::QueryUsers,
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
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

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
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let resp = QueryUserByNameResponse {
            user: user.map(Into::into),
        };

        Ok(tonic::Response::new(resp))
    }

    #[instrument(skip(self), err(Debug))]
    async fn query_user_by_ap_id(
        &self,
        request: Request<QueryUserByApIdRequest>,
    ) -> Result<Response<QueryUserByApIdResponse>, Status> {
        let id = request.into_inner().ap_id;

        let user = sqlx::query_as!(entity::User, "select * from \"user\" where ap_id = $1", id)
            .fetch_optional(&self.services.postgres)
            .await
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;

        let resp = QueryUserByApIdResponse {
            user: user.map(Into::into),
        };

        Ok(tonic::Response::new(resp))
    }

    #[instrument(skip(self), err(Debug))]
    async fn query_user_following(
        &self,
        request: Request<QueryUsersFollowingRequest>,
    ) -> Result<Response<QueryUsersFollowingResponse>, Status> {
        let id = request.into_inner().ap_id;

        let users = sqlx::query_scalar!(
            "select u.ap_id from \"user\" u where $1 = any(u.followers)",
            id
        )
        .fetch_all(&self.services.postgres)
        .await
        .map_err(|e| tonic::Status::unavailable(e.to_string()))?;

        let resp = QueryUsersFollowingResponse { users };

        Ok(tonic::Response::new(resp))
    }
}
