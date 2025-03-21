use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
    data: Option<T>,
    next: Option<Link<T>>,
}

pub struct CircularBuffer<T: std::fmt::Debug> {
    write: Link<T>,
    read: Link<T>,
    count: usize,
    capacity: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone + std::fmt::Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("Capacity cannot be zero.");
        }
        let mut list: Vec<Link<T>> = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            let node = Rc::new(RefCell::new(Node {
                data: None,
                next: None,
            }));
            if let Some(next) = list.last() {
                node.borrow_mut().next = Some(Rc::clone(next));
            }
            list.push(node);
        }

        list[0].borrow_mut().next = Some(Rc::clone(&list[capacity - 1]));

        Self {
            write: list[0].clone(),
            read: list[0].clone(),
            count: 0,
            capacity,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.count == self.capacity {
            return Err(Error::FullBuffer);
        }
        self.write.borrow_mut().data = Some(element);
        let write = self.write.borrow_mut().next.clone().unwrap();
        self.write = write;
        self.count += 1;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.count == 0 {
            return Err(Error::EmptyBuffer);
        }
        let data = self.read.borrow_mut().data.take().unwrap();
        let read = self.read.borrow_mut().next.clone().unwrap();
        self.read = read;
        self.count -= 1;
        Ok(data)
    }

    pub fn clear(&mut self) {
        while self.read().is_ok() {}
    }

    pub fn overwrite(&mut self, element: T) {
        if self.write(element.clone()).is_err() {
            self.read().unwrap();
            self.write(element).unwrap();
        }
    }
}

impl<T: std::fmt::Debug> Drop for CircularBuffer<T> {
    fn drop(&mut self) {
        let mut next = self.write.borrow_mut().next.take();
        while let Some(n) = next.take() {
            next = n.borrow_mut().next.take();
        }
    }
}
