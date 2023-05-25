use std::sync::{Arc, Mutex};
use chrono::prelude::*;

use mongodb::Database as MongoDatabase;

use mongodb::{
    bson::{self, doc, oid::ObjectId},
    error::{Error},
    Client,
    options::{ClientOptions,FindOptions}
};

use futures::stream::StreamExt;

use crate::models::alert::Alert;


const DB_NAME: &str = "alerts_db";
const COLL: &str = "alerts";
pub struct Database {
    alerts: Arc<Mutex<MongoDatabase>>,
}

impl Database {
    pub async fn new() -> Result<Self, Error> {
       
        let client_uri = "mongodb://localhost:27017";
        let options = ClientOptions::parse(&client_uri).await?;
                
        let client = Client::with_options(options)?;
        let db = client.database(DB_NAME);
        Ok(Database {
            alerts: Arc::new(Mutex::new(db)),
        })
    }

   

    pub async fn get_alerts(&self) -> Result<Vec<Alert>, Error> {
        let alerts_collection = self.alerts.lock().unwrap().collection(COLL);
        let filter = doc! {};
        let find_options = FindOptions::builder().sort(doc! { "_id": -1}).build();
        let mut cursor = alerts_collection.find(filter, find_options).await?;
    
        let mut alerts = Vec::new();
        while let Some(document_result) = cursor.next().await {
            let document = document_result?;
            let alert: Alert = bson::from_document(document)?;
            alerts.push(alert);
        }
        Ok(alerts)
    }

    pub async fn get_alert_by_id(&self, id: &str) -> Result<Option<Alert>, Error> {
        let alerts_collection = self.alerts.lock().unwrap().collection(COLL);
        let object_id = match ObjectId::parse_str(id) {
            Ok(oid) => oid,
            Err(_) => return Ok(None),
        };
        let filter = doc! { "_id": object_id };
        let alert = alerts_collection.find_one(filter, None).await?;
        match alert {
            Some(doc) => {
                let result: Alert = bson::from_document(doc)?;
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }

    pub async fn create_alert(&self, mut alert: Alert) -> Result<Alert, Error> {
        let alerts_collection = self.alerts.lock().unwrap().collection(COLL);
        let object_id = ObjectId::new();
        alert.id = Some(object_id);
        let created_at = Utc::now();
        let updated_at = Utc::now();
        alert.created_at = Some(created_at);
        alert.updated_at = Some(updated_at);
        let document = bson::to_document(&alert)?;
        alerts_collection.insert_one(document, None).await?;
        Ok(alert)
    }

    pub async fn update_alert_by_id(&self, id: &str, mut alert: Alert) -> Result<Option<Alert>, Error> {
        let alerts_collection = self.alerts.lock().unwrap().collection(COLL);
        let object_id = match ObjectId::parse_str(id) {
            Ok(oid) => oid,
            Err(_) => return Ok(None),
        };
        let filter = doc! { "_id": object_id };
        let updated_at = Utc::now();
        alert.id = Some(object_id);
        alert.updated_at = Some(updated_at);
        let update = doc! { "$set": bson::to_document(&alert)? };
        let result = alerts_collection.find_one_and_update(filter, update, None).await?;
        match result {
            Some(doc) => {
                let result: Alert = bson::from_document(doc)?;
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }

    pub async fn delete_alert_by_id(&self, id: &str) -> Result<Option<Alert>, Error> {
        let alerts_collection = self.alerts.lock().unwrap().collection(COLL);
        let object_id = match ObjectId::parse_str(id) {
            Ok(oid) => oid,
            Err(_) => return Ok(None),
        };
        let filter = doc! { "_id": object_id };
        let result = alerts_collection.find_one_and_delete(filter, None).await?;
        match result {
            Some(doc) => {
                let result: Alert = bson::from_document(doc)?;
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }
}