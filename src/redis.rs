use redis::Commands;
use std::collections::HashMap;

pub fn get_hash_data_from_redis() -> Result<(), redis::RedisError> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut connection = client.get_connection()?;

    let hash_name = "binance_ETHUSDT"; // put in the name of the key
    let hash_fields: Vec<&str> = vec!["bid", "bidQ", "ask", "askQ", "price"]; // Fetch the hash fields and values
    let mut hash_map = HashMap::new(); // create a new HashMap

    for field in &hash_fields {
        let value: Option<String> = connection.hget(hash_name, field)?;
        if let Some(value_str) = value {
            if let Ok(value_f64) = value_str.parse::<f64>() {
                hash_map.insert(*field, value_f64);
            } else {
                eprintln!("Failed to parse value as f64: {}", value_str);
            }
        }
    }

    // Iterate over the hash fields and values
    for (field, value) in &hash_map {
        println!("{}: {}", field, value);
    }

    Ok(())
}

//TODO:
// Figure out a way to be able to get a hold of multiple dynamic keys being inserted in the db
// Figure out a way to serialize the data into json so that it can be passed to a frontend
