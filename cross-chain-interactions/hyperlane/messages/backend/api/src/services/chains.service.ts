import fs from "fs";
import path from "path";

export interface ChainConfig {
    name: string;
    chainId: number;
    rpcUrl: string;
}

export const loadChains = (): ChainConfig[] => {
    const filePath = path.resolve("src/config/chains.json");
    const data = fs.readFileSync(filePath, "utf-8");
    return JSON.parse(data);
};
