use pinocchio::pubkey::Pubkey;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RollState {
    Pending = 0,      // Waiting for VRF
    Fulfilled = 1,    // VRF completed, ready to confirm
    Confirmed = 2,    // Rewards claimed
}

impl RollState {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(RollState::Pending),
            1 => Some(RollState::Fulfilled),
            2 => Some(RollState::Confirmed),
            _ => None,
        }
    }
}

/// Roll session - slot machine roll state
#[repr(C)]
pub struct RollSession {
    pub discriminator: u64,
    pub hero: Pubkey,
    pub rng_seed: [u8; 32],
    pub state: u8,
    pub created_at: i64,
    pub nonce: u64,
}

impl RollSession {
    pub const DISCRIMINATOR: u64 = 0x726f6c6c_00000000; // "roll" in hex

    pub fn new(hero: Pubkey, created_at: i64, nonce: u64) -> Self {
        Self {
            discriminator: Self::DISCRIMINATOR,
            hero,
            rng_seed: [0u8; 32],
            state: RollState::Pending as u8,
            created_at,
            nonce,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.discriminator == Self::DISCRIMINATOR
    }

    pub fn get_state(&self) -> Option<RollState> {
        RollState::from_u8(self.state)
    }

    pub fn set_state(&mut self, state: RollState) {
        self.state = state as u8;
    }

    pub fn set_rng_seed(&mut self, seed: [u8; 32]) {
        self.rng_seed = seed;
    }

    pub fn is_pending(&self) -> bool {
        self.get_state() == Some(RollState::Pending)
    }

    pub fn is_fulfilled(&self) -> bool {
        self.get_state() == Some(RollState::Fulfilled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_session_initialization() {
        let hero = Pubkey::default();
        let roll = RollSession::new(hero, 1234567890, 0);
        
        assert!(roll.is_initialized());
        assert!(roll.is_pending());
    }

    #[test]
    fn test_roll_state_transitions() {
        let hero = Pubkey::default();
        let mut roll = RollSession::new(hero, 1234567890, 0);
        
        assert!(roll.is_pending());
        roll.set_state(RollState::Fulfilled);
        assert!(roll.is_fulfilled());
        roll.set_state(RollState::Confirmed);
        assert_eq!(roll.get_state(), Some(RollState::Confirmed));
    }
}
