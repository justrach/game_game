# Instruction Reference

Complete reference for all program instructions.

## Admin Instructions

### 12. initialize_treasury
Initialize the treasury account (one-time setup).

**Accounts:**
- `[signer, writable]` Admin wallet
- `[writable]` Treasury PDA
- `[]` System program

**Data:** `[12]`

### 11. create_enemy_template
Create a new enemy template.

**Accounts:**
- `[signer, writable]` Admin wallet
- `[writable]` Enemy template PDA
- `[]` System program

**Data:** `[11, enemy_id(4), attributes(16), level(2), ai_flags(4), max_hp(4), loot_table(32)]`

## Player Instructions

### 0. player_initialize
Create a player profile.

**Accounts:**
- `[signer, writable]` User wallet
- `[writable]` Player PDA
- `[]` System program

**Data:** `[0]`

### 1. buy_hero
Purchase a new hero (costs 1 SOL).

**Accounts:**
- `[signer, writable]` User wallet
- `[writable]` Player PDA
- `[writable]` Hero PDA
- `[writable]` Treasury PDA
- `[]` System program

**Data:** `[1]`

### 2. level_up_hero
Level up a hero if enough XP.

**Accounts:**
- `[signer]` User wallet
- `[writable]` Hero PDA

**Data:** `[2]`

## Equipment Instructions

### 3. equip_item
Equip an item to a hero.

**Accounts:**
- `[signer]` User wallet
- `[writable]` Hero PDA
- `[]` Item PDA

**Data:** `[3]`

### 4. unequip_item
Unequip an item from a hero.

**Accounts:**
- `[signer]` User wallet
- `[writable]` Hero PDA

**Data:** `[4, item_type]`

## Slot Roll Instructions

### 5. roll_start
Start a slot machine roll (costs 0.1 SOL).

**Accounts:**
- `[signer, writable]` User wallet
- `[]` Hero PDA
- `[writable]` Roll session PDA
- `[writable]` Treasury PDA
- `[]` System program

**Data:** `[5]`

### 6. roll_fulfill
Fulfill roll with VRF (oracle only).

**Accounts:**
- `[signer]` Oracle wallet
- `[writable]` Roll session PDA

**Data:** `[6, rng_seed(32)]`

### 7. roll_confirm
Confirm roll and claim rewards.

**Accounts:**
- `[signer]` User wallet
- `[writable]` Hero PDA
- `[writable]` Player PDA
- `[writable]` Roll session PDA

**Data:** `[7]`

## Battle Instructions

### 8. battle_start
Start a battle (costs 0.05 SOL).

**Accounts:**
- `[signer, writable]` User wallet
- `[]` Hero PDA
- `[]` Enemy template PDA
- `[writable]` Battle PDA
- `[writable]` Treasury PDA
- `[]` System program

**Data:** `[8]`

### 9. battle_turn
Execute a battle turn.

**Accounts:**
- `[signer]` User wallet
- `[]` Hero PDA
- `[]` Enemy template PDA
- `[writable]` Battle PDA

**Data:** `[9, action]`

Actions: 0=Attack, 1=Defend, 2=Skill, 3=Escape

### 10. battle_settle
Settle completed battle.

**Accounts:**
- `[signer]` User wallet
- `[writable]` Hero PDA
- `[writable]` Player PDA
- `[]` Enemy template PDA
- `[writable]` Battle PDA

**Data:** `[10]`
