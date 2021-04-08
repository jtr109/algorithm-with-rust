pub struct Node<Key: PartialOrd, Value: Copy> {
    key: Key,
    value: Value,
    left: Option<Box<Node<Key, Value>>>,
    right: Option<Box<Node<Key, Value>>>,
}

impl<Key: PartialOrd, Value: Copy> Node<Key, Value> {
    pub fn new(key: Key, value: Value) -> Self {
        return Node {
            key,
            value,
            left: None,
            right: None,
        };
    }

    pub fn get(&self, key: Key) -> Option<Value> {
        if key == self.key {
            return Some(self.value);
        } else if key < self.key {
            return self.left.as_ref().and_then(|x| x.get(key));
        } else {
            return self.right.as_ref().and_then(|x| x.get(key));
        }
    }

    pub fn put(&mut self, key: Key, value: Value) {
        if key == self.key {
            self.value = value
        } else if key < self.key {
            match self.left.as_mut() {
                Some(x) => x.put(key, value),
                _ => self.left = Some(Box::new(Node::new(key, value))),
            }
        } else {
            match self.right.as_mut() {
                Some(x) => x.put(key, value),
                _ => self.right = Some(Box::new(Node::new(key, value))),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut node = Node::new(1, "1");
        node.put(2, "2");
        node.put(3, "3");
        assert_eq!(node.get(1), Some("1"));
        assert_eq!(node.get(2), Some("2"));
        assert_eq!(node.get(3), Some("3"));
        assert_eq!(node.get(4), None);
    }
}
