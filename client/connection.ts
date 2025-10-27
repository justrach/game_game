import { Connection, Keypair, PublicKey, clusterApiUrl } from '@solana/web3.js';

export const PROGRAM_ID = new PublicKey('52TdioxTshbR5JuD52afPJH1HwVgNA9CYKwBjSxVW2io');

export function getConnection(cluster: 'devnet' | 'testnet' | 'mainnet-beta' = 'devnet'): Connection {
  return new Connection(clusterApiUrl(cluster), 'confirmed');
}

export function getKeypair(): Keypair {
  // In production, load from file or environment
  return Keypair.generate();
}

export async function checkBalance(connection: Connection, publicKey: PublicKey): Promise<number> {
  const balance = await connection.getBalance(publicKey);
  return balance / 1e9; // Convert lamports to SOL
}
