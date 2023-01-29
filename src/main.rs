use warp::Filter;

async fn hello_handler() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("Hello World."))
}

#[tokio::main]
async fn main() {
    let hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::end())
        .and_then(hello_handler);

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
