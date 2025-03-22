// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

type Link<T> = *const Node<T>;

struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    _marker: std::marker::PhantomData<T>,
}

unsafe impl<T> Sync for LinkedList<T> {}

unsafe impl<T> Send for LinkedList<T> {}

enum Direction {
    Front,
    Back,
}

pub struct Cursor<'a, T> {
    current: Link<T>,
    direction: Direction,
    linked: &'a mut LinkedList<T>,
    _marker: std::marker::PhantomData<&'a mut T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
    _marker: std::marker::PhantomData<&'a T>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
            len: 0,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.head,
            direction: Direction::Front,
            linked: self,
            _marker: std::marker::PhantomData,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.tail,
            direction: Direction::Back,
            linked: self,
            _marker: std::marker::PhantomData,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: unsafe { self.head.as_ref() }, _marker: std::marker::PhantomData }
    }

    fn take(&mut self, node: Link<T>) -> Option<T> {
        if self.len == 0 || node.is_null() {
            return None;
        }
        unsafe {
            if let Some(next) = (*node).next.cast_mut().as_mut() {
                next.prev = (*node).prev;
            }
            if let Some(prev) = (*node).prev.cast_mut().as_mut() {
                prev.next = (*node).next;
            }
        }

        self.len -= 1;

        if self.len == 0 {
            self.head = std::ptr::null();
            self.tail = std::ptr::null();
        } else if self.head == node {
            self.head = unsafe { (*node).next }
        } else if self.tail == node {
            self.tail = unsafe { (*node).prev }
        }

        unsafe {
            // SAFETY: 将指针指向的内容转换为了Box，take方法结束时将销毁这个Box
            Some(Box::from_raw(node as *mut Node<T>).value)
        }
    }

    fn insert_after(&mut self, node: Link<T>, value: T) -> Link<T> {
        self.len += 1;
        let item = Box::into_raw(Box::new(Node {
            value,
            prev: std::ptr::null(),
            next: std::ptr::null(),
        }));

        unsafe {
            if let Some(node) = node.cast_mut().as_mut() {
                if let Some(next) = node.next.cast_mut().as_mut() {
                    next.prev = item;
                    (*item).next = next;
                }
                node.next = item;
                (*item).prev = node;
            } else {
                self.head = item;
                self.tail = item;
            }
        }
        
        if self.tail == node {
            self.tail = item;
        }

        item
    }

    fn insert_before(&mut self, node: Link<T>, value: T) -> Link<T> {
        self.len += 1;
        let item = Box::into_raw(Box::new(Node {
            value,
            prev: std::ptr::null(),
            next: std::ptr::null(),
        }));

        unsafe {
            if let Some(node) = node.cast_mut().as_mut() {
                if let Some(prev) = node.prev.cast_mut().as_mut() {
                    prev.next = item;
                    (*item).prev = prev;
                }
                node.prev = item;
                (*item).next = node;
            } else {
                self.head = item;
                self.tail = item;
            }
        }
        
        if self.head == node {
            self.head = item;
        }

        item
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.take(self.head).is_some() {
            
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.current.is_null() {
            return None;
        }
        unsafe { self.current.cast_mut().as_mut().map(|x| &mut x.value) }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        if self.current.is_null() {
            return None;
        }

        unsafe {
            self.current = (*self.current).next;
            self.peek_mut()
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        if self.current.is_null() {
            return None;
        }

        unsafe {
            self.current = (*self.current).prev;
            self.peek_mut()
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if self.current.is_null() {
            return None;
        }

        unsafe {
            let node = self.current;
            self.current = match self.direction {
                Direction::Front => (*node).next,
                Direction::Back => (*node).prev,
            };
            let res = self.linked.take(node);
            if self.current.is_null() {
                self.current = self.linked.tail
            }
            res
        }
    }

    pub fn insert_after(&mut self, element: T) {
        let cur = self.linked.insert_after(self.current, element);
        if self.current.is_null() {
            self.current = cur;
        }
    }

    pub fn insert_before(&mut self, element: T) {
        let cur = self.linked.insert_before(self.current, element);
        if self.current.is_null() {
            self.current = cur;
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.next.map(|node| {
            self.next = unsafe { node.next.as_ref() };
            &node.value
        })
    }
}
