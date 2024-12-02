use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tarefa {
    pub nome: String,
    pub descricao: String,
    pub prazo: NaiveDate,
    pub prioridade: Prioridade,
    pub duracao: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Prioridade {
    Alta,
    Media,
    Baixa,
}

impl Tarefa {
    pub fn nova(nome: &str, descricao: &str, prazo: &str, prioridade: Prioridade, duracao: u32) -> Result<Self, String> {
        let prazo = NaiveDate::parse_from_str(prazo, "%Y-%m-%d")
            .map_err(|_| "Data inválida".to_string())?;
        
        Ok(Tarefa {
            nome: nome.to_string(),
            descricao: descricao.to_string(),
            prazo,
            prioridade,
            duracao,
        })
    }
}