use std::i32;

fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1").expect("error connecting to Redis");
    let mut con = client
        .get_connection()
        .expect("error in getting the client");

    let v: String = redis::cmd("GET")
        .arg("mylist")
        .query(&mut con)
        .expect("error querying redis");

    let x: Vec<i32> = v
        .replace("[", "")
        .replace("]", "")
        .replace(",", "")
        .split_whitespace()
        .map(|c| c.parse().expect("error parsing value"))
        .collect();

    let _: () = redis::cmd("SET")
        .arg("mylist2")
        .arg(454)
        .query(&mut con)
        .expect("error inserting redis");

    let _: () = redis::cmd("INCR")
        .arg("mylist2")
        .query(&mut con)
        .expect("error getting value redis");

    let u: i32 = redis::cmd("GET")
        .arg("mylist2")
        .query(&mut con)
        .expect("error getting value redis");

    let _: () = redis::cmd("DEL")
        .arg("mylist2")
        .query(&mut con)
        .expect("error deleting item");

    let _: () = redis::cmd("GET")
        .arg("mylist22")
        .query(&mut con)
        .expect("error getting value redis");

    x.into_iter().for_each(|c| println!("{c}"));
    println!("printing value of : {u}");
    // println!("printing value of : {y}");
    Ok(())
}
