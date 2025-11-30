use entity::user;
use iroh::SecretKey;

use crate::{error::Result, key::hash_password};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, EntityTrait, FromQueryResult, InsertResult, IntoActiveModel,
    QueryFilter, QuerySelect, RelationTrait, TryIntoModel,
};
use sea_orm::{Database, DatabaseConnection, DbErr};
pub async fn create_user(db: DatabaseConnection) -> Result<entity::user::Model> {
    let name = inquire::Text::new("Enter your name:").prompt()?;
    let phone_str = inquire::Text::new("Enter phone number:").prompt()?;
    let phone: i64 = phone_str.parse()?;
    let password_input = inquire::Password::new("Enter password").prompt()?;
    let password = hash_password(&password_input);
    let secret_key = SecretKey::generate(&mut rand::rng());
    let mut user_model = entity::user::ActiveModel {
        name: Set(name),
        phone_no: Set(phone),
        password: Set(password),
        admin: Set(false),
        ..Default::default()
    };

    let res = user::Entity::insert(user_model.clone()).exec(&db).await?;
    let id = res.last_insert_id;
    user_model.id = Set(id);
    Ok(user_model.try_into_model()?)
}
