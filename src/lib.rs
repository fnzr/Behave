use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Status {
    Invalid,
    Running,
    Success,
    Failure,
}

type Node = Rc<RefCell<dyn NodeTrait>>;

pub trait NodeTrait {
    fn initialize(&mut self, bt: &mut BehaviorTree, self_rc: Node) {}

    fn terminate(&mut self) {}

    fn on_child_complete(&mut self, status: &Status, bt: &mut BehaviorTree, self_rc: Node) {}

    fn tick(&mut self) -> &Status;
}

pub fn action<T>(node: T) -> Node
where
    T: NodeTrait + 'static,
{
    Rc::new(RefCell::new(node))
}

pub fn sequence(nodes: Vec<Node>) -> Node
where
{
    Sequence::new(nodes)
}

pub struct Sequence {
    children: Vec<Node>,
    current_child: usize,
    status: Status,
}

impl Sequence {
    fn new(children: Vec<Node>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Sequence {
            children,
            current_child: 0,
            status: Status::Invalid,
        }))
    }
}

impl NodeTrait for Sequence {
    fn initialize(&mut self, bt: &mut BehaviorTree, self_rc: Node) {
        if let Some(child) = self.children.get(0) {
            enqueue_node(bt, child, Some(self_rc));
            self.status = Status::Running;
            self.current_child = 0;
        } else {
            self.status = Status::Invalid;
        }
    }

    fn tick(&mut self) -> &Status {
        &self.status
    }

    fn on_child_complete(&mut self, result: &Status, bt: &mut BehaviorTree, self_rc: Node) {
        if result == &Status::Success {
            self.current_child += 1;
            if let Some(child) = self.children.get_mut(self.current_child) {
                enqueue_node(bt, child, Some(self_rc));
            } else {
                self.status = Status::Success;
            }
        } else {
            self.status = result.clone();
        }
    }
}

fn enqueue_node(bt: &mut BehaviorTree, node: &Node, parent_rc: Option<Node>) {
    let mut tmp = node.borrow_mut();
    tmp.initialize(bt, node.clone());
    bt.events.push_back((node.clone(), parent_rc));
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
    pub fn start(&mut self, root: &mut Node) {
        enqueue_node(self, root, None);
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

pub struct PrintEven {
    x: Rc<RefCell<i32>>,
}

impl NodeTrait for PrintEven {
    fn tick(&mut self) -> &Status {
        if *self.x.borrow() % 2 == 0 {
            println!("Even");
        } else {
            println!("Skip");
        }
        &Status::Success
    }
}

pub struct PrintOdd {
    x: Rc<RefCell<i32>>,
}

impl NodeTrait for PrintOdd {
    fn tick(&mut self) -> &Status {
        if *self.x.borrow() % 2 != 0 {
            println!("Odd");
        } else {
            println!("Skip");
        }
        &Status::Success
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

        let mut bt = BehaviorTree::new();
        let mut root = sequence(vec![
            action(PrintOdd { x: x.clone() }),
            action(PrintEven { x: x.clone() }),
        ]);
        bt.start(&mut root);
        loop {
            while bt.step() {}
            let c = x.clone();
            let mut v = c.borrow_mut();
            *v += 1;
            //let y = rf.borrow();
            //println!("{:?}", y);
            bt.start(&mut root);
        }
    }
}
