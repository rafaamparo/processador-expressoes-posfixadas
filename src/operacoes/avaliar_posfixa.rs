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

pub fn avalia_posfixa(expressao: String) -> String {
    let tokens_assoc: Vec<&str> = [ "+", "-", "*", "/", "^"].to_vec();
    let tokens: Vec<&str> = expressao.split_whitespace().collect();
    let mut pilha = Stack::new(100);
    for token in tokens{
        if !(tokens_assoc.contains(&token)){
            pilha.push(token);
            continue;
        } if tokens_assoc.contains(&token){
            let operando1: i32 = pilha.top().unwrap().parse().expect("erro");
            pilha.pop();
            let operando2: i32 = pilha.top().unwrap().parse().expect("erro");
            pilha.pop();
            let resultado = match token {
                "+" => operando2 + operando1,
                "-" => operando2 - operando1,
                "*" => operando2 * operando1,
                "/" => {{if operando1 == 0 {panic!("divisão por zero não é permitida");}} operando2/operando1}, 
                "^" => operando2.pow(operando1 as u32),
                _ => unreachable!()
            };
            pilha.push(resultado.to_string().as_str());
            continue;
        }
    }
    if !(pilha.top().is_some()){
        panic!("Pilha vazia")
    }

    return pilha.top().unwrap().to_string();
}