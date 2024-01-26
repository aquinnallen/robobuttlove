#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};

mod ass_handler;
mod response;

#[actix_rt::main]
async fn main() -> io::Result<()> {
  env::set_var("RUST_LOG", "actix_web=debug,actix+server=info");
  env_logger::init();

  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())

      .service(ass_handler::query)
      .service(ass_handler::ass_blast)
  })
  .bind("0.0.0.0:666")?
  .run()
  .await()
}

