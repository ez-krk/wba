import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import {
  Commitment,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { WbaVault } from "../target/types/wba_vault";

const commitment: Commitment = "finalized";

describe("wba_vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const connection: Connection = anchor.getProvider().connection;
  const owner = new Keypair();
  const program = anchor.workspace.WbaVault as Program<WbaVault>;

  const decimals = 6;
  const amount = 100;

  const mint = Keypair.generate();

  let mintAddr: string;

  const state = PublicKey.findProgramAddressSync(
    [Buffer.from("state"), owner.publicKey.toBuffer()],
    program.programId
  )[0];
  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), state.toBytes()],
    program.programId
  )[0];
  const splVault = PublicKey.findProgramAddressSync(
    [Buffer.from("spl_vault"), state.toBytes()],
    program.programId
  )[0];

  const auth = PublicKey.findProgramAddressSync(
    [Buffer.from("auth"), state.toBytes()],
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
  });

  it("Create a new mint, creates an ATA and mints 100 tokens to our account", async () => {
    let token = await createMint(
      connection,
      owner,
      owner.publicKey,
      owner.publicKey,
      decimals,
      mint
    );
    console.log("Token : ", token.toBase58());
    mintAddr = token.toBase58();
    let fromAta = await getOrCreateAssociatedTokenAccount(
      connection,
      owner,
      token,
      owner.publicKey
    );
    console.log("ATA : ", fromAta.address.toBase58());
    let sendToken = await mintTo(
      connection,
      owner,
      token,
      fromAta.address,
      owner.publicKey,
      amount * 1 * 10 ** decimals
    );
    console.log(`https://explorer.solana.com/tx/${sendToken}?cluster=devnet`);
    let tokenAmount = await connection.getTokenAccountBalance(fromAta.address);
    console.log(
      `minted ${tokenAmount.value.uiAmountString} ${token.toBase58()} tokens`
    );
  });

  it("Initialize", async () => {
    await program.methods
      .initialize()
      .accounts({
        owner: owner.publicKey,
        auth,
        vault,
        state,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc()
      .then(confirmTx);
  });

  it("Deposits 1 SOL to the vault", async () => {
    await program.methods
      .deposit(new BN(1e9))
      .accounts({
        owner: owner.publicKey,
        vault,
        state,
      })
      .signers([owner])
      .rpc()
      .then(confirmTx);
  });

  it("Withdraw 1 SOL from the vault", async () => {
    await program.methods
      .withdraw(new BN(1e9))
      .accounts({
        owner: owner.publicKey,
        vault,
        state,
      })
      .signers([owner])
      .rpc()
      .then(confirmTx);
  });

  it("Deposit SPL", async () => {
    let ownerAta = await getOrCreateAssociatedTokenAccount(
      connection,
      owner,
      new PublicKey(mintAddr),
      owner.publicKey
    );
    await program.methods
      .depositSpl(new BN(amount * 1 * 10 ** decimals))
      .accounts({
        owner: owner.publicKey,
        auth,
        vault: splVault,
        state,
        ownerAta: ownerAta.address,
        mint: new PublicKey(mintAddr),
      })
      .signers([owner])
      .rpc()
      .then(confirmTx);
  });

  it("Withdraw SPL", async () => {
    let ownerAta = await getOrCreateAssociatedTokenAccount(
      connection,
      owner,
      new PublicKey(mintAddr),
      owner.publicKey
    );
    await program.methods
      .withdrawSpl(new BN(amount * 1 * 10 ** decimals))
      .accounts({
        owner: owner.publicKey,
        auth,
        vault: splVault,
        state,
        ownerAta: ownerAta.address,
        mint: new PublicKey(mintAddr),
      })
      .signers([owner])
      .rpc()
      .then(confirmTx);
  });

  it("Closes Vaults", async () => {});
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
