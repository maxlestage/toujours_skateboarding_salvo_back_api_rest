use chrono::Local;
use entities::*;
use sea_orm::ActiveModelTrait;
// use sea_orm::ActiveValue;
use entities::prelude::User;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

pub async fn create_data(
    db: DatabaseConnection,
    user_input: data::ActiveModel,
) -> Option<data::Model> {
    let mut user_inputed = user_input;

    user_inputed.created_at = sea_orm::ActiveValue::Set(Some(Local::now().to_owned().date_naive()));

    let user_id = user_inputed.user_id.as_ref().clone();
    if User::find()
        .filter(user::Column::Id.eq(user_id))
        .one(&db)
        .await
        .expect("pas gérée")
        .is_some()
    {
        return None;
    }

    let create_data: data::Model = user_inputed.insert(&db).await.expect("Insertion loupé");
    Some(create_data)
}
