use pinocchio::{
    account_info::AccountInfo,
    entrypoint::InstructionContext,
    msg,
    program_error::ProgramError,
    ProgramResult,
    sysvars::{clock::Clock, Sysvar},
};
use crate::{
    constants::{CRIT_MULTIPLIER, MAX_BATTLE_TURNS},
    errors::RpgError,
    state::{Battle, BattleAction, BattleState, EnemyTemplate, Hero},
    utils::{math::*, rng::*, validation::*},
};

/// Execute a battle turn
/// 
/// Accounts:
/// 0. [signer] User wallet
/// 1. [] Hero PDA
/// 2. [] Enemy template PDA
/// 3. [writable] Battle PDA
/// 4. [] Clock sysvar
/// 
/// Instruction data:
/// - byte 0: discriminator (9)
/// - byte 1: action (0=attack, 1=defend, 2=skill, 3=escape)
pub fn process(context: &mut InstructionContext) -> ProgramResult {
    // Get accounts
    let user = context.next_account()?;
    let hero_account = context.next_account()?;
    let enemy_account = context.next_account()?;
    let battle_account = context.next_account()?;
    
    // Validate
    assert_signer(user)?;
    assert_writable(battle_account)?;
    
    let program_id = context.program_id();
    assert_owned_by_program(hero_account, program_id)?;
    assert_owned_by_program(enemy_account, program_id)?;
    assert_owned_by_program(battle_account, program_id)?;
    
    // Get instruction data
    let instruction_data = context.instruction_data();
    if instruction_data.len() < 2 {
        return Err(RpgError::InvalidInstruction.into());
    }
    
    let action = BattleAction::from_u8(instruction_data[1]).ok_or(RpgError::InvalidAction)?;
    
    // Get battle data
    let battle_data = unsafe { &mut *(battle_account.borrow_mut_data().as_mut_ptr() as *mut Battle) };
    if !battle_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Validate battle state
    if !battle_data.is_active() {
        return Err(RpgError::BattleNotActive.into());
    }
    
    // Verify it's hero's turn
    if !battle_data.is_hero_turn() {
        return Err(RpgError::NotYourTurn.into());
    }
    
    // Get hero data
    let hero_data = unsafe { &*(hero_account.borrow_data().as_ptr() as *const Hero) };
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
    
    // Get enemy data
    let enemy_data = unsafe { &*(enemy_account.borrow_data().as_ptr() as *const EnemyTemplate) };
    if !enemy_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Get clock
    let clock = Clock::get()?;
    let current_slot = clock.slot;
    
    // Prevent spam (must wait at least 1 slot)
    if battle_data.last_action_slot > 0 && current_slot <= battle_data.last_action_slot {
        return Err(ProgramError::InvalidArgument);
    }
    
    battle_data.last_action_slot = current_slot;
    
    // Calculate turn number for RNG
    let turn_count = if battle_data.turn == 0 { 0 } else { 1 };
    
    // Execute hero action
    match action {
        BattleAction::Attack => {
            execute_hero_attack(battle_data, hero_data, enemy_data, turn_count)?;
        }
        BattleAction::Defend => {
            msg!("Hero defends (reduces next damage by 50%)");
            // In a full implementation, you'd store a defense flag
        }
        BattleAction::Skill => {
            execute_hero_skill(battle_data, hero_data, enemy_data, turn_count)?;
        }
        BattleAction::Escape => {
            let escape_chance = calculate_escape_chance(
                hero_data.base_attributes.agility,
                enemy_data.base_attributes.agility,
            );
            
            if rng_check(&battle_data.rng_seed, turn_count, escape_chance) {
                battle_data.set_state(BattleState::Escaped);
                msg!("Hero escaped from battle!");
                return Ok(());
            } else {
                msg!("Escape failed!");
            }
        }
    }
    
    // Check if enemy defeated
    if battle_data.enemy_defeated() {
        battle_data.set_state(BattleState::Won);
        msg!("Victory! Enemy defeated!");
        return Ok(());
    }
    
    // Switch to enemy turn
    battle_data.next_turn();
    
    // Execute enemy turn (AI)
    execute_enemy_turn(battle_data, hero_data, enemy_data, turn_count + 1)?;
    
    // Check if hero defeated
    if battle_data.hero_defeated() {
        battle_data.set_state(BattleState::Lost);
        msg!("Defeat! Hero was defeated!");
        return Ok(());
    }
    
    // Switch back to hero turn
    battle_data.next_turn();
    
    msg!("Turn complete. Hero HP: {} | Enemy HP: {}", battle_data.hero_hp, battle_data.enemy_hp);
    
    Ok(())
}

fn execute_hero_attack(
    battle: &mut Battle,
    hero: &Hero,
    enemy: &EnemyTemplate,
    turn: u8,
) -> ProgramResult {
    let base_damage = hero.calculate_damage(true);
    let defense = enemy.calculate_defense(true);
    let mut damage = calculate_damage_with_defense(base_damage, defense);
    
    // Check for crit
    let crit_chance = hero.crit_chance();
    if rng_check(&battle.rng_seed, turn, crit_chance * 100) {
        damage = apply_critical(damage, CRIT_MULTIPLIER);
        msg!("Critical hit! {} damage", damage);
    } else {
        msg!("Hero attacks for {} damage", damage);
    }
    
    battle.apply_damage_to_enemy(damage);
    Ok(())
}

fn execute_hero_skill(
    battle: &mut Battle,
    hero: &Hero,
    enemy: &EnemyTemplate,
    turn: u8,
) -> ProgramResult {
    // Magic attack based on intelligence
    let base_damage = hero.calculate_damage(false);
    let defense = enemy.calculate_defense(false);
    let damage = calculate_damage_with_defense(base_damage, defense);
    
    msg!("Hero uses skill for {} magic damage", damage);
    battle.apply_damage_to_enemy(damage);
    Ok(())
}

fn execute_enemy_turn(
    battle: &mut Battle,
    hero: &Hero,
    enemy: &EnemyTemplate,
    turn: u8,
) -> ProgramResult {
    // Simple AI: 70% attack, 30% skill
    let ai_choice = rng_percentage(&battle.rng_seed, turn);
    
    if ai_choice < 70 {
        // Physical attack
        let base_damage = enemy.calculate_damage(true);
        let defense = hero.calculate_defense(true);
        let mut damage = calculate_damage_with_defense(base_damage, defense);
        
        // Check for evasion
        let evasion_chance = hero.evasion_chance();
        if rng_check(&battle.rng_seed, turn + 1, evasion_chance * 100) {
            msg!("Hero evaded the attack!");
            return Ok(());
        }
        
        // Check for enemy crit
        if rng_percentage(&battle.rng_seed, turn + 2) < 10 {
            damage = apply_critical(damage, CRIT_MULTIPLIER);
            msg!("Enemy critical hit! {} damage", damage);
        } else {
            msg!("Enemy attacks for {} damage", damage);
        }
        
        battle.apply_damage_to_hero(damage);
    } else {
        // Magic attack
        let base_damage = enemy.calculate_damage(false);
        let defense = hero.calculate_defense(false);
        let damage = calculate_damage_with_defense(base_damage, defense);
        
        msg!("Enemy uses skill for {} magic damage", damage);
        battle.apply_damage_to_hero(damage);
    }
    
    Ok(())
}
