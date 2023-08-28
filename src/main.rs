mod redis;

fn main() {
    let red = redis::RedisClass {};
    red.convert_data_to_json();
}
