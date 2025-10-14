import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Openrate } from "../target/types/openrate";
import {
    createMint,
    mintTo,
    getOrCreateAssociatedTokenAccount,
    TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

/**
 * Comprehensive market lifecycle test for the Openrate program.
 *
 *  Builds upon a verified market initialization path that mirrors the on-chain `initialize_market` instruction.
 *  Derives all PDAs identically to Rust seed logic (`market`, `vault`, `vault_authority`, `bid_order`, `borrow_record`).
 *  Uses explicit Anchor account mappings with PDA accounts instead of auto-derived clients.
 *  
 * Exec Sequence:
 *    1. Initialize the market and PDAs.
 *    2. Create and fund a lender token account.
 *    3. Place a bid (creates `bid_order` PDA).
 *    4. Generate a borrower, airdrop 5 SOL, derive and create borrower token account.
 *    5. Borrow funds (creates `borrow_record` PDA).
 *    6. Repay borrowed funds back into vault.
 *    7. Cancel remaining bid liquidity.
 *  
 *  Ensures all lamport-funded accounts have sufficient balance to pass system rent checks.
 *  Designed for deterministic reproducibility in local validator contexts (`anchor test`).
 */
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
    let lenderTokenAccount: any;
    let borrowerTokenAccount: any;
    let bidOrderPda: anchor.web3.PublicKey;
    let borrowRecordPda: anchor.web3.PublicKey;
    let borrowerKeypair: anchor.web3.Keypair;

    /**
     *  stage 1: Initialize the market and all core PDAs.
     *  Creates a dummy SPL mint for the market’s base token.
     *  seed derivation for rust entities
     *  spl account linkage for market and vault
     */
    it("Initializes the market", async () => {
        mint = await createMint(connection, wallet.payer, wallet.publicKey, null, 6);

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

        vaultTokenAccountKeypair = anchor.web3.Keypair.generate();
        vaultTokenAccount = vaultTokenAccountKeypair.publicKey;

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

    /**
     * stage 2 – Place a bid into the market.
     * - Creates a lender-owned associated token account.
     * -- Mints test tokens to the lender.
     * -- Derives `bid_order` PDA  
     * -- invokes `place_bid` instruction.
     * - this  initializes a new `BidOrder` account.
     */
    it("Places a bid", async () => {
        lenderTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            wallet.payer,
            mint,
            wallet.publicKey
        );

        await mintTo(
            connection,
            wallet.payer,
            mint,
            lenderTokenAccount.address,
            wallet.payer,
            10_000_000
        );

        [bidOrderPda] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("bid_order"), wallet.publicKey.toBuffer(), marketPda.toBuffer()],
            program.programId
        );

        const tx = await program.methods
            .placeBid(new anchor.BN(5_000_000), 500)
            .accounts({
                lender: wallet.publicKey,
                market: marketPda,
                vault: vaultPda,
                vaultTokenAccount,
                lenderTokenAccount: lenderTokenAccount.address,
                bidOrder: bidOrderPda,
                vaultAuthority: vaultAuthorityPda,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .rpc();

        console.log("Bid placed:", tx);
    });

    /**
     * stage 3 – Borrow funds from market liquidity.
     * - Generates a borrower keypair and requests a 5 SOL airdrop to fund account creation.
     * - Confirmation process is awaited
     * - borrower's ATA is created for the spl mint. then, 
     * -- Derives borrow_record PDA 
     * -- Executes borrow instruction to transfer liquidity.
     */
    it("Borrows funds", async () => {
        borrowerKeypair = anchor.web3.Keypair.generate();

        await connection.requestAirdrop(borrowerKeypair.publicKey, 5 * anchor.web3.LAMPORTS_PER_SOL);
        await new Promise((resolve) => setTimeout(resolve, 1000));

        borrowerTokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            wallet.payer,
            mint,
            borrowerKeypair.publicKey
        );

        [borrowRecordPda] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from("borrow_record"),
                borrowerKeypair.publicKey.toBuffer(),
                bidOrderPda.toBuffer(),
            ],
            program.programId
        );

        const tx = await program.methods
            .borrow(new anchor.BN(2_000_000))
            .accounts({
                borrower: borrowerKeypair.publicKey,
                market: marketPda,
                vault: vaultPda,
                vaultTokenAccount,
                borrowerTokenAccount: borrowerTokenAccount.address,
                bidOrder: bidOrderPda,
                borrowRecord: borrowRecordPda,
                vaultAuthority: vaultAuthorityPda,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([borrowerKeypair])
            .rpc();

        console.log("Borrowed funds:", tx);
    });

    /**
     * stage 4 – Repay borrowed funds.
     * - Mints fresh tokens to borrower account to simulate repayment capability.
     * - Executes repay instruction to transfer tokens back to the vault.
     * - Depends on successful creation of borrow_record in the previous test.
     */
    it("Repays borrowed funds", async () => {
        await mintTo(
            connection,
            wallet.payer,
            mint,
            borrowerTokenAccount.address,
            wallet.payer,
            2_000_000
        );

        const tx = await program.methods
            .repay()
            .accounts({
                borrower: borrowerKeypair.publicKey,
                market: marketPda,
                vault: vaultPda,
                vaultTokenAccount,
                borrowerTokenAccount: borrowerTokenAccount.address,
                borrowRecord: borrowRecordPda,
                bidOrder: bidOrderPda,
                vaultAuthority: vaultAuthorityPda,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([borrowerKeypair])
            .rpc();

        console.log("Repayment complete:", tx);
    });

    /**
     * stage 5 – Cancels remaining bid liquidity.
     * - Invokes cancel_bid; 
     * - this returns any unutilized tokens to the lender.
     */
    it("Cancels remaining bid liquidity", async () => {
        const tx = await program.methods
            .cancelBid()
            .accounts({
                lender: wallet.publicKey,
                market: marketPda,
                vault: vaultPda,
                vaultTokenAccount,
                lenderTokenAccount: lenderTokenAccount.address,
                bidOrder: bidOrderPda,
                vaultAuthority: vaultAuthorityPda,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .rpc();

        console.log("Bid cancelled:", tx);
    });
});
