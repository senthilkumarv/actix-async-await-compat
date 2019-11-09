use std::time::{Duration, Instant};

use actix_web::{App, HttpServer, Responder, web, web::Path};
use failure::Error;
use tokio_timer::delay;

use actix_async_await_compat::wrap_async;

async fn index(seconds: Path<u64>) -> Result<impl Responder, Error> {
  let delay_seconds = seconds.into_inner();
  delay(Instant::now() + Duration::from_secs(delay_seconds)).await;
  Ok(format!("Response after {}", delay_seconds))
}

fn main() {
  let _ = HttpServer::new(|| {
    App::new()
        .service(web::resource("/index/{delay}").route(web::get().to(wrap_async(index))))
  })
      .bind("0.0.0.0:8000")
      .unwrap()
      .run();
}
