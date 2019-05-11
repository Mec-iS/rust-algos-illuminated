//! Examples from other books

/// Implementing simple queue of characters
/// Programming Rust, chapter 9
pub struct Queue {
    pub q_out: Vec<char>,
    pub q_in: Vec<char>,
}

impl Queue {
    /// push to the queue
    pub fn push(&mut self, element: char) -> () {
        self.q_in.push(element);
    }

    /// pop from the queue using clone, check if the queue is empty, return Some(c)
    pub fn pop_raw(&mut self) -> Option<char> {
        if self.q_out.is_empty() {
            if self.q_in.is_empty() {
                return None;
            }

            // move in-elements to out and reverse them to out-ordered
            // raw solution:
            self.q_out = self.q_in.clone();
            self.q_out.reverse();

            self.q_in = vec![];
        }

        self.q_out.pop()
    }

    /// pop from the queue using swap, check if the queue is empty, return Some(c)
    pub fn pop_swap(&mut self) -> Option<char> {
        if self.q_out.is_empty() {
            if self.q_in.is_empty() {
                return None;
            }

            // move in-elements to out and reverse them to out-ordered
            // original solution:
            use std::mem::swap;
            swap(&mut self.q_in, &mut self.q_out);
            self.q_out.reverse();

            self.q_in = vec![];
        }

        self.q_out.pop()
    }
}
