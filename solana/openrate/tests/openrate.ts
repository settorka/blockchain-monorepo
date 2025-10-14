import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Openrate } from "../target/types/openrate";
import {
  createMint,
  createAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("openrate", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Openrate as Program<Openrate>;
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  let mint: anchor.web3.PublicKey;
  let marketPda: anchor.web3.PublicKey;
  let vaultPda: anchor.web3.PublicKey;
  let vaultAuthorityPda: anchor.web3.PublicKey;
  let vaultTokenAccount: anchor.web3.PublicKey;
  let vaultTokenAccountKeypair: anchor.web3.Keypair;

  it("Initializes the market", async () => {
    // Create dummy mint
    mint = await createMint(connection, wallet.payer, wallet.publicKey, null, 6);

    // PDAs per Rust seeds
    [marketPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("market"), mint.toBuffer()],
      program.programId
    );

    [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), mint.toBuffer()],
      program.programId
    );

    [vaultAuthorityPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault_authority"), marketPda.toBuffer()],
      program.programId
    );

    // Vault token account (owner = vaultAuthorityPda)
    // new acc not created as Anchor will initialize this
    vaultTokenAccountKeypair = anchor.web3.Keypair.generate();
    vaultTokenAccount = vaultTokenAccountKeypair.publicKey;


    // Sends initialize instruction
    const tx = await program.methods
      .initializeMarket()
      .accounts({
        authority: wallet.publicKey,
        market: marketPda,
        vault: vaultPda,
        tokenMint: mint,
        vaultTokenAccount,
        vaultAuthority: vaultAuthorityPda,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([vaultTokenAccountKeypair])
      .rpc();

    console.log("Market initialized:", tx);
  });
});
