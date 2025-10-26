# Architecture Documentation

## Overview

This on-chain RPG is built using the Pinocchio framework, a zero-dependency Solana program SDK that optimizes for minimal compute units and binary size. The architecture follows a modular design with clear separation between state management, business logic, and instruction handling.

## Design Principles

### 1. Zero-Copy Architecture
All account data structures use `#[repr(C)]` for direct memory mapping, eliminating serialization overhead.

### 2. PDA-Based Accounts
Every account is a Program Derived Address (PDA), ensuring:
- Deterministic account addresses
- Program-controlled signing authority
- Protection against unauthorized access

### 3. Stateless Instructions
Each instruction is self-contained and validates all inputs, preventing state corruption.

### 4. Gas Optimization
- Minimal allocations
- Efficient memory layout
- Saturating arithmetic to prevent panics
- Direct pointer operations where safe

## Account Architecture

### Account Hierarchy

```
┌─────────────────────────────────────────────────────────────┐
│                        Program ID                            │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
   ┌────▼────┐          ┌────▼────┐          ┌────▼────┐
   │ Player  │          │Treasury │          │ Enemy   │
   │   PDA   │          │   PDA   │          │Template │
   └────┬────┘          └─────────┘          └─────────┘
        │
   ┌────┴────────────────────────┐
   │                             │
┌──▼──┐                    ┌────▼─────┐
│Hero │                    │Hero (N)  │
│ PDA │                    │   PDA    │
└──┬──┘                    └────┬─────┘
   │                            │
   ├──────┬──────┬──────────────┼──────┬──────┐
   │      │      │              │      │      │
┌──▼──┐┌─▼──┐┌──▼───┐    ┌────▼──┐┌──▼──┐┌──▼───┐
│Item ││Item││Roll  │    │Battle ││Item ││Roll  │
│ PDA ││PDA ││Session│    │  PDA  ││PDA  ││Session│
└─────┘└────┘└──────┘    └───────┘└─────┘└──────┘
```

### PDA Seeds

| Account | Seeds | Purpose |
|---------|-------|---------|
| Player | `["player", wallet]` | One per wallet |
| Hero | `["hero", wallet, index]` | Multiple per player |
| Item | `["item", hero, index]` | Multiple per hero |
| Battle | `["battle", hero, nonce]` | Temporary, unique per battle |
| RollSession | `["roll", hero, nonce]` | Temporary, unique per roll |
| EnemyTemplate | `["enemy_template", id]` | Global, admin-created |
| Treasury | `["treasury"]` | Singleton |

### Account Sizes

Optimized for rent-exemption:

```rust
Player:         62 bytes  (~1.0 SOL rent)
Hero:          170 bytes  (~1.5 SOL rent)
Item:           70 bytes  (~1.0 SOL rent)
EnemyTemplate:  94 bytes  (~1.5 SOL rent)
Battle:        130 bytes  (~1.5 SOL rent)
RollSession:    82 bytes  (~1.0 SOL rent)
Treasury:       48 bytes  (~2.0 SOL rent + buffer)
```

## Data Flow

### Battle Flow

```
┌──────────┐
│  User    │
└────┬─────┘
     │
     │ 1. battle_start(hero, enemy)
     ▼
┌────────────────┐
│ Validate Hero  │──── Check ownership
│ Validate Enemy │──── Check exists
│ Pay Entry Fee  │──── Transfer to treasury
└────┬───────────┘
     │
     │ 2. Create Battle PDA
     ▼
┌────────────────┐
│ Initialize     │
│ Battle State   │──── hero_hp, enemy_hp
│                │──── rng_seed (VRF)
└────┬───────────┘
     │
     │ 3. battle_turn(action) [Loop]
     ▼
┌────────────────┐
│ Hero Action    │──── Attack/Defend/Skill/Escape
│ Calculate DMG  │──── With crit/evasion checks
│ Apply to Enemy │
└────┬───────────┘
     │
     ▼
┌────────────────┐
│ Enemy AI Turn  │──── 70% attack, 30% skill
│ Calculate DMG  │──── With crit/evasion checks
│ Apply to Hero  │
└────┬───────────┘
     │
     │ Check victory/defeat
     ▼
┌────────────────┐
│ battle_settle  │
│ Award XP       │──── Based on enemy level
│ Award Loot     │──── Based on luck stat
│ Close Battle   │──── Reclaim rent
└────────────────┘
```

### Roll Flow

```
┌──────────┐
│  User    │
└────┬─────┘
     │
     │ 1. roll_start(hero)
     ▼
┌────────────────┐
│ Pay Roll Fee   │──── 0.1 SOL to treasury
│ Create Roll PDA│──── With nonce
│ Request VRF    │──── (In production)
└────┬───────────┘
     │
     │ 2. roll_fulfill(seed) [Oracle]
     ▼
┌────────────────┐
│ Receive RNG    │──── 32-byte seed
│ Update State   │──── Pending → Fulfilled
└────┬───────────┘
     │
     │ 3. roll_confirm()
     ▼
┌────────────────┐
│ Calculate      │
│ Rewards        │──── 1% Legendary (max XP)
│                │──── 5% Rare (high XP)
│                │──── 94% Common (low XP)
└────┬───────────┘
     │
     ▼
┌────────────────┐
│ Apply XP       │──── To hero & player
│ Check Level Up │──── Auto-level if XP sufficient
│ Close Roll PDA │──── Reclaim rent
└────────────────┘
```

## Randomness System

### VRF Integration (Production)

```
┌──────────────┐
│ User Request │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Escrow Fee   │──── Pay for VRF service
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ VRF Oracle   │──── Switchboard/Chainlink
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Callback     │──── Fulfill with verified seed
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Use Seed     │──── Derive per-turn randomness
└──────────────┘
```

### Per-Turn RNG

```rust
// Single VRF seed used for entire battle/roll
pub fn rng_u16(seed: &[u8; 32], turn: u8) -> u16 {
    // XOR seed bytes with turn counter
    let mut result = [0u8; 2];
    result[0] = seed[0] ^ seed[16] ^ turn;
    result[1] = seed[1] ^ seed[17] ^ turn.wrapping_mul(3);
    u16::from_le_bytes(result)
}
```

**Benefits:**
- One VRF call per battle (cheap)
- Deterministic per-turn randomness
- Verifiable by anyone
- Cannot be manipulated

## Security Model

### Validation Layers

```
┌─────────────────────────────────────────┐
│         Instruction Received            │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│  1. Account Ownership Validation        │
│     - Check program owns account        │
│     - Verify PDA derivation             │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│  2. Signer Verification                 │
│     - User must sign transaction        │
│     - Authority checks for admin ops    │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│  3. State Validation                    │
│     - Account initialized               │
│     - Valid state transitions           │
│     - No replay attacks                 │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│  4. Business Logic Validation           │
│     - Sufficient funds                  │
│     - Valid game state                  │
│     - Cooldowns respected               │
└─────────────────┬───────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────┐
│         Execute Instruction             │
└─────────────────────────────────────────┘
```

### Anti-Cheat Measures

| Attack Vector | Prevention |
|---------------|------------|
| Fake stats | All stats in PDAs, validated on read |
| Item duplication | Items linked to hero via PDA |
| RNG manipulation | VRF-based randomness |
| Replay attacks | Nonce-based PDAs for battles/rolls |
| Spam actions | Slot-based cooldowns |
| Unauthorized access | Ownership + signer checks |
| State corruption | Atomic operations, validation |

## Performance Optimizations

### Compute Unit Savings

**Pinocchio vs Standard SDK:**
- CPI operations: **55-93% fewer CUs**
- Account parsing: **Zero-copy** (no deserialization)
- Binary size: **~50% smaller**

### Memory Layout

```rust
#[repr(C)]  // C-compatible layout
pub struct Hero {
    pub discriminator: u64,     // 8 bytes
    pub owner: Pubkey,          // 32 bytes
    pub level: u16,             // 2 bytes
    pub xp: u32,                // 4 bytes
    pub base_attributes: Attributes,  // 16 bytes (8x u16)
    pub weapon: Option<Pubkey>, // 33 bytes (1 + 32)
    pub armor: Option<Pubkey>,  // 33 bytes
    pub accessory: Option<Pubkey>, // 33 bytes
    pub created_at: i64,        // 8 bytes
}
// Total: 170 bytes (aligned)
```

### Pointer Operations

```rust
// Direct memory access (unsafe but fast)
let hero_data = unsafe { 
    &mut *(account.borrow_mut_data().as_mut_ptr() as *mut Hero) 
};
```

**Safety guarantees:**
- Account size validated before cast
- Discriminator checked for type safety
- Ownership verified before mutation

## Upgrade Strategy

### Version 1 (Current)
- Single program
- PvE battles
- Basic progression

### Version 2 (Planned)
- Add PvP battle instruction
- Extend Battle struct with pvp flag
- Maintain backward compatibility

### Version 3 (Future)
- Separate programs for:
  - Core game logic
  - Battle system
  - Economy/treasury
- Cross-program invocations

## Testing Strategy

### Unit Tests
- State structure initialization
- Attribute calculations
- RNG determinism
- Math utilities

### Integration Tests
- Full instruction flows
- Account creation
- PDA derivation
- Error conditions

### Fuzzing (Recommended)
- Random instruction sequences
- Invalid account combinations
- Edge case values

## Deployment Checklist

- [ ] Treasury initialized
- [ ] Enemy templates created
- [ ] Test on devnet
- [ ] Security audit
- [ ] VRF oracle configured
- [ ] Frontend integration tested
- [ ] Monitoring setup
- [ ] Upgrade authority managed

## Monitoring

### Key Metrics
- Total players created
- Active battles
- Treasury balance
- Average battle duration
- XP distribution
- Item creation rate

### Logs
All instructions emit structured logs:
```rust
msg!("Battle started! Hero HP: {} | Enemy HP: {}", hero_hp, enemy_hp);
msg!("Victory! Earned {} XP", xp_reward);
```

## Future Enhancements

### Technical
- [ ] Compressed accounts (Solana state compression)
- [ ] Batch operations (multiple heroes)
- [ ] Delegation (guild leaders)
- [ ] Time-locked rewards

### Gameplay
- [ ] Skill trees
- [ ] Crafting system
- [ ] Pet companions
- [ ] Seasonal events

---

**Last Updated:** 2024
**Version:** 1.0.0
