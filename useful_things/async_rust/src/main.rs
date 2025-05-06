use std::future::Future;

async fn borrow_x(x: &str) {
    println!("value: {}", x);
}

/*
fn bad() -> impl Future<Output = ()> {
    let x = String::from("valami");
    async {
        borrow_x(&x).await // ERROR: `x` does not live long enough
    }
}
*/

fn good() -> impl Future<Output = ()> {
    async {
        let x = String::from("valami");
        borrow_x(&x).await
    }
}

#[tokio::main]
async fn main() {
    good().await;
}
