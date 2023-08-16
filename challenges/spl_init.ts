import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createAccount,
  createMint,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";

const AIRDROP_AMOUNT = 2 * LAMPORTS_PER_SOL; // 1 SOL

const connection = new Connection(clusterApiUrl("devnet"));

const initMint = async () => {
  const initializer = Keypair.generate();

  console.log(`Requesting airdrop for ${initializer.publicKey}`);
  const signature = await connection.requestAirdrop(
    initializer.publicKey,
    AIRDROP_AMOUNT
  );
  const { blockhash, lastValidBlockHeight } =
    await connection.getLatestBlockhash();
  await connection.confirmTransaction(
    {
      blockhash,
      lastValidBlockHeight,
      signature,
    },
    "finalized"
  );
  console.log(
    `Tx Complete: https://explorer.solana.com/tx/${signature}?cluster=devnet`
  );

  const tokenMint = await createMint(
    connection,
    initializer,
    new PublicKey("7sydHcmax59DZJ523tFQEakwkJ3vBDWUE64auHy7yn1N"),
    new PublicKey("7sydHcmax59DZJ523tFQEakwkJ3vBDWUE64auHy7yn1N"),
    6
  );
  console.log(
    `Token Mint: https://explorer.solana.com/address/${tokenMint}?cluster=devnet`
  );
};

initMint();
