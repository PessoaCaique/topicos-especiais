use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tarefa {
    pub titulo: String,
    pub prioridade: u8,
    pub data_limite: String,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Pendente,
    EmAndamento,
    Concluiddo
}

fn main() {
}