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
    const { uri: metadataUri } = await metaplex.nfts().uploadMetadata({
      name: "zrug",
      description: "rare & exotic rug",
      symbol: "ZRUG",
      image:
        "https://5x44kba47m2escn5mev6m5nv2aejx4ijo7tr4yd3hdabynkj3s2a.arweave.net/7fnFBBz7NEkJvWEr5nW10Aib8Ql35x5gezjAHDVJ3LQ",
      attributes: [
        {
          color: "",
          size: "",
        },
        { class: "premium", value: "empty" },
        { trait_type: "empty", value: "empty" },
        { trait_type: "empty", value: "empty" },
      ],
      properties: {
        files: [
          {
            type: "image/png",
            uri: "https://",
          },
        ],
        creators: [
          {
            address: keypair.publicKey.toBase58(),
            share: 100,
          },
        ],
      },
    });
    console.log(
      `You've uploaded your metadata:\n\n${metadataUri}\n\nSave this URI so you can use it to mint an NFT!`
    );
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
