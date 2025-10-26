use pinocchio::pubkey::Pubkey;

/// Treasury account - holds game fees and rewards
#[repr(C)]
pub struct Treasury {
    pub discriminator: u64,
    pub authority: Pubkey,      // Admin who can withdraw
    pub total_collected: u64,   // Total fees collected
}

impl Treasury {
    pub const DISCRIMINATOR: u64 = 0x7472656173757279; // "treasury" in hex

    pub fn new(authority: Pubkey) -> Self {
        Self {
            discriminator: Self::DISCRIMINATOR,
            authority,
            total_collected: 0,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.discriminator == Self::DISCRIMINATOR
    }

    pub fn add_collection(&mut self, amount: u64) {
        self.total_collected = self.total_collected.saturating_add(amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treasury_initialization() {
        let authority = Pubkey::default();
        let treasury = Treasury::new(authority);
        
        assert!(treasury.is_initialized());
        assert_eq!(treasury.total_collected, 0);
    }

    #[test]
    fn test_treasury_collection() {
        let authority = Pubkey::default();
        let mut treasury = Treasury::new(authority);
        
        treasury.add_collection(1000);
        assert_eq!(treasury.total_collected, 1000);
        
        treasury.add_collection(500);
        assert_eq!(treasury.total_collected, 1500);
    }
}
