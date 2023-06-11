struct Node<T> {
    elem: T,
    next: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, value:T) -> () {
        let node = Box::new(Node {
            elem: value,
            next: self.head.take(),
        }); 
        self.head = Some(node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn peek(&mut self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {next: self.head.as_deref()}
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();

        while let Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
        }
    }
}
pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop()
    }
}
pub struct Iter<'a,T> {
    next: Option<&'a Node<T>>
}
impl<'a,T> Iterator for Iter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<'a,T> Iterator for IterMut<'a,T> {
    type Item = &'a mut T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn pop_empty() {
        let mut list: List<u32> = List::new();

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
    #[test]
    fn peek() {
        let mut list = List::new();

        // add values to change
        list.push(5);

        // test immutable
        assert_eq!(list.peek(), Some(&5));

        // test change
        list.peek_mut().map(|val| {
            *val = 6
        });

        assert_eq!(list.pop(), Some(6));

        // test on empty
        assert_eq!(list.peek_mut(), None);
        assert_eq!(list.peek(), None);
    }
    #[test]
    fn intoiter() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
