use chrono::Local;

use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;

use entities::*;

use entities::prelude::Data;
use entities::prelude::User;

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

pub async fn get_data(db: DatabaseConnection, id: i32) -> Option<data::Model> {
    let selected: Option<data::Model> = Data::find_by_id(id).one(&db).await.expect("Id not found");
    selected
}

pub async fn get_all_data(db: DatabaseConnection) -> Vec<entities::data::Model> {
    let all_data: Vec<data::Model> = Data::find()
        .all(&db)
        .await
        .expect("No data for the moment.");
    all_data
}

pub async fn update_data(
    db: DatabaseConnection,
    id: i32,
    user_input: data::ActiveModel,
) -> Option<data::Model> {
    let data: Option<data::Model> = Data::find_by_id(id).one(&db).await.expect("Not found");

    let mut data: data::ActiveModel = data.unwrap().into();

    let title: String = user_input.title.unwrap();
    let description: String = user_input.description.unwrap();

    data.title = sea_orm::ActiveValue::Set(title);
    data.description = sea_orm::ActiveValue::Set(description);

    let data: data::Model = data.update(&db).await.expect("Not updated");
    Some(data)
}

pub async fn delete_data(db: DatabaseConnection, id: i32) {
    let remove_data: Option<data::Model> = Data::find_by_id(id).one(&db).await.expect("Not Found");
    let remove_data: data::Model = remove_data.unwrap();
    remove_data
        .delete(&db)
        .await
        .expect("Error delete not work.");
}
