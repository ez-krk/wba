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
import { Mail } from "../target/types/mail";
const crypto = require("crypto");

const commitment: Commitment = "finalized";

describe("mail", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Mail as Program<Mail>;
  const connection: Connection = anchor.getProvider().connection;

  const sender = new Keypair();
  const receiver = new Keypair();

  const seed = new BN(1);

  const message = new Uint8Array([4, 123, 82, 201, 77, 181, 1, 94, 165, 47, 19, 121, 237, 205, 155, 44, 151, 64, 216, 179, 28, 13, 172, 233, 31, 75, 1, 231, 170, 79, 20, 55, 200, 247, 173, 52, 177, 192, 216, 13, 85, 110, 56, 42, 133, 115, 167, 0, 102, 157, 27, 153, 226, 131, 210, 205, 167, 54, 172, 18, 124, 137, 4, 221, 93, 223, 197, 252, 200, 248, 105, 188, 151, 153, 237, 215, 242, 73, 105, 189, 8, 77, 98, 85, 113, 42, 195, 14, 221, 104, 236, 160, 141, 113, 230, 25, 41, 235, 29, 199, 13, 246, 142, 217, 177, 174, 9, 130, 131]);

  const senderInbox = PublicKey.findProgramAddressSync(
    [Buffer.from("inbox"), sender.publicKey.toBuffer()],
    program.programId
  )[0];

  const receiverInbox = PublicKey.findProgramAddressSync(
    [Buffer.from("inbox"), receiver.publicKey.toBuffer()],
    program.programId
  )[0];

  const mail = PublicKey.findProgramAddressSync(
    // "mail", sender.key().as_ref(), inbox.key().as_ref(), seed.to_le_bytes().as_ref()
    [
      Buffer.from("mail"),
      sender.publicKey.toBytes(),
      receiverInbox.toBytes(),
      seed.toArrayLike(Buffer, "le", 16),
    ],
    program.programId
  )[0];

  it("Airdrop", async () => {
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        sender.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        receiver.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
  });

  it("Create sender inbox", async () => {
    await program.methods
      .newInbox()
      .accounts({
        owner: sender.publicKey,
        inbox: senderInbox,
        systemProgram: SystemProgram.programId,
      })
      .signers([sender])
      .rpc()
      .then(confirmTx);
  });

  it("Create receiver inbox", async () => {
    await program.methods
      .newInbox()
      .accounts({
        owner: receiver.publicKey,
        inbox: receiverInbox,
        systemProgram: SystemProgram.programId,
      })
      .signers([receiver])
      .rpc()
      .then(confirmTx);
  });

  it("Send a mail", async () => {
    await program.methods
      .newMail(Buffer.from(message), seed)
      .accounts({
        sender: sender.publicKey,
        inbox: receiverInbox,
        mail,
        systemProgram: SystemProgram.programId,
      })
      .signers([sender])
      .rpc()
      .then(confirmTx);
  });

  it("Cost ", async () => {
    const senderBalance = await anchor
      .getProvider()
      .connection.getBalance(sender.publicKey);
    console.log("sender new balance : ", senderBalance / LAMPORTS_PER_SOL);
    // console.log("user new balance : ", userBalance / LAMPORTS_PER_SOL);
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
