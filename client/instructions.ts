import {
  PublicKey,
  TransactionInstruction,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from '@solana/web3.js';
import { PROGRAM_ID } from './connection';

// Instruction discriminators
export enum InstructionType {
  PlayerInitialize = 0,
  BuyHero = 1,
  LevelUpHero = 2,
  EquipItem = 3,
  UnequipItem = 4,
  RollStart = 5,
  RollFulfill = 6,
  RollConfirm = 7,
  BattleStart = 8,
  BattleTurn = 9,
  BattleSettle = 10,
  CreateEnemyTemplate = 11,
  InitializeTreasury = 12,
}

// PDA seeds
export const PLAYER_SEED = Buffer.from('player');
export const HERO_SEED = Buffer.from('hero');
export const TREASURY_SEED = Buffer.from('treasury');
export const BATTLE_SEED = Buffer.from('battle');
export const ROLL_SEED = Buffer.from('roll');
export const ENEMY_SEED = Buffer.from('enemy_template');

export function findPlayerPDA(wallet: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [PLAYER_SEED, wallet.toBuffer()],
    PROGRAM_ID
  );
}

export function findHeroPDA(wallet: PublicKey, index: number): [PublicKey, number] {
  const indexBuffer = Buffer.alloc(8);
  indexBuffer.writeBigUInt64LE(BigInt(index));
  return PublicKey.findProgramAddressSync(
    [HERO_SEED, wallet.toBuffer(), indexBuffer],
    PROGRAM_ID
  );
}

export function findTreasuryPDA(): [PublicKey, number] {
  return PublicKey.findProgramAddressSync([TREASURY_SEED], PROGRAM_ID);
}

export function createPlayerInitializeInstruction(
  wallet: PublicKey
): TransactionInstruction {
  const [playerPDA] = findPlayerPDA(wallet);

  return new TransactionInstruction({
    keys: [
      { pubkey: wallet, isSigner: true, isWritable: true },
      { pubkey: playerPDA, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_ID,
    data: Buffer.from([InstructionType.PlayerInitialize]),
  });
}

export function createInitializeTreasuryInstruction(
  admin: PublicKey
): TransactionInstruction {
  const [treasuryPDA] = findTreasuryPDA();

  return new TransactionInstruction({
    keys: [
      { pubkey: admin, isSigner: true, isWritable: true },
      { pubkey: treasuryPDA, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_ID,
    data: Buffer.from([InstructionType.InitializeTreasury]),
  });
}

export function createBuyHeroInstruction(
  wallet: PublicKey,
  heroIndex: number
): TransactionInstruction {
  const [playerPDA] = findPlayerPDA(wallet);
  const [heroPDA] = findHeroPDA(wallet, heroIndex);
  const [treasuryPDA] = findTreasuryPDA();

  return new TransactionInstruction({
    keys: [
      { pubkey: wallet, isSigner: true, isWritable: true },
      { pubkey: playerPDA, isSigner: false, isWritable: true },
      { pubkey: heroPDA, isSigner: false, isWritable: true },
      { pubkey: treasuryPDA, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_ID,
    data: Buffer.from([InstructionType.BuyHero]),
  });
}

// Simple invoke instruction for testing
export function createInvokeInstruction(wallet: PublicKey): TransactionInstruction {
  return new TransactionInstruction({
    keys: [{ pubkey: wallet, isSigner: true, isWritable: false }],
    programId: PROGRAM_ID,
    data: Buffer.from([]), // Empty data for simple invoke
  });
}
