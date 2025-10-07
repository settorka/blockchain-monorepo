// backend/api/src/services/messages.service.ts
import crypto from "crypto";
import { loadChains } from "./chains.service.js";
import { ethers } from "ethers";
import { pool } from "../db/index.js";

export const sendMessage = async (
    fromChain: string,
    toChain: string,
    payload: string
) => {
    const chains = loadChains();
    const source = chains.find(c => c.name.toLowerCase() === fromChain.toLowerCase());
    const dest = chains.find(c => c.name.toLowerCase() === toChain.toLowerCase());

    if (!source || !dest) throw new Error("Invalid chain(s)");

    const provider = new ethers.JsonRpcProvider(source.rpcUrl);

    // simulate a tx hash for now
    console.log(`Sending message from ${source.name} → ${dest.name}: ${payload}`);
    const txHash = "0x" + crypto.randomUUID().replace(/-/g, "").slice(0, 64);

    // check DB connectivity before trying to insert
    try {
        await pool.query("SELECT 1");
    } catch (err) {
        console.error("❌ Database not connected:", err);
        throw new Error("Database not connected");
    }

    // insert into DB
    await pool.query(
        `INSERT INTO messages (from_chain, to_chain, payload, tx_hash)
     VALUES ($1, $2, $3, $4)`,
        [source.name, dest.name, payload, txHash]
    );

    console.log(`📨 message stored in DB with txHash: ${txHash}`);
    return { txHash, from: source.name, to: dest.name, payload };
};
