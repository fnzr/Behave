use crate::{Behavior, FnOnComplete, Node, Status};
use std::collections::VecDeque;
pub struct Action {
    pub status: Status,
    pub update: Box<dyn FnMut() -> Status>,
    pub on_complete_cb: FnOnComplete,
}

impl Action {
    pub fn new<T>(update: Box<T>, on_complete_cb: FnOnComplete) -> Self
    where
        T: FnMut() -> Status + Copy + 'static,
    {
        Self {
            update,
            on_complete_cb,
            status: Status::Invalid,
        }
    }
}

impl Behavior for Action {
    fn status(&self) -> Status {
        self.status
    }

    fn update(&mut self, _: &mut VecDeque<Node>) -> Status {
        self.status = (self.update)();
        self.status.clone()
    }

    fn on_complete(&mut self, result: Status, events: &mut VecDeque<Node>) {
        if let Some(cb) = &mut self.on_complete_cb {
            cb(result, events)
        }
    }
}
