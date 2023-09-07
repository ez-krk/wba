import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import {
  Commitment,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { Feedback } from "../target/types/feedback";
const crypto = require("crypto");

const commitment: Commitment = "finalized";

describe("feedback", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Feedback as Program<Feedback>;
  const connection: Connection = anchor.getProvider().connection;

  const receiver = new Keypair();
  const sender = new Keypair();

  const seed = new BN(1);

  const session = PublicKey.findProgramAddressSync(
    [Buffer.from("session"), receiver.publicKey.toBuffer()],
    program.programId
  )[0];

  const user = PublicKey.findProgramAddressSync(
    [Buffer.from("user"), sender.publicKey.toBuffer()],
    program.programId
  )[0];

  const feedback = PublicKey.findProgramAddressSync(
    [
      Buffer.from("feedback"),
      // session.toBytes(),
      sender.publicKey.toBytes(),
      seed.toBuffer("le", 8),
    ],
    program.programId
  )[0];

  it("Airdrop", async () => {
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        receiver.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        sender.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
  });

  it("Create a new session", async () => {
    await program.methods
      .newSession(
        "krk.finance",
        "https://krk.finance/",
        "blockchain developer guild"
      )
      .accounts({
        owner: receiver.publicKey,
        session,
        systemProgram: SystemProgram.programId,
      })
      .signers([receiver])
      .rpc()
      .then(confirmTx);
  });

  it("Create a new user", async () => {
    await program.methods
      .newUser()
      .accounts({
        owner: sender.publicKey,
        user,
        systemProgram: SystemProgram.programId,
      })
      .signers([sender])
      .rpc()
      .then(confirmTx);
  });

  it("Create a new feedback", async () => {
    await program.methods
      .newFeedback(
        "wtf is krk ?",
        "the name feels hard to market and related to drugs!",
        seed
      )
      .accounts({
        owner: sender.publicKey,
        session,
        user,
        feedback,
        systemProgram: SystemProgram.programId,
      })
      .signers([sender])
      .rpc()
      .then(confirmTx);
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
