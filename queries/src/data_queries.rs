pub async fn create_data(
    db: DatabaseConnection,
    user_input: data::ActiveModel,
) -> Option<data::Model> {
}

pub async fn create_user(
    db: DatabaseConnection,
    user_input: user::ActiveModel,
) -> Option<user::Model> {
    let mut user_inputed = user_input;

    let user_password = user_inputed.password.as_ref().clone();
    let hashed = hash(&user_password, DEFAULT_COST).unwrap();

    user_inputed.role = sea_orm::ActiveValue::Set(Role::User);

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
