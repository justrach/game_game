use pinocchio::pubkey::Pubkey;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BattleState {
    Active = 0,
    Won = 1,
    Lost = 2,
    Escaped = 3,
}

impl BattleState {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(BattleState::Active),
            1 => Some(BattleState::Won),
            2 => Some(BattleState::Lost),
            3 => Some(BattleState::Escaped),
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BattleAction {
    Attack = 0,
    Defend = 1,
    Skill = 2,
    Escape = 3,
}

impl BattleAction {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(BattleAction::Attack),
            1 => Some(BattleAction::Defend),
            2 => Some(BattleAction::Skill),
            3 => Some(BattleAction::Escape),
            _ => None,
        }
    }
}

/// Battle account - temporary state for active battles
#[repr(C)]
pub struct Battle {
    pub discriminator: u64,
    pub hero: Pubkey,
    pub enemy_template: Pubkey,
    pub hero_hp: u32,
    pub enemy_hp: u32,
    pub rng_seed: [u8; 32],         // VRF-provided randomness
    pub turn: u8,                   // 0 = hero, 1 = enemy
    pub state: u8,                  // BattleState
    pub last_action_slot: u64,      // Prevent spam
    pub nonce: u64,                 // Battle instance counter
}

impl Battle {
    pub const DISCRIMINATOR: u64 = 0x626174746c65_0000; // "battle" in hex

    pub fn new(
        hero: Pubkey,
        enemy_template: Pubkey,
        hero_hp: u32,
        enemy_hp: u32,
        nonce: u64,
    ) -> Self {
        Self {
            discriminator: Self::DISCRIMINATOR,
            hero,
            enemy_template,
            hero_hp,
            enemy_hp,
            rng_seed: [0u8; 32],
            turn: 0,
            state: BattleState::Active as u8,
            last_action_slot: 0,
            nonce,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.discriminator == Self::DISCRIMINATOR
    }

    pub fn get_state(&self) -> Option<BattleState> {
        BattleState::from_u8(self.state)
    }

    pub fn set_state(&mut self, state: BattleState) {
        self.state = state as u8;
    }

    pub fn is_active(&self) -> bool {
        self.get_state() == Some(BattleState::Active)
    }

    pub fn is_hero_turn(&self) -> bool {
        self.turn == 0
    }

    pub fn next_turn(&mut self) {
        self.turn = if self.turn == 0 { 1 } else { 0 };
    }

    pub fn set_rng_seed(&mut self, seed: [u8; 32]) {
        self.rng_seed = seed;
    }

    pub fn hero_defeated(&self) -> bool {
        self.hero_hp == 0
    }

    pub fn enemy_defeated(&self) -> bool {
        self.enemy_hp == 0
    }

    pub fn apply_damage_to_hero(&mut self, damage: u32) {
        self.hero_hp = self.hero_hp.saturating_sub(damage);
    }

    pub fn apply_damage_to_enemy(&mut self, damage: u32) {
        self.enemy_hp = self.enemy_hp.saturating_sub(damage);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battle_initialization() {
        let hero = Pubkey::default();
        let enemy = Pubkey::default();
        let battle = Battle::new(hero, enemy, 100, 50, 0);
        
        assert!(battle.is_initialized());
        assert!(battle.is_active());
        assert!(battle.is_hero_turn());
    }

    #[test]
    fn test_battle_turn_switching() {
        let hero = Pubkey::default();
        let enemy = Pubkey::default();
        let mut battle = Battle::new(hero, enemy, 100, 50, 0);
        
        assert!(battle.is_hero_turn());
        battle.next_turn();
        assert!(!battle.is_hero_turn());
        battle.next_turn();
        assert!(battle.is_hero_turn());
    }
}
