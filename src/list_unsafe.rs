use std::ptr::NonNull;

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

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
        let node_ptr = NonNull::from(Box::leak(node));

        unsafe {
            (*node_ptr.as_ptr()).next = self.head;
            (*node_ptr.as_ptr()).prev = None;

            match self.head {
                Some(head) => {
                    (*head.as_ptr()).prev = Some(node_ptr);
                }
                None => {
                    self.tail = Some(node_ptr);
                }
            }
        }

        self.head = Some(node_ptr);
        self.len += 1;
    }

    pub fn pop_left(&mut self) -> Option<T> {
        self.head.map(|head| unsafe {
            let node = Box::from_raw(head.as_ptr());
            self.head = node.next;

            match self.head {
                Some(head) => {
                    (*head.as_ptr()).prev = None;
                }
                None => self.tail = None,
            }

            node.element
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
