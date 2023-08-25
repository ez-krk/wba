import { IDL } from "./programs/wba_vault";

import { Connection, Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider, Address } from "@coral-xyz/anchor";

import wallet from "../keys/wba-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");

const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});

const program = new Program(
  IDL,
  "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address,
  provider
);

const vaultState = Keypair.generate();
console.log("vault state :\n", vaultState);

const vault_auth_seeds = [Buffer.from("auth"), vaultState.publicKey.toBuffer()];

const vaultAuth = PublicKey.findProgramAddressSync(
  vault_auth_seeds,
  program.programId
)[0];

console.log("vault auth :\n", vaultAuth);

const vault_seeds = [Buffer.from("vault"), vaultAuth.toBuffer()];

const vault = PublicKey.findProgramAddressSync(
  vault_seeds,
  program.programId
)[0];

console.log("vault auth :\n", vaultAuth);

(async () => {
  try {
    const tx = await program.methods
      .initialize()
      .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth: vaultAuth,
        vault: vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair, vaultState])
      .rpc();
    console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${tx}?cluster=devnet`);
  } catch (error) {
    console.log(error);
  }
})();
