use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    msg,
    pubkey::Pubkey,
    ProgramResult,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello from On-Chain RPG!");
    msg!("This is a minimal Pinocchio program");
    Ok(())
}
