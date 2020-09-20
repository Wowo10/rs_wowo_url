extern crate redis;

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    save(&mut con, "dupa", "44").unwrap();
    let dupa_value = read(&mut con, "dupa").unwrap();

    println!("{}", dupa_value);
}

fn save(con: &mut redis::Connection, key: &str, value: &str) -> redis::RedisResult<()> {
    let _ : () = redis::cmd("SET").arg(key).arg(value).query(con)?;
    Ok(())
}

fn read(con: &mut redis::Connection, key: &str) -> redis::RedisResult<String> {
    let value: String = redis::cmd("GET").arg(key).query(con)?;
    Ok(value)
}