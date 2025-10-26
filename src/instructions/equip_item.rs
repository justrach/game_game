use pinocchio::{
    account_info::AccountInfo,
    entrypoint::InstructionContext,
    msg,
    program_error::ProgramError,
    ProgramResult,
};
use crate::{
    errors::RpgError,
    state::{Hero, Item, ItemType},
    utils::validation::*,
};

/// Equip an item to a hero
/// 
/// Accounts:
/// 0. [signer] User wallet
/// 1. [writable] Hero PDA
/// 2. [] Item PDA
pub fn process(context: &mut InstructionContext) -> ProgramResult {
    // Get accounts
    let user = context.next_account()?;
    let hero_account = context.next_account()?;
    let item_account = context.next_account()?;
    
    // Validate
    assert_signer(user)?;
    assert_writable(hero_account)?;
    assert_owned_by_program(hero_account, context.program_id())?;
    assert_owned_by_program(item_account, context.program_id())?;
    
    // Get hero data
    let hero_data = unsafe { &mut *(hero_account.borrow_mut_data().as_mut_ptr() as *mut Hero) };
    if !hero_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Verify ownership
    if hero_data.owner != *user.key() {
        return Err(RpgError::Unauthorized.into());
    }
    
    // Get item data
    let item_data = unsafe { &*(item_account.borrow_data().as_ptr() as *const Item) };
    if !item_data.is_initialized() {
        return Err(RpgError::AccountNotInitialized.into());
    }
    
    // Verify item is owned by this hero
    if item_data.owner != *hero_account.key() {
        return Err(RpgError::Unauthorized.into());
    }
    
    // Get item type
    let item_type = item_data.get_item_type().ok_or(RpgError::InvalidItemType)?;
    let item_key = *item_account.key();
    
    // Equip to appropriate slot
    match item_type {
        ItemType::Weapon => {
            if hero_data.weapon.is_some() {
                return Err(RpgError::ItemSlotOccupied.into());
            }
            hero_data.weapon = Some(item_key);
            msg!("Weapon equipped to hero");
        }
        ItemType::Armor => {
            if hero_data.armor.is_some() {
                return Err(RpgError::ItemSlotOccupied.into());
            }
            hero_data.armor = Some(item_key);
            msg!("Armor equipped to hero");
        }
        ItemType::Accessory => {
            if hero_data.accessory.is_some() {
                return Err(RpgError::ItemSlotOccupied.into());
            }
            hero_data.accessory = Some(item_key);
            msg!("Accessory equipped to hero");
        }
    }
    
    Ok(())
}
