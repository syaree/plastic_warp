use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::get()
        .and(warp::path!("hi" / String))
        .map(|nama| {
            format!("Hi, {}", nama)
        });

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
