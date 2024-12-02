// tests/main.rs
use topicos_especiais::{Tarefa, Prioridade};

#[test]
fn criar_tarefa_valida() {
    let tarefa = Tarefa::nova("Teste", "Descrição", "2024-12-01", Prioridade::Media, 3);
    assert!(tarefa.is_ok());
}

#[test]
fn criar_tarefa_com_erro() {
    let tarefa = Tarefa::nova("", "Descrição", "2024-12-01", Prioridade::Media, 0);
    assert!(tarefa.is_err());
}

#[test]
fn criar_tarefa_com_data_invalida() {
    let tarefa = Tarefa::nova("Teste", "Descrição", "2024-31-12", Prioridade::Baixa, 5);
    assert!(tarefa.is_err());
}
