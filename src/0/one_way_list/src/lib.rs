#[derive(Debug, PartialEq)]
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn new(item: T) -> Self { Self { item: item, next: None } }
    fn push(&mut self, item: T) {
        if self.next.is_none() {
            let new_node = Node::new(item);
            self.next = Some(Box::new(new_node));
        } else {
            let tmp_node = std::mem::replace(&mut self.next, None); // self.nextに一旦Noneを入れる
            let mut new_node = Node::new(item);
            new_node.next = tmp_node;
            self.next = Some(Box::new(new_node));
        }
    }
    /*
    // 前の要素を知る必要がある
    fn remove(&mut self) {
        // 末尾なら
        if self.next == None {
            self.prev.next = None
        }
        // 先頭なら
        else if self.next.prev == None {
        }
        else {
            self.next.prev.next = self.next;
        }
    }
    */
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn Node_new() {
        let first = Node::new(0);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, None);
        let second = Node::new(1);
        assert_eq!(second.item, 1);
        assert_eq!(second.next, None);
    }
    #[test]
    fn Node_new_2() {
        let mut first = Node::new(0);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, None);
        let mut second = Node::new(1);
        assert_eq!(second.item, 1);
        assert_eq!(second.next, None);
        first.next = Some(Box::new(first));
    }
    #[test]
    fn Node_new_3() {
        let mut first = Node::new(0);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, None);
        let mut second = Node::new(1);
        assert_eq!(second.item, 1);
        assert_eq!(second.next, None);
        first.next = Some(Box::new(first));
        let mut third = Node::new(2);
        assert_eq!(third.item, 2);
        assert_eq!(third.next, None);
        second.next = Some(Box::new(third));
    }
    #[test]
    fn Node_new_string() {
        let first = Node::new(String::from("AA"));
        assert_eq!(first.item, String::from("AA"));
        assert_eq!(first.next, None);
        let second = Node::new(String::from("BB"));
        assert_eq!(second.item, String::from("BB"));
        assert_eq!(second.next, None);
    }
    #[test]
    fn Node_push() {
        let mut first = Node::new(0);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, None);

        first.push(1);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, Some(Box::new(Node { item: 1, next: None })));
        let mut second = first.next.unwrap();
        assert_eq!(second.item, 1);
        assert_eq!(second.next, None);

        second.push(2);
        assert_eq!(second.item, 1);
        assert_eq!(second.next, Some(Box::new(Node { item: 2, next: None })));
        let mut third = second.next.unwrap();
        assert_eq!(third.item, 2);
        assert_eq!(third.next, None);
    }
    #[test]
    fn Node_push_string() {
        let mut first = Node::new(String::from("AA"));
        assert_eq!(first.item, String::from("AA"));
        assert_eq!(first.next, None);

        first.push(String::from("BB"));
        assert_eq!(first.item, String::from("AA"));
        assert_eq!(first.next, Some(Box::new(Node { item: String::from("BB"), next: None })));
        let mut second = first.next.unwrap();
        assert_eq!(second.item, String::from("BB"));
        assert_eq!(second.next, None);

        second.push(String::from("CC"));
        assert_eq!(second.item, String::from("BB"));
        assert_eq!(second.next, Some(Box::new(Node { item: String::from("CC"), next: None })));
        let mut third = second.next.unwrap();
        assert_eq!(third.item, String::from("CC"));
        assert_eq!(third.next, None);
    }
    #[test]
    fn Node_push_not_last() {
        let mut first = Node::new(0);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, None);

        first.push(1);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, Some(Box::new(Node { item: 1, next: None })));
        let second = first.next.as_ref().unwrap();
        assert_eq!(second.item, 1);
        assert_eq!(second.next, None);

        first.push(2);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, Some(Box::new(Node { item: 2, next: Some(Box::new(Node { item: 1, next: None })) })));
        let third = first.next.as_ref().unwrap();
        assert_eq!(third.item, 2);
        assert_eq!(third.next, Some(Box::new(Node { item: 1, next: None })));
    }
}
