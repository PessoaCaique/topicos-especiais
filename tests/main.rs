// tests/main.rs
use topicos_especiais::{
    atualizar_tarefa, criar_tarefa, listar_tarefas, remover_tarefa, Prioridade, Tarefa,
};

#[test]
fn test_criar_tarefa() {
    let nome = "Estudar Rust".to_string();
    let descricao = "Estudar conceitos avançados de Rust.".to_string();
    let prazo = "2024-12-05".to_string();
    let duracao = 3;
    let prioridade = Prioridade::Alta;

    // Clonando o valor de `prioridade` para usá-lo novamente após movê-lo
    let tarefa = criar_tarefa(&nome, &descricao, &prazo, duracao, prioridade.clone()).unwrap();

    assert_eq!(tarefa.nome, nome);
    assert_eq!(tarefa.descricao, descricao);
    assert_eq!(tarefa.prazo, prazo);
    assert_eq!(tarefa.duracao, duracao);
    assert_eq!(tarefa.prioridade, prioridade); // Aqui `prioridade` pode ser usada novamente
}

#[test]
fn test_atualizar_tarefa() {
    let mut tarefas = vec![criar_tarefa(
        "Estudar Rust",
        "Estudar conceitos básicos de Rust",
        "2024-12-05",
        3,
        Prioridade::Alta,
    )
    .unwrap()];

    let nome = "Estudar Rust".to_string();

    // Crie uma nova tarefa com a nova prioridade e os outros campos que permanecem iguais
    let nova_tarefa = Tarefa {
        nome: "Estudar Rust".to_string(),
        descricao: "Estudar conceitos básicos de Rust".to_string(),
        prazo: "2024-12-05".to_string(),
        duracao: 3,
        prioridade: Prioridade::Media, // nova prioridade
    };

    // Passa a tarefa completa para o método atualizar_tarefa
    let resultado = atualizar_tarefa(&mut tarefas, &nome, nova_tarefa);

    // Assegure que a atualização foi bem-sucedida
    assert_eq!(resultado.is_ok(), true);
    assert_eq!(tarefas[0].prioridade, Prioridade::Media); // verifica se a prioridade foi atualizada
}

#[test]
fn test_remover_tarefa() {
    let nome = "Estudar Rust".to_string();
    let descricao = "Estudar conceitos avançados de Rust.".to_string();
    let prazo = "2024-12-05".to_string();
    let duracao = 3;
    let prioridade = Prioridade::Alta;

    let mut tarefas = Vec::new();
    let tarefa = criar_tarefa(&nome, &descricao, &prazo, duracao, prioridade).unwrap();
    tarefas.push(tarefa);

    remover_tarefa(&mut tarefas, &nome); // Agora passamos a lista de tarefas e o nome da tarefa a ser removida

    assert_eq!(tarefas.len(), 0); // A lista de tarefas deve estar vazia após a remoção
}

#[test]
fn test_listar_tarefas() {
    let mut tarefas = Vec::new();

    let tarefa1 = criar_tarefa(
        &"Estudar Rust".to_string(),
        &"Estudar conceitos básicos de Rust.".to_string(),
        &"2024-12-05".to_string(),
        3,
        Prioridade::Alta,
    )
    .unwrap();
    let tarefa2 = criar_tarefa(
        &"Praticar Rust".to_string(),
        &"Exercícios para fixar conhecimento.".to_string(),
        &"2024-12-10".to_string(),
        2,
        Prioridade::Media,
    )
    .unwrap();

    tarefas.push(tarefa1);
    tarefas.push(tarefa2);

    listar_tarefas(&tarefas);
}
