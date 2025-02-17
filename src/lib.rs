#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::ConstU32,
        BoundedVec,
    };
    
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
    use frame_system::pallet_prelude::*;
    use codec::{Decode, Encode};
    use scale_info::TypeInfo;

    pub const MAX_NOME_LEN: u32 = 50;
    pub const MAX_DESCRICAO_LEN: u32 = 200;
    pub const MAX_PRAZO_LEN: u32 = 10;

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub enum Prioridade {
        Baixa,
        Media,
        Alta,
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct Tarefa {
        pub nome: BoundedVec<u8, ConstU32<MAX_NOME_LEN>>,
        pub descricao: BoundedVec<u8, ConstU32<MAX_DESCRICAO_LEN>>,
        pub prazo: BoundedVec<u8, ConstU32<MAX_PRAZO_LEN>>,
        pub duracao: u32,
        pub prioridade: Prioridade,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    #[pallet::getter(fn tarefas)]
    pub type Tarefas<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Tarefa, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TarefaCriada(Vec<u8>),
        TarefaAtualizada(Vec<u8>),
        TarefaRemovida(Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        TarefaJaExiste,
        TarefaNaoEncontrada,
        NomeVazio,
        DescricaoVazia,
        PrazoVazio,
        DuracaoZero,
        OverflowBoundedVec,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Cria uma nova tarefa.
        #[pallet::weight(10_000)]
        pub fn criar_tarefa(
            origin: OriginFor<T>,
            nome: Vec<u8>,
            descricao: Vec<u8>,
            prazo: Vec<u8>,
            duracao: u32,
            prioridade: Prioridade,
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            ensure!(!nome.is_empty(), Error::<T>::NomeVazio);
            ensure!(!descricao.is_empty(), Error::<T>::DescricaoVazia);
            ensure!(!prazo.is_empty(), Error::<T>::PrazoVazio);
            ensure!(duracao > 0, Error::<T>::DuracaoZero);

            // Verifica se já existe uma tarefa com o mesmo nome.
            ensure!(!Tarefas::<T>::contains_key(&nome), Error::<T>::TarefaJaExiste);

            let bounded_nome: BoundedVec<u8, _> = nome.clone()
                .try_into()
                .map_err(|_| Error::<T>::OverflowBoundedVec)?;
            let bounded_descricao: BoundedVec<u8, _> = descricao.clone()
                .try_into()
                .map_err(|_| Error::<T>::OverflowBoundedVec)?;
            let bounded_prazo: BoundedVec<u8, _> = prazo.clone()
                .try_into()
                .map_err(|_| Error::<T>::OverflowBoundedVec)?;

            let tarefa = Tarefa {
                nome: bounded_nome,
                descricao: bounded_descricao,
                prazo: bounded_prazo,
                duracao,
                prioridade,
            };

            Tarefas::<T>::insert(nome.clone(), tarefa);
            Self::deposit_event(Event::TarefaCriada(nome));
            Ok(())
        }

        /// Atualiza os dados de uma tarefa existente.
        #[pallet::weight(10_000)]
        pub fn atualizar_tarefa(
            origin: OriginFor<T>,
            nome: Vec<u8>,
            nova_descricao: Vec<u8>,
            novo_prazo: Vec<u8>,
            nova_duracao: u32,
            nova_prioridade: Prioridade,
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            Tarefas::<T>::try_mutate(&nome, |maybe_tarefa| -> DispatchResult {
                let tarefa = maybe_tarefa.as_mut().ok_or(Error::<T>::TarefaNaoEncontrada)?;

                ensure!(!nova_descricao.is_empty(), Error::<T>::DescricaoVazia);
                ensure!(!novo_prazo.is_empty(), Error::<T>::PrazoVazio);
                ensure!(nova_duracao > 0, Error::<T>::DuracaoZero);

                let bounded_descricao: BoundedVec<u8, _> = nova_descricao
                    .try_into()
                    .map_err(|_| Error::<T>::OverflowBoundedVec)?;
                let bounded_prazo: BoundedVec<u8, _> = novo_prazo
                    .try_into()
                    .map_err(|_| Error::<T>::OverflowBoundedVec)?;

                tarefa.descricao = bounded_descricao;
                tarefa.prazo = bounded_prazo;
                tarefa.duracao = nova_duracao;
                tarefa.prioridade = nova_prioridade;

                Ok(())
            })?;

            Self::deposit_event(Event::TarefaAtualizada(nome));
            Ok(())
        }

        /// Remove uma tarefa existente.
        #[pallet::weight(10_000)]
        pub fn remover_tarefa(
            origin: OriginFor<T>,
            nome: Vec<u8>,
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            ensure!(Tarefas::<T>::contains_key(&nome), Error::<T>::TarefaNaoEncontrada);
            Tarefas::<T>::remove(&nome);
            Self::deposit_event(Event::TarefaRemovida(nome));
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as pallet_tarefas;
    use frame_support::{assert_ok, assert_noop, parameter_types};
    use sp_core::H256;
    use sp_runtime::{
        traits::IdentityLookup,
        testing::Header,
    };
    use frame_system as system;

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system,
            TarefasModule: pallet_tarefas,
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
    }

    impl system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type RuntimeOrigin = Origin;
        type RuntimeCall = Call;
        type Hash = H256;
        type Hashing = sp_runtime::traits::BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type RuntimeEvent = Event;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type DbWeight = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type Nonce = u64;
        type Block = Block;
        type MaxConsumers = frame_support::traits::ConstU32<16>;
    }

    impl Config for Test {
        type RuntimeEvent = Event;
    }

    #[test]
    fn test_criar_tarefa() {
        let nome = b"Estudar Rust".to_vec();
        let descricao = b"Estudar conceitos avancados de Rust.".to_vec();
        let prazo = b"2024-12-05".to_vec();
        let duracao = 3;
        let prioridade = Prioridade::Alta;

        // Chama o extrinsic de criação de tarefa.
        assert_ok!(TarefasModule::criar_tarefa(
            Origin::signed(1),
            nome.clone(),
            descricao,
            prazo,
            duracao,
            prioridade
        ));

        let tarefa = Tarefas::<Test>::get(nome.clone()).unwrap();
        assert_eq!(tarefa.duracao, 3);
    }

    #[test]
    fn test_atualizar_tarefa() {
        let nome = b"Estudar Rust".to_vec();
        let descricao = b"Estudar conceitos basicos de Rust".to_vec();
        let prazo = b"2024-12-05".to_vec();
        let duracao = 3;
        let prioridade = Prioridade::Alta;

        // Cria uma tarefa inicialmente.
        assert_ok!(TarefasModule::criar_tarefa(
            Origin::signed(1),
            nome.clone(),
            b"Inicial".to_vec(),
            b"2024-12-05".to_vec(),
            3,
            Prioridade::Baixa
        ));

        // Atualiza a tarefa.
        assert_ok!(TarefasModule::atualizar_tarefa(
            Origin::signed(1),
            nome.clone(),
            descricao.clone(),
            prazo.clone(),
            duracao,
            prioridade
        ));

        let tarefa = Tarefas::<Test>::get(nome.clone()).unwrap();
        assert_eq!(tarefa.prioridade, prioridade);
    }

    #[test]
    fn test_remover_tarefa() {
        let nome = b"Estudar Rust".to_vec();
        // Cria uma tarefa.
        assert_ok!(TarefasModule::criar_tarefa(
            Origin::signed(1),
            nome.clone(),
            b"Descricao".to_vec(),
            b"2024-12-05".to_vec(),
            3,
            Prioridade::Alta
        ));

        // Remove a tarefa.
        assert_ok!(TarefasModule::remover_tarefa(
            Origin::signed(1),
            nome.clone()
        ));
        assert!(Tarefas::<Test>::get(nome).is_none());
    }
}
