# On-Chain RPG - Solana Smart Contract

A fully on-chain RPG game built with **Pinocchio** - a zero-dependency, lightweight Solana program framework. This smart contract implements a complete turn-based battle system, hero management, item equipment, slot machine rolls, and verifiable randomness.

## 🎮 Features

### Core Systems
- ✅ **Player Profiles** - Persistent player accounts with stats tracking
- ✅ **Hero Management** - Create, level up, and customize heroes
- ✅ **Attribute System** - 8 attributes (STR, DEX, VIT, INT, WIS, AGI, PREC, LUCK)
- ✅ **Equipment System** - Weapon, Armor, and Accessory slots
- ✅ **Turn-Based Battles** - PvE combat with AI enemies
- ✅ **Slot Machine Rolls** - Randomized XP rewards
- ✅ **Treasury System** - On-chain fee collection and rewards
- ✅ **Verifiable Randomness** - VRF-ready RNG system

### Technical Highlights
- **Zero Dependencies** - Built with Pinocchio for minimal compute units
- **PDA Architecture** - All accounts are Program Derived Addresses
- **Modular Design** - Clean separation of state, instructions, and utilities
- **Security First** - Comprehensive validation and anti-cheat measures
- **Gas Optimized** - Efficient memory layout and operations

## 📋 Architecture

### Account Structure

```
Player PDA ["player", wallet]
├── Hero PDA ["hero", wallet, index]
│   ├── Item PDA ["item", hero, index] (Weapon)
│   ├── Item PDA ["item", hero, index] (Armor)
│   └── Item PDA ["item", hero, index] (Accessory)
├── Battle PDA ["battle", hero, nonce]
└── RollSession PDA ["roll", hero, nonce]

EnemyTemplate PDA ["enemy_template", id]
Treasury PDA ["treasury"]
```

### Instruction Set

| ID | Instruction | Description |
|----|-------------|-------------|
| 0 | `player_initialize` | Create player profile |
| 1 | `buy_hero` | Purchase a new hero |
| 2 | `level_up_hero` | Level up hero with XP |
| 3 | `equip_item` | Equip item to hero |
| 4 | `unequip_item` | Remove equipped item |
| 5 | `roll_start` | Start slot machine roll |
| 6 | `roll_fulfill` | Fulfill roll with VRF |
| 7 | `roll_confirm` | Claim roll rewards |
| 8 | `battle_start` | Start battle vs enemy |
| 9 | `battle_turn` | Execute battle turn |
| 10 | `battle_settle` | Settle battle & rewards |
| 11 | `create_enemy_template` | Create enemy (admin) |
| 12 | `initialize_treasury` | Initialize treasury (admin) |

## 🚀 Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Add BPF target
rustup target add bpfel-unknown-unknown
```

### Build

```bash
# Build the program
cargo build-bpf

# Or use release mode for production
cargo build-bpf --release
```

### Test

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test-bpf
```

### Deploy

```bash
# Deploy to devnet
solana program deploy target/deploy/onchain_rpg.so --url devnet

# Deploy to mainnet (use with caution)
solana program deploy target/deploy/onchain_rpg.so --url mainnet-beta
```

## 🎯 Game Mechanics

### Attributes

| Attribute | Effect |
|-----------|--------|
| **Strength** | Physical damage |
| **Dexterity** | Attack speed, dodge |
| **Vitality** | HP, physical defense |
| **Intelligence** | Magic damage |
| **Wisdom** | Magic defense, mana |
| **Agility** | Evasion chance |
| **Precision** | Critical hit chance |
| **Luck** | Loot quality, rare drops |

### Battle System

**Turn Flow:**
1. Player chooses action (Attack/Defend/Skill/Escape)
2. Damage calculated with defense reduction
3. Critical hits and evasion checks
4. Enemy AI responds
5. Battle continues until victory/defeat/escape

**Damage Formula:**
```
damage = max(1, base_damage - defense/2)
critical_damage = damage * 2
```

**Escape Chance:**
```
base_chance = 30%
bonus = (hero_agility - enemy_agility) * 1%
final_chance = clamp(base_chance + bonus, 10%, 80%)
```

### Progression

**Leveling:**
- XP required per level: `1000 * level`
- Attribute points per level: `5`
- Max level: `100`

**Economy:**
- Hero purchase: `1 SOL`
- Slot roll: `0.1 SOL`
- Battle entry: `0.05 SOL`

## 📊 State Structures

### Hero
```rust
pub struct Hero {
    pub owner: Pubkey,
    pub level: u16,
    pub xp: u32,
    pub base_attributes: Attributes,
    pub weapon: Option<Pubkey>,
    pub armor: Option<Pubkey>,
    pub accessory: Option<Pubkey>,
}
```

### Battle
```rust
pub struct Battle {
    pub hero: Pubkey,
    pub enemy_template: Pubkey,
    pub hero_hp: u32,
    pub enemy_hp: u32,
    pub rng_seed: [u8; 32],
    pub turn: u8,
    pub state: u8,
}
```

## 🔒 Security Features

- **PDA Validation** - All accounts verified against expected seeds
- **Ownership Checks** - Users can only modify their own data
- **Signer Verification** - Critical operations require signatures
- **State Validation** - Battle/roll states prevent replay attacks
- **Rent-Exempt** - All accounts properly funded
- **Overflow Protection** - Saturating arithmetic throughout

## 🛠️ Development

### Project Structure

```
src/
├── lib.rs                 # Program entrypoint
├── constants.rs           # Game constants
├── errors.rs              # Error definitions
├── state/                 # Account structures
│   ├── player.rs
│   ├── hero.rs
│   ├── item.rs
│   ├── enemy_template.rs
│   ├── battle.rs
│   ├── roll_session.rs
│   └── treasury.rs
├── instructions/          # Instruction handlers
│   ├── player_initialize.rs
│   ├── buy_hero.rs
│   ├── battle_start.rs
│   ├── battle_turn.rs
│   └── ...
└── utils/                 # Utilities
    ├── rng.rs
    ├── math.rs
    └── validation.rs
```

### Adding New Features

1. **New Enemy Type:**
   - Call `create_enemy_template` with custom attributes
   - Set AI flags for behavior patterns
   - Configure loot table

2. **New Item:**
   - Create item account with bonus attributes
   - Link to hero via equipment slots
   - Attributes automatically apply in battle

3. **New Battle Action:**
   - Add to `BattleAction` enum
   - Implement logic in `battle_turn.rs`
   - Update damage calculations

## 📈 Roadmap

### Phase 1 (Current)
- ✅ Core RPG mechanics
- ✅ Turn-based battles
- ✅ Hero progression
- ✅ Slot rolls

### Phase 2 (Planned)
- [ ] PvP battles
- [ ] Guild system
- [ ] Raid bosses
- [ ] NFT items (tradable)

### Phase 3 (Future)
- [ ] World events
- [ ] Crafting system
- [ ] Marketplace
- [ ] Tournament mode

## 🤝 Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details

## 🔗 Resources

- [Pinocchio Documentation](https://github.com/anza-xyz/pinocchio)
- [Solana Documentation](https://docs.solana.com)
- [Anchor Framework](https://www.anchor-lang.com) (alternative)

## 💬 Support

For questions and support:
- Open an issue on GitHub
- Join our Discord community
- Check the documentation wiki

---

**Built with ❤️ using Pinocchio on Solana**
