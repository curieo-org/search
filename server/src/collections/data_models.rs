use crate::custom_types::DateTime;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::FromRow;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum CategoryType {
  CategoryA,
  CategoryB,
  CategoryC,
}

impl From<i32> for CategoryType {
  fn from(value: i32) -> Self {
      match value {
          0 => CategoryType::CategoryA,
          1 => CategoryType::CategoryB,
          _ => CategoryType::CategoryC,
      }
  }
}

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct Collection {
    pub collection_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: CategoryType,

    pub context: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum CollectionItemType {
  Search,
  Source,
}

impl From<i32> for CollectionItemType {
  fn from(value: i32) -> Self {
      match value {
          0 => CollectionItemType::Search,
          _ => CollectionItemType::Source,
      }
  }
}

#[derive(FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct CollectionItems {
    pub collection_item_id: uuid::Uuid,
    pub collection_id: uuid::Uuid,
    pub item_id: uuid::Uuid,
    pub item_type: CollectionItemType,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}