use serde::{Serialize, Deserialize};
use serde_json::{Result};
use std::fs::{self, File};
use std::io::prelude::*;
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Debug)]
struct Tarefa {
    nome: String,
    descricao: String,
    prazo: NaiveDate,
    prioridade: Prioridade,
    tempo_estimado: u32,
}

#[derive(Serialize, Deserialize, Debug)]
enum Prioridade {
    Alta,
    Media,
    Baixa,
}

fn salvar_tarefas(tarefas: &Vec<Tarefa>, arquivo: &str) -> Result<()> {
    let dados = serde_json::to_string(tarefas)?;
    let mut file = File::create(arquivo)?;
    file.write_all(dados.as_bytes())?;
    Ok(())
}

fn carregar_tarefas(arquivo: &str) -> Result<Vec<Tarefa>> {
    let dados = fs::read_to_string(arquivo)?;
    let tarefas: Vec<Tarefa> = serde_json::from_str(&dados)?;
    Ok(tarefas)
}

fn main() {
    let tarefas = vec![
        Tarefa {
            nome: String::from("Estudar Rust"),
            descricao: String::from("Estudar conceitos avançados de Rust."),
            prazo: NaiveDate::from_ymd(2024, 12, 5),
            prioridade: Prioridade::Alta,
            tempo_estimado: 5,
        },
    ];

    if let Err(e) = salvar_tarefas(&tarefas, "tarefas.json") {
        eprintln!("Erro ao salvar tarefas: {}", e);
    }

    match carregar_tarefas("tarefas.json") {
        Ok(tarefas) => println!("{:?}", tarefas),
        Err(e) => eprintln!("Erro ao carregar tarefas: {}", e),
    }
}
