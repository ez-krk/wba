import { Keypair, PublicKey } from "@solana/web3.js";

import sender from "./sender.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(sender));


