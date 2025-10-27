# On-Chain RPG TypeScript Client

TypeScript/Bun client for interacting with the On-Chain RPG Solana program.

## Prerequisites

- [Bun](https://bun.sh) installed
- Solana devnet access

## Installation

```bash
bun install
```

## Usage

### Run Demo

```bash
bun run demo
```

This will:
1. Connect to Solana devnet
2. Verify the program exists
3. Create a test wallet
4. Request an airdrop
5. Invoke the program
6. Display transaction logs

### Run Tests

```bash
bun test
```

## Project Structure

```
client/
├── connection.ts    # Connection and wallet utilities
└── instructions.ts  # Instruction builders and PDAs

tests/
├── basic.test.ts    # Basic integration tests
└── demo.ts          # Interactive demo script
```

## Program ID

**Devnet:** `52TdioxTshbR5JuD52afPJH1HwVgNA9CYKwBjSxVW2io`

## API Reference

### Connection

```typescript
import { getConnection, PROGRAM_ID } from './client/connection';

const connection = getConnection('devnet');
```

### Instructions

```typescript
import { createInvokeInstruction } from './client/instructions';

const instruction = createInvokeInstruction(wallet.publicKey);
```

### PDAs

```typescript
import { findPlayerPDA, findHeroPDA } from './client/instructions';

const [playerPDA, bump] = findPlayerPDA(wallet.publicKey);
const [heroPDA, bump] = findHeroPDA(wallet.publicKey, 0);
```

## Development

The client is built with:
- **Bun** - Fast JavaScript runtime
- **@solana/web3.js** - Solana JavaScript SDK
- **TypeScript** - Type safety

## Testing

Tests use Bun's built-in test runner:

```bash
# Run all tests
bun test

# Run specific test
bun test tests/basic.test.ts

# Watch mode
bun test --watch
```

## Examples

### Simple Program Invocation

```typescript
import { Connection, Keypair, Transaction } from '@solana/web3.js';
import { getConnection, PROGRAM_ID } from './client/connection';
import { createInvokeInstruction } from './client/instructions';

const connection = getConnection('devnet');
const wallet = Keypair.generate();

const instruction = createInvokeInstruction(wallet.publicKey);
const transaction = new Transaction().add(instruction);

const signature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [wallet]
);

console.log('Transaction:', signature);
```

## License

MIT
