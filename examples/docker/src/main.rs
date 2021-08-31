use funes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Ok(funes::server::new().await?)
}
