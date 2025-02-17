#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod gerenciador_tarefas {
    use ink::prelude::{string::String, vec::Vec};
    use ink::storage::Mapping;

    #[derive(scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Tarefa {
        pub id: u32,
        pub nome: String,
        pub descricao: String,
        pub prazo: String,
        pub duracao: u32,
        pub prioridade: Prioridade,
        pub concluida: bool,
    }

    impl Clone for Tarefa {
        fn clone(&self) -> Self {
            Self {
                id: self.id,
                nome: self.nome.clone(),
                descricao: self.descricao.clone(),
                prazo: self.prazo.clone(),
                duracao: self.duracao,
                prioridade: self.prioridade.clone(),
                concluida: self.concluida,
            }
        }
    }
    
    impl PartialEq for Tarefa {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id &&
            self.nome == other.nome &&
            self.descricao == other.descricao &&
            self.prazo == other.prazo &&
            self.duracao == other.duracao &&
            self.prioridade == other.prioridade &&
            self.concluida == other.concluida
        }
    }
    
    impl Eq for Tarefa {}

    #[repr(u8)]
    #[derive(Clone, scale::Encode, scale::Decode, PartialEq, Eq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Prioridade {
        Baixa = 0,
        Media = 1,
        Alta = 2,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct GerenciadorTarefas {
        tarefas: Mapping<u32, Tarefa>,
        next_id: u32,
    }

    impl GerenciadorTarefas {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn criar_tarefa(
            &mut self,
            nome: String,
            descricao: String,
            prazo: String,
            duracao: u32,
            prioridade: Prioridade,
        ) -> Result<u32, String> {
            if nome.is_empty() || descricao.is_empty() || prazo.is_empty() {
                return Err("Campos não podem estar vazios".into());
            }

            let id = self.next_id;
            let tarefa = Tarefa {
                id,
                nome,
                descricao,
                prazo,
                duracao,
                prioridade,
                concluida: false,
            };

            self.tarefas.insert(id, &tarefa);
            self.next_id = self.next_id.checked_add(1).ok_or("ID overflow")?;

            Ok(id)
        }

        #[ink(message)]
        pub fn atualizar_tarefa(
            &mut self,
            id: u32,
            nome: String,
            descricao: String,
            prazo: String,
            duracao: u32,
            prioridade: Prioridade,
        ) -> Result<(), String> {
            let mut tarefa = self.tarefas.get(id).ok_or("Tarefa não encontrada")?;

            if nome.is_empty() || descricao.is_empty() || prazo.is_empty() {
                return Err("Campos não podem estar vazios".into());
            }

            tarefa.nome = nome;
            tarefa.descricao = descricao;
            tarefa.prazo = prazo;
            tarefa.duracao = duracao;
            tarefa.prioridade = prioridade;

            self.tarefas.insert(id, &tarefa);
            Ok(())
        }

        #[ink(message)]
        pub fn marcar_concluida(&mut self, id: u32) -> Result<(), String> {
            let mut tarefa = self.tarefas.get(id).ok_or("Tarefa não encontrada")?;
            tarefa.concluida = true;
            self.tarefas.insert(id, &tarefa);
            Ok(())
        }

        #[ink(message)]
        pub fn remover_tarefa(&mut self, id: u32) -> Result<(), String> {
            if self.tarefas.contains(id) {
                self.tarefas.remove(id);
                Ok(())
            } else {
                Err("Tarefa não encontrada".into())
            }
        }

        #[ink(message)]
        pub fn listar_tarefas(&self) -> Vec<Tarefa> {
            let mut lista = Vec::new();
            for id in 0..self.next_id {
                if let Some(tarefa) = self.tarefas.get(id) {
                    lista.push(tarefa);
                }
            }
            lista
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_criar_tarefa() {
            let mut manager = GerenciadorTarefas::new();
            let result = manager.criar_tarefa(
                "Estudar Rust".into(),
                "Estudar smart contracts".into(),
                "2024-12-31".into(),
                3,
                Prioridade::Alta,
            );
            assert!(result.is_ok());
        }

        #[ink::test]
        fn test_atualizar_tarefa() {
            let mut manager = GerenciadorTarefas::new();
            let id = manager.criar_tarefa(
                "Estudar Rust".into(),
                "Estudar smart contracts".into(),
                "2024-12-31".into(),
                3,
                Prioridade::Alta,
            ).unwrap();
            
            let result = manager.atualizar_tarefa(
                id,
                "Atualizado".into(),
                "Descrição atualizada".into(),
                "2025-01-01".into(),
                5,
                Prioridade::Media,
            );
            
            assert!(result.is_ok());
        }

        #[ink::test]
        fn test_remover_tarefa() {
            let mut manager = GerenciadorTarefas::new();
            let id = manager.criar_tarefa(
                "Estudar Rust".into(),
                "Estudar Smart Contracts".into(),
                "2024-12-31".into(),
                3,
                Prioridade::Alta,
            ).unwrap();
            
            assert!(manager.remover_tarefa(id).is_ok());
            assert!(manager.listar_tarefas().is_empty());
        }
    }
}