#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_tarefa_valida() {
        let tarefa = Tarefa {
            nome: "Estudar Rust".to_string(),
            descricao: "Estudar conceitos avançados de Rust.".to_string(),
            prazo: "2024-12-05".to_string(),
            duracao: 3,
            prioridade: Prioridade::Alta,
        };
        assert!(tarefa.validar().is_ok());
    }

    #[test]
    fn test_validar_tarefa_nome_vazio() {
        let tarefa = Tarefa {
            nome: "".to_string(),
            descricao: "Descrição válida.".to_string(),
            prazo: "2024-12-05".to_string(),
            duracao: 3,
            prioridade: Prioridade::Alta,
        };
        assert_eq!(tarefa.validar().unwrap_err(), "Nome da tarefa não pode ser vazio.");
    }

    #[test]
    fn test_validar_tarefa_duracao_zero() {
        let tarefa = Tarefa {
            nome: "Estudar Rust".to_string(),
            descricao: "Descrição válida.".to_string(),
            prazo: "2024-12-05".to_string(),
            duracao: 0,
            prioridade: Prioridade::Alta,
        };
        assert_eq!(tarefa.validar().unwrap_err(), "A duração da tarefa deve ser maior que zero.");
    }
}
