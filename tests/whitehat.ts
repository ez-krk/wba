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
import { Whitehat } from "../target/types/whitehat";
const crypto = require("crypto");

const commitment: Commitment = "finalized";

describe("whitehat", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Whitehat as Program<Whitehat>;
  const connection: Connection = anchor.getProvider().connection;

  const owner = new Keypair();
  const signer = new Keypair();
  const payout = new Keypair();

  //  owner
  //  auth
  //  sol_vault
  //  state
  //  protocol
  //  system_program

  const protocol = PublicKey.findProgramAddressSync(
    [Buffer.from("protocol"), owner.publicKey.toBuffer()],
    program.programId
  )[0];

  const auth = PublicKey.findProgramAddressSync(
    [Buffer.from("auth"), protocol.toBytes()],
    program.programId
  )[0];

  const solVault = PublicKey.findProgramAddressSync(
    [Buffer.from("sol_vault"), protocol.toBytes()],
    program.programId
  )[0];

  const percent = new BN(10);
  const seed = new BN(1);

  const message = new Uint8Array([
    4, 123, 82, 201, 77, 181, 1, 94, 165, 47, 19, 121, 237, 205, 155, 44, 151,
    64, 216, 179, 28, 13, 172, 233, 31, 75, 1, 231, 170, 79, 20, 55, 200, 247,
    173, 52, 177, 192, 216, 13, 85, 110, 56, 42, 133, 115, 167, 0, 102, 157, 27,
    153, 226, 131, 210, 205, 167, 54, 172, 18, 124, 137, 4, 221, 93, 223, 197,
    252, 200, 248, 105, 188, 151, 153, 237, 215, 242, 73, 105, 189, 8, 77, 98,
    85, 113, 42, 195, 14, 221, 104, 236, 160, 141, 113, 230, 25, 41, 235, 29,
    199, 13, 246, 142, 217, 177, 174, 9, 130, 131,
  ]);

  const hacker = PublicKey.findProgramAddressSync(
    [Buffer.from("hacker"), Buffer.from("€$¥")],
    program.programId
  )[0];

  const hack = PublicKey.findProgramAddressSync(
    // "doxx", protocol.key().as_ref(), signer.key().as_ref(), seed.to_le_bytes().as_ref()
    [
      Buffer.from("hack"),
      protocol.toBytes(),
      signer.publicKey.toBuffer(),
      seed.toArrayLike(Buffer, "le", 8),
      // doxxVulnerabilitySeed.toBuffer("le", 8)
    ],
    program.programId
  )[0];

  it("Airdrop", async () => {
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        owner.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        signer.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
  });

  it("register protocol", async () => {
    await program.methods
      .registerProtocol("whitehat", percent)
      .accounts({
        owner: owner.publicKey,
        auth,
        solVault,
        protocol,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc()
      .then(confirmTx);
  });

  it("register hacker", async () => {
    await program.methods
      .registerHacker("€$¥")
      .accounts({
        signer: signer.publicKey,
        hacker,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc()
      .then(confirmTx);
  });

  it("report doxx vulnerability", async () => {
    const protocolPda = await program.account.protocol.fetch(protocol);

    console.log(
      "protocol vulnerabilities : ",
      protocolPda.vulnerabilities.toNumber()
    );

    const doxxVulnerability = PublicKey.findProgramAddressSync(
      // "doxx", protocol.key().as_ref(), protocol.vulnerabilities.to_le_bytes().as_ref()
      [
        Buffer.from("doxx"),
        protocol.toBytes(),
        protocolPda.vulnerabilities.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];

    await program.methods
      .reportDoxxVulnerability(Buffer.from(message), seed)
      .accounts({
        signer: signer.publicKey,
        payout: payout.publicKey,
        protocol,
        hacker,
        vulnerability: doxxVulnerability,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc()
      .then(confirmTx);
  });

  it("report anon vulnerability", async () => {
    const protocolPda = await program.account.protocol.fetch(protocol);

    const anonVulnerability = PublicKey.findProgramAddressSync(
      // "anon", protocol.key().as_ref(), protocol.vulnerabilities.to_le_bytes().as_ref(), seed.to_le_bytes().as_ref()
      [
        Buffer.from("anon"),
        protocol.toBytes(),
        protocolPda.vulnerabilities.toArrayLike(Buffer, "le", 8),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];

    await program.methods
      .reportAnonVulnerability(Buffer.from(message), seed)
      .accounts({
        signer: signer.publicKey,
        payout: payout.publicKey,
        protocol,
        vulnerability: anonVulnerability,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc()
      .then(confirmTx);
  });

  it("deposit hacked funds (doxx)", async () => {
    const amount = new BN(10 * LAMPORTS_PER_SOL);

    try {
      await program.methods
        .newDoxxSolHack(amount, seed)
        .accounts({
          signer: signer.publicKey,
          payout: payout.publicKey,
          protocol,
          hacker,
          hack,
          solVault,
          // vulnerability: doxxVulnerability,
          systemProgram: SystemProgram.programId,
        })
        .signers([signer])
        .rpc()
        .then(confirmTx)
        .then(() => {
          console.log(
            "protocol vault new balance : ",
            connection.getBalance(solVault)
          );
        });
    } catch (error) {
      console.log(error);
    }
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
