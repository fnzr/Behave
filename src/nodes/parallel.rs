use crate::nodes::{Behavior, ChildrenNodes, Node, Status};
use crate::BehaviorTree;
#[derive(PartialEq)]
pub enum ParallelPolicy {
    One,
    OneDelayed,
    All,
}

pub struct Parallel {
    pub children: ChildrenNodes,
    pub status: Status,
    pub success_policy: ParallelPolicy,
    pub failure_policy: ParallelPolicy,
    pub success_count: usize,
    pub failure_count: usize,
}

fn on_parallel_child_complete(
    parent: &mut Parallel,
    status: &Status,
    bt: &mut BehaviorTree,
    parent_rc: Node,
) {
    match status {
        &Status::Success => parent.success_count += 1,
        _ => parent.failure_count += 1,
    }
    if parent.failure_policy == ParallelPolicy::One && parent.failure_count > 0 {
        parent.status = Status::Failure;
    } else if parent.success_policy == ParallelPolicy::One && parent.success_count > 0 {
        parent.status = Status::Success
    } else if let Some(child) = parent.children.next() {
        bt.events
            .push_back((child.clone(), Some(parent_rc.clone())));
        child.borrow_mut().initialize(bt, parent_rc);
    } else {
        let len = parent.children.nodes.len();
        if (parent.failure_policy == ParallelPolicy::OneDelayed && parent.failure_count > 0)
            || (parent.failure_policy == ParallelPolicy::All && parent.failure_count == len)
        {
            parent.status = Status::Failure;
        } else if (parent.success_policy == ParallelPolicy::OneDelayed && parent.success_count > 0)
            || (parent.success_policy == ParallelPolicy::All && parent.success_count == len)
        {
            parent.status = Status::Success;
        }
    }
}

impl Behavior for Parallel {
    fn status(&self) -> &Status {
        &self.status
    }

    fn tick(&mut self) -> &Status {
        &self.status
    }

    fn initialize(&mut self, bt: &mut BehaviorTree, self_rc: Node) {
        self.children.reset();
        self.success_count = 0;
        self.failure_count = 0;
        if let Some(child) = self.children.next() {
            bt.events.push_back((child.clone(), Some(self_rc.clone())));
            child.borrow_mut().initialize(bt, self_rc);
            self.status = Status::Running;
        } else {
            self.status = Status::Failure;
        }
    }

    fn on_child_complete(&mut self, status: &Status, bt: &mut BehaviorTree, self_rc: Node) {
        on_parallel_child_complete(self, status, bt, self_rc);
        if self.status != Status::Running {
            self.terminate()
        }
    }

    fn terminate(&mut self) {}

    fn abort(&mut self) {
        for node_rc in self.children.nodes.iter() {
            let mut node = node_rc.borrow_mut();
            if node.status() == &Status::Running {
                node.abort();
            }
        }
        self.status = Status::Aborted
    }
}

pub struct Monitor {
    pub conditions: Node,
    pub actions: Node,
    pub status: Status,
}

impl Behavior for Monitor {
    fn initialize(&mut self, bt: &mut BehaviorTree, self_rc: Node) {
        self.status = Status::Running;
        bt.events
            .push_back((self.conditions.clone(), Some(self_rc.clone())));
        self.conditions.borrow_mut().initialize(bt, self_rc);
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

    fn on_child_complete(&mut self, status: &Status, bt: &mut BehaviorTree, self_rc: Node) {
        let conditions = self.conditions.borrow();
        match conditions.status() {
            &Status::Success => {
                let actions = self.actions.borrow();
                match actions.status() {
                    &Status::Invalid => {
                        bt.events
                            .push_back((self.actions.clone(), Some(self_rc.clone())));
                        self.actions.borrow_mut().initialize(bt, self_rc);
                    }
                    &Status::Running => {
                        bt.events.push_back((self.actions.clone(), Some(self_rc)));
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

pub struct ActiveSelector {
    status: Status,
    high_priority: Node,
    low_priority: Parallel,
}

impl Behavior for ActiveSelector {
    fn initialize(&mut self, bt: &mut BehaviorTree, self_rc: Node) {
        self.status = Status::Running
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

    fn on_child_complete(&mut self, status: &Status, bt: &mut BehaviorTree, self_rc: Node) {
        let high_priority = self.high_priority.borrow();
        if high_priority.status() == &Status::Success {
            self.status = Status::Success;
        } else {
            on_parallel_child_complete(&mut self.low_priority, status, bt, self_rc.clone());
            if self.low_priority.status == Status::Running {
                bt.events
                    .push_back((self.high_priority.clone(), Some(self_rc.clone())));
                self.high_priority
                    .borrow_mut()
                    .initialize(bt, self_rc.clone());

                let child =
                    self.low_priority.children.next().expect(
                        "Paralell node didnt complete but there are no more children to run",
                    );
                bt.events.push_back((child.clone(), Some(self_rc.clone())));
                child.borrow_mut().initialize(bt, self_rc);
            } else {
                self.status = self.low_priority.status.clone();
            }
        }
    }
}
