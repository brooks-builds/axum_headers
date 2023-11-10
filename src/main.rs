use axum_headers::App;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    let port = dotenv!("PORT")
        .parse()
        .expect("PORT Environment variable must be set and a valid number");
    let app = App::new(port);

    app.run().await.expect("App crashed");
}
