//! Program instruction processor

use solana_program::{
    //TODO
};



pub struct Proccesor {}
impl Processor {

    // init_account???
    // init_multisig???

    // revoke???
    // set_authority??? 

    // transfer

    // approve



    // close_account???
    // toggle_freeze_account ??? 

    //! Harvest 
    pub fn process_harvest(
        //TODO args
    ) -> ProgramResult {
        //TODO Call harvest on function

        //TODO Swap harvested token for desired token &or LP

        //TODO Take fee

        //TODO Deposit desired into farm
    }

    //! Withdraw & Burn Shares
    pub fn process_withdraw (
        //TODO args
    ) -> ProgramResult {
        // TODO safety check, caller has Amount of shares

        // TODO, get ratio  Tokens/Shares

        // TODO, transfer Amount*Tokens/Shares

        // TODO, burn Shares
    }
    // TODO burn -> only callable by withdraw

    //! Deposit & Mint Shares
    pub fn process_deposit (
        //TODO args
    ) -> ProgramResult {
        //TODO
    }
    // init_mint???
    // mint --> only callable by deposit
    // mint_to ???



    // Process any give instruction
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input &[u8] -> ProgramResult {
        // extract instruction from input
        let instruction = TokenInstruction::unpack(input)?;

        // mux to appropriate instruction
        match instruction {

            //! Functions to Deposit & Withdraw
            TokenInstruction::Withdraw { amount } => {
                info!("Instruction: Withdraw");
                //Self::process_withdraw( TODO )
            }
            TokenInstruction::Deposit { amount } => {
                info!("Instruction: Deposit");
                //Self::process_deposit( TODO )
            }
            TokenInstruction::Harvest => {
                info!("Instruction: Harvest");
                //Self::process_harvest
            //TODO
        }

        // Life is good
        Ok(())
    }
}
 
// TODO error printing things

// TODO tests
#[cfg(test)]
mod tests {
    //TODO????

}
