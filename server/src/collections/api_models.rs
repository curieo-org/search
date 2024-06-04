use crate::collections::CategoryType;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCollectionRequest {
  pub name: String,
  pub description: Option<String>,
  pub category: CategoryType,
}