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
        // 1. 末尾ノードを指すポインタを返す(OnewayList.head or Node.next)
        fn get_tail_node_ptr<T>(node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
            match *node {
                Some(ref mut _n) => get_tail_node_ptr(&mut _n.next),
                _ => node
            }
        }
        // 2. 末尾ノードを指すポインタを取得する
        let last = get_tail_node_ptr(&mut self.head);
        // 3. 末尾ノードポインタの値として生成した新ノードを代入する
        *last = Some(Box::new(Node::new(item)));
    }
    pub fn remove_head(&mut self) {
        if let Some(ref mut node) = self.head {
            let first = std::mem::replace(&mut self.head, None);
            let first = std::mem::replace(&mut self.head, first.unwrap().next);
        };
    }
    pub fn remove_tail(&mut self) {
        /*
        if None == self.head { return; }
        let &mut tail = self.head;
        while let Some(ref mut n) = tail.next { tail = tail.next; };
        */
        if self.head.is_none() { return; }
        // 1. 末尾ノードを指すポインタを返す(OnewayList.head or Node.next)
        fn get_booby_node_ptr<T>(node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
            match *node {
                Some(ref mut _n) if _n.next.is_some() => get_booby_node_ptr(&mut _n.next),
                _ => node
            }
        }
        let booby = get_booby_node_ptr(&mut self.head);
//        println!("booby: {:?}", *booby.as_ref().unwrap());
//        println!("booby: {:?}", *booby.unwrap());
//        println!("booby: {:?}", booby.unwrap().item);
        println!("booby.is_some: {}", booby.is_some());
        if booby.is_some() { std::mem::replace(&mut booby.as_mut().unwrap().next, None); }
        /*
        let first = std::mem::replace(&mut self.head, None);


        fn get_booby_node_ptr<T>(node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
            match *node {
                Some(ref mut _n1) => {
                    match *n1 {
                        Some(ref mut n2) => get_booby_node_ptr(&mut _n.next),
                        _ => node
                    }


                },
                _ => node
            }
        }
        */
        // 末尾から2番目のノードを取得する
//        let booby = self.head;
//        let &mut booby = self.head;
//        let &mut booby = self.head.as_mut();
//        while let Some(ref mut n) = tail.next { tail = tail.next; };
//        while let Some(ref mut n) = booby.next.as_mut().next { booby = booby.next; };
//        while let Some(ref mut n) = booby.as_mut().next.as_mut().next { booby = booby.next; };
//        while let Some(ref mut n) = *(booby.as_mut()).next.as_mut().next { booby = booby.next; };
//        while let Some(ref mut n1) = booby.as_mut().next { while let Some(ref mut n2) = n1.as_mut().next { booby = booby.next; } };
//        while let Some(ref mut n1) = booby.as_mut() { while let Some(ref mut n2) = n1.as_mut() { booby = booby.next; } };
        /*
        // 1. 末尾から2番目のノードを取得する
        fn get_booby_node<T>(node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
            match *node {
                Some(ref mut _n) => {
                    match *_n {
//                        Some(ref mut _n) => get_booby_node(&mut _n.next),
                        Some(ref mut _n) => get_booby_node(&mut node.next),
                        _ => node
                    }
                },
                _ => None
            }
        }
        let booby = get_booby_node(&mut self.head);
        if booby.is_some() {
            // 末尾から2番目のノードがもつnextを取り出してdropさせ、代わりにNoneを代入する
            std::mem::replace(&mut booby, None);
        }
        */
    }
    pub fn remove_from_index(&mut self, index: u32) {}
    pub fn remove(&mut self, item: T) {} // 指定要素に一致するノードを検索する必要がある
    fn search(&self, item: T) -> Result<&Option<Box<Node<T>>>, &'static str> { Err("Not found") }
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
    #[test]
    fn OnewayList_remove_head_0() {
        let mut list: OnewayList<i32> = OnewayList::new();
        list.remove_head();
        assert_eq!(list.head, None);
    }
    #[test]
    fn OnewayList_remove_head_1() {
        let mut list: OnewayList<i32> = OnewayList::new();
        list.push(0);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: None })));
        list.remove_head();
        assert_eq!(list.head, None);
    }
    #[test]
    fn OnewayList_remove_head_2() {
        let mut list: OnewayList<i32> = OnewayList::new();
        list.push(0);
        list.push(1);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: None }))
        })));
        list.remove_head();
        assert_eq!(list.head, Some(Box::new(Node { item: 1, next: None })));
        list.remove_head();
        assert_eq!(list.head, None);
    }
    #[test]
    fn OnewayList_remove_head_3() {
        let mut list: OnewayList<i32> = OnewayList::new();
        list.push(0);
        list.push(1);
        list.push(2);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: 
                Some(Box::new(Node { item: 2, next: None }))
            }))
        })));
        list.remove_head();
        assert_eq!(list.head, Some(Box::new(Node { item: 1, next: 
            Some(Box::new(Node { item: 2, next: None }))
        })));
        list.remove_head();
        assert_eq!(list.head, Some(Box::new(Node { item: 2, next: None })));
        list.remove_head();
        assert_eq!(list.head, None);
    }
    #[test]
    fn OnewayList_remove_tail_3() {
        let mut list: OnewayList<i32> = OnewayList::new();
        list.push(0);
        list.push(1);
        list.push(2);
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: 
                Some(Box::new(Node { item: 2, next: None }))
            }))
        })));
        list.remove_tail();
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: 
            Some(Box::new(Node { item: 1, next: None }))
        })));
        list.remove_tail();
        assert_eq!(list.head, Some(Box::new(Node { item: 0, next: None })));
        list.remove_tail();
        assert_eq!(list.head, None);
    }

}
