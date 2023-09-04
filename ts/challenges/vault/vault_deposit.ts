import { IDL } from "./programs/wba_vault";

import { Connection, Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
  BN,
} from "@coral-xyz/anchor";

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

(async () => {
  try {
    const rentExemptionAmount =
      await connection.getMinimumBalanceForRentExemption(0);
    console.log(`rent is ${rentExemptionAmount} lamports`);
    const tx = await program.methods
      .deposit(new BN(1 + rentExemptionAmount))
      .accounts({
        owner: keypair.publicKey,
        vaultState: new PublicKey(
          "33DDYP98kBrpDH1fKXVqnpQgrcQ3sGa8fvpxBnTmxakT"
        ),
        vaultAuth: new PublicKey(
          "HYEfGWmcR1VHmFhsw7B6rsBCoCGUbzR7bdEAtwp5mac4"
        ),
        vault: new PublicKey("HGxzts5edQeCuxmoeas9rEQkbsaYJuPmufdTYqJKXw8R"),
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
