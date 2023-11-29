use crate::app::service::CreateParam;
use crate::model::model;
use crate::{implement, AppState};
use actix_web::web::{Json, Path, Query};
use actix_web::{get, patch, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TodoQuery {
    state: Option<bool>,
}

pub fn crud_todo(cfg: &mut web::ServiceConfig) {
    cfg.service(get_todo)
        .service(get_recent)
        .service(create_todo)
        .service(toggle_state);
}

#[get("")]
async fn get_todo(
    state: web::Data<AppState<implement::repo::Todo>>,
    Query(query): Query<TodoQuery>,
) -> impl Responder {
    let todo_list = state.service.get_todo(query.state);
    HttpResponse::Ok().json(todo_list)
}

#[get("/recent")]
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
#[post("")]
async fn create_todo(
    state: web::Data<AppState<implement::repo::Todo>>,
    Json(body): Json<CreateTodo>,
) -> impl Responder {
    let created = state.service.create(CreateParam { label: body.label });
    HttpResponse::Created().json(CreatedTodo { todo: created })
}

#[patch("/{todo_id}/state")]
async fn toggle_state(
    state: web::Data<AppState<implement::repo::Todo>>,
    todo_id: Path<u32>,
) -> impl Responder {
    let updated_todo = state.service.toggle_state(todo_id.into_inner());
    HttpResponse::Ok().json(updated_todo)
}
