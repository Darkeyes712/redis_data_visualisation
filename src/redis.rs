use redis::{Commands, Connection, RedisResult};
use serde::ser::Error;
use serde_json::{json, Error as JsonError, Value};
use std::collections::HashMap;

trait Redis {
    fn create_connection(&self) -> Result<Connection, redis::RedisError>;
}

pub struct RedisClass {}

impl Redis for RedisClass {
    fn create_connection(&self) -> Result<Connection, redis::RedisError> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let connection = client.get_connection()?;

        Ok(connection)
    }
}

impl RedisClass {
    pub fn get_all_keys(&self) -> RedisResult<Vec<String>> {
        let mut connection = self.create_connection()?;
        let keys: Vec<String> = connection.scan_match("*")?.collect();

        Ok(keys)
    }

    pub fn fetch_data_for_hash_keys(
        &self,
    ) -> RedisResult<HashMap<String, HashMap<String, String>>> {
        let mut connection = self.create_connection()?;
        let hash_keys = self.get_all_keys()?;

        let mut data_map: HashMap<String, HashMap<String, String>> = HashMap::new();

        for hash_key in hash_keys {
            let hash_data: HashMap<String, String> = connection.hgetall(&hash_key)?;
            data_map.insert(hash_key, hash_data);
        }

        println!("{:?}", data_map);
        Ok(data_map)
    }

    pub fn convert_data_to_json(&self) -> Result<String, JsonError> {
        match self.fetch_data_for_hash_keys() {
            Ok(data) => {
                let mut json_map: HashMap<String, Value> = HashMap::new();

                for (key, value) in data {
                    json_map.insert(key, json!(value));
                }

                let json_result = serde_json::to_string(&json_map)?;
                Ok(json_result)
            }
            Err(err) => Err(JsonError::custom(format!("Redis error: {}", err))),
        }
    }
}

//TODO:
// Figure out a way to be able to get a hold of multiple dynamic keys being inserted in the db - Done
// Figure out a way to get all data for all keys and loop through it - Done
// Figure out how to group all data (data from each hash that we get from Redis) and output it as json - Done
// Build the API
// Find a way to pass the data to the frontend
