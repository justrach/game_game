import { Connection, Keypair, PublicKey, clusterApiUrl } from '@solana/web3.js';

export const PROGRAM_ID = new PublicKey('52TdioxTshbR5JuD52afPJH1HwVgNA9CYKwBjSxVW2io');

export function getConnection(cluster: 'devnet' | 'testnet' | 'mainnet-beta' = 'devnet'): Connection {
  return new Connection(clusterApiUrl(cluster), 'confirmed');
}

export function getKeypair(): Keypair {
  // In production, load from file or environment
  return Keypair.generate();
}

export async function airdrop(connection: Connection, publicKey: PublicKey, amount: number = 1): Promise<string> {
  console.log(`Requesting airdrop of ${amount} SOL...`);
  const signature = await connection.requestAirdrop(publicKey, amount * 1e9);
  await connection.confirmTransaction(signature);
  console.log(`Airdrop successful: ${signature}`);
  return signature;
}
