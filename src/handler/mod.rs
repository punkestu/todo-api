use crate::app::service::CreateParam;
use crate::model::model;
use crate::{implement, AppState};
use actix_web::web::{Json, Query};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TodoQuery {
    state: Option<bool>,
}

pub fn crud_todo(cfg: &mut web::ServiceConfig) {
    cfg.service(hello).service(get_todo).service(create_todo);
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/todo")]
async fn get_todo(
    state: web::Data<AppState<implement::repo::Todo>>,
    Query(query): Query<TodoQuery>,
) -> impl Responder {
    let todo_list = state.service.get_todo(query.state);
    HttpResponse::Ok().json(todo_list)
}

#[get("/todo/recent")]
async fn get_recent(state: web::Data<AppState<implement::repo::Todo>>) -> impl Responder {
    let recent_todo = state.service.get_recent();
    HttpResponse::Ok().json(recent_todo)
}

#[derive(Deserialize)]
struct CreateTodo {
    label: String,
}
#[derive(Serialize)]
struct CreatedTodo {
    todo: model::Todo,
}
#[post("/todo")]
async fn create_todo(
    state: web::Data<AppState<implement::repo::Todo>>,
    Json(body): Json<CreateTodo>,
) -> impl Responder {
    let created = state.service.create(CreateParam { label: body.label });
    HttpResponse::Created().json(CreatedTodo { todo: created })
}