//! Instruction <-> Transaction account compilation, with key deduplication,
//! privilege handling, and program account stubbing.

use {
    mollusk_svm_keys::{
        accounts::{
            compile_instruction_accounts, compile_instruction_without_data,
            compile_transaction_accounts_for_instruction,
        },
        keys::KeyMap,
    },
    solana_account::{Account, WritableAccount},
    solana_instruction::Instruction,
    solana_pubkey::Pubkey,
    solana_transaction_context::{InstructionAccount, TransactionAccount},
};

pub struct CompiledAccounts {
    pub program_id_index: u16,
    pub instruction_accounts: Vec<InstructionAccount>,
    pub transaction_accounts: Vec<TransactionAccount>,
}

pub fn compile_accounts(
    instruction: &Instruction,
    accounts: &[(Pubkey, Account)],
    loader_key: Pubkey,
) -> CompiledAccounts {
    let stub_out_program_account = move || {
        let mut program_account = Account::default();
        program_account.set_owner(loader_key);
        program_account.set_executable(true);
        program_account
    };

    let key_map = KeyMap::compile_from_instruction(instruction);
    let compiled_instruction = compile_instruction_without_data(&key_map, instruction);
    let instruction_accounts = compile_instruction_accounts(&key_map, &compiled_instruction);
    let transaction_accounts = compile_transaction_accounts_for_instruction(
        &key_map,
        instruction,
        accounts,
        Some(Box::new(stub_out_program_account)),
    );

    CompiledAccounts {
        program_id_index: compiled_instruction.program_id_index as u16,
        instruction_accounts,
        transaction_accounts,
    }
}
