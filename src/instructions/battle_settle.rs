use pinocchio::{
    account_info::AccountInfo,
    entrypoint::InstructionContext,
    msg,
    program_error::ProgramError,
    ProgramResult,
};
use crate::{
    errors::RpgError,
    state::{Battle, BattleState, EnemyTemplate, Hero, Player},
    utils::{math::*, validation::*},
};

/// Settle a completed battle and distribute rewards
/// 
/// Accounts:
/// 0. [signer] User wallet
/// 1. [writable] Hero PDA
/// 2. [writable] Player PDA
/// 3. [] Enemy template PDA
/// 4. [writable] Battle PDA
pub fn process(context: &mut InstructionContext) -> ProgramResult {
    // Get accounts
    let user = context.next_account()?;
    let hero_account = context.next_account()?;
    let player_account = context.next_account()?;
    let enemy_account = context.next_account()?;
    let battle_account = context.next_account()?;
    
    // Validate
    assert_signer(user)?;
    assert_writable(hero_account)?;
    assert_writable(player_account)?;
    assert_writable(battle_account)?;
    
    let program_id = context.program_id();
    assert_owned_by_program(hero_account, program_id)?;
    assert_owned_by_program(player_account, program_id)?;
    assert_owned_by_program(enemy_account, program_id)?;
    assert_owned_by_program(battle_account, program_id)?;
    
    // Get battle data
    let battle_data = unsafe { &mut *(battle_account.borrow_mut_data().as_mut_ptr() as *mut Battle) };
    if !battle_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Validate battle is complete
    let battle_state = battle_data.get_state().ok_or(RpgError::InvalidBattleState)?;
    if battle_state == BattleState::Active {
        return Err(RpgError::BattleAlreadyActive.into());
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
    
    // Verify battle is for this hero
    if battle_data.hero != *hero_account.key() {
        return Err(RpgError::Unauthorized.into());
    }
    
    // Get player data
    let player_data = unsafe { &mut *(player_account.borrow_mut_data().as_mut_ptr() as *mut Player) };
    if !player_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Get enemy data
    let enemy_data = unsafe { &*(enemy_account.borrow_data().as_ptr() as *const EnemyTemplate) };
    if !enemy_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Process rewards based on outcome
    match battle_state {
        BattleState::Won => {
            // Calculate XP reward
            let xp_reward = calculate_xp_reward(enemy_data.level, hero_data.level);
            
            // Apply luck bonus to XP
            let luck_bonus = (xp_reward * hero_data.base_attributes.luck as u32) / 1000;
            let total_xp = xp_reward + luck_bonus;
            
            // Award XP
            let leveled_up = hero_data.add_xp(total_xp);
            player_data.add_xp(total_xp);
            
            msg!("Victory! Earned {} XP", total_xp);
            if leveled_up {
                msg!("Hero leveled up to level {}!", hero_data.level);
            }
            
            // In a full implementation, you'd also:
            // - Roll for loot drops
            // - Create item accounts
            // - Award currency
        }
        BattleState::Lost => {
            msg!("Defeat. No rewards.");
        }
        BattleState::Escaped => {
            // Small XP for attempting
            let escape_xp = 10;
            hero_data.add_xp(escape_xp);
            player_data.add_xp(escape_xp);
            msg!("Escaped. Earned {} XP", escape_xp);
        }
        BattleState::Active => {
            // Should never reach here due to earlier check
            return Err(RpgError::InvalidBattleState.into());
        }
    }
    
    // Note: In production, you'd close the battle_account here to reclaim rent
    // For simplicity, we're leaving it open in this example
    
    msg!("Battle settled successfully");
    
    Ok(())
}
