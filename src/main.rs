mod checker;


use actix::prelude::*;

use actix_web::{App, HttpServer};

use mimalloc::MiMalloc;

#[global_allocator]
/// Alternative allocator which should better perform with musl
static MI_MALLOC: MiMalloc = MiMalloc;

/// Entrypoint for the application
#[actix_web::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>>{
    let _checker = checker::Checker::from_registry();

    HttpServer::new(move || App::new())
        .bind("0.0.0.0:8080")?
        .run().await?;
    Ok(())
}


