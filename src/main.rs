mod redis_repo;
use std::collections::VecDeque;
use std::env;

extern crate nanoid;
use nanoid::nanoid;

fn main() {
    let mut args: VecDeque<String> = env::args().skip(1).collect(); //skipping path arg

    println!("{:?}", args);

    let repo = redis_repo::RedisRepo::create("redis://127.0.0.1/");

    match &args.pop_front().unwrap()[..] {
        "s" | "save" => {

            let guid = nanoid!();
            let url = args.pop_front().unwrap();

            repo.save(&guid, &url).unwrap();

            println!("Saved as {}", guid);
            println!("Try -- open <hash>");
        },
        "list" | _ => {
            let list = repo.list_all();

            &list
                .iter()
                .for_each(|pair| println!("{} : {}", pair.0, pair.1));
        }
    }
}
