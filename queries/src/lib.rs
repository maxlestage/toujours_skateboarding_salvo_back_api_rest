use chrono::Local;
use entities::prelude::*;
use entities::*;

use sea_orm::ActiveModelTrait;
// use sea_orm::ActiveValue;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

use bcrypt::{hash, verify, DEFAULT_COST};

pub async fn create_user(
    db: DatabaseConnection,
    user_input: user::ActiveModel,
) -> Option<user::Model> {
    let mut user_inputed = user_input;

    let user_password = user_inputed.password.as_ref().clone();
    let hashed = hash(&user_password, DEFAULT_COST).unwrap();

    user_inputed.sign_up_date =
        sea_orm::ActiveValue::Set(Some(Local::now().to_owned().date_naive()));

    user_inputed.password = sea_orm::ActiveValue::Set(hashed);

    let user_mail = user_inputed.mail.as_ref().clone();
    if User::find()
        .filter(user::Column::Mail.eq(user_mail))
        .one(&db)
        .await
        .expect("pas gérée")
        .is_some()
    {
        return None;
    }

    let user: user::Model = user_inputed.insert(&db).await.expect("Insertion loupé");
    Some(user)
}

pub async fn delete_user_by_id(db: DatabaseConnection, id: i32) {
    let deleted: DeleteResult = User::delete_by_id(id)
        .exec(&db)
        .await
        .expect("Deletion loupé");
    dbg!(deleted);
}

pub async fn select_user_by_id(db: DatabaseConnection, id: i32) {
    let _select_user: Option<user::Model> =
        User::find_by_id(id).one(&db).await.expect("Select loupé");
    // dbg!(select_user);
}

pub async fn select_user_by_email(db: DatabaseConnection, mail: String) -> Option<user::Model> {
    User::find()
        .filter(user::Column::Mail.eq(mail))
        .one(&db)
        .await
        .expect("pas gérée")
}

pub async fn password_is_valid(user_password: String, actual_password_in_db: String) -> bool {
    verify(&user_password, &actual_password_in_db).unwrap()
}
