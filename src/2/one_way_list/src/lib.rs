#[derive(Debug, PartialEq)]
pub struct OnewayList<T> {
    head: Option<Box<Node<T>>>,
}
#[derive(Debug, PartialEq)]
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn new(item: T) -> Self { Self { item: item, next: None } }
}
impl<T> OnewayList<T> {
    pub fn new() -> Self { Self { head: None } }
    pub fn push(&mut self, item: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(item)));
        } else {
            // 1. 最後の要素を取得する
            fn last_node<T>(node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
                if let Some(ref mut _n) = *node { last_node(&mut _n.next) }
                else { node }
            }
            // 2. 最後の要素の`next`に新要素へのポインタをセットする
            let last = last_node(&mut self.head);
            *last = Some(Box::new(Node::new(item)));
        }
    }
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
    fn OnewayList_new() {
        let list: OnewayList<i32> = OnewayList::new();
        assert_eq!(list.head, None);
    }
    #[test]
    fn OnewayList_push_1() {
        let mut list: OnewayList<i32> = OnewayList::new();
        list.push(0);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: None })));
    }
    #[test]
    fn OnewayList_push_2() {
        let mut list: OnewayList<i32> = OnewayList::new();
        list.push(0);
        list.push(1);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: Some(Box::new(Node { item: 1, next: None })) })));
    }
    #[test]
    fn OnewayList_push_3() {
        let mut list: OnewayList<i32> = OnewayList::new();
        list.push(0);
        list.push(1);
        list.push(2);
        assert_eq!(list.head, 
            Some(Box::new(Node { item: 0, next: 
                Some(Box::new(Node { item: 1, next: 
                    Some(Box::new(Node { item:2, next: None }))
                }))
            }))
        );
    }
    #[test]
    fn OnewayList_new_string() {
        let list: OnewayList<String> = OnewayList::new();
        assert_eq!(list.head, None);
    }
    #[test]
    fn OnewayList_push_3_string() {
        let mut list: OnewayList<String> = OnewayList::new();
        list.push(String::from("AA"));
        list.push(String::from("BB"));
        list.push(String::from("CC"));
        assert_eq!(list.head, 
            Some(Box::new(Node { item: String::from("AA"), next: 
                Some(Box::new(Node { item: String::from("BB"), next: 
                    Some(Box::new(Node { item: String::from("CC"), next: None }))
                }))
            }))
        );
    }
}
