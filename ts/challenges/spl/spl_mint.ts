import { Connection, Keypair, PublicKey, clusterApiUrl } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { TOKEN_ADDRESS } from "../../constants";

import wallet from "../../keys/wba-wallet.json";

const signer = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection(clusterApiUrl("devnet"));

const mint = async () => {
  const tokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    signer,
    new PublicKey("25ZFsHJgbHd2XvJcdndXzo7CgK7CPSecVuKoPJVMTssm"),
    signer.publicKey,
    false,

  );
  console.log(`token account : ${tokenAccount.address}`);

  const transactionSignature = await mintTo(
    connection,
    signer,
    new PublicKey("25ZFsHJgbHd2XvJcdndXzo7CgK7CPSecVuKoPJVMTssm"),
    tokenAccount.address,
    signer.publicKey,
    404
  );

  console.log(
    `Tokens Minted : https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`
  );
};

mint();
