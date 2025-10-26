/// Math utilities for game calculations

/// Calculate damage with defense reduction
pub fn calculate_damage_with_defense(base_damage: u32, defense: u32) -> u32 {
    // Damage = max(1, base_damage - defense/2)
    let reduction = defense / 2;
    if base_damage <= reduction {
        1 // Minimum 1 damage
    } else {
        base_damage - reduction
    }
}

/// Apply critical hit multiplier
pub fn apply_critical(damage: u32, multiplier: u32) -> u32 {
    damage.saturating_mul(multiplier)
}

/// Calculate XP reward based on enemy level and player level
pub fn calculate_xp_reward(enemy_level: u16, player_level: u16) -> u32 {
    let base_xp = 50u32;
    let level_diff = if enemy_level > player_level {
        enemy_level - player_level
    } else {
        0
    };
    
    // More XP for higher level enemies
    base_xp + (enemy_level as u32 * 10) + (level_diff as u32 * 20)
}

/// Calculate escape chance based on agility difference
pub fn calculate_escape_chance(hero_agility: u16, enemy_agility: u16) -> u16 {
    let base_chance = 3000u16; // 30% base
    
    if hero_agility > enemy_agility {
        let bonus = ((hero_agility - enemy_agility) as u32 * 100) as u16;
        base_chance.saturating_add(bonus).min(8000) // Max 80%
    } else {
        let penalty = ((enemy_agility - hero_agility) as u32 * 50) as u16;
        base_chance.saturating_sub(penalty).max(1000) // Min 10%
    }
}

/// Calculate loot quality based on luck stat
pub fn calculate_loot_quality(luck: u16, base_quality: u16) -> u16 {
    // Luck increases loot quality
    let bonus = luck / 10;
    base_quality.saturating_add(bonus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_calculation() {
        assert_eq!(calculate_damage_with_defense(100, 20), 90);
        assert_eq!(calculate_damage_with_defense(10, 20), 1); // Minimum damage
    }

    #[test]
    fn test_critical_hit() {
        assert_eq!(apply_critical(50, 2), 100);
        assert_eq!(apply_critical(75, 3), 225);
    }

    #[test]
    fn test_xp_reward() {
        let xp = calculate_xp_reward(5, 1);
        assert!(xp > 50); // Should be more than base
    }

    #[test]
    fn test_escape_chance() {
        let chance = calculate_escape_chance(50, 30);
        assert!(chance > 3000); // Should be higher with more agility
    }
}
