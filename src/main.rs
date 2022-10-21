use futures::join;
use tokio::time::{sleep, Duration};

async fn read_kv_a() {
    println!("read kv a start!");
    sleep(Duration::from_secs(1)).await;
    println!("read kv a end!")
}
async fn read_kv_b() {
    println!("read kv b start!");
    sleep(Duration::from_secs(2)).await;
    println!("read kv b end!")
}
async fn read_kv_c() {
    println!("read kv c start!");
    sleep(Duration::from_secs(4)).await;
    println!("read kv c end!")
}
async fn read_kv_d() {
    println!("read kv d start!");
    sleep(Duration::from_secs(1)).await;
    println!("read kv d end!")
}

//       a
//      /  \
//      b  c
//      \  /
//       d
#[tokio::main]
async fn main() {
    read_kv_a().await;
    join!(read_kv_b(), read_kv_c());
    read_kv_d().await;
}
