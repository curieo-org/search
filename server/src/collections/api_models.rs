use crate::collections::data_models;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCollectionRequest {
  pub name: String,
  pub description: Option<String>,
  pub category: data_models::CategoryType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOneCollectionRequest {
  pub collection_id: uuid::Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCollectionsRequest {
  pub limit: Option<u8>,
  pub offset: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCollectionsResponse {
  pub collections:Vec<data_models::Collection>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCollectionRequest {
  pub collection_id: uuid::Uuid,
  pub name: Option<String>,
  pub description: Option<String>,
  pub category: Option<data_models::CategoryType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteCollectionRequest {
  pub collection_id: uuid::Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionItem {
  pub item_id: uuid::Uuid,
  pub item_type:data_models::CollectionItemType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddItemsToCollectionRequest {
  pub collection_id: uuid::Uuid,
  pub items: Vec<CollectionItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetItemsFromCollectionRequest {
  pub collection_id: uuid::Uuid,
  pub item_type: data_models::CollectionItemType,
  pub limit: Option<u8>,
  pub offset: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetItemsFromCollectionResponse {
  pub items: Vec<data_models::CollectionItem>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteItemsFromCollectionRequest {
  pub collection_id: uuid::Uuid,
  pub items: Vec<uuid::Uuid>,
}





