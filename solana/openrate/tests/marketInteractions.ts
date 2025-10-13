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
 * Full market lifecycle test.
 * 
 * Builds on verified InitializeMarket behavior.
 * Subsequent instructions use consistent PDA derivations and
 * rely on Anchor's init constraints for new accounts.
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
     * Initializes the market.
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
     * Places a bid from the lender.
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
            [
                Buffer.from("bid_order"),
                wallet.publicKey.toBuffer(),
                marketPda.toBuffer(),
            ],
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
     * Borrows funds using the market liquidity.
     */
    it("Borrows funds", async () => {
        borrowerKeypair = anchor.web3.Keypair.generate();

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
     * Repays borrowed funds.
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
     * Cancels remaining bid liquidity.
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
