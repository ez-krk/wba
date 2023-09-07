import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import {
  Commitment,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { Form } from "../target/types/form";
const crypto = require("crypto");

const commitment: Commitment = "finalized";

describe("form", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Form as Program<Form>;
  const connection: Connection = anchor.getProvider().connection;

  const initializer = new Keypair();
  const user = new Keypair();

  const seed = new BN(1);

  const questions = PublicKey.findProgramAddressSync(
    [
      Buffer.from("questions"),
      // session.toBytes(),
      initializer.publicKey.toBytes(),
      seed.toBuffer("le", 8),
    ],
    program.programId
  )[0];

  const answers = PublicKey.findProgramAddressSync(
    [Buffer.from("answers"), questions.toBuffer(), user.publicKey.toBuffer()],
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

  it("Create questions", async () => {
    await program.methods
      .newQuestions(["sign-in /w twitter", "sign-in /w discord"], seed)
      .accounts({
        owner: initializer.publicKey,
        questions,
        systemProgram: SystemProgram.programId,
      })
      .signers([initializer])
      .rpc()
      .then(confirmTx);
  });

  it("Create answers", async () => {
    await program.methods
      .newAnswers(["beep boop", "test"])
      .accounts({
        owner: user.publicKey,
        questions,
        answers,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
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
