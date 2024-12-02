use topicos_especiais::{Tarefa, Prioridade, salvar_tarefas, carregar_tarefas, adicionar_tarefa, atualizar_tarefa, deletar_tarefa};
use std::fs;

fn main() {
    // Criando a lista de tarefas
    let mut tarefas = vec![
        Tarefa {
            nome: String::from("Estudar Rust"),
            descricao: String::from("Estudar conceitos avançados de Rust."),
            prazo: String::from("2024-12-05"),
            prioridade: Prioridade::Alta,
            duracao: 3,
        },
    ];

    // Salvar tarefas no arquivo
    if let Err(e) = salvar_tarefas(&tarefas, "tarefas.json") {
        eprintln!("Erro ao salvar tarefas: {}", e);
        return;
    }

    // Carregar as tarefas
    match carregar_tarefas("tarefas.json") {
        Ok(tarefas_carregadas) => println!("Tarefas carregadas: {:?}", tarefas_carregadas),
        Err(e) => eprintln!("Erro ao carregar tarefas: {}", e),
    }

    // Adicionar uma nova tarefa
    let nova_tarefa = Tarefa {
        nome: String::from("Fazer Exercício"),
        descricao: String::from("Fazer exercícios práticos de Rust."),
        prazo: String::from("2024-12-06"),
        prioridade: Prioridade::Media,
        duracao: 2,
    };
    adicionar_tarefa(&mut tarefas, nova_tarefa);

    // Atualizar uma tarefa existente
    let tarefa_atualizada = Tarefa {
        nome: String::from("Estudar Rust"),
        descricao: String::from("Estudar a fundo os conceitos de Rust."),
        prazo: String::from("2024-12-07"),
        prioridade: Prioridade::Alta,
        duracao: 4,
    };
    atualizar_tarefa(&mut tarefas, "Estudar Rust", tarefa_atualizada);

    // Deletar uma tarefa
    deletar_tarefa(&mut tarefas, "Fazer Exercício");

    // Salvar as tarefas atualizadas
    if let Err(e) = salvar_tarefas(&tarefas, "tarefas.json") {
        eprintln!("Erro ao salvar tarefas: {}", e);
        return;
    }

    // Exibir tarefas após as alterações
    println!("Tarefas após alterações: {:?}", tarefas);
}
