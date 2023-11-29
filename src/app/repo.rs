use crate::model::model;

#[derive(Default)]
pub struct GetManyOpt {
    pub state: Option<bool>,
}

#[derive(Default)]
pub struct GetOpt {
    pub id: Option<u32>,
    pub state: Option<bool>,
}

pub trait Todo {
    fn save(&self, todo: model::Todo) -> model::Todo {
        todo
    }
    fn get_many(&self, _opt: GetManyOpt) -> Vec<model::Todo> {
        vec![]
    }
    fn get(&self, _opt: GetOpt) -> model::Todo {
        model::Todo {
            ..Default::default()
        }
    }
}
