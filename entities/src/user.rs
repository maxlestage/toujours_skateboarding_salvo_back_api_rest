//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0
use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use sea_orm::DeriveRelation;
use serde::{Deserialize, Serialize};

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Role {
    #[sea_orm(string_value = "Admin")]
    Admin,
    #[sea_orm(string_value = "User")]
    User,
}

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
    pub role: Option<Role>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::data::Entity")]
    Data,
}

impl Related<super::data::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Data.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
