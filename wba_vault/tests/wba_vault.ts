import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Commitment, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { WbaVault } from "../target/types/wba_vault";

describe("wba_vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const owner = new Keypair();
  const program = anchor.workspace.WbaVault as Program<WbaVault>;

  const commitment: Commitment = "finalized";

  const programId = new PublicKey(
    "83hrzxnkP5nP3vshjDZpN89RzC8WH4NFQWofYYTWhe1H"
  );

  const state = PublicKey.findProgramAddressSync(
    [Buffer.from("state"), owner.publicKey.toBuffer()],
    program.programId
  )[0];
  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), state.toBytes()],
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
    "confirmed"
  );
};
