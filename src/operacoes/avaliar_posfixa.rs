use crate::data_structures::stack::Stack;

pub fn avaliar_posfixa(expressao: &str) -> String {
    let tokens: Vec<&str> = expressao.split_whitespace().collect();

    let mut pilha = Stack::new(100);

    let lista_operadores: Vec<&str> = ["+", "-", "*", "/", "^"].to_vec();

    for token in tokens {
        println!("Analisando Token: {}", token);
        if !lista_operadores.contains(&token) {
            pilha.push(token.to_string());
            println!("Pilha: {:?}", pilha.data);
            continue;
        }

        if lista_operadores.contains(&token) {
            if pilha.len() < 2 {
                panic!(
                    "
                    Pilha não possui elementos suficientes para realizar a operação.
                    Pilha: {:?}
                    Operador: {}",
                    pilha.data, token
                );
            }

            let operando1: i32 = pilha.pop().parse().expect("Error ao transformar");
            let operando2: i32 = pilha.pop().parse().expect("Error ao transformar");

            let resultado = match token {
                "+" => operando2 + operando1,
                "-" => operando2 - operando1,
                "*" => operando2 * operando1,
                "/" => {
                    if operando1 == 0 {
                        panic!("Divisão por zero não é permitida.");
                    }
                    operando2 / operando1
                }
                "^" => operando2.pow(operando1 as u32),
                _ => unreachable!(),
            };
            pilha.push(resultado.to_string());
        }
        println!("Pilha: {:?}", pilha.data);
        continue;
    }

    if !pilha.top().is_some() {
        panic!("Não há elementos na pilha")
    }

    return pilha.top().unwrap().to_string();
}
