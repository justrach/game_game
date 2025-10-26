use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};
use crate::errors::RpgError;

/// Validate that an account is owned by the program
pub fn assert_owned_by_program(account: &AccountInfo, program_id: &Pubkey) -> Result<(), ProgramError> {
    if account.owner() != program_id {
        return Err(RpgError::InvalidAccountOwner.into());
    }
    Ok(())
}

/// Validate that an account is a signer
pub fn assert_signer(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_signer() {
        return Err(RpgError::Unauthorized.into());
    }
    Ok(())
}

/// Validate that an account is writable
pub fn assert_writable(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_writable() {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}

/// Validate PDA derivation
pub fn assert_pda(
    account: &AccountInfo,
    seeds: &[&[u8]],
    program_id: &Pubkey,
) -> Result<u8, ProgramError> {
    let (expected_key, bump) = Pubkey::find_program_address(seeds, program_id);
    
    if account.key() != &expected_key {
        return Err(RpgError::InvalidPDA.into());
    }
    
    Ok(bump)
}

/// Validate that account has sufficient lamports
pub fn assert_sufficient_lamports(account: &AccountInfo, required: u64) -> Result<(), ProgramError> {
    if unsafe { *account.borrow_lamports() } < required {
        return Err(RpgError::InsufficientFunds.into());
    }
    Ok(())
}

/// Validate that account data is the expected size
pub fn assert_account_size(account: &AccountInfo, expected_size: usize) -> Result<(), ProgramError> {
    if account.data_len() != expected_size {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Full testing requires mock AccountInfo which is complex
    // These tests demonstrate the validation logic structure
}
