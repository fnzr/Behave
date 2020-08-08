use crate::nodes::{is_terminated, Behavior, ChildrenNodes, Node, Status};
use crate::BehaviorTree;
#[derive(PartialEq)]
pub enum ParallelPolicy {
    One,
    OneDelayed,
    All,
}

fn parallel_tick(
    children: ChildrenNodes,
    failure_policy: &ParallelPolicy,
    success_policy: &ParallelPolicy,
) -> Status {
    let success_count = 0;
    let failure_count = 0;
    for rc_node in children.nodes.iter() {
        let mut node = rc_node.borrow_mut();
        let mut status = node.status();
        if !is_terminated(status) {
            status = node.tick();
        }
        match status {
            &Status::Failure => {
                failure_count += 1;
                if failure_policy == &ParallelPolicy::One {
                    return Status::Failure;
                }
            }
            &Status::Success => {
                success_count += 1;
                if success_policy == &ParallelPolicy::One {
                    return Status::Success;
                }
            }
        }
    }
    let len = children.nodes.len();
    if (failure_policy == &ParallelPolicy::OneDelayed && failure_count > 1)
        || failure_policy == &ParallelPolicy::All && len == failure_count
    {
        Status::Failure
    } else if (success_policy == &ParallelPolicy::OneDelayed && success_count > 1)
        || success_policy == &ParallelPolicy::All && len == success_count
    {
        Status::Success
    } else {
        Status::Running
    }
}

pub struct Parallel {
    children: ChildrenNodes,
    status: Status,
    success_policy: ParallelPolicy,
    failure_policy: ParallelPolicy,
}

impl Behavior for Parallel {
    fn status(&self) -> &Status {
        &self.status
    }

    fn tick(&mut self) -> &Status {
        self.status = parallel_tick(self.children, &self.failure_policy, &self.success_policy);
        if self.status != Status::Running {
            self.terminate()
        }
        &self.status
    }

    fn terminate(&mut self) {
        for rc_node in self.children.nodes.iter() {
            let mut node = rc_node.borrow_mut();
            if !is_terminated(node.status()) {
                node.terminate()
            }
        }
    }

    fn initialize(&mut self, bt: &mut BehaviorTree, self_rc: Node) {
        for rc_node in self.children.nodes.iter() {
            let mut node = rc_node.borrow_mut();
            node.initialize(bt, rc_node.clone())
        }
        self.status = Status::Running
    }
}

pub struct Monitor {
    conditions: Parallel,
    actions: Parallel,
    status: Status,
}

impl Behavior for Monitor {
    fn initialize(&mut self, bt: &mut BehaviorTree, self_rc: Node) {
        self.status = Status::Running
    }

    fn status(&self) -> &Status {
        &self.status
    }

    fn tick(&mut self) -> &Status {
        let conditions_status = self.conditions.tick();
        if conditions_status == &Status::Success {
            self.status = *self.actions.tick();
            if is_terminated(&self.status) {
                self.terminate();
            }
        }
        &self.status
    }
}
