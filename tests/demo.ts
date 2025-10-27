#!/usr/bin/env bun
import { Keypair, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { getConnection, PROGRAM_ID, airdrop } from '../client/connection';
import { createInvokeInstruction } from '../client/instructions';

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
  
  // Create test wallet
  const wallet = Keypair.generate();
  console.log('👛 Generated test wallet:', wallet.publicKey.toBase58());
  
  // Airdrop SOL
  console.log('💰 Requesting airdrop...');
  await airdrop(connection, wallet.publicKey, 1);
  
  const balance = await connection.getBalance(wallet.publicKey);
  console.log('   Balance:', balance / 1e9, 'SOL\n');
  
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
