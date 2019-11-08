use actix_web::{App, Responder, HttpServer, web, web::Path};
use failure::Error;
use actix_web_async_await::{compat};
use std::time::{Instant, Duration};
use tokio_timer::delay;

async fn index(seconds: Path<u64>) -> Result<impl Responder, Error> {
    let delay_seconds = seconds.into_inner();
    delay(Instant::now() + Duration::from_secs(delay_seconds)).await;
    Ok(format!("Response after {}", delay_seconds))
}

fn main() {
    let _ = HttpServer::new(|| {
        App::new()
            .service(web::resource("/index/{delay}").route(web::get().to(compat(index))))
    })
        .bind("0.0.0.0:8000")
        .unwrap()
        .run();
}
