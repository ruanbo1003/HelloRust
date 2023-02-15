
/*
 interior mutability
`RefCell` can only used in single thread, like `Rc`.

immutable reference: borrow();
mutable reference: borrow_mut();
at anytime, it can be multiple borrow(), or one borrow_mut().

 */


pub fn tests() {

}

pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
    where T: Messenger
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning");
        } else {
            // do nothing
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger{ sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn refcell_test_1() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(95);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 2);

        limit_tracker.set_value(10);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 2);
    }
}