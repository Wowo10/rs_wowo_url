extern crate redis;

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    save(&mut con, "dupa", "43").unwrap();
    let dupa : i32 = redis::cmd("GET").arg("dupa").query(&mut con).unwrap();

    println!("{}", dupa);
}

fn save(con: &mut redis::Connection, key: &str, value: &str) -> redis::RedisResult<()> {
    let _ : () = redis::cmd("SET").arg(key).arg(value).query(con)?;
    Ok(())
}