use serde::{Serialize, Deserialize};
use std::fs::{File, self};
use std::io::{Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tarefa {
    pub nome: String,
    pub descricao: String,
    pub prazo: String,
    pub duracao: u32,
    pub prioridade: Prioridade,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Prioridade {
    Alta,
    Media,
    Baixa,
}

impl Tarefa {
    // Método de validação de dados
    pub fn validar(&self) -> Result<(), String> {
        if self.nome.trim().is_empty() {
            return Err("Nome da tarefa não pode ser vazio.".to_string());
        }
        if self.descricao.trim().is_empty() {
            return Err("Descrição da tarefa não pode ser vazia.".to_string());
        }
        if self.prazo.trim().is_empty() {
            return Err("Prazo da tarefa não pode ser vazio.".to_string());
        }
        if self.duracao <= 0 {
            return Err("A duração da tarefa deve ser maior que zero.".to_string());
        }
        Ok(())
    }
}

// Função para criar tarefa
pub fn criar_tarefa(nome: &str, descricao: &str, prazo: &str, duracao: u32, prioridade: Prioridade) -> Result<Tarefa, String> {
    let tarefa = Tarefa {
        nome: nome.to_string(),
        descricao: descricao.to_string(),
        prazo: prazo.to_string(),
        duracao,
        prioridade,
    };

    // Validando a tarefa
    tarefa.validar()?;

    Ok(tarefa)
}

// Função para salvar tarefas em um arquivo
pub fn salvar_tarefas(tarefas: &Vec<Tarefa>, arquivo: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dados = serde_json::to_string(tarefas)?;

    let mut file = File::create(arquivo)?;
    file.write_all(dados.as_bytes())?;
    Ok(())
}

// Função para carregar tarefas de um arquivo
pub fn carregar_tarefas(arquivo: &str) -> Result<Vec<Tarefa>, Box<dyn std::error::Error>> {
    let dados = fs::read_to_string(arquivo)?;
    let tarefas: Vec<Tarefa> = serde_json::from_str(&dados)?;
    Ok(tarefas)
}

// Função para atualizar uma tarefa existente
pub fn atualizar_tarefa(tarefas: &mut Vec<Tarefa>, nome: &str, nova_tarefa: Tarefa) -> Result<(), String> {
    nova_tarefa.validar()?;

    for tarefa in tarefas.iter_mut() {
        if tarefa.nome == nome {
            *tarefa = nova_tarefa;
            return Ok(());
        }
    }

    Err("Tarefa não encontrada.".to_string())
}

// Função para remover uma tarefa
pub fn remover_tarefa(tarefas: &mut Vec<Tarefa>, nome: &str) -> Result<(), String> {
    if let Some(pos) = tarefas.iter().position(|t| t.nome == nome) {
        tarefas.remove(pos);
        Ok(())
    } else {
        Err("Tarefa não encontrada.".to_string())
    }
}
