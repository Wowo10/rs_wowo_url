extern crate redis;

mod redis_repo;

fn main() {

    let repo = redis_repo::RedisRepo::create("redis://127.0.0.1/");

    repo.save("dupa2", "15").unwrap();
    let value = repo.read("dupa2").unwrap();
    println!("{}", value);
}