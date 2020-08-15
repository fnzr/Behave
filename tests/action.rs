extern crate behave;
use behave::*;
use std::collections::VecDeque;

pub struct CallCounterAction {
    pub call_count: i32,
    status: Status,
    on_complete_cb: FnOnComplete,
    result: Status,
}

impl CallCounterAction {
    pub fn new(result: Status) -> Self {
        Self {
            call_count: 0,
            status: Status::Invalid,
            on_complete_cb: None,
            result,
        }
    }
}

impl Behavior for CallCounterAction {
    fn initialize(&mut self, _: &mut VecDeque<Node>) {
        self.status = Status::Running
    }

    fn status(&self) -> Status {
        Status::Running
    }

    fn update(&mut self, _: &mut VecDeque<Node>) -> Status {
        self.call_count += 1;
        self.status = self.result;
        self.result
    }

    fn on_complete(&mut self, result: Status, events: &mut VecDeque<Node>) {
        if let Some(cb) = &mut self.on_complete_cb {
            cb(result, events)
        }
    }
}
impl CustomBehavior for CallCounterAction {
    fn set_on_complete(&mut self, on_complete: FnOnComplete) {
        self.on_complete_cb = on_complete;
    }
}
