use std::cell::RefCell;
use std::rc::Rc;

pub struct Queue<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, elem: T) {
        let new_link = Some(Rc::new(RefCell::new(Node { elem, next: None })));
        match self.tail.take() {
            Some(last_node) => {
                last_node.borrow_mut().next = new_link.clone();
                self.tail = new_link;
            }
            None => {
                self.head = new_link.clone();
                self.tail = new_link;
            }
        };
    }

    // pub fn dequeue(&mut self) -> Option<T> {
    //     // match self.head.take() {
    //     //     Some(first_node) => {
    //     //         self.head = first_node.borrow_mut().next.clone();
    //     //     }
    //     //     None => None,
    //     // }
    //     self.head.take().map(|first_node| {
    //         self.head = first_node.borrow_mut().next.clone();
    //         first_node.borrow_mut().elem
    //     })
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
    }
}
