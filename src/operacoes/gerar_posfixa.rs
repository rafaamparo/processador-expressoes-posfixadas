use::std::collections::HashMap;

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

    fn push(&mut self, value: &str) {
        if self.data.len() < self.size as usize {
            self.data.push(value.to_string());
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

pub fn infixa_para_posfixa(expressao: String) -> String {
    let tokens_assoc = HashMap::from([
    ("(", 0),
    (")", 0),
    ("+", 1),
    ("-", 1),
    ("*", 2),
    ("/", 2),
    ("^", 3),
]);
    let binding = expressao.replace("(", " ( ").replace(")", " ) ");
    let tokens: Vec<&str> = binding.split_whitespace().collect();
    let mut pilha = Stack::new(100);
    let mut saida: Stack = Stack::new(100);
    for token in tokens{
        if !(tokens_assoc.contains_key(token)){
            saida.push(token);
            continue;
        }
        if token =="("{
            pilha.push(token);
            continue;
        } if token == ")"{
            let mut ultimo_elemento = pilha.top();
            while ultimo_elemento.is_some() && ultimo_elemento.unwrap() != "("{
                saida.push(ultimo_elemento.unwrap());
                pilha.pop();
                ultimo_elemento = pilha.top();
            }
            if ultimo_elemento.is_some() && ultimo_elemento.unwrap() == "("{
               pilha.pop();
            }
            continue;
        } if tokens_assoc.contains_key(token){
            let prioridade_token_atual = tokens_assoc.get(token).unwrap();
            let mut ultimo_elemento = pilha.top().unwrap();
            let mut prioridade_ultimo_elemento = tokens_assoc.get(ultimo_elemento.as_str()).unwrap();
            while prioridade_token_atual <= prioridade_ultimo_elemento{
                saida.push(&ultimo_elemento);
                pilha.pop();
                ultimo_elemento = pilha.top().unwrap();
                prioridade_ultimo_elemento = tokens_assoc.get(ultimo_elemento.as_str()).unwrap();
            }
            if prioridade_token_atual > prioridade_ultimo_elemento{
                pilha.push(token);
            }
        }
    }

    return saida.data.join(" ");
}