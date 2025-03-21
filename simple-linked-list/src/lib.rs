use std::marker::PhantomData;
use std::mem::swap;

type Link<T> = *mut Node<T>;

pub struct Node<T> {
    data: T,
    next: Link<T>,
    prev: Link<T>,
}

pub struct SimpleLinkedList<T> {
    len: usize,
    head: Link<T>,
    tail: Link<T>,
    dummy: PhantomData<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
            dummy: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        let node_ptr = Box::into_raw(Box::new(Node {
            data: element,
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
        }));
        if self.head.is_null() {
            self.head = node_ptr;
        } else {
            unsafe {
                (*node_ptr).prev = self.tail;
                (*self.tail).next = node_ptr;
            };
        }
        self.tail = node_ptr;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let node = self.tail;
        let prev = unsafe {
            if (*node).prev.is_null() {
                None
            } else {
                let prev = (*node).prev;
                (*prev).next = std::ptr::null_mut();
                (*node).prev = std::ptr::null_mut();
                Some(prev)
            }
        };

        if let Some(prev) = prev {
            self.tail = prev;
        } else {
            self.head = std::ptr::null_mut();
            self.tail = std::ptr::null_mut();
        }

        self.len -= 1;
        unsafe { Some(Box::from_raw(node).data) }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        unsafe { self.tail.as_ref().map(|x| &x.data) }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut node = self.head;
        loop {
            if node.is_null() {
                break;
            }
            unsafe {
                swap(&mut (*node).next, &mut (*node).prev);
                node = (*node).prev;
            }
        }
        swap(&mut self.head, &mut self.tail);
        self
    }
}

impl<T> Drop for SimpleLinkedList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for element in iter {
            list.push(element);
        }
        list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut list = Vec::new();
        let mut linked_list = linked_list.rev();
        while let Some(element) = linked_list.pop() {
            list.push(element);
        }
        list
    }
}
