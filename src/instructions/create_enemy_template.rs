use pinocchio::{
    account_info::AccountInfo,
    entrypoint::InstructionContext,
    msg,
    program_error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;
use crate::{
    constants::{ENEMY_TEMPLATE_SEED, ENEMY_TEMPLATE_SIZE},
    errors::RpgError,
    state::{Attributes, EnemyTemplate},
    utils::validation::*,
};

/// Create an enemy template (admin only)
/// 
/// Accounts:
/// 0. [signer, writable] Admin wallet
/// 1. [writable] Enemy template PDA
/// 2. [] System program
/// 
/// Instruction data:
/// - byte 0: discriminator (11)
/// - bytes 1-4: enemy_id (u32)
/// - bytes 5-20: attributes (8 x u16)
/// - bytes 21-22: level (u16)
/// - bytes 23-26: ai_flags (u32)
/// - bytes 27-30: max_hp (u32)
/// - bytes 31-62: loot_table (Pubkey)
pub fn process(context: &mut InstructionContext) -> ProgramResult {
    // Get accounts
    let admin = context.next_account()?;
    let enemy_account = context.next_account()?;
    let system_program = context.next_account()?;
    
    // Validate
    assert_signer(admin)?;
    assert_writable(admin)?;
    assert_writable(enemy_account)?;
    
    // Get instruction data
    let instruction_data = context.instruction_data();
    if instruction_data.len() < 63 {
        return Err(RpgError::InvalidInstruction.into());
    }
    
    // Parse enemy data
    let enemy_id = u32::from_le_bytes([
        instruction_data[1],
        instruction_data[2],
        instruction_data[3],
        instruction_data[4],
    ]);
    
    let attributes = Attributes {
        strength: u16::from_le_bytes([instruction_data[5], instruction_data[6]]),
        dexterity: u16::from_le_bytes([instruction_data[7], instruction_data[8]]),
        vitality: u16::from_le_bytes([instruction_data[9], instruction_data[10]]),
        intelligence: u16::from_le_bytes([instruction_data[11], instruction_data[12]]),
        wisdom: u16::from_le_bytes([instruction_data[13], instruction_data[14]]),
        agility: u16::from_le_bytes([instruction_data[15], instruction_data[16]]),
        precision: u16::from_le_bytes([instruction_data[17], instruction_data[18]]),
        luck: u16::from_le_bytes([instruction_data[19], instruction_data[20]]),
    };
    
    let level = u16::from_le_bytes([instruction_data[21], instruction_data[22]]);
    
    let ai_flags = u32::from_le_bytes([
        instruction_data[23],
        instruction_data[24],
        instruction_data[25],
        instruction_data[26],
    ]);
    
    let max_hp = u32::from_le_bytes([
        instruction_data[27],
        instruction_data[28],
        instruction_data[29],
        instruction_data[30],
    ]);
    
    let mut loot_table_bytes = [0u8; 32];
    loot_table_bytes.copy_from_slice(&instruction_data[31..63]);
    let loot_table = pinocchio::pubkey::Pubkey::from(loot_table_bytes);
    
    // Derive enemy template PDA
    let enemy_id_bytes = enemy_id.to_le_bytes();
    let enemy_seeds = &[ENEMY_TEMPLATE_SEED, &enemy_id_bytes];
    let bump = assert_pda(enemy_account, enemy_seeds, context.program_id())?;
    
    // Check if already exists
    if enemy_account.data_len() > 0 {
        return Err(RpgError::AccountAlreadyInitialized.into());
    }
    
    // Create enemy template account
    let enemy_seeds_with_bump = &[ENEMY_TEMPLATE_SEED, &enemy_id_bytes, &[bump]];
    
    CreateAccount {
        from: admin,
        to: enemy_account,
        lamports: 1_500_000, // Rent-exempt
        space: ENEMY_TEMPLATE_SIZE as u64,
        owner: context.program_id(),
    }
    .invoke_signed(&[enemy_seeds_with_bump])?;
    
    // Initialize enemy template
    let enemy_data = EnemyTemplate::new(
        enemy_id,
        attributes,
        level,
        ai_flags,
        max_hp,
        loot_table,
    );
    
    let data = unsafe { &mut *enemy_account.borrow_mut_data() };
    let enemy_ptr = data.as_mut_ptr() as *mut EnemyTemplate;
    unsafe {
        *enemy_ptr = enemy_data;
    }
    
    msg!("Enemy template #{} created (Level {})", enemy_id, level);
    
    Ok(())
}
