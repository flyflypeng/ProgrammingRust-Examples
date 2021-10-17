use std::mem::swap;

pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

fn main() {
    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };

    q.push('P');
    q.push('D');

    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();
    // now q is uninitialized
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);
}
