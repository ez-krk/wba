import { IDL } from "./program/whitehat";

import { Connection, Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
  BN,
} from "@coral-xyz/anchor";

import wallet from "../../keys/wba-wallet.json";
import {
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");

const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});

const program = new Program(
  IDL,
  "WHATz4jFpiMbaz578KCU8188Ni3AT5ktxqqFLn4CTkd" as Address,
  provider
);

const analytics = PublicKey.findProgramAddressSync(
  [Buffer.from("analytics")],
  program.programId
)[0];
const whauth = PublicKey.findProgramAddressSync(
  [Buffer.from("auth")],
  program.programId
)[0];
const whvault = PublicKey.findProgramAddressSync(
  [Buffer.from("vault")],
  program.programId
)[0];

(async () => {
  try {
    const tx = await program.methods
      .initialize()
      .accounts({
        admin: keypair.publicKey,
        auth: whauth,
        vault: whvault,
        analytics,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair])
      .rpc();
    console.log(`Success! Check out your TX here:
      https://explorer.solana.com/tx/${tx}?cluster=devnet`);
  } catch (error) {
    console.log(error);
  }
})();
