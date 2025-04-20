use crate::data_structures::stack::Stack;
use std::collections::HashMap;

pub fn infixa_para_posfixa(expressao: &str) -> String {
    let binding = expressao.replace("(", " ( ").replace(")", " ) ");
    let tokens: Vec<&str> = binding.split_whitespace().collect();

    let mut pilha = Stack::new(100);
    let mut saida: Stack = Stack::new(100);

    let tokens_assoc = HashMap::from([
        ("(", 0),
        (")", 0),
        ("+", 1),
        ("-", 1),
        ("*", 2),
        ("/", 2),
        ("^", 3),
    ]);

    for token in tokens {
        println!("Analisando Token: {}", token);
        if !tokens_assoc.contains_key(token) {
            saida.push(token.to_string());
            println!("Pilha: {:?}", pilha.data);
            println!("Saída: {:?}", saida.data);
            continue;
        }

        if token == "(" {
            pilha.push(token.to_string());
            println!("Pilha: {:?}", pilha.data);
            println!("Saída: {:?}", saida.data);
            continue;
        }

        if token == ")" {
            let mut ultimo_elemento = pilha.top();

            while ultimo_elemento.is_some() && ultimo_elemento.unwrap() != "(" {
                saida.push(ultimo_elemento.unwrap().to_string());
                pilha.pop();
                ultimo_elemento = pilha.top();
            }
            if ultimo_elemento.is_some() && ultimo_elemento.unwrap() == "(" {
                pilha.pop(); // Remove o "(" da pilha
            }
            println!("Pilha: {:?}", pilha.data);
            println!("Saída: {:?}", saida.data);
            continue;
        }

        if tokens_assoc.contains_key(token) {
            let prioridade_token = tokens_assoc.get(token).unwrap();
            if pilha.top().is_some() {
                let mut ultimo_elemento = pilha.top().unwrap();
                let mut prioridade_ultimo_elemento =
                    tokens_assoc.get(ultimo_elemento.as_str()).unwrap();

                while prioridade_token <= prioridade_ultimo_elemento {
                    saida.push(ultimo_elemento.to_string());
                    pilha.pop();
                    ultimo_elemento = pilha.top().unwrap();
                    prioridade_ultimo_elemento =
                        tokens_assoc.get(ultimo_elemento.as_str()).unwrap();
                }
            }
            pilha.push(token.to_string());
        }
        println!("Pilha: {:?}", pilha.data);
        println!("Saída: {:?}", saida.data);
    }

    println!("-> Desempilhando resto");
    while !pilha.data.is_empty() {
        let ultimo_elemento = pilha.top().unwrap();
        if ultimo_elemento != "(" {
            saida.push(ultimo_elemento.to_string());
            pilha.pop();
        } else {
            pilha.pop(); // Remove o "(" da pilha
        }
    }

    println!("Pilha: {:?}", pilha.data);
    println!("Saída: {:?}", saida.data);

    return saida.data.join(" ");
}
