mod redis_repo;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let d = args.last().unwrap();

    println!("{:?}", args);

    let repo = redis_repo::RedisRepo::create("redis://127.0.0.1/");

    let list = repo.list_all();
    // for pair in list {
    //     println!("{} : {}", pair.0, pair.1)
    // }

    &list
        .iter()
        .for_each(|pair| println!("{} : {}", pair.0, pair.1));

    // repo.save("dupa2", "15").unwrap();
    // let value = repo.read("dupa2").unwrap();
    // println!("{}", value);
}
