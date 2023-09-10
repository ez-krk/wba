import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { Pastebin } from "../target/types/pastebin";
const crypto = require("crypto");

const commitment: Commitment = "finalized";

describe("pastebin", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Pastebin as Program<Pastebin>;
  const connection: Connection = anchor.getProvider().connection;

  const initializer = new Keypair();

  const seed = new BN(1);

  const entry = PublicKey.findProgramAddressSync(
    [
      Buffer.from("entry"),
      // session.toBytes(),
      initializer.publicKey.toBytes(),
      seed.toBuffer("le", 8),
    ],
    program.programId
  )[0];

  it("Airdrop", async () => {
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        initializer.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
  });

  it("Create entry", async () => {
    await program.methods
      .newEntry("aaa", seed)
      .accounts({
        owner: initializer.publicKey,
        entry,
        systemProgram: SystemProgram.programId,
      })
      .signers([initializer])
      .rpc()
      .then(confirmTx);
  });

  it("Cost", async () => {
    const receiverBalance = await anchor
      .getProvider()
      .connection.getBalance(initializer.publicKey);
    console.log(
      "initializer new balance : ",
      receiverBalance / LAMPORTS_PER_SOL
    );

    console.log("total tx cost : ", 100 - receiverBalance / LAMPORTS_PER_SOL);
  });
});

const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    commitment
  );
};
