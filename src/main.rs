use std::io::{self, Write};
use topicos_especiais::{Tarefa, Prioridade, criar_tarefa, salvar_tarefas, carregar_tarefas, atualizar_tarefa, remover_tarefa};

fn main() {
    let mut tarefas = Vec::new();

    // Menu de interação
    loop {
        println!("Escolha uma opção:");
        println!("1. Adicionar Tarefa");
        println!("2. Listar Tarefas");
        println!("3. Atualizar Tarefa");
        println!("4. Remover Tarefa");
        println!("5. Salvar Tarefas em Arquivo");
        println!("6. Carregar Tarefas de Arquivo");
        println!("7. Sair");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).unwrap();
        let opcao: u32 = opcao.trim().parse().unwrap();

        match opcao {
            1 => {
                // Adicionar tarefa
                let mut nome = String::new();
                let mut descricao = String::new();
                let mut prazo = String::new();
                let mut duracao = String::new();
                let mut prioridade = String::new();

                println!("Digite o nome da tarefa:");
                io::stdin().read_line(&mut nome).unwrap();

                println!("Digite a descrição da tarefa:");
                io::stdin().read_line(&mut descricao).unwrap();

                println!("Digite o prazo da tarefa (formato YYYY-MM-DD):");
                io::stdin().read_line(&mut prazo).unwrap();

                println!("Digite a duração da tarefa (em horas):");
                io::stdin().read_line(&mut duracao).unwrap();
                let duracao: u32 = duracao.trim().parse().unwrap();

                println!("Digite a prioridade (Alta, Média, Baixa):");
                io::stdin().read_line(&mut prioridade).unwrap();

                let prioridade = match prioridade.trim() {
                    "Alta" => Prioridade::Alta,
                    "Média" => Prioridade::Media,
                    "Baixa" => Prioridade::Baixa,
                    _ => Prioridade::Media,
                };

                match criar_tarefa(&nome.trim(), &descricao.trim(), &prazo.trim(), duracao, prioridade) {
                    Ok(tarefa) => {
                        tarefas.push(tarefa);
                        println!("Tarefa adicionada com sucesso!");
                    },
                    Err(e) => println!("Erro ao adicionar tarefa: {}", e),
                }
            },
            2 => {
                // Listar tarefas
                println!("Tarefas:");
                for tarefa in &tarefas {
                    println!("{:?}", tarefa);
                }
            },
            3 => {
                // Atualizar tarefa
                let mut nome = String::new();
                println!("Digite o nome da tarefa que deseja atualizar:");
                io::stdin().read_line(&mut nome).unwrap();

                let mut descricao = String::new();
                let mut prazo = String::new();
                let mut duracao = String::new();
                let mut prioridade = String::new();

                println!("Digite a nova descrição da tarefa:");
                io::stdin().read_line(&mut descricao).unwrap();

                println!("Digite o novo prazo da tarefa (formato YYYY-MM-DD):");
                io::stdin().read_line(&mut prazo).unwrap();

                println!("Digite a nova duração da tarefa (em horas):");
                io::stdin().read_line(&mut duracao).unwrap();
                let duracao: u32 = duracao.trim().parse().unwrap();

                println!("Digite a nova prioridade (Alta, Média, Baixa):");
                io::stdin().read_line(&mut prioridade).unwrap();

                let prioridade = match prioridade.trim() {
                    "Alta" => Prioridade::Alta,
                    "Média" => Prioridade::Media,
                    "Baixa" => Prioridade::Baixa,
                    _ => Prioridade::Media,
                };

                let nova_tarefa = Tarefa {
                    nome: nome.trim().to_string(),
                    descricao: descricao.trim().to_string(),
                    prazo: prazo.trim().to_string(),
                    duracao,
                    prioridade,
                };

                match atualizar_tarefa(&mut tarefas, nome.trim(), nova_tarefa) {
                    Ok(_) => println!("Tarefa atualizada com sucesso!"),
                    Err(e) => println!("{}", e),
                }
            },
            4 => {
                // Remover tarefa
                let mut nome = String::new();
                println!("Digite o nome da tarefa que deseja remover:");
                io::stdin().read_line(&mut nome).unwrap();

                match remover_tarefa(&mut tarefas, nome.trim()) {
                    Ok(_) => println!("Tarefa removida com sucesso!"),
                    Err(e) => println!("{}", e),
                }
            },
            5 => {
                // Salvar tarefas
                match salvar_tarefas(&tarefas, "tarefas.json") {
                    Ok(_) => println!("Tarefas salvas com sucesso!"),
                    Err(e) => println!("Erro ao salvar tarefas: {}", e),
                }
            },
            6 => {
                // Carregar tarefas
                match carregar_tarefas("tarefas.json") {
                    Ok(tarefas_carregadas) => {
                        tarefas = tarefas_carregadas;
                        println!("Tarefas carregadas com sucesso!");
                    },
                    Err(e) => println!("Erro ao carregar tarefas: {}", e),
                }
            },
            7 => {
                break;
            },
            _ => {
                println!("Opção inválida!");
            },
        }
    }
}
