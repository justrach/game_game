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
    constants::{BATTLE_ENTRY_FEE, BATTLE_SEED, BATTLE_SIZE, TREASURY_SEED},
    errors::RpgError,
    state::{Battle, EnemyTemplate, Hero, Treasury},
    utils::validation::*,
};

/// Start a battle against an enemy
/// 
/// Accounts:
/// 0. [signer, writable] User wallet
/// 1. [] Hero PDA
/// 2. [] Enemy template PDA
/// 3. [writable] Battle PDA (to be created)
/// 4. [writable] Treasury PDA
/// 5. [] System program
pub fn process(context: &mut InstructionContext) -> ProgramResult {
    // Get accounts
    let user = context.next_account()?;
    let hero_account = context.next_account()?;
    let enemy_account = context.next_account()?;
    let battle_account = context.next_account()?;
    let treasury_account = context.next_account()?;
    let system_program = context.next_account()?;
    
    // Validate
    assert_signer(user)?;
    assert_writable(user)?;
    assert_writable(battle_account)?;
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
    
    // Validate enemy template
    assert_owned_by_program(enemy_account, program_id)?;
    let enemy_data = unsafe { &*(enemy_account.borrow_data().as_ptr() as *const EnemyTemplate) };
    if !enemy_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Validate treasury
    let treasury_seeds = &[TREASURY_SEED];
    assert_pda(treasury_account, treasury_seeds, program_id)?;
    
    // Check user has enough funds
    assert_sufficient_lamports(user, BATTLE_ENTRY_FEE)?;
    
    // Transfer entry fee to treasury
    Transfer {
        from: user,
        to: treasury_account,
        lamports: BATTLE_ENTRY_FEE,
    }
    .invoke()?;
    
    // Update treasury stats
    let treasury_data = unsafe { &mut *(treasury_account.borrow_mut_data().as_mut_ptr() as *mut Treasury) };
    treasury_data.add_collection(BATTLE_ENTRY_FEE);
    
    // Get clock for nonce
    let clock = Clock::get()?;
    
    // Derive battle PDA with nonce
    let hero_key = hero_account.key();
    let nonce = clock.unix_timestamp as u64;
    let nonce_bytes = nonce.to_le_bytes();
    let battle_seeds = &[BATTLE_SEED, hero_key.as_ref(), &nonce_bytes];
    let bump = assert_pda(battle_account, battle_seeds, program_id)?;
    
    // Create battle account
    let battle_seeds_with_bump = &[BATTLE_SEED, hero_key.as_ref(), &nonce_bytes, &[bump]];
    
    CreateAccount {
        from: user,
        to: battle_account,
        lamports: 1_500_000, // Rent-exempt
        space: BATTLE_SIZE as u64,
        owner: program_id,
    }
    .invoke_signed(&[battle_seeds_with_bump])?;
    
    // Initialize battle
    let hero_hp = hero_data.max_hp();
    let enemy_hp = enemy_data.max_hp;
    
    let mut battle_data = Battle::new(
        *hero_key,
        *enemy_account.key(),
        hero_hp,
        enemy_hp,
        nonce,
    );
    
    // In production, you'd request VRF here
    // For now, we'll use a simple seed based on clock
    let mut simple_seed = [0u8; 32];
    let timestamp_bytes = clock.unix_timestamp.to_le_bytes();
    simple_seed[0..8].copy_from_slice(&timestamp_bytes);
    simple_seed[8..16].copy_from_slice(hero_key.as_ref()[0..8].as_ref());
    simple_seed[16..24].copy_from_slice(enemy_account.key().as_ref()[0..8].as_ref());
    battle_data.set_rng_seed(simple_seed);
    
    let data = unsafe { &mut *battle_account.borrow_mut_data() };
    let battle_ptr = data.as_mut_ptr() as *mut Battle;
    unsafe {
        *battle_ptr = battle_data;
    }
    
    msg!("Battle started!");
    msg!("Hero HP: {} | Enemy HP: {}", hero_hp, enemy_hp);
    
    Ok(())
}
