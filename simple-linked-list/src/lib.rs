use std::iter::FromIterator;

struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        SimpleLinkedList { head: None, len: 0 }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList::default()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            element,
            next: self.head.take(),
        }));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|x| {
            self.head = x.next;
            self.len -= 1;
            x.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.element)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = self;
        let mut ans = SimpleLinkedList::new();
        while let Some(x) = list.pop() {
            ans.push(x);
        }
        ans
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ans = SimpleLinkedList::new();
        for x in iter {
            ans.push(x);
        }
        ans
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut list = self;
        let mut ans = Vec::with_capacity(list.len);
        while let Some(x) = list.pop() {
            ans.push(x);
        }
        ans.reverse();
        ans
    }
}
