#!/usr/bin/env bun
import { getOrCreateWallet } from '../client/wallet';
import { getConnection, checkBalance } from '../client/connection';

async function main() {
  console.log('🔧 Wallet Setup\n');
  
  const wallet = getOrCreateWallet();
  const connection = getConnection('devnet');
  
  const balance = await checkBalance(connection, wallet.publicKey);
  
  console.log('\n📊 Wallet Status:');
  console.log('   Address:', wallet.publicKey.toBase58());
  console.log('   Balance:', balance, 'SOL');
  
  if (balance < 0.01) {
    console.log('\n⚠️  Low balance! Fund this wallet:');
    console.log('   solana transfer', wallet.publicKey.toBase58(), '0.2 --url devnet');
  } else {
    console.log('\n✅ Wallet is ready to use!');
  }
}

main().catch(console.error);
