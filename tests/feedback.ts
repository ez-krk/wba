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
import { Feedback } from "../target/types/feedback";
const crypto = require("crypto");

const commitment: Commitment = "finalized";

describe("feedback", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Feedback as Program<Feedback>;
  const connection: Connection = anchor.getProvider().connection;

  const initializer = new Keypair();
  const user = new Keypair();

  const seed = new BN(1);

  const session = PublicKey.findProgramAddressSync(
    [Buffer.from("session"), initializer.publicKey.toBuffer()],
    program.programId
  )[0];

  const userPDA = PublicKey.findProgramAddressSync(
    [Buffer.from("user"), user.publicKey.toBuffer()],
    program.programId
  )[0];

  const feedback = PublicKey.findProgramAddressSync(
    [
      Buffer.from("feedback"),
      // session.toBytes(),
      user.publicKey.toBytes(),
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
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        user.publicKey,
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
        owner: initializer.publicKey,
        session,
        systemProgram: SystemProgram.programId,
      })
      .signers([initializer])
      .rpc()
      .then(confirmTx);
  });

  it("Create a new user", async () => {
    await program.methods
      .newUser()
      .accounts({
        owner: user.publicKey,
        user: userPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
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
        owner: user.publicKey,
        session,
        user: userPDA,
        feedback,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc()
      .then(confirmTx);
  });

  it("Cost", async () => {
    const receiverBalance = await anchor
      .getProvider()
      .connection.getBalance(initializer.publicKey);
    const userBalance = await anchor
      .getProvider()
      .connection.getBalance(user.publicKey);
    console.log(
      "initializer new balance : ",
      receiverBalance / LAMPORTS_PER_SOL
    );
    console.log("user new balance : ", userBalance / LAMPORTS_PER_SOL);
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
