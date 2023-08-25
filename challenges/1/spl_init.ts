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

import wallet from "../../keys/wba-wallet.json";
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

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
    keypair,
    keypair.publicKey,
    keypair.publicKey,
    6
  );
  console.log(
    `Token Mint: https://explorer.solana.com/address/${tokenMint}?cluster=devnet`
  );
};

initMint();
