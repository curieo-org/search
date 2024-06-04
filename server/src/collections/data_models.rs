use crate::custom_types::DateTime;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::FromRow;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum CategoryType {
  CategoryA = 0,
  CategoryB = 1,
  CategoryC = 2,
}

impl CategoryType {
  /// String value of the enum field names used in the ProtoBuf definition.
  ///
  /// The values are not transformed in any way and thus are considered stable
  /// (if the ProtoBuf definition does not change) and safe for programmatic use.
  pub fn as_str_name(&self) -> &'static str {
      match self {
          CategoryType::CategoryA => "CategoryA",
          CategoryType::CategoryB => "CategoryB",
          CategoryType::CategoryC => "CategoryC",
      }
  }
  /// Creates an enum from field names used in the ProtoBuf definition.
  pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
      match value {
          "CategoryA" => Some(Self::CategoryA),
          "CategoryB" => Some(Self::CategoryB),
          "CategoryC" => Some(Self::CategoryC),
          _ => None,
      }
  }
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

impl From<CategoryType> for i32 {
  fn from(value: CategoryType) -> Self {
      match value {
          CategoryType::CategoryA => 0,
          CategoryType::CategoryB => 1,
          CategoryType::CategoryC => 2,
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