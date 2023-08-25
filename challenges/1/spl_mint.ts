import { Connection, Keypair, PublicKey, clusterApiUrl } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";

import wallet from "../keys/wba-wallet.json";
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection(clusterApiUrl("devnet"));

const mint = async () => {
  const tokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    keypair,
    new PublicKey("2KghEgNJY6VK8sP9LPGpFqDzmn77ofqmdvvBBsn2bctL"),
    keypair.publicKey,
    false,
    "confirmed"
  );
  console.log(`token account : ${tokenAccount.address}`);

  const transactionSignature = await mintTo(
    connection,
    keypair,
    new PublicKey("2KghEgNJY6VK8sP9LPGpFqDzmn77ofqmdvvBBsn2bctL"),
    tokenAccount.address,
    keypair.publicKey,
    42069 * 1e6
  );
  console.log(
    `Tokens Minted : https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`
  );
};

mint();
