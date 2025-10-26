use pinocchio::pubkey::Pubkey;
use super::Attributes;

/// Enemy template - reusable enemy configuration
#[repr(C)]
pub struct EnemyTemplate {
    pub discriminator: u64,
    pub id: u32,                    // Unique enemy ID
    pub base_attributes: Attributes,
    pub level: u16,
    pub ai_flags: u32,              // Behavior flags for AI
    pub max_hp: u32,
    pub loot_table: Pubkey,         // Reference to loot configuration
}

impl EnemyTemplate {
    pub const DISCRIMINATOR: u64 = 0x656e656d79_000000; // "enemy" in hex

    pub fn new(
        id: u32,
        base_attributes: Attributes,
        level: u16,
        ai_flags: u32,
        max_hp: u32,
        loot_table: Pubkey,
    ) -> Self {
        Self {
            discriminator: Self::DISCRIMINATOR,
            id,
            base_attributes,
            level,
            ai_flags,
            max_hp,
            loot_table,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.discriminator == Self::DISCRIMINATOR
    }

    pub fn calculate_damage(&self, is_physical: bool) -> u32 {
        if is_physical {
            10 + (self.base_attributes.strength as u32 * 2)
        } else {
            10 + (self.base_attributes.intelligence as u32 * 2)
        }
    }

    pub fn calculate_defense(&self, is_physical: bool) -> u32 {
        if is_physical {
            self.base_attributes.vitality as u32
        } else {
            self.base_attributes.wisdom as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_template_initialization() {
        let attributes = Attributes::new_base();
        let loot_table = Pubkey::default();
        let enemy = EnemyTemplate::new(1, attributes, 5, 0, 100, loot_table);
        
        assert!(enemy.is_initialized());
        assert_eq!(enemy.id, 1);
        assert_eq!(enemy.level, 5);
    }
}
