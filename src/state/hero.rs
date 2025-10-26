use pinocchio::pubkey::Pubkey;
use super::Attributes;

/// Hero account - player's character with stats and equipment
#[repr(C)]
pub struct Hero {
    pub discriminator: u64,
    pub owner: Pubkey,
    pub level: u16,
    pub xp: u32,
    pub base_attributes: Attributes,
    pub weapon: Option<Pubkey>,      // Equipped weapon item
    pub armor: Option<Pubkey>,       // Equipped armor item
    pub accessory: Option<Pubkey>,   // Equipped accessory item
    pub created_at: i64,
}

impl Hero {
    pub const DISCRIMINATOR: u64 = 0x6865726f_00000000; // "hero" in hex

    pub fn new(owner: Pubkey, created_at: i64) -> Self {
        Self {
            discriminator: Self::DISCRIMINATOR,
            owner,
            level: 1,
            xp: 0,
            base_attributes: Attributes::new_base(),
            weapon: None,
            armor: None,
            accessory: None,
            created_at,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.discriminator == Self::DISCRIMINATOR
    }

    pub fn add_xp(&mut self, xp: u32) -> bool {
        self.xp = self.xp.saturating_add(xp);
        
        // Check if level up is possible
        let xp_needed = self.xp_needed_for_next_level();
        if self.xp >= xp_needed && self.level < crate::constants::MAX_LEVEL {
            self.level_up();
            return true;
        }
        false
    }

    pub fn level_up(&mut self) {
        if self.level >= crate::constants::MAX_LEVEL {
            return;
        }

        self.level += 1;
        self.xp = 0;

        // Add attribute points on level up
        let points = crate::constants::ATTRIBUTE_POINTS_PER_LEVEL;
        self.base_attributes.strength += points / 8;
        self.base_attributes.dexterity += points / 8;
        self.base_attributes.vitality += points / 8;
        self.base_attributes.intelligence += points / 8;
        self.base_attributes.wisdom += points / 8;
        self.base_attributes.agility += points / 8;
        self.base_attributes.precision += points / 8;
        self.base_attributes.luck += points / 8;
    }

    pub fn xp_needed_for_next_level(&self) -> u32 {
        crate::constants::XP_PER_LEVEL * (self.level as u32)
    }

    pub fn max_hp(&self) -> u32 {
        // Base HP = 100 + (vitality * 10) + (level * 5)
        100 + (self.base_attributes.vitality as u32 * 10) + (self.level as u32 * 5)
    }

    pub fn calculate_damage(&self, is_physical: bool) -> u32 {
        if is_physical {
            // Physical damage based on strength
            10 + (self.base_attributes.strength as u32 * 2)
        } else {
            // Magic damage based on intelligence
            10 + (self.base_attributes.intelligence as u32 * 2)
        }
    }

    pub fn calculate_defense(&self, is_physical: bool) -> u32 {
        if is_physical {
            // Physical defense based on vitality
            self.base_attributes.vitality as u32
        } else {
            // Magic defense based on wisdom
            self.base_attributes.wisdom as u32
        }
    }

    pub fn crit_chance(&self) -> u16 {
        // Precision gives crit chance (precision / 10 = % chance)
        self.base_attributes.precision / 10
    }

    pub fn evasion_chance(&self) -> u16 {
        // Agility gives evasion chance (agility / 10 = % chance)
        self.base_attributes.agility / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hero_initialization() {
        let owner = Pubkey::default();
        let hero = Hero::new(owner, 1234567890);
        
        assert!(hero.is_initialized());
        assert_eq!(hero.level, 1);
        assert_eq!(hero.xp, 0);
    }

    #[test]
    fn test_hero_level_up() {
        let owner = Pubkey::default();
        let mut hero = Hero::new(owner, 1234567890);
        
        let initial_strength = hero.base_attributes.strength;
        hero.add_xp(1000);
        
        assert_eq!(hero.level, 2);
        assert!(hero.base_attributes.strength > initial_strength);
    }
}
