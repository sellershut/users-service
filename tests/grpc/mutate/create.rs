use anyhow::Result;
use sellershut_core::users::{CreateUserRequest, QueryUserByIdRequest, User};
use sqlx::PgPool;
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("users")))]
async fn create_user(pool: PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;

    let mut user = User::default();
    user.username = "test_username".to_string();

    let user_request = CreateUserRequest { user };

    let response = app
        .mutate
        .create_user(user_request.into_request())
        .await?
        .into_inner()
        .user
        .ap_id;

    let getter = QueryUserByIdRequest { id: response }.into_request();

    let response = app.query.query_user_by_id(getter).await?.into_inner().user;

    assert!(response.is_some());

    Ok(())
}
