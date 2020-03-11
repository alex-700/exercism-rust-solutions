use core::mem::{replace, MaybeUninit};

#[derive(Default)]
pub struct CircularBuffer<T> {
    head: Option<usize>,
    tail: usize,
    data: Vec<MaybeUninit<T>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        CircularBuffer {
            head: None,
            tail: 0,
            data: std::iter::repeat_with(MaybeUninit::uninit)
                .take(capacity)
                .collect(),
        }
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }

    fn full(&self) -> bool {
        self.head.map_or(false, |head| head == self.tail)
    }

    fn inc(&self, idx: usize) -> usize {
        let ans = idx + 1;
        if ans == self.capacity() {
            0
        } else {
            ans
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.full() {
            return Err(Error::FullBuffer);
        }
        self.head.get_or_insert(self.tail);
        self.data[self.tail] = MaybeUninit::new(element);
        self.tail = self.inc(self.tail);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.head
            .take()
            .map(|head| {
                let ans = replace(&mut self.data[head], MaybeUninit::uninit());
                self.head = Some(self.inc(head)).filter(|&head| head != self.tail);
                unsafe { ans.assume_init() }
            })
            .ok_or(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        while self.head.is_some() {
            self.read().unwrap();
        }
    }

    pub fn overwrite(&mut self, element: T) {
        if self.full() {
            self.read().unwrap();
        }
        self.write(element).unwrap();
    }
}
