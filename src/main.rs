use topicos_especiais::{Tarefa, Prioridade};

fn main() {
    let tarefa = Tarefa::nova("Estudar Rust", "Estudar conceitos avançados de Rust.", "2024-12-05", Prioridade::Alta, 3);
    match tarefa {
        Ok(t) => println!("{:?}", t),
        Err(e) => eprintln!("Erro ao criar tarefa: {}", e),
    }
}