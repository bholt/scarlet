extern crate redis;
use redis::Commands;
use redis::{RedisResult, Connection};

macro_rules! argmap(
    { $($key:expr => $value:expr),+ } => { &[ $( ($key, &*$value) ),+ ] };
);

fn check(c: &Connection) -> RedisResult<isize> {
    try!(c.set("foo", 42));
    c.get("foo") as RedisResult<isize>
}

struct BetterClient<'a> { conn: &'a Connection }

impl<'a> BetterClient<'a> {
    fn set<T: Entity>(&self, e: &T) -> RedisResult<()> {
        e.set(&self.conn)
    }
}

trait Entity {
    fn key(i64) -> String;
    fn set(&self, c: &Connection) -> RedisResult<()>;
}

struct User {
    id: i64,
    username: String,
    password: String,
    first: String,
    last: String
}

impl User {
    fn new(id: i64, username: &str, password: &str, first: &str, last: &str) -> User {
        User {
            id: id,
            username: username.to_string(),
            password: password.to_string(),
            first: first.to_string(),
            last: last.to_string()
        }
    }
}

impl Entity for User {
    
    fn key(id: i64) -> String { id.to_string() + ":user" }
    
    fn set(&self, c: &Connection) -> RedisResult<()> {
        c.hset_multiple(Self::key(self.id), argmap!(
            "username" => self.username,
            "password" => self.password,
            "first"    => self.first,
            "last"     => self.last
        ))
    }

}

fn main() {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let c = client.get_connection().unwrap();
    
    println!("check: {}", check(&c).unwrap());
    
    let u = User::new(1, "bholt", ".", "Brandon", "Holt");
    
    println!("user.key => {}", User::key(u.id));

    {    
        let b = BetterClient{ conn: &c };
        b.set(&u).unwrap();
    }
        
    let r : Vec<String> = c.hget(User::key(u.id), &["username","first","last"]).unwrap();
    println!("check user: {:?}", r);
    
    println!("done");
}
