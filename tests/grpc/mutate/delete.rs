use anyhow::Result;
use sellershut_core::users::{DeleteUserRequest, QueryUserByIdRequest};
use tonic::IntoRequest;

use crate::helpers::TestApp;

#[sqlx::test(fixtures(path = "../.././fixtures", scripts("users")))]
async fn delete_user(pool: sqlx::PgPool) -> Result<()> {
    let mut app = TestApp::new(pool).await;
    let id = String::from("https://example.com/users/johndoe");

    app.mutate
        .delete_user(DeleteUserRequest { id: id.clone() }.into_request())
        .await?;

    let getter = QueryUserByIdRequest { id }.into_request();

    let response = app.query.query_user_by_id(getter).await?.into_inner().user;

    assert!(response.is_none());

    Ok(())
}
