import { Connection, Keypair, PublicKey, clusterApiUrl } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { TOKEN_ADDRESS } from "../../../constants";

import wallet from "../../../keys/wba-wallet.json";

const signer = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection(clusterApiUrl("devnet"));

const mint = async () => {
  const tokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    signer,
    new PublicKey("9Nw4cjn6fNcidKgqQoTH3igFuCVsLr2vrRSBrBSbYF23"),
    signer.publicKey,
    false
  );
  console.log(`token account : ${tokenAccount.address}`);

  const transactionSignature = await mintTo(
    connection,
    signer,
    new PublicKey("9Nw4cjn6fNcidKgqQoTH3igFuCVsLr2vrRSBrBSbYF23"),
    tokenAccount.address,
    signer.publicKey,
    404 * 1e6
  );

  console.log(
    `Tokens Minted : https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`
  );
};

mint();
