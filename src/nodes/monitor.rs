use crate::{Behavior, BehaviorTree, Node, Status};

pub struct Monitor {
    pub conditions: Node,
    pub actions: Node,
    pub status: Status,
}

impl Behavior for Monitor {
    fn initialize(&mut self, bt: &mut BehaviorTree, rc: Node) {
        self.status = Status::Running;
        bt.events
            .push_back((self.conditions.clone(), Some(rc.clone())));
        self.conditions.borrow_mut().initialize(bt, rc);
    }

    fn status(&self) -> &Status {
        &self.status
    }

    fn tick(&mut self) -> &Status {
        &self.status
    }

    fn abort(&mut self) {
        let mut cond = self.conditions.borrow_mut();
        if cond.status() == &Status::Running {
            cond.abort();
        }
        let mut act = self.actions.borrow_mut();
        if act.status() == &Status::Running {
            act.abort();
        }
        self.status = Status::Aborted;
    }

    fn on_child_complete(&mut self, _: &Status, bt: &mut BehaviorTree, rc: Node) {
        let conditions = self.conditions.borrow();
        match conditions.status() {
            &Status::Success => {
                let actions = self.actions.borrow();
                match actions.status() {
                    &Status::Invalid => {
                        bt.events
                            .push_back((self.actions.clone(), Some(rc.clone())));
                        self.actions.borrow_mut().initialize(bt, rc);
                    }
                    action_status => {
                        self.status = action_status.clone();
                    }
                }
            }
            conditions_status => self.status = conditions_status.clone(),
        }
        drop(conditions);
        if self.status != Status::Running {
            self.terminate();
        }
    }
}
