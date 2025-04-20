pub struct Stack {
    pub size: i32,
    pub data: Vec<String>,
}

impl Stack {
    pub fn new(size: i32) -> Stack {
        Stack {
            size,
            data: Vec::with_capacity(size as usize),
        }
    }

    pub fn push(&mut self, value: String) {
        if self.data.len() < self.size as usize {
            self.data.push(value);
        } else {
            panic!("Stack overflow");
        }
    }

    pub fn pop(&mut self) -> String {
        if !self.data.is_empty() {
            self.data.pop().unwrap()
        } else {
            panic!("Stack underflow")
        }
    }

    pub fn top(&self) -> Option<&String> {
        self.data.last()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
