use std::cell::RefCell;
use std::rc::Rc;

pub struct LinkedList<T> {
    head: Option<SharedNode<T>>,
    tail: Option<SharedNode<T>>,
    len: usize,
}

struct Node<T> {
    element: T,
    next: Option<SharedNode<T>>,
    prev: Option<SharedNode<T>>,
}

type SharedNode<T> = Rc<RefCell<Box<Node<T>>>>;

impl<T> Default for LinkedList<T> {
    fn default() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {
            element,
            next: None,
            prev: None,
        }
    }

    fn into_element(self) -> T {
        self.element
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList::default()
    }

    pub fn push_left(&mut self, element: T) {
        let node = Box::new(Node {
            prev: None,
            next: None,
            element,
        });
        let node_rc = Rc::new(RefCell::new(node));

        match &self.head {
            Some(head) => {
                node_rc.borrow_mut().next = Some(Rc::clone(head));
                head.borrow_mut().prev = Some(Rc::clone(&node_rc));
            }
            None => {
                self.tail = Some(Rc::clone(&node_rc));
            }
        }

        self.head = Some(Rc::clone(&node_rc));
        self.len += 1;
    }

    pub fn pop_left(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            match &head.borrow().next {
                Some(next) => {
                    self.head = Some(Rc::clone(next));
                    next.borrow_mut().prev = None;
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            };
            let node = Rc::try_unwrap(head)
                .ok()
                .expect("Head have more than one strong counters")
                .into_inner();
            self.len -= 1;
            node.into_element()
        })
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn test_push_left() {
        let mut list = LinkedList::new();

        let element = list.pop_left();
        assert_eq!(element, None);

        list.push_left(3.0);
        let element = list.pop_left().unwrap();
        assert_eq!(element, 3.0);

        list.push_left(3.0);
        list.push_left(4.0);
        let element = list.pop_left().unwrap();
        assert_eq!(element, 4.0);

        list.push_left(5.0);
        let element = list.pop_left().unwrap();
        assert_eq!(element, 5.0);

        let element = list.pop_left().unwrap();
        assert_eq!(element, 3.0);
    }
}
