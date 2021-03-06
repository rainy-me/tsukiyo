use crate::utils::not_empty;
use crate::{sql::schema::contacts, utils::SlackWebHookPayload};
use chrono::prelude::NaiveDateTime;
use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};
use std::default::Default;
use validator::Validate;

#[derive(Queryable, Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "A new contact")]
pub struct Contact {
    pub id: i32,
    pub title: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub checked: bool,
}

#[derive(
    Debug, Default, Clone, Validate, Insertable, Serialize, Deserialize, GraphQLInputObject,
)]
#[graphql(description = "A new contact input")]
#[table_name = "contacts"]
pub struct ContactInput {
    #[validate(length(max = 100), custom = "not_empty")]
    pub title: String,
    #[validate(length(max = 50), custom = "not_empty")]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub phone: Option<String>,
    #[validate(custom = "not_empty")]
    pub body: String,
}

#[derive(Debug, Default, Insertable, AsChangeset, Serialize, Deserialize, GraphQLInputObject)]
#[table_name = "contacts"]
pub struct ContactUpdate {
    checked: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, GraphQLInputObject)]
pub struct ContactQuery {
    pub search_string: Option<String>,
    pub checked: Option<bool>,
}

impl ContactInput {
    pub fn to_notify_message(self) -> SlackWebHookPayload {
        SlackWebHookPayload::new(format!(
            "*New Contact*
`title:` {}
`name:` {}
`email:` {}
`phone:` {}

```
{}
```
",
            self.title,
            self.name,
            self.email,
            self.phone.unwrap_or("".to_string()),
            self.body,
        ))
    }
}
