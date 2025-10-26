use pinocchio::pubkey::Pubkey;

/// Player account - main profile for each wallet
#[repr(C)]
pub struct Player {
    pub discriminator: u64,    // Account type identifier
    pub owner: Pubkey,         // Wallet that owns this player
    pub hero_count: u64,       // Number of heroes owned
    pub total_xp: u32,         // Lifetime XP earned
    pub created_at: i64,       // Unix timestamp
}

impl Player {
    pub const DISCRIMINATOR: u64 = 0x706c61796572_0000; // "player" in hex

    pub fn new(owner: Pubkey, created_at: i64) -> Self {
        Self {
            discriminator: Self::DISCRIMINATOR,
            owner,
            hero_count: 0,
            total_xp: 0,
            created_at,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.discriminator == Self::DISCRIMINATOR
    }

    pub fn add_hero(&mut self) {
        self.hero_count = self.hero_count.saturating_add(1);
    }

    pub fn add_xp(&mut self, xp: u32) {
        self.total_xp = self.total_xp.saturating_add(xp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_initialization() {
        let owner = Pubkey::default();
        let player = Player::new(owner, 1234567890);
        
        assert!(player.is_initialized());
        assert_eq!(player.hero_count, 0);
        assert_eq!(player.total_xp, 0);
    }
}
