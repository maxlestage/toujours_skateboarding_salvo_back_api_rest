use chrono::Local;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

use entities::*;
// use sea_orm::DeleteResult;
use entities::prelude::Data;
use entities::prelude::User;

//use entities::user::Relation::Data;

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
        let create_data: data::Model = user_inputed.insert(&db).await.expect("Insertion loupé");
        Some(create_data)
    } else {
        return None;
    }
}

pub async fn get_data(db: DatabaseConnection, user_input: i32) -> Option<data::Model> {
    let selected: Option<data::Model> = Data::find_by_id(user_input)
        .one(&db)
        .await
        .expect("not found");

    selected
}
