extern crate behave;
mod action;

mod sequence {
    use crate::action::CallCounterAction;
    use behave::helpers::*;
    use behave::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn exit_on_first_child_failure() {
        let a1 = Rc::new(RefCell::new(CallCounterAction::new(Status::Success)));
        let a2 = Rc::new(RefCell::new(CallCounterAction::new(Status::Failure)));
        let a3 = Rc::new(RefCell::new(CallCounterAction::new(Status::Success)));

        let mut tree = Tree::new(sequence(vec![
            custom(a1.clone()),
            custom(a2.clone()),
            custom(a3.clone()),
        ]));
        tree.run();

        assert_eq!(a1.borrow().call_count, 1);
        assert_eq!(a2.borrow().call_count, 1);
        assert_eq!(a3.borrow().call_count, 0);
    }
    #[test]
    fn fail_if_any_fail() {
        let mut tree = Tree::new(sequence(vec![
            action(|| Status::Success),
            action(|| Status::Failure),
        ]));
        assert_eq!(Status::Failure, tree.run())
    }

    #[test]
    fn succeed_if_all_succeed() {
        let mut tree = Tree::new(sequence(vec![
            action(|| Status::Success),
            action(|| Status::Success),
        ]));
        assert_eq!(Status::Success, tree.run())
    }
}

mod selector {
    use crate::action::CallCounterAction;
    use behave::helpers::*;
    use behave::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn exit_on_first_child_success() {
        let a1 = Rc::new(RefCell::new(CallCounterAction::new(Status::Success)));
        let a2 = Rc::new(RefCell::new(CallCounterAction::new(Status::Failure)));
        let a3 = Rc::new(RefCell::new(CallCounterAction::new(Status::Success)));

        let mut tree = Tree::new(selector(vec![
            custom(a1.clone()),
            custom(a2.clone()),
            custom(a3.clone()),
        ]));
        tree.run();

        assert_eq!(a1.borrow().call_count, 1);
        assert_eq!(a2.borrow().call_count, 0);
        assert_eq!(a3.borrow().call_count, 0);
    }
    #[test]
    fn fail_if_all_fail() {
        let mut tree = Tree::new(selector(vec![
            action(|| Status::Failure),
            action(|| Status::Failure),
        ]));
        assert_eq!(Status::Failure, tree.run())
    }

    #[test]
    fn succeed_if_any_succeed() {
        let mut tree = Tree::new(selector(vec![
            action(|| Status::Failure),
            action(|| Status::Success),
        ]));
        assert_eq!(Status::Success, tree.run())
    }
}

mod decorators {
    use crate::action::CallCounterAction;
    use behave::helpers::*;
    use behave::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    pub fn repeater_count() {
        let a = Rc::new(RefCell::new(CallCounterAction::new(Status::Success)));
        let mut tree = Tree::new(repeater(custom(a.clone()), 3));
        assert_eq!(Status::Success, tree.run());
        assert_eq!(a.borrow().call_count, 3);
    }
}
