use pinocchio::{
    account_info::AccountInfo,
    entrypoint::InstructionContext,
    msg,
    program_error::ProgramError,
    ProgramResult,
    sysvars::{clock::Clock, Sysvar},
};
use pinocchio_system::instructions::CreateAccount;
use crate::{
    constants::{PLAYER_SEED, PLAYER_SIZE},
    errors::RpgError,
    state::Player,
    utils::validation::*,
};

/// Initialize a new player account
/// 
/// Accounts:
/// 0. [signer, writable] User wallet
/// 1. [writable] Player PDA
/// 2. [] System program
pub fn process(context: &mut InstructionContext) -> ProgramResult {
    // Get accounts
    let user = context.next_account()?;
    let player_account = context.next_account()?;
    let system_program = context.next_account()?;
    
    // Validate
    assert_signer(user)?;
    assert_writable(user)?;
    assert_writable(player_account)?;
    
    // Derive PDA
    let user_key = user.key();
    let seeds = &[PLAYER_SEED, user_key.as_ref()];
    let bump = assert_pda(player_account, seeds, context.program_id())?;
    
    // Check if already initialized
    if player_account.data_len() > 0 {
        let data = unsafe { &*player_account.borrow_data() };
        let player = unsafe { &*(data.as_ptr() as *const Player) };
        if player.is_initialized() {
            return Err(RpgError::AccountAlreadyInitialized.into());
        }
    }
    
    // Create account
    let seeds_with_bump = &[PLAYER_SEED, user_key.as_ref(), &[bump]];
    
    CreateAccount {
        from: user,
        to: player_account,
        lamports: 1_000_000, // Rent-exempt minimum
        space: PLAYER_SIZE as u64,
        owner: context.program_id(),
    }
    .invoke_signed(&[seeds_with_bump])?;
    
    // Initialize player data
    let clock = Clock::get()?;
    let player_data = Player::new(*user_key, clock.unix_timestamp);
    
    let data = unsafe { &mut *player_account.borrow_mut_data() };
    let player_ptr = data.as_mut_ptr() as *mut Player;
    unsafe {
        *player_ptr = player_data;
    }
    
    msg!("Player initialized for wallet: {}", user_key);
    
    Ok(())
}
