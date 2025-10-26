use pinocchio::{
    account_info::AccountInfo,
    entrypoint::InstructionContext,
    msg,
    program_error::ProgramError,
    ProgramResult,
    sysvars::{clock::Clock, Sysvar},
};
use pinocchio_system::instructions::{CreateAccount, Transfer};
use crate::{
    constants::{ROLL_PRICE, ROLL_SESSION_SEED, ROLL_SESSION_SIZE, TREASURY_SEED},
    errors::RpgError,
    state::{Hero, RollSession, Treasury},
    utils::validation::*,
};

/// Start a slot machine roll
/// 
/// Accounts:
/// 0. [signer, writable] User wallet
/// 1. [] Hero PDA
/// 2. [writable] Roll session PDA (to be created)
/// 3. [writable] Treasury PDA
/// 4. [] System program
pub fn process(context: &mut InstructionContext) -> ProgramResult {
    // Get accounts
    let user = context.next_account()?;
    let hero_account = context.next_account()?;
    let roll_account = context.next_account()?;
    let treasury_account = context.next_account()?;
    let system_program = context.next_account()?;
    
    // Validate
    assert_signer(user)?;
    assert_writable(user)?;
    assert_writable(roll_account)?;
    assert_writable(treasury_account)?;
    
    let program_id = context.program_id();
    
    // Validate hero
    assert_owned_by_program(hero_account, program_id)?;
    let hero_data = unsafe { &*(hero_account.borrow_data().as_ptr() as *const Hero) };
    if !hero_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Verify ownership
    if hero_data.owner != *user.key() {
        return Err(RpgError::Unauthorized.into());
    }
    
    // Validate treasury
    let treasury_seeds = &[TREASURY_SEED];
    assert_pda(treasury_account, treasury_seeds, program_id)?;
    
    // Check user has enough funds
    assert_sufficient_lamports(user, ROLL_PRICE)?;
    
    // Transfer payment to treasury
    Transfer {
        from: user,
        to: treasury_account,
        lamports: ROLL_PRICE,
    }
    .invoke()?;
    
    // Update treasury stats
    let treasury_data = unsafe { &mut *(treasury_account.borrow_mut_data().as_mut_ptr() as *mut Treasury) };
    treasury_data.add_collection(ROLL_PRICE);
    
    // Get clock for timestamp
    let clock = Clock::get()?;
    
    // Derive roll session PDA with nonce
    let hero_key = hero_account.key();
    let nonce = clock.unix_timestamp as u64;
    let nonce_bytes = nonce.to_le_bytes();
    let roll_seeds = &[ROLL_SESSION_SEED, hero_key.as_ref(), &nonce_bytes];
    let bump = assert_pda(roll_account, roll_seeds, program_id)?;
    
    // Create roll session account
    let roll_seeds_with_bump = &[ROLL_SESSION_SEED, hero_key.as_ref(), &nonce_bytes, &[bump]];
    
    CreateAccount {
        from: user,
        to: roll_account,
        lamports: 1_000_000, // Rent-exempt
        space: ROLL_SESSION_SIZE as u64,
        owner: program_id,
    }
    .invoke_signed(&[roll_seeds_with_bump])?;
    
    // Initialize roll session
    let roll_data = RollSession::new(*hero_key, clock.unix_timestamp, nonce);
    
    let data = unsafe { &mut *roll_account.borrow_mut_data() };
    let roll_ptr = data.as_mut_ptr() as *mut RollSession;
    unsafe {
        *roll_ptr = roll_data;
    }
    
    msg!("Roll session started for hero: {}", hero_key);
    msg!("Waiting for VRF fulfillment...");
    
    Ok(())
}
