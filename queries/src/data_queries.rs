use chrono::Local;
use entities::prelude::User;
use entities::*;
use sea_orm::ActiveModelTrait;

use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
// use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

pub async fn create_data(
    db: DatabaseConnection,
    user_input: data::ActiveModel,
) -> Option<data::Model> {
    let mut user_inputed = user_input.clone();

    user_inputed.created_at = sea_orm::ActiveValue::Set(Some(Local::now().to_owned().date_naive()));

    let id = match user_inputed.user_id.clone() {
        sea_orm::ActiveValue::Set(user_id) => user_id, // extract the user ID if it's present
        _ => return None,                              // return early if user ID is not set
    };

    println!("{:#?}", id);

    if User::find()
        .filter(user::Column::Id.eq(id))
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
