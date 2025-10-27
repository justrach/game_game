#!/usr/bin/env bun
import { Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { getConnection, PROGRAM_ID, checkBalance } from '../client/connection';
import { createInvokeInstruction } from '../client/instructions';
import { getOrCreateWallet } from '../client/wallet';

async function main() {
  console.log('🎮 On-Chain RPG Demo\n');
  
  const connection = getConnection('devnet');
  console.log('📡 Connected to devnet');
  
  // Verify program
  const programInfo = await connection.getAccountInfo(PROGRAM_ID);
  if (!programInfo) {
    console.error('❌ Program not found!');
    process.exit(1);
  }
  
  console.log('✅ Program found:', PROGRAM_ID.toBase58());
  console.log('   Executable:', programInfo.executable);
  console.log('   Data length:', programInfo.data.length, 'bytes\n');
  
  // Load or create wallet
  const wallet = getOrCreateWallet();
  
  // Check balance
  console.log('\n💰 Checking balance...');
  const balance = await checkBalance(connection, wallet.publicKey);
  console.log('   Balance:', balance, 'SOL');
  
  if (balance < 0.01) {
    console.log('\n❌ Insufficient balance! Please send SOL to:', wallet.publicKey.toBase58());
    console.log('   Run: solana transfer', wallet.publicKey.toBase58(), '0.2 --url devnet');
    process.exit(1);
  }
  
  console.log('   ✅ Sufficient balance\n');
  
  // Invoke program
  console.log('🎯 Invoking program...');
  const instruction = createInvokeInstruction(wallet.publicKey);
  const transaction = new Transaction().add(instruction);
  
  const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [wallet],
    { commitment: 'confirmed' }
  );
  
  console.log('✅ Transaction successful!');
  console.log('   Signature:', signature);
  console.log('   Explorer: https://explorer.solana.com/tx/' + signature + '?cluster=devnet');
  
  // Get transaction logs
  const txDetails = await connection.getTransaction(signature, {
    commitment: 'confirmed',
    maxSupportedTransactionVersion: 0
  });
  
  if (txDetails?.meta?.logMessages) {
    console.log('\n📜 Program Logs:');
    txDetails.meta.logMessages.forEach(log => {
      if (log.includes('Program log:')) {
        console.log('   ', log.replace('Program log: ', ''));
      }
    });
  }
  
  console.log('\n🎉 Demo complete!');
}

main().catch(console.error);
