use crate::app::repo;
use crate::app::repo::{GetManyOpt, GetOpt};
use crate::model::model;

pub struct Todo {}

impl Todo {
    pub fn new() -> Self {
        Self {}
    }
}

impl repo::Todo for Todo {
    fn save(&self, mut todo: crate::model::model::Todo) -> model::Todo {
        todo.id = Some(1);
        todo
    }
    fn get_many(&self, _opt: GetManyOpt) -> Vec<model::Todo> {
        if let Some(state) = _opt.state {
            return if state {
                vec![model::Todo {
                    id: Some(1),
                    label: String::from("test completed"),
                    state: true,
                }]
            } else {
                vec![model::Todo {
                    id: Some(2),
                    label: String::from("test not completed"),
                    state: false,
                }]
            }
        }
        vec![]
    }
    fn get(&self, _opt: GetOpt) -> model::Todo {
        println!("get");
        model::Todo {
            ..Default::default()
        }
    }
}
