import { loadChains } from "./chains.service.js";
import { ethers } from "ethers";

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

    // here we’ll call the Hyperlane Mailbox contract
    // for now just simulate a tx hash
    console.log(`Sending message from ${source.name} → ${dest.name}: ${payload}`);

    const txHash = "0x" + crypto.randomUUID().replace(/-/g, "").slice(0, 64);
    return { txHash, from: source.name, to: dest.name, payload };
};
