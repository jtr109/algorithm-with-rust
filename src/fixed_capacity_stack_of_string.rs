trait FixedCapacityStackOfStrings {
    type Item;

    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Self::Item;
    fn is_empty(self) -> bool;
    fn size(&self) -> isize;
}

struct FixedStack {
    _size: isize,
    content: Vec<String>,
}

impl FixedStack {
    fn new() -> Self {
        FixedStack {
            _size: 0,
            content: vec![],
        }
    }
}

impl FixedCapacityStackOfStrings for FixedStack {
    type Item = String;

    fn push(&mut self, item: Self::Item) {
        self._size += 1;
        self.content.push(item);
    }

    fn pop(&mut self) -> Self::Item {
        self._size -= 1;
        self.content.pop().unwrap()
    }

    fn size(&self) -> isize {
        self._size
    }

    fn is_empty(self) -> bool {
        self.size() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let fs = FixedStack::new();
        assert!(fs.is_empty());
    }

    #[test]
    fn test_push() {
        let mut fs = FixedStack::new();
        fs.push("to".to_string());
        assert_eq!(fs.size(), 1);
        fs.push("be".to_string());
        assert_eq!(fs.size(), 2);
    }

    #[test]
    fn test_pop() {
        let mut fs = FixedStack::new();
        fs.push("to".to_string());
        fs.push("be".to_string());
        fs.pop();
        assert_eq!(fs.size(), 1);
    }

    #[test]
    fn test_all() {
        let mut fs = FixedStack::new();
        let text = "to be or not to - be - - that - - - is";
        for s in text.split(' ') {
            if s != "-" {
                fs.push(s.to_string());
            } else {
                fs.pop();
            }
        }
        assert_eq!(fs.size(), 2);
    }
}
