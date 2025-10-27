import { describe, test, expect } from 'bun:test';
import { Connection, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { getConnection, PROGRAM_ID, checkBalance } from '../client/connection';
import { createInvokeInstruction } from '../client/instructions';
import { getOrCreateWallet } from '../client/wallet';

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
    const wallet = getOrCreateWallet();
    
    // Check balance
    const balance = await checkBalance(connection, wallet.publicKey);
    console.log('💰 Wallet balance:', balance, 'SOL');
    
    if (balance < 0.01) {
      console.log('⚠️  Insufficient balance, skipping transaction test');
      console.log('   Fund wallet: solana transfer', wallet.publicKey.toBase58(), '0.2 --url devnet');
      return; // Skip test if no balance
    }
    
    // Create and send transaction
    const instruction = createInvokeInstruction(wallet.publicKey);
    const transaction = new Transaction().add(instruction);
    
    const signature = await sendAndConfirmTransaction(
      connection,
      transaction,
      [wallet],
      { commitment: 'confirmed' }
    );
    
    expect(signature).toBeDefined();
    console.log('✅ Program invoked successfully');
    console.log('   Signature:', signature);
  }, 30000);
});
