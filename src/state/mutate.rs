use crate::{entity, utils::generate_id};
use sellershut_core::users::{
    mutate_users_server::MutateUsers, CreateUserRequest, CreateUserResponse, DeleteUserRequest,
    DeleteUserResponse, FollowUserRequest, FollowUserResponse, UpsertUserRequest,
    UpsertUserResponse, User,
};
use tonic::{Request, Response, Status};
use tracing::instrument;

use super::AppState;

#[tonic::async_trait]
impl MutateUsers for AppState {
    #[instrument(skip(self), err(Debug))]
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let data = request.into_inner().user;
        let id = generate_id();

        let user = sqlx::query_as!(
            entity::User,
            "insert into \"user\" (id, username, followers, avatar_url, inbox, public_key, private_key, local, email, display_name, ap_id)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) returning *",
                id,
            &data.username,
            &data.followers,
            data.avatar_url,
            &data.inbox,
            &data.public_key,
            data.private_key.as_deref(),
            &data.local,
            data.email,
            data.display_name,
            &data.ap_id,
        )
        .fetch_one(&self.services.postgres)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let user = User::from(user);

        let resp = CreateUserResponse { user };

        Ok(Response::new(resp))
    }

    #[instrument(skip(self), err(Debug))]
    async fn upsert_user(
        &self,
        request: Request<UpsertUserRequest>,
    ) -> Result<Response<UpsertUserResponse>, Status> {
        let data = request.into_inner().user;
        let id = generate_id();

        let user = sqlx::query_as!(
            entity::User,
            "insert into \"user\" (id, username, followers, avatar_url, inbox, public_key, private_key, local, email, display_name, ap_id)
                values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                on conflict (ap_id)
                do update 
                set username = excluded.username,
                    followers = excluded.followers,
                    avatar_url = excluded.avatar_url,
                    inbox = excluded.inbox,
                    public_key = excluded.public_key,
                    private_key = excluded.private_key,
                    id = excluded.id,
                    local = excluded.local
                returning *",
            id,
            &data.username,
            &data.followers,
            data.avatar_url,
            &data.inbox,
            &data.public_key,
            data.private_key.as_deref(),
            &data.local,
            data.email,
            data.display_name,
            &data.ap_id,
        )
        .fetch_one(&self.services.postgres)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let user = User::from(user);

        let req = UpsertUserResponse { user };

        Ok(Response::new(req))
    }

    #[instrument(skip(self), err(Debug))]
    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, tonic::Status> {
        let id = request.into_inner().id;

        let user = sqlx::query_as!(
            entity::User,
            "delete from \"user\" where ap_id = $1 returning *",
            &id
        )
        .fetch_one(&self.services.postgres)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let user = User::from(user);

        let resp = DeleteUserResponse { user };

        Ok(Response::new(resp))
    }

    #[instrument(skip(self), err(Debug))]
    async fn follow_user(
        &self,
        request: Request<FollowUserRequest>,
    ) -> Result<Response<FollowUserResponse>, tonic::Status> {
        let data = request.into_inner();

        let user = sqlx::query_as!(
            entity::User,
            "update \"user\" set followers = array_append(followers, $1)
            where ap_id = $2 and not $1 = any(followers)
            returning *",
            &data.follow_url,
            &data.ap_id,
        )
        .fetch_one(&self.services.postgres)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let user = User::from(user);

        let req = FollowUserResponse { user };

        Ok(Response::new(req))
    }
}
