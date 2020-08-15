use crate::{Behavior, FnOnComplete, Node, Status};
use std::collections::VecDeque;

pub struct Repeater {
    pub node: Node,
    pub repeat_for: i32,
    pub current_loop: i32,
    pub status: Status,
    pub on_complete_cb: FnOnComplete,
}

impl Repeater {
    pub fn new(node: Node, repeat_for: i32, on_complete: FnOnComplete) -> Self {
        Self {
            node,
            repeat_for,
            current_loop: 0,
            status: Status::Invalid,
            on_complete_cb: on_complete,
        }
    }
}

impl Behavior for Repeater {
    fn status(&self) -> Status {
        self.status
    }

    fn initialize(&mut self, events: &mut VecDeque<Node>) {
        self.node.borrow_mut().initialize(events);
        self.status = Status::Running
    }

    fn update(&mut self, events: &mut VecDeque<Node>) -> Status {
        let mut node = self.node.borrow_mut();
        let status = node.update(events);
        if status == Status::Running {
            Status::Running
        } else {
            self.current_loop += 1;
            if self.current_loop < self.repeat_for {
                node.initialize(events);
                Status::Running
            } else {
                self.status = status;
                status
            }
        }
    }

    fn on_complete(&mut self, result: Status, events: &mut VecDeque<Node>) {
        self.status = result.clone();
        if let Some(cb) = &mut self.on_complete_cb {
            cb(result, events)
        }
    }
}
