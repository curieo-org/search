use crate::collections::{CategoryType, Collection};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCollectionRequest {
  pub name: String,
  pub description: Option<String>,
  pub category: CategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCollectionsRequest {
  pub limit: Option<u8>,
  pub offset: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCollectionsResponse {
  pub collections:Vec<Collection>
}