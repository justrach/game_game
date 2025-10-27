import { describe, test, expect } from 'bun:test';
import { Connection, Keypair, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { getConnection, PROGRAM_ID, airdrop } from '../client/connection';
import { createInvokeInstruction } from '../client/instructions';

describe('On-Chain RPG Basic Tests', () => {
  test('should connect to devnet', async () => {
    const connection = getConnection('devnet');
    const version = await connection.getVersion();
    expect(version).toBeDefined();
    console.log('✅ Connected to Solana devnet:', version);
  });

  test('should verify program exists', async () => {
    const connection = getConnection('devnet');
    const accountInfo = await connection.getAccountInfo(PROGRAM_ID);
    expect(accountInfo).not.toBeNull();
    expect(accountInfo?.executable).toBe(true);
    console.log('✅ Program verified on devnet');
    console.log('   Program ID:', PROGRAM_ID.toBase58());
    console.log('   Data length:', accountInfo?.data.length);
  });

  test('should invoke program successfully', async () => {
    const connection = getConnection('devnet');
    const payer = Keypair.generate();
    
    // Airdrop SOL for transaction fees
    await airdrop(connection, payer.publicKey, 1);
    
    // Create and send transaction
    const instruction = createInvokeInstruction(payer.publicKey);
    const transaction = new Transaction().add(instruction);
    
    const signature = await sendAndConfirmTransaction(
      connection,
      transaction,
      [payer],
      { commitment: 'confirmed' }
    );
    
    expect(signature).toBeDefined();
    console.log('✅ Program invoked successfully');
    console.log('   Signature:', signature);
  }, 30000); // 30 second timeout for airdrop
});
