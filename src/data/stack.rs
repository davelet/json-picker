use std::collections::VecDeque;

#[derive(Clone)]
pub struct Stack<T> {
    vec: VecDeque<T>,
}

impl<T> Stack<T> {
    pub(crate) fn new() -> Self {
        Self {
            vec: VecDeque::new(),
        }
    }

    pub(crate) fn push(&mut self, item: T) {
        self.vec.push_back(item);
    }

    pub(crate) fn pop(&mut self) -> Option<T> {
        self.vec.pop_back()
    }

    pub(crate) fn peek(&self) -> Option<&T> {
        self.vec.back()
    }
}
