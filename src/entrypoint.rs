//! Program entrypoint


use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};



// Declare & export the program's entrypoint
entrypoint!(process_instruction);

// Entrypoint
// Maybe need a <'a> generic
pub fn process_instruction(
    // Arguments here
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Call processor on instruciton
    crate::processor::process(program_id, accounts, instruction_data)

    // Does this mean success???
    // Ok(()) // maybe don't need this, not doing a token?
}
