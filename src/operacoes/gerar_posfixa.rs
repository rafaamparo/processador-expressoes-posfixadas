struct Stack {
    size: i32,
    data: Vec<String>,
}

impl Stack {
    fn new(size: i32) -> Stack {
        Stack {
            size,
            data: Vec::with_capacity(size as usize),
        }
    }

    fn push(&mut self, value: String) {
        if self.data.len() < self.size as usize {
            self.data.push(value);
        } else {
            panic!("Stack overflow");
        }
    }

    fn pop(&mut self) -> String {
        if !self.data.is_empty() {
            self.data.pop().unwrap()
        } else {
            panic!("Stack underflow")
        }
    }

    fn top(&self) -> Option<&String> {
        self.data.last()
    }
}

pub fn infixa_para_posfixa(expressao: &str) -> &str {
    let binding = expressao.replace("(", " ( ").replace(")", " ) ");
    let tokens: Vec<&str> = binding.split_whitespace().collect();
    println!("Tokens: {:?}", tokens);

    let mut pilha = Stack::new(100);
    let mut saida: Stack = Stack::new(100);

    return expressao;
}