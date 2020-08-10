extern crate event_behavior_tree;

use event_behavior_tree::helpers::*;
use event_behavior_tree::*;

pub fn succeed() -> Node {
    pure_action(Box::new(|| Status::Success))
}

pub fn fail() -> Node {
    pure_action(Box::new(|| Status::Failure))
}

pub fn panic() -> Node {
    pure_action(Box::new(|| {
        assert!(false);
        Status::Failure
    }))
}
