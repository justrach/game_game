/// Program constants

// PDA Seeds
pub const PLAYER_SEED: &[u8] = b"player";
pub const HERO_SEED: &[u8] = b"hero";
pub const ITEM_SEED: &[u8] = b"item";
pub const ENEMY_TEMPLATE_SEED: &[u8] = b"enemy_template";
pub const BATTLE_SEED: &[u8] = b"battle";
pub const ROLL_SESSION_SEED: &[u8] = b"roll";
pub const TREASURY_SEED: &[u8] = b"treasury";

// Game balance constants
pub const HERO_BASE_PRICE: u64 = 1_000_000_000; // 1 SOL
pub const ROLL_PRICE: u64 = 100_000_000; // 0.1 SOL
pub const BATTLE_ENTRY_FEE: u64 = 50_000_000; // 0.05 SOL

// Battle constants
pub const MAX_BATTLE_TURNS: u8 = 20;
pub const CRIT_MULTIPLIER: u32 = 2;

// Slot roll constants
pub const ROLL_XP_MIN: u32 = 10;
pub const ROLL_XP_MAX: u32 = 100;
pub const ROLL_LEGENDARY_THRESHOLD: u16 = 9900; // 1% chance (out of 10000)
pub const ROLL_RARE_THRESHOLD: u16 = 9500; // 5% chance

// Level up constants
pub const XP_PER_LEVEL: u32 = 1000;
pub const MAX_LEVEL: u16 = 100;

// Attribute constants
pub const BASE_ATTRIBUTE_POINTS: u16 = 10;
pub const ATTRIBUTE_POINTS_PER_LEVEL: u16 = 5;

// Account sizes
pub const PLAYER_SIZE: usize = 8 + 32 + 8 + 4 + 8; // discriminator + owner + hero_count + total_xp + created_at
pub const HERO_SIZE: usize = 8 + 32 + 2 + 4 + 16 + 33 + 33 + 33 + 8; // discriminator + owner + level + xp + attributes + 3 equipment slots + created_at
pub const ITEM_SIZE: usize = 8 + 32 + 1 + 16 + 8; // discriminator + owner + item_type + attributes + created_at
pub const ENEMY_TEMPLATE_SIZE: usize = 8 + 4 + 16 + 2 + 4 + 4 + 32; // discriminator + id + attributes + level + ai_flags + max_hp + loot_table
pub const BATTLE_SIZE: usize = 8 + 32 + 32 + 4 + 4 + 32 + 1 + 1 + 8 + 8; // discriminator + hero + enemy_template + hero_hp + enemy_hp + rng_seed + turn + state + last_action_slot + nonce
pub const ROLL_SESSION_SIZE: usize = 8 + 32 + 32 + 1 + 8 + 8; // discriminator + hero + rng_seed + state + created_at + nonce
pub const TREASURY_SIZE: usize = 8 + 32 + 8; // discriminator + authority + total_collected
