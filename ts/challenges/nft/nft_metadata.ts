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
      name: "KRK WL",
      description: "KRK WL Token",
      symbol: "KRKWL",
      image:
        "https://wjxylzgf4b3whncox676wnggwppvcz5gcerbexb4j3c24lt234sa.arweave.net/sm-F5MXgd2O0Tr-_6zTGs99RZ6YRIhJcPE7Fri563yQ",
      attributes: [
        { class: "whitelisted", value: "whitelisted" },
      ],
      properties: {
        files: [
          {
            type: "image/png",
            uri: "https://wjxylzgf4b3whncox676wnggwppvcz5gcerbexb4j3c24lt234sa.arweave.net/sm-F5MXgd2O0Tr-_6zTGs99RZ6YRIhJcPE7Fri563yQ",
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
