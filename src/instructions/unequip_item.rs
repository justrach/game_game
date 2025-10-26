use pinocchio::{
    account_info::AccountInfo,
    entrypoint::InstructionContext,
    msg,
    program_error::ProgramError,
    ProgramResult,
};
use crate::{
    errors::RpgError,
    state::{Hero, ItemType},
    utils::validation::*,
};

/// Unequip an item from a hero
/// 
/// Accounts:
/// 0. [signer] User wallet
/// 1. [writable] Hero PDA
/// 
/// Instruction data:
/// - byte 0: discriminator (4)
/// - byte 1: item_type (0=weapon, 1=armor, 2=accessory)
pub fn process(context: &mut InstructionContext) -> ProgramResult {
    // Get accounts
    let user = context.next_account()?;
    let hero_account = context.next_account()?;
    
    // Validate
    assert_signer(user)?;
    assert_writable(hero_account)?;
    assert_owned_by_program(hero_account, context.program_id())?;
    
    // Get instruction data
    let instruction_data = context.instruction_data();
    if instruction_data.len() < 2 {
        return Err(RpgError::InvalidInstruction.into());
    }
    
    let item_type = ItemType::from_u8(instruction_data[1]).ok_or(RpgError::InvalidItemType)?;
    
    // Get hero data
    let hero_data = unsafe { &mut *(hero_account.borrow_mut_data().as_mut_ptr() as *mut Hero) };
    if !hero_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Verify ownership
    if hero_data.owner != *user.key() {
        return Err(RpgError::Unauthorized.into());
    }
    
    // Unequip from appropriate slot
    match item_type {
        ItemType::Weapon => {
            if hero_data.weapon.is_none() {
                return Err(RpgError::ItemSlotEmpty.into());
            }
            hero_data.weapon = None;
            msg!("Weapon unequipped from hero");
        }
        ItemType::Armor => {
            if hero_data.armor.is_none() {
                return Err(RpgError::ItemSlotEmpty.into());
            }
            hero_data.armor = None;
            msg!("Armor unequipped from hero");
        }
        ItemType::Accessory => {
            if hero_data.accessory.is_none() {
                return Err(RpgError::ItemSlotEmpty.into());
            }
            hero_data.accessory = None;
            msg!("Accessory unequipped from hero");
        }
    }
    
    Ok(())
}
