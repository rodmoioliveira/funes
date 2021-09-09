#[actix_web::main]
async fn main() -> std::io::Result<()> {
    funes::server::new().await
}
