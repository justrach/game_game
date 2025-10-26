use pinocchio::{
    account_info::AccountInfo,
    entrypoint::InstructionContext,
    msg,
    program_error::ProgramError,
    ProgramResult,
};
use crate::{
    errors::RpgError,
    state::Hero,
    utils::validation::*,
};

/// Level up a hero (if enough XP)
/// 
/// Accounts:
/// 0. [signer] User wallet
/// 1. [writable] Hero PDA
pub fn process(context: &mut InstructionContext) -> ProgramResult {
    // Get accounts
    let user = context.next_account()?;
    let hero_account = context.next_account()?;
    
    // Validate
    assert_signer(user)?;
    assert_writable(hero_account)?;
    assert_owned_by_program(hero_account, context.program_id())?;
    
    // Get hero data
    let hero_data = unsafe { &mut *(hero_account.borrow_mut_data().as_mut_ptr() as *mut Hero) };
    if !hero_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Verify ownership
    if hero_data.owner != *user.key() {
        return Err(RpgError::Unauthorized.into());
    }
    
    // Check if can level up
    if hero_data.level >= crate::constants::MAX_LEVEL {
        return Err(RpgError::MaxLevelReached.into());
    }
    
    let xp_needed = hero_data.xp_needed_for_next_level();
    if hero_data.xp < xp_needed {
        return Err(RpgError::InsufficientXP.into());
    }
    
    // Level up
    let old_level = hero_data.level;
    hero_data.level_up();
    
    msg!("Hero leveled up: {} -> {}", old_level, hero_data.level);
    
    Ok(())
}
