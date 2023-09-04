import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
const fs = require("fs");

const files: string[] = fs.readdirSync("keys");

console.log(files);

for (const f of files) {
  console.log(f);
  fs.readFile(`keys/${f}`, (err: any, data: string) => {
    if (err) throw err;
    let kp = JSON.parse(data);
    console.log(kp._keypair.publicKey);
    const keypair = Keypair.fromSecretKey(
      new Uint8Array(Object.values(kp._keypair.secretKey))
    );
    const connection = new Connection("https://api.devnet.solana.com");
    (async () => {
      try {
        // We're going to claim 2 devnet SOL tokens
        const txhash = await connection.requestAirdrop(
          keypair.publicKey,
          2 * LAMPORTS_PER_SOL
        );
        console.log(`Success! Check out your TX here:
        https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
      } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
      }
    })();
  });
}
