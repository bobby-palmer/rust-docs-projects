use std::rc::Rc;

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    data: T,
    next: Link<T>,
}
pub struct List<T> {
    head: Link<T>,
}
