#[cfg_attr(not(feature = "std"), no_std, no_main)]
#[allow(clippy::cast_possible_truncation)]

#[ink::contract]
mod gerenciador_tarefas {
    use ink::prelude::{string::String, vec::Vec};
    use ink::storage::Mapping;

    /// Define as prioridades disponíveis para uma tarefa.
    #[repr(u32)]
    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    #[allow(clippy::cast_possible_truncation)]
    pub enum Prioridade {
        Baixa, // será representado como 0u8
        Media, // 1u8
        Alta,  // 2u8
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Tarefa {
        pub nome: String,
        pub descricao: String,
        pub prazo: String,
        pub duracao: u32,
        pub prioridade: Prioridade,
    }

    /// Contrato Ink! que atua como gerenciador de tarefas.
    #[ink(storage)]
    #[derive(Default)]
    pub struct GerenciadorTarefas {
        tarefas: Mapping<u32, Tarefa>,
        next_id: u32,
    }

    impl GerenciadorTarefas {
        /// Construtor que inicializa o contrato.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                tarefas: Mapping::default(),
                next_id: 0,
            }
        }

        /// Valida os dados da tarefa.
        fn validar_tarefa(tarefa: &Tarefa) -> Result<(), String> {
            if tarefa.nome.trim().is_empty() {
                return Err(String::from("Nome da tarefa não pode ser vazio."));
            }
            if tarefa.descricao.trim().is_empty() {
                return Err(String::from("Descrição da tarefa não pode ser vazia."));
            }
            if tarefa.prazo.trim().is_empty() {
                return Err(String::from("Prazo da tarefa não pode ser vazio."));
            }
            if tarefa.duracao == 0 {
                return Err(String::from("A duração da tarefa deve ser maior que zero."));
            }
            Ok(())
        }

        /// Cria uma nova tarefa e a armazena.
        #[ink(message)]
        pub fn criar_tarefa(
            &mut self,
            nome: String,
            descricao: String,
            prazo: String,
            duracao: u32,
            prioridade: Prioridade,
        ) -> Result<u32, String> {
            let tarefa = Tarefa {
                nome: nome.clone(),
                descricao,
                prazo,
                duracao,
                prioridade,
            };
            Self::validar_tarefa(&tarefa)?;
            let id = self.next_id;
            self.tarefas.insert(id, &tarefa);
            self.next_id = self.next_id.checked_add(1).ok_or("ID overflow")?;
            Ok(id)
        }

        /// Retorna uma lista com os IDs e as tarefas armazenadas.
        #[ink(message)]
        pub fn listar_tarefas(&self) -> Vec<(u32, Tarefa)> {
            let mut lista = Vec::new();
            for id in 0..self.next_id {
                if let Some(tarefa) = self.tarefas.get(id) {
                    lista.push((id, tarefa));
                }
            }
            lista
        }

        /// Atualiza uma tarefa existente pelo seu ID.
        #[ink(message)]
        pub fn atualizar_tarefa(&mut self, id: u32, nova_tarefa: Tarefa) -> Result<(), String> {
            Self::validar_tarefa(&nova_tarefa)?;
            if self.tarefas.get(id).is_none() {
                return Err(String::from("Tarefa não encontrada."));
            }
            self.tarefas.insert(id, &nova_tarefa);
            Ok(())
        }

        /// Remove uma tarefa pelo seu ID.
        #[ink(message)]
        pub fn remover_tarefa(&mut self, id: u32) -> Result<(), String> {
            if self.tarefas.get(id).is_some() {
                self.tarefas.remove(id);
                Ok(())
            } else {
                Err(String::from("Tarefa não encontrada."))
            }
        }
    }

    // Testes para verificar o funcionamento do contrato.
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_criar_listar() {
            let mut contrato = GerenciadorTarefas::new();
            let id = contrato.criar_tarefa(
                "Estudar Ink!".into(),
                "Estudar a criação de smart contracts com Ink!".into(),
                "2024-12-31".into(),
                3,
                Prioridade::Alta,
            ).expect("Falha ao criar tarefa");
            let tarefas = contrato.listar_tarefas();
            assert_eq!(tarefas.len(), 1);
            assert_eq!(tarefas[0].0, id);
        }

        #[ink::test]
        fn test_atualizar() {
            let mut contrato = GerenciadorTarefas::new();
            let id = contrato.criar_tarefa(
                "Estudar Ink!".into(),
                "Estudar a criação de smart contracts com Ink!".into(),
                "2024-12-31".into(),
                3,
                Prioridade::Alta,
            ).expect("Falha ao criar tarefa");
            let tarefa_atualizada = Tarefa {
                nome: "Estudar Ink - Atualizado".into(),
                descricao: "Atualizar conhecimentos em Ink".into(),
                prazo: "2025-01-15".into(),
                duracao: 4,
                prioridade: Prioridade::Media,
            };
            assert!(contrato.atualizar_tarefa(id, tarefa_atualizada.clone()).is_ok());
            let lista = contrato.listar_tarefas();
            assert_eq!(lista[0].1, tarefa_atualizada);
        }

        #[ink::test]
        fn test_remover() {
            let mut contrato = GerenciadorTarefas::new();
            let id = contrato.criar_tarefa(
                "Estudar Ink!".into(),
                "Estudar a criação de smart contracts com Ink!".into(),
                "2024-12-31".into(),
                3,
                Prioridade::Alta,
            ).expect("Falha ao criar tarefa");
            assert!(contrato.remover_tarefa(id).is_ok());
            let lista = contrato.listar_tarefas();
            assert_eq!(lista.len(), 0);
        }
    }
}
