extern crate redis;

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let _ : () = redis::cmd("SET").arg("dupa").arg(42).query(&mut con).unwrap();
    let dupa : i32 = redis::cmd("GET").arg("dupa").query(&mut con).unwrap();

    println!("{}", dupa);
}
