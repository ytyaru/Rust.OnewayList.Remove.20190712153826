#[derive(Debug, PartialEq)]
struct OnewayList<T> {
    head: Option<Box<Node<T>>>,
}
#[derive(Debug, PartialEq)]
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn new(item: T) -> Self { Self { item: item, next: None } }
    /*
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
    */
}
impl<T> OnewayList<T> {
    fn new() -> Self { Self { head: None } }
    fn push(&mut self, item: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(item)));
        } else {
            // 1. 最後の要素を取得する
            // 2. 最後の要素の`next`に新要素へのポインタをセットする
            fn last_node<T>(node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
                if let Some(ref mut _n) = *node { last_node(&mut _n.next) }
                else { node }
            }
            let last = last_node(&mut self.head);
            *last = Some(Box::new(Node { item: item, next: None, }));
            /*
//            let &mut last = self.head.as_mut().unwrap().next;
            let last = self.head.as_mut().unwrap().next;
            while last.is_some() { last = last.as_mut().unwrap().next; }
//            while last.is_some() { last = last.as_mut().unwrap().next.as_mut(); }
//            last.unwrap().next = Some(Box::new(Node::new(item))); // error[E0594]: cannot assign to data in a `&` reference
//            std::mem::replace(last.unwrap().next.as_mut(), Some(Box::new(Node::new(item))));
//            last.as_mut().unwrap().next = Some(Box::new(Node::new(item)));
            last.unwrap().next = Some(Box::new(Node::new(item)));
            */


            /*
//            let last = self.head.as_ref().unwrap().next;
//            let mut last = self.head.as_ref().unwrap().next.as_ref();
            let mut last = self.head.as_ref().unwrap().next.as_ref();
            while last.is_some() { last = last.as_ref().unwrap().next.as_ref(); }
//            last.unwrap().next = Some(Box::new(Node::new(item))); // error[E0594]: cannot assign to data in a `&` reference
            std::mem::replace(last.unwrap().next.as_mut(), Some(Box::new(Node::new(item))));
//            std::mem::replace(last.unwrap().next, Some(Box::new(Node::new(item)))); // error[E0308]: mismatched types
//            last.as_ref().unwrap().next = Some(Box::new(Node::new(item))); // error[E0594]: cannot assign to data in a `&` reference
//            last.as_mut().unwrap().next = Some(Box::new(Node::new(item))); // error[E0594]: cannot assign to data in a `&` reference
//            last.as_ref().unwrap().next = Some(Box::new(Node::new(item))); // error[E0594]: cannot assign to data in a `&` reference
//            let last = self.head.next; // error[E0609]: no field `next` on type `std::option::Option<std::boxed::Box<Node<T>>>`
//            while last.is_some() { last = last.next; }

//            while let Some(node) = last { last = last.next; }
//            last.next = Some(Box::new(Node::new(item)));
            */
            /*
            for node in last {
                if last.next.is_some() { last = last.next; }
                else { break; }
            }
            let tmp_node = std::mem::replace(&mut self.next, None); // self.nextに一旦Noneを入れる
            let mut new_node = Node::new(item);
            new_node.next = tmp_node;
            self.next = Some(Box::new(new_node));
            */
        }
    }
    /*
    fn remove(&mut self) {
        if self.next == None {
            // prev.next = None
        } else {
            // prev.next = self.next
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
    /*
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
    */
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
