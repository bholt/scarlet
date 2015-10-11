extern crate redis;
use redis::Commands;
use redis::{RedisResult, Connection};

fn check(c: &Connection) -> RedisResult<isize> {
    let _ : () = try!(c.set("foo", 42));
    c.get("foo") as RedisResult<isize>
}

fn main() {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let c = client.get_connection().unwrap();
    
    println!("check: {}", check(&c).unwrap());
    println!("success!");
}
