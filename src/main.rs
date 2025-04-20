mod operacoes {
    pub mod avaliar_posfixa;
    pub mod gerar_posfixa;
}
mod data_structures {
    pub mod stack;
}

use operacoes::avaliar_posfixa::avaliar_posfixa;
use operacoes::gerar_posfixa::infixa_para_posfixa;
use std::io;

fn main() {
    'fluxo_app: loop {
        let mut opcao = String::new();
        println!("Olá, selecione uma opção:");
        println!("1. Gerar expressão pós fixada");
        println!("2. Avaliar expressão pós fixada");
        println!("3. Sair");

        io::stdin()
            .read_line(&mut opcao)
            .expect("Falha ao ler a entrada");
        let opcao: i32 = opcao.trim().parse().expect("Entrada inválida");
        match opcao {
            1 => {
                println!("Digite a expressão infixa:");
                let mut expressao = String::new();
                io::stdin()
                    .read_line(&mut expressao)
                    .expect("Falha ao ler a entrada");
                let expressao = expressao.trim();
                let resultado = infixa_para_posfixa(expressao);
                println!("Expressão pós-fixada: {}", resultado);
            }
            2 => {
                println!("Digite a expressão pós fixada:");
                let mut expressao = String::new();
                io::stdin()
                    .read_line(&mut expressao)
                    .expect("Falha ao ler a entrada");
                let expressao = expressao.trim();
                let resultado = avaliar_posfixa(expressao);
                println!("Resultado da avaliação: {}", resultado);
            }
            3 => {
                println!("Saindo...");
                break 'fluxo_app;
            }
            _ => {
                println!("Opção inválida.");
            }
        }
    }
}
