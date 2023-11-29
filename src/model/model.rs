use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Todo {
    pub id: Option<u32>,
    pub label: String,
    pub state: bool
}