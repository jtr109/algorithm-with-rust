pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    fn new(elem: T) -> Self {
        Stack {
            head: Some(Box::new(Node { elem, next: None })),
        }
    }
}
