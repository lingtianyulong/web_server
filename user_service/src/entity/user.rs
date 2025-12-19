use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
#[allow(dead_code)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub user_name: String,
    pub password: String,
    pub create_time: NaiveDateTime,
    pub update_time: Option<NaiveDateTime>,
    pub delete_time: Option<NaiveDateTime>,
    pub unregistered: i32,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
