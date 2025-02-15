use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct User {
    pub id: String,
    pub ap_id: String,
    #[cfg_attr(test, dummy(faker = "fake::faker::internet::en::Username()"))]
    pub username: String,
    pub avatar_url: Option<String>,
    #[cfg_attr(test, dummy(faker = "fake::faker::name::en::Name()"))]
    pub display_name: Option<String>,
    #[cfg_attr(test, dummy(faker = "fake::faker::internet::en::FreeEmail()"))]
    pub email: Option<String>,
    pub followers: Followers,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub last_refreshed_at: OffsetDateTime,
    pub local: bool,
    pub private_key: Option<String>,
    pub public_key: String,
    pub inbox: String,
    pub outbox: String,
    pub summary: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct Followers {
    col: HashSet<String>,
}

impl From<Vec<std::string::String>> for Followers {
    fn from(value: Vec<std::string::String>) -> Self {
        Self {
            col: value.into_iter().collect::<HashSet<_>>(),
        }
    }
}

impl From<User> for sellershut_core::users::User {
    fn from(value: User) -> Self {
        Self {
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
            followers: value.followers.col.into_iter().collect::<Vec<_>>(),
            id: value.id,
            ap_id: value.ap_id,
            avatar_url: value.avatar_url,
            display_name: value.display_name,
            email: value.email,
            username: value.username,
            last_refreshed_at: value.last_refreshed_at.into(),
            local: value.local,
            inbox: value.inbox.to_string(),
            private_key: value.private_key,
            public_key: value.public_key,
            outbox: value.outbox,
            summary: value.summary,
        }
    }
}

impl TryFrom<sellershut_core::users::User> for User {
    type Error = anyhow::Error;

    fn try_from(value: sellershut_core::users::User) -> anyhow::Result<Self> {
        let created = value.created_at;
        let updated = value.updated_at;
        Ok(Self {
            id: value.id,
            ap_id: value.ap_id,
            username: value.username,
            avatar_url: value.avatar_url,
            email: value.email,
            display_name: value.display_name,
            followers: Followers {
                col: value.followers.into_iter().collect::<HashSet<_>>(),
            },
            created_at: created.try_into()?,
            updated_at: updated.try_into()?,
            inbox: value.inbox,
            last_refreshed_at: value.last_refreshed_at.try_into()?,
            local: value.local,
            private_key: value.private_key,
            public_key: value.public_key,
            outbox: value.outbox,
            summary: value.summary,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};
    use sellershut_core::google::protobuf::Timestamp;
    use time::macros::datetime;

    // Test conversion from User to sellershut_core::users::User
    #[test]
    fn test_user_to_proto_user_conversion() {
        let user: User = Faker.fake();
        let proto_user: sellershut_core::users::User = user.clone().into();

        assert_eq!(user.id, proto_user.id);
        assert_eq!(user.ap_id, proto_user.ap_id);
        assert_eq!(user.username, proto_user.username);
        assert_eq!(user.avatar_url, proto_user.avatar_url);
        assert_eq!(user.display_name, proto_user.display_name);
        assert_eq!(user.email, proto_user.email);
        assert_eq!(user.followers.col.len(), proto_user.followers.len());
        assert_eq!(Timestamp::from(user.created_at), proto_user.created_at);
        assert_eq!(Timestamp::from(user.updated_at), proto_user.updated_at);
        assert_eq!(
            Timestamp::from(user.last_refreshed_at),
            proto_user.last_refreshed_at
        );
        assert_eq!(user.local, proto_user.local);
        assert_eq!(user.inbox, proto_user.inbox);
        assert_eq!(user.private_key, proto_user.private_key);
        assert_eq!(user.public_key, proto_user.public_key);
    }

    // Test conversion from sellershut_core::users::User to User
    #[test]
    fn test_proto_user_to_user_conversion() {
        let proto_user = sellershut_core::users::User {
            created_at: datetime!(2023-01-01 12:00:00.000 UTC).into(),
            updated_at: datetime!(2023-01-02 12:00:00.000 UTC).into(),
            followers: vec!["follower1".to_string(), "follower2".to_string()],
            id: "user_id".to_string(),
            ap_id: "ap_id".to_string(),
            avatar_url: Some("http://avatar.url".to_string()),
            display_name: Some("Display Name".to_string()),
            email: Some("user@example.com".to_string()),
            username: "username".to_string(),
            last_refreshed_at: datetime!(2023-01-03 12:00:00.000 UTC).into(),
            local: true,
            inbox: "inbox_value".to_string(),
            outbox: "outbox_value".to_string(),
            summary: None,
            private_key: Some("private_key_value".to_string()),
            public_key: "public_key_value".to_string(),
        };

        let user: User = proto_user
            .clone()
            .try_into()
            .expect("Conversion should succeed");

        assert_eq!(proto_user.id, user.id);
        assert_eq!(proto_user.ap_id, user.ap_id);
        assert_eq!(proto_user.username, user.username);
        assert_eq!(proto_user.avatar_url, user.avatar_url);
        assert_eq!(proto_user.display_name, user.display_name);
        assert_eq!(proto_user.email, user.email);
        assert_eq!(proto_user.followers.len(), user.followers.col.len());
        assert_eq!(proto_user.created_at, user.created_at.into());
        assert_eq!(proto_user.updated_at, user.updated_at.into());
        assert_eq!(proto_user.last_refreshed_at, user.last_refreshed_at.into());
        assert_eq!(proto_user.local, user.local);
        assert_eq!(proto_user.outbox, user.outbox);
        assert_eq!(proto_user.summary, user.summary);
        assert_eq!(proto_user.inbox, user.inbox);
        assert_eq!(proto_user.private_key, user.private_key);
        assert_eq!(proto_user.public_key, user.public_key);
    }
}
