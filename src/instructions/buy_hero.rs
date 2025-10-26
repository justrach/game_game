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
    constants::{HERO_BASE_PRICE, HERO_SEED, HERO_SIZE, PLAYER_SEED, TREASURY_SEED},
    errors::RpgError,
    state::{Hero, Player, Treasury},
    utils::validation::*,
};

/// Buy a new hero
/// 
/// Accounts:
/// 0. [signer, writable] User wallet
/// 1. [writable] Player PDA
/// 2. [writable] Hero PDA (to be created)
/// 3. [writable] Treasury PDA
/// 4. [] System program
pub fn process(context: &mut InstructionContext) -> ProgramResult {
    // Get accounts
    let user = context.next_account()?;
    let player_account = context.next_account()?;
    let hero_account = context.next_account()?;
    let treasury_account = context.next_account()?;
    let system_program = context.next_account()?;
    
    // Validate
    assert_signer(user)?;
    assert_writable(user)?;
    assert_writable(player_account)?;
    assert_writable(hero_account)?;
    assert_writable(treasury_account)?;
    
    let user_key = user.key();
    let program_id = context.program_id();
    
    // Validate player PDA
    let player_seeds = &[PLAYER_SEED, user_key.as_ref()];
    assert_pda(player_account, player_seeds, program_id)?;
    assert_owned_by_program(player_account, program_id)?;
    
    // Get player data
    let player_data = unsafe { &mut *(player_account.borrow_mut_data().as_mut_ptr() as *mut Player) };
    if !player_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Validate treasury PDA
    let treasury_seeds = &[TREASURY_SEED];
    assert_pda(treasury_account, treasury_seeds, program_id)?;
    
    // Check user has enough funds
    assert_sufficient_lamports(user, HERO_BASE_PRICE)?;
    
    // Transfer payment to treasury
    Transfer {
        from: user,
        to: treasury_account,
        lamports: HERO_BASE_PRICE,
    }
    .invoke()?;
    
    // Update treasury stats
    let treasury_data = unsafe { &mut *(treasury_account.borrow_mut_data().as_mut_ptr() as *mut Treasury) };
    treasury_data.add_collection(HERO_BASE_PRICE);
    
    // Derive hero PDA
    let hero_index = player_data.hero_count;
    let hero_index_bytes = hero_index.to_le_bytes();
    let hero_seeds = &[HERO_SEED, user_key.as_ref(), &hero_index_bytes];
    let bump = assert_pda(hero_account, hero_seeds, program_id)?;
    
    // Create hero account
    let hero_seeds_with_bump = &[HERO_SEED, user_key.as_ref(), &hero_index_bytes, &[bump]];
    
    CreateAccount {
        from: user,
        to: hero_account,
        lamports: 1_500_000, // Rent-exempt
        space: HERO_SIZE as u64,
        owner: program_id,
    }
    .invoke_signed(&[hero_seeds_with_bump])?;
    
    // Initialize hero data
    let clock = Clock::get()?;
    let hero_data = Hero::new(*user_key, clock.unix_timestamp);
    
    let data = unsafe { &mut *hero_account.borrow_mut_data() };
    let hero_ptr = data.as_mut_ptr() as *mut Hero;
    unsafe {
        *hero_ptr = hero_data;
    }
    
    // Update player stats
    player_data.add_hero();
    
    msg!("Hero #{} created for player: {}", hero_index, user_key);
    
    Ok(())
}
