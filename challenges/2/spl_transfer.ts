import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import wallet from "../keys/wba-wallet.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("2KghEgNJY6VK8sP9LPGpFqDzmn77ofqmdvvBBsn2bctL");

// Recipient address
const to = new PublicKey("5kRot8UnMEqoDkAc72e7pqaEaF5hxGmbDNowMmPiCDmb");

(async () => {
  try {
    const fromATA = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );
    const toATA = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to
    );

    const signature = await transfer(
      connection,
      keypair,
      fromATA.address,
      toATA.address,
      keypair,
      42069 * 1e6
    );
    console.log(
      `Tx Complete: https://explorer.solana.com/tx/${signature}?cluster=devnet`
    );
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
