mod redis;

fn main() {
    if let Err(error) = redis::get_hash_data_from_redis() {
        eprintln!("An error occurred: {}", error);
    }
}
