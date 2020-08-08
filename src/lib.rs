use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
pub mod nodes;

pub fn action(node: impl Behavior + 'static) -> Node {
    Rc::new(RefCell::new(node))
}

pub fn pure_action(action: fn()) -> Node {
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
        enqueue_node(self, &root, None);
    }

    pub fn step(&mut self) -> bool {
        if let Some((node, opt_parent)) = self.events.pop_front() {
            let mut tmp = node.borrow_mut();
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
    fn initialize(&mut self, bt: &mut BehaviorTree, self_rc: Node);

    fn terminate(&mut self) {}

    fn on_child_complete(&mut self, status: &Status, bt: &mut BehaviorTree, self_rc: Node) {}

    fn tick(&mut self) -> &Status;

    fn status(&self) -> &Status;

    fn abort(&mut self) {
        self.terminate()
    }
}

fn enqueue_node(bt: &mut BehaviorTree, node: &Node, parent_rc: Option<Node>) {
    let mut tmp = node.borrow_mut();
    tmp.initialize(bt, node.clone());
    bt.events.push_back((node.clone(), parent_rc));
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Status {
    Invalid,
    Running,
    Success,
    Failure,
}

pub struct PrintEven {
    x: Rc<RefCell<i32>>,
}

impl Behavior for PrintEven {
    fn tick(&mut self) -> &Status {
        if *self.x.borrow() % 2 == 0 {
            println!("Even");
            &Status::Success
        } else {
            &Status::Failure
        }
    }
}

pub struct PrintOdd {
    x: Rc<RefCell<i32>>,
}

impl Behavior for PrintOdd {
    fn tick(&mut self) -> &Status {
        if *self.x.borrow() % 2 != 0 {
            println!("Odd");
            &Status::Success
        } else {
            &Status::Failure
        }
    }
}

trait A {}
struct B {}
impl A for B {}

struct C {
    x: u32,
}
impl A for C {}
fn f(b: Box<dyn A>) {}
enum X {
    Duh(Box<dyn A>),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test() {
        let x = Rc::new(RefCell::new(1));

        let a = pure_action(|| print!("hey"));

        let mut bt = BehaviorTree::new();
        let mut root = selector(vec![
            action(PrintOdd { x: x.clone() }),
            action(PrintEven { x: x.clone() }),
        ]);
        bt.start(root.clone());
        for _ in 1..10 {
            while bt.step() {}
            let c = x.clone();
            let mut v = c.borrow_mut();
            *v += 1;
            //let y = rf.borrow();
            //println!("{:?}", y);
            bt.start(root.clone());
        }
    }
}
