use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub mod nodes;

pub fn action(node: impl Behavior + 'static) -> Node {
    Rc::new(RefCell::new(node))
}

pub fn pure_action(action: fn() -> Status) -> Node {
    Rc::new(RefCell::new(nodes::Action {
        status: Status::Invalid,
        action,
    }))
}

pub fn decorator(decoration: fn(&mut nodes::Decorator, Node) -> Status, child: Node) -> Node {
    Rc::new(RefCell::new(nodes::Decorator {
        status: Status::Invalid,
        child,
        decoration,
    }))
}

pub fn sequence(nodes: Vec<Node>) -> Node {
    Rc::new(RefCell::new(nodes::Sequence {
        children: nodes::ChildrenNodes::new(nodes),
        status: Status::Invalid,
    }))
}

pub fn selector(nodes: Vec<Node>) -> Node {
    Rc::new(RefCell::new(nodes::Selector {
        children: nodes::ChildrenNodes::new(nodes),
        status: Status::Invalid,
    }))
}

pub fn parallel(
    success_policy: nodes::ParallelPolicy,
    failure_policy: nodes::ParallelPolicy,
    nodes: Vec<Node>,
) -> Node {
    Rc::new(RefCell::new(nodes::Parallel {
        children: nodes::ChildrenNodes::new(nodes),
        status: Status::Invalid,
        success_policy,
        failure_policy,
        success_count: 0,
        failure_count: 0,
    }))
}

pub struct BehaviorTree {
    events: VecDeque<(Node, Option<Node>)>,
}

impl BehaviorTree {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }
    pub fn start(&mut self, root: Node) {
        self.events.push_back((root.clone(), None));
        root.borrow_mut().initialize(self, root.clone());
    }

    pub fn step(&mut self) -> bool {
        if let Some((node, opt_parent)) = self.events.pop_front() {
            let mut tmp = node.borrow_mut();
            if tmp.status() == &Status::Aborted {
                return true;
            }
            let result = tmp.tick();
            let new_node = node.clone();

            if result == &Status::Running {
                self.events.push_back((new_node, opt_parent));
            } else if let Some(parent) = opt_parent {
                parent
                    .borrow_mut()
                    .on_child_complete(&result, self, parent.clone());
            }
            true
        } else {
            false
        }
    }
}

type Node = Rc<RefCell<dyn Behavior>>;

pub trait Behavior {
    fn initialize(&mut self, bt: &mut BehaviorTree, rc: Node);

    fn terminate(&mut self) {}

    #[allow(dead_code)]
    fn on_child_complete(&mut self, status: &Status, bt: &mut BehaviorTree, rc: Node) {}

    fn tick(&mut self) -> &Status;

    fn status(&self) -> &Status;

    fn abort(&mut self);
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Status {
    Invalid,
    Running,
    Success,
    Failure,
    Aborted,
}

pub struct PrintEven {
    x: Rc<RefCell<i32>>,
    status: Status,
}

impl Behavior for PrintEven {
    fn tick(&mut self) -> &Status {
        if *self.x.borrow() % 2 == 0 {
            print!("Even");
            self.status = Status::Success
        } else {
            self.status = Status::Failure
        }
        &self.status
    }

    #[allow(dead_code)]
    fn initialize(&mut self, bt: &mut BehaviorTree, rc: Node) {}

    fn status(&self) -> &Status {
        &self.status
    }

    fn abort(&mut self) {
        self.status = Status::Aborted
    }
}

pub struct PrintOdd {
    x: Rc<RefCell<i32>>,
    status: Status,
}

impl Behavior for PrintOdd {
    fn tick(&mut self) -> &Status {
        if *self.x.borrow() % 2 != 0 {
            print!("Odd");
            &Status::Success
        } else {
            &Status::Failure
        }
    }

    fn initialize(&mut self, bt: &mut BehaviorTree, rc: Node) {}

    fn status(&self) -> &Status {
        &self.status
    }

    fn abort(&mut self) {
        self.status = Status::Aborted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test() {
        let x = Rc::new(RefCell::new(1));

        let a = pure_action(|| {
            print!("hey");
            Status::Success
        });

        let mut bt = BehaviorTree::new();
        let root = parallel(
            nodes::ParallelPolicy::One,
            nodes::ParallelPolicy::All,
            vec![
                a,
                action(PrintOdd {
                    x: x.clone(),
                    status: Status::Invalid,
                }),
                action(PrintEven {
                    x: x.clone(),
                    status: Status::Invalid,
                }),
            ],
        );
        bt.start(root.clone());
        for _ in 1..10 {
            while bt.step() {}
            let c = x.clone();
            let mut v = c.borrow_mut();
            *v += 1;
            print!("\n");
            //let y = rf.borrow();
            //println!("{:?}", v);
            bt.start(root.clone());
        }
    }
}
