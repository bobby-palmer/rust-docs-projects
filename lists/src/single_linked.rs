use std::mem;

struct Node {
    elem: i32,
    next: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}
pub struct List {
    head: Link,
}
impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, value:i32) -> () {
        let node = Box::new(Node {
            elem: value,
            next: mem::replace(&mut self.head, Link::Empty)
        }); 
        self.head = Link::More(node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                None
            }
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = current_link {
            current_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn pop_empty() {
        let mut list = List::new();

        // check empty list
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn push_then_pop() {
        let mut list = List::new();

        // add values
        list.push(5);
        list.push(4);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));

    }
}
