use serde::{Serialize, Deserialize};
use serde_json::{self, Error};
use std::fs::{self, File};
use std::io::prelude::*;

// Estrutura de dados Tarefa
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)] // Adicionando o trait Clone
pub struct Tarefa {
    pub nome: String,
    pub descricao: String,
    pub prazo: String,
    pub prioridade: Prioridade,
    pub duracao: u32,
}

// Enum para Prioridade
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Prioridade {
    Alta,
    Media,
    Baixa,
}

// Função para salvar tarefas no arquivo
pub fn salvar_tarefas(tarefas: &Vec<Tarefa>, arquivo: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dados = serde_json::to_string(tarefas)?;
    let mut file = File::create(arquivo)?;
    file.write_all(dados.as_bytes())?;
    Ok(())
}

// Função para carregar tarefas do arquivo
pub fn carregar_tarefas(arquivo: &str) -> Result<Vec<Tarefa>, Box<dyn std::error::Error>> {
    let dados = fs::read_to_string(arquivo)?;
    let tarefas: Vec<Tarefa> = serde_json::from_str(&dados)?;
    Ok(tarefas)
}

pub fn adicionar_tarefa(tarefas: &mut Vec<Tarefa>, nova_tarefa: Tarefa) {
    tarefas.push(nova_tarefa);
}

pub fn atualizar_tarefa(tarefas: &mut Vec<Tarefa>, nome: &str, nova_tarefa: Tarefa) {
    for tarefa in tarefas.iter_mut() {
        if tarefa.nome == nome {
            *tarefa = nova_tarefa.clone();  // Clone a tarefa para evitar o movimento
        }
    }
}


// Função para deletar uma tarefa
pub fn deletar_tarefa(tarefas: &mut Vec<Tarefa>, nome: &str) {
    tarefas.retain(|tarefa| tarefa.nome != nome);
}
