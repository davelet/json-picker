use std::collections::VecDeque;
use std::fmt::Display;

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

impl<T: ToString + Clone> Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cp = Self { vec: self.vec.clone() };
        let mut result = String::from("<");
        let mut nod = cp.pop();
        while let Some(ref t) = nod {
            result.push_str(&*t.to_string());
            result.push(',');
            result.push(' ');
            nod = cp.pop();
        }
        if result.len() > 2 {
            result.pop();
            result.pop();
        }
        result.push('>');
        write!(f, "{}", result)
    }
}
