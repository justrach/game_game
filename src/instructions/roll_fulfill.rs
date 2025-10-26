use pinocchio::{
    account_info::AccountInfo,
    entrypoint::InstructionContext,
    msg,
    program_error::ProgramError,
    ProgramResult,
};
use crate::{
    errors::RpgError,
    state::{RollSession, RollState},
    utils::validation::*,
};

/// Fulfill roll with VRF randomness (called by oracle or admin)
/// 
/// Accounts:
/// 0. [signer] Oracle/Admin wallet
/// 1. [writable] Roll session PDA
/// 
/// Instruction data:
/// - byte 0: discriminator (6)
/// - bytes 1-32: rng_seed (32 bytes)
pub fn process(context: &mut InstructionContext) -> ProgramResult {
    // Get accounts
    let oracle = context.next_account()?;
    let roll_account = context.next_account()?;
    
    // Validate
    assert_signer(oracle)?;
    assert_writable(roll_account)?;
    assert_owned_by_program(roll_account, context.program_id())?;
    
    // Get instruction data
    let instruction_data = context.instruction_data();
    if instruction_data.len() < 33 {
        return Err(RpgError::InvalidInstruction.into());
    }
    
    // Extract RNG seed
    let mut rng_seed = [0u8; 32];
    rng_seed.copy_from_slice(&instruction_data[1..33]);
    
    // Validate seed is not all zeros
    if rng_seed.iter().all(|&b| b == 0) {
        return Err(RpgError::InvalidRNGSeed.into());
    }
    
    // Get roll session data
    let roll_data = unsafe { &mut *(roll_account.borrow_mut_data().as_mut_ptr() as *mut RollSession) };
    if !roll_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Check state
    if !roll_data.is_pending() {
        return Err(RpgError::RollAlreadyFulfilled.into());
    }
    
    // Set RNG seed and update state
    roll_data.set_rng_seed(rng_seed);
    roll_data.set_state(RollState::Fulfilled);
    
    msg!("Roll fulfilled with randomness");
    
    Ok(())
}
