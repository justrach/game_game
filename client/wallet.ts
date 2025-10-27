import { Keypair } from '@solana/web3.js';
import { writeFileSync, readFileSync, existsSync } from 'fs';
import { join } from 'path';

const WALLET_PATH = join(process.cwd(), '.wallet', 'test-wallet.json');

export function saveWallet(keypair: Keypair): void {
  const secretKey = Array.from(keypair.secretKey);
  const walletDir = join(process.cwd(), '.wallet');
  
  // Create .wallet directory if it doesn't exist
  if (!existsSync(walletDir)) {
    const fs = require('fs');
    fs.mkdirSync(walletDir, { recursive: true });
  }
  
  writeFileSync(WALLET_PATH, JSON.stringify(secretKey));
  console.log('💾 Wallet saved to:', WALLET_PATH);
  console.log('📍 Address:', keypair.publicKey.toBase58());
}

export function loadWallet(): Keypair | null {
  if (!existsSync(WALLET_PATH)) {
    return null;
  }
  
  try {
    const secretKeyString = readFileSync(WALLET_PATH, 'utf-8');
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    const keypair = Keypair.fromSecretKey(secretKey);
    console.log('✅ Loaded wallet from:', WALLET_PATH);
    console.log('📍 Address:', keypair.publicKey.toBase58());
    return keypair;
  } catch (error) {
    console.error('❌ Failed to load wallet:', error);
    return null;
  }
}

export function getOrCreateWallet(): Keypair {
  let wallet = loadWallet();
  
  if (!wallet) {
    console.log('🆕 Creating new wallet...');
    wallet = Keypair.generate();
    saveWallet(wallet);
    console.log('\n⚠️  IMPORTANT: Send 0.2 SOL to this address:');
    console.log('   ', wallet.publicKey.toBase58());
    console.log('   Use: solana transfer', wallet.publicKey.toBase58(), '0.2 --url devnet\n');
  }
  
  return wallet;
}
