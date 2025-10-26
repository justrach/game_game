use pinocchio::{
    account_info::AccountInfo,
    entrypoint::InstructionContext,
    msg,
    program_error::ProgramError,
    ProgramResult,
};
use crate::{
    constants::{ROLL_LEGENDARY_THRESHOLD, ROLL_RARE_THRESHOLD, ROLL_XP_MAX, ROLL_XP_MIN},
    errors::RpgError,
    state::{Hero, Player, RollSession, RollState},
    utils::{rng::*, validation::*},
};

/// Confirm roll and claim rewards
/// 
/// Accounts:
/// 0. [signer] User wallet
/// 1. [writable] Hero PDA
/// 2. [writable] Player PDA
/// 3. [writable] Roll session PDA
pub fn process(context: &mut InstructionContext) -> ProgramResult {
    // Get accounts
    let user = context.next_account()?;
    let hero_account = context.next_account()?;
    let player_account = context.next_account()?;
    let roll_account = context.next_account()?;
    
    // Validate
    assert_signer(user)?;
    assert_writable(hero_account)?;
    assert_writable(player_account)?;
    assert_writable(roll_account)?;
    
    let program_id = context.program_id();
    assert_owned_by_program(hero_account, program_id)?;
    assert_owned_by_program(player_account, program_id)?;
    assert_owned_by_program(roll_account, program_id)?;
    
    // Get roll session data
    let roll_data = unsafe { &mut *(roll_account.borrow_mut_data().as_mut_ptr() as *mut RollSession) };
    if !roll_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Check state
    if !roll_data.is_fulfilled() {
        return Err(RpgError::RollNotReady.into());
    }
    
    // Get hero data
    let hero_data = unsafe { &mut *(hero_account.borrow_mut_data().as_mut_ptr() as *mut Hero) };
    if !hero_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Verify ownership
    if hero_data.owner != *user.key() {
        return Err(RpgError::Unauthorized.into());
    }
    
    // Verify roll is for this hero
    if roll_data.hero != *hero_account.key() {
        return Err(RpgError::Unauthorized.into());
    }
    
    // Get player data
    let player_data = unsafe { &mut *(player_account.borrow_mut_data().as_mut_ptr() as *mut Player) };
    if !player_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Calculate rewards based on RNG
    let seed = &roll_data.rng_seed;
    
    // Roll for rarity
    let rarity_roll = rng_u16(seed, 0);
    
    let (reward_type, xp_amount) = if rarity_roll >= ROLL_LEGENDARY_THRESHOLD {
        // Legendary - 1% chance
        ("Legendary", ROLL_XP_MAX)
    } else if rarity_roll >= ROLL_RARE_THRESHOLD {
        // Rare - 5% chance
        ("Rare", ROLL_XP_MAX * 2 / 3)
    } else {
        // Common
        ("Common", rng_range(seed, 1, ROLL_XP_MIN, ROLL_XP_MAX / 2))
    };
    
    // Apply XP rewards
    let leveled_up = hero_data.add_xp(xp_amount);
    player_data.add_xp(xp_amount);
    
    // Update roll state
    roll_data.set_state(RollState::Confirmed);
    
    msg!("Roll result: {} - {} XP gained", reward_type, xp_amount);
    if leveled_up {
        msg!("Hero leveled up to level {}!", hero_data.level);
    }
    
    // Note: In production, you'd close the roll_account here to reclaim rent
    // For simplicity, we're leaving it open in this example
    
    Ok(())
}
