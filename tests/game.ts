import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  Commitment,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { Game } from "../target/types/game";

const commitment: Commitment = "finalized";

describe("game", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const connection: Connection = anchor.getProvider().connection;
  const owner = new Keypair();
  const program = anchor.workspace.Game as Program<Game>;

  const game = PublicKey.findProgramAddressSync(
    [Buffer.from("game")],
    program.programId
  )[0];

  const mint = PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
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

  it("Initialize", async () => {
    await program.methods
      .initialize("", "KRK.FINANCE", "KRK")
      .accounts({
        owner: owner.publicKey,
        game,
        mint,
        metadataAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMetadataProgram: new PublicKey(
          "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        ),
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
