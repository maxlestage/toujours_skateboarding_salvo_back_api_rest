//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0
use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
// use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub sign_up_date: Option<NaiveDate>,
    pub mail: String,
    pub password: String,
    pub role: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
