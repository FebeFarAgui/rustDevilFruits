use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::model::DevilFruit;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use std::str::FromStr;

pub struct MongoRepo {
    col: Collection<DevilFruit>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(&uri)
            .await
            .expect("Error connecting to mongo");
        let db = client.database("devilfruitAPI");
        let col: Collection<DevilFruit> = db.collection("febefaragui");
        MongoRepo { col }
    }
    pub async fn create_devilfruit(
        &self,
        devilfruit: DevilFruit,
    ) -> Result<InsertOneResult, Error> {
        let new_df = DevilFruit {
            id: None,
            name: devilfruit.name,
            df_type: devilfruit.df_type,
            description: devilfruit.description,
            current_user: devilfruit.current_user,
            image_url: devilfruit.image_url,
        };
        let df = self
            .col
            .insert_one(new_df, None)
            .await
            .ok()
            .expect("Error creating devilfruit");
        Ok(df)
    }
    pub async fn get_all_devilfruits(&self) -> Result<Vec<DevilFruit>, Error> {
        let devilfruits = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting devilfruits");
        let devilfruits = devilfruits
            .filter_map(|item| async move { item.ok() })
            .collect::<Vec<DevilFruit>>()
            .await;
        Ok(devilfruits)
    }
    pub async fn get_devilfruit_by_id(&self, id: String) -> Result<DevilFruit, Error> {
        let devilfruit = self
            .col
            .find_one(
                Some(doc! {
                    "_id": ObjectId::from_str(&id).unwrap()
                }),
                None,
            )
            .await
            .ok()
            .expect("Error getting devilfruit");
        Ok(devilfruit.unwrap())
    }
    pub async fn delete_devilfruit_by_id(&self, id: String) -> Result<DeleteResult, Error> {
        let result = self
            .col
            .delete_one(
                doc! {
                    "_id": ObjectId::from_str(&id).unwrap()
                },
                None,
            )
            .await
            .ok()
            .expect("Error deleting devilfruit");
        Ok(result)
    }
    pub async fn update_devilfruit_by_id(
        &self,
        id: String,
        patch: DevilFruit,
    ) -> Result<UpdateResult, Error> {
        let df = self
            .col
            .update_one(
                doc! {
                    "_id": ObjectId::from_str(&id).unwrap()
                },
                doc! {
                    "$set": {
                        "name": patch.name,
                        "df_type": patch.df_type,
                        "description": patch.description,
                        "current_user": patch.current_user,
                        "image_url": patch.image_url,
                    }
                },
                None,
            )
            .await
            .ok()
            .expect("Error updating devilfruit");
        Ok(df)
    }
}
