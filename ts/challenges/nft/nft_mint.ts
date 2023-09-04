import {
  bundlrStorage,
  keypairIdentity,
  Metaplex,
} from "@metaplex-foundation/js";
import { Commitment, Connection, Keypair } from "@solana/web3.js";
import wallet from "../../keys/wba-wallet.json";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const metaplex = Metaplex.make(connection)
  .use(keypairIdentity(keypair))
  .use(
    bundlrStorage({
      address: "https://devnet.bundlr.network",
      providerUrl: "https://api.devnet.solana.com",
      timeout: 60000,
    })
  );

(async () => {
  try {
    const { nft } = await metaplex.nfts().create({
      uri: "https://arweave.net/K9tvXf2bTYC5csTryAl6aQQzE5G92m8ogKZnuOs392Y",
      sellerFeeBasisPoints: 0,
      name: "KRK WL",
      symbol: "KRK WL",
    });
    console.log(`You've minted your nft:\n\n${nft}`);
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
