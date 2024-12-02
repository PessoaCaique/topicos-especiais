use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::prelude::*;

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

impl Tarefa {
    fn nova(
        nome: &str,
        descricao: &str,
        prazo: &str,
        prioridade: Prioridade,
        tempo_estimado: u32,
    ) -> Result<Self, String> {
        if nome.is_empty() || descricao.is_empty() {
            return Err("Nome e descrição são obrigatórios.".into());
        }
        if tempo_estimado == 0 {
            return Err("Tempo estimado deve ser maior que zero.".into());
        }
        let prazo = NaiveDate::parse_from_str(prazo, "%Y-%m-%d")
            .map_err(|_| "Data inválida. Use o formato YYYY-MM-DD.".to_string())?;

        Ok(Tarefa {
            nome: nome.to_string(),
            descricao: descricao.to_string(),
            prazo,
            prioridade,
            tempo_estimado,
        })
    }
}

fn salvar_tarefas(tarefas: &Vec<Tarefa>, arquivo: &str) -> std::io::Result<()> {
    let dados = serde_json::to_string(tarefas)?;
    let mut file = File::create(arquivo)?;
    file.write_all(dados.as_bytes())?;
    Ok(())
}

fn carregar_tarefas(arquivo: &str) -> std::io::Result<Vec<Tarefa>> {
    let dados = fs::read_to_string(arquivo)?;
    let tarefas: Vec<Tarefa> = serde_json::from_str(&dados)?;
    Ok(tarefas)
}

fn main() {
    match Tarefa::nova(
        "Estudar Rust",
        "Estudar conceitos avançados.",
        "2024-12-05",
        Prioridade::Alta,
        5,
    ) {
        Ok(tarefa) => {
            let tarefas = vec![tarefa];
            if let Err(e) = salvar_tarefas(&tarefas, "tarefas.json") {
                eprintln!("Erro ao salvar tarefas: {}", e);
            }
        }
        Err(e) => eprintln!("Erro ao criar tarefa: {}", e),
    }

    match carregar_tarefas("tarefas.json") {
        Ok(tarefas) => println!("{:?}", tarefas),
        Err(e) => eprintln!("Erro ao carregar tarefas: {}", e),
    }
}
