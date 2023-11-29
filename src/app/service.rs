use std::rc::Rc;
use crate::app::repo::GetOpt;
use crate::model::model;
use super::repo;

pub struct Todo<T: repo::Todo> {
    repo: Rc<T>,
}

impl<T: repo::Todo> Todo<T> {
    pub fn new(repo: Rc<T>) -> Self {
        Todo { repo }
    }
    pub fn create(&self, param: CreateParam) -> model::Todo {
        self.repo.save(model::Todo{
            label: param.label,
            ..Default::default()
        })
    }
    pub fn get_todo(&self, state: Option<bool>) -> Vec<model::Todo> {
        self.repo.get_many(repo::GetManyOpt{
            state
        })
    }
    pub fn get_recent(&self) -> model::Todo {
        self.repo.get(GetOpt{
            state: Some(false),
            sort_by_id: Some(true),
            ..Default::default()
        })
    }
    pub fn toggle_state(&self, id: u32) -> model::Todo {
        let mut updated_todo = self.repo.get(GetOpt{
            id: Some(id),
            ..Default::default()
        });
        updated_todo.state = !updated_todo.state;
        self.repo.save(updated_todo)
    }
}

pub struct CreateParam {
    pub label: String
}