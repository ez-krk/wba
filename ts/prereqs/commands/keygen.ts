import { Keypair } from "@solana/web3.js";
const fs = require("fs");

let kp = Keypair.generate();
console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`);
console.log(kp);
// let wallet = JSON.stringify(kp);
fs.writeFileSync(
  `keys/${kp.publicKey.toBase58()}.json`,
  `[${kp.secretKey.toString()}]`
);
