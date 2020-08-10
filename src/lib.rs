use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub mod helpers;
pub mod nodes;

pub type Node = Rc<RefCell<NodeWrapper>>;

pub trait Behavior {
    fn initialize(&mut self, bt: &mut BehaviorTree) -> Status;

    fn tick(&mut self, bt: &mut BehaviorTree) -> Status;

    fn terminate(&mut self) {}

    fn abort(&mut self, _: &mut BehaviorTree) -> Status {
        self.terminate();
        Status::Aborted
    }
}

pub struct NodeWrapper {
    status: Status,
    behavior: Box<dyn Behavior>,
}

impl NodeWrapper {
    fn initialize(&mut self, bt: &mut BehaviorTree) {
        self.status = self.behavior.initialize(bt)
    }

    fn tick(&mut self, bt: &mut BehaviorTree) -> &Status {
        self.status = self.behavior.tick(bt);
        &self.status
    }

    fn abort(&mut self, bt: &mut BehaviorTree) -> &Status {
        self.status = self.behavior.abort(bt);
        &self.status
    }
}

pub struct BehaviorTree {
    events: VecDeque<Node>,
    root: Node,
}

impl BehaviorTree {
    pub fn new(root: Node) -> Self {
        Self {
            events: VecDeque::<Node>::new(),
            root,
        }
    }

    pub fn start(&mut self) {
        self.events.clear();
        let root = self.root.clone();
        root.borrow_mut().initialize(self);
        self.events.push_back(self.root.clone());
    }

    pub fn run(&mut self) -> Status {
        self.start();
        while self.step() {}
        self.root.borrow().status
    }

    pub fn step(&mut self) -> bool {
        if let Some(node_rc) = self.events.pop_front() {
            let mut node = node_rc.borrow_mut();
            if node.status == Status::Aborted {
                return true;
            }
            if node.tick(self) == &Status::Running {
                drop(node);
                self.events.push_back(node_rc);
            }
            true
        } else {
            false
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Status {
    Invalid,
    Running,
    Success,
    Failure,
    Aborted,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a1 = helpers::pure_action(|| {
            print!("pa");
            Status::Success
        });
        let a2 = helpers::pure_action(|| {
            print!("ta");
            Status::Success
        });
        let a3 = helpers::pure_action(|| {
            print!("pum");
            Status::Success
        });
        let a4 = helpers::pure_action(|| {
            print!("\n");
            Status::Success
        });

        let b1 = helpers::pure_action(|| {
            println!("Here");
            Status::Success
        });

        let b2 = helpers::pure_action(|| {
            print!("Never here");
            Status::Success
        });

        let root1 = helpers::sequence(vec![a1, a2, a3, a4]);
        let root2 = helpers::selector(vec![b1, b2]);
        let mut bt = BehaviorTree::new(root1.clone());
        let mut bt2 = BehaviorTree::new(root2.clone());
        /*
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
        );*/
        //bt1.start();
        //while bt.step() {}
        //bt.start(root2.clone());
        //while bt.step() {}

        //for _ in 1..10 {
        //let c = x.clone();
        //let mut v = c.borrow_mut();
        //*v += 1;
        //break;
        //print!("\n");
        //let y = rf.borrow();
        //println!("{:?}", v);
        // bt.start(root.clone());
        //}
    }
}
