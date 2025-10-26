use pinocchio::{
    account_info::AccountInfo,
    entrypoint::InstructionContext,
    msg,
    program_error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;
use crate::{
    constants::{TREASURY_SEED, TREASURY_SIZE},
    errors::RpgError,
    state::Treasury,
    utils::validation::*,
};

/// Initialize the treasury account (admin only, one-time)
/// 
/// Accounts:
/// 0. [signer, writable] Admin wallet
/// 1. [writable] Treasury PDA
/// 2. [] System program
pub fn process(context: &mut InstructionContext) -> ProgramResult {
    // Get accounts
    let admin = context.next_account()?;
    let treasury_account = context.next_account()?;
    let system_program = context.next_account()?;
    
    // Validate
    assert_signer(admin)?;
    assert_writable(admin)?;
    assert_writable(treasury_account)?;
    
    // Derive treasury PDA
    let treasury_seeds = &[TREASURY_SEED];
    let bump = assert_pda(treasury_account, treasury_seeds, context.program_id())?;
    
    // Check if already initialized
    if treasury_account.data_len() > 0 {
        let data = unsafe { &*treasury_account.borrow_data() };
        let treasury = unsafe { &*(data.as_ptr() as *const Treasury) };
        if treasury.is_initialized() {
            return Err(RpgError::AccountAlreadyInitialized.into());
        }
    }
    
    // Create treasury account
    let treasury_seeds_with_bump = &[TREASURY_SEED, &[bump]];
    
    CreateAccount {
        from: admin,
        to: treasury_account,
        lamports: 2_000_000, // Rent-exempt + buffer for operations
        space: TREASURY_SIZE as u64,
        owner: context.program_id(),
    }
    .invoke_signed(&[treasury_seeds_with_bump])?;
    
    // Initialize treasury data
    let treasury_data = Treasury::new(*admin.key());
    
    let data = unsafe { &mut *treasury_account.borrow_mut_data() };
    let treasury_ptr = data.as_mut_ptr() as *mut Treasury;
    unsafe {
        *treasury_ptr = treasury_data;
    }
    
    msg!("Treasury initialized with authority: {}", admin.key());
    
    Ok(())
}
