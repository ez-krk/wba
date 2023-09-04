import {
  Connection,
  Keypair,
  clusterApiUrl,
} from "@solana/web3.js";
import {
  createMint,
} from "@solana/spl-token";

import wallet from "../../keys/wba-wallet.json";

const signer = Keypair.fromSecretKey(new Uint8Array(wallet));


const connection = new Connection(clusterApiUrl("devnet"));

const initMint = async () => {

  const tokenMint = await createMint(
    connection,
    signer,
    signer.publicKey,
    null,
    0
  );
  console.log(tokenMint.toBase58());
};

initMint();
