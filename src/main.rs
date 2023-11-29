mod app;
mod handler;
mod implement;
mod model;

use crate::handler::crud_todo;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::rc::Rc;

pub struct AppState<Repo: app::repo::Todo> {
    pub(crate) service: Rc<app::service::Todo<Repo>>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let todo_repo = Rc::new(implement::repo::Todo::new());
        let todo_service = Rc::new(app::service::Todo::new(todo_repo.clone()));
        App::new()
            .app_data(web::Data::new(AppState {
                service: todo_service,
            }))
            .service(hello)
            .service(web::scope("/todo").configure(crud_todo))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
