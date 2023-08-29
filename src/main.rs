mod redis;

fn main() {
    let red = redis::RedisClass {};
    match red.convert_data_to_json() {
        Ok(result) => println!("{}", result),
        Err(error) => println!("{}", error),
    }
}
