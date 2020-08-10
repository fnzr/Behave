use crate::nodes::{on_parallel_child_complete, Parallel};
use crate::{Behavior, BehaviorTree, Node, Status};

pub struct ActiveSelector {
    status: Status,
    high_priority: Node,
    low_priority: Parallel,
}

impl Behavior for ActiveSelector {
    fn initialize(&mut self, bt: &mut BehaviorTree, rc: Node) {
        let mut hp = self.high_priority.borrow_mut();
        bt.events
            .push_back((self.high_priority.clone(), Some(rc.clone())));
        hp.initialize(bt, rc);
        self.status = Status::Running;
    }

    fn status(&self) -> &Status {
        &self.status
    }

    fn tick(&mut self) -> &Status {
        &self.status
    }

    fn abort(&mut self) {
        let mut hp = self.high_priority.borrow_mut();
        if hp.status() == &Status::Running {
            hp.abort();
        }
        if self.low_priority.status == Status::Running {
            self.low_priority.abort();
        }
        self.status = Status::Aborted;
    }

    fn on_child_complete(&mut self, status: &Status, bt: &mut BehaviorTree, rc: Node) {
        let high_priority = self.high_priority.borrow();
        if high_priority.status() == &Status::Success {
            self.status = Status::Success;
        } else {
            if self.low_priority.status != Status::Running {
                let child =
                    self.low_priority.children.next().expect(
                        "Paralell node didnt complete but there are no more children to run",
                    );
                bt.events.push_back((child.clone(), Some(rc.clone())));
                child.borrow_mut().initialize(bt, rc);
            } else {
                on_parallel_child_complete(&mut self.low_priority, status, bt, rc.clone());
                if self.low_priority.status == Status::Running {
                    bt.events
                        .push_back((self.high_priority.clone(), Some(rc.clone())));
                    self.high_priority.borrow_mut().initialize(bt, rc.clone());

                    let child = self.low_priority.children.next().expect(
                        "Paralell node didnt complete but there are no more children to run",
                    );
                    bt.events.push_back((child.clone(), Some(rc.clone())));
                    child.borrow_mut().initialize(bt, rc);
                } else {
                    self.status = self.low_priority.status.clone();
                }
            }
        }
    }
}
