import { register } from "node:module";
import { pathToFileURL } from "node:url";

// Registers ts-node ESM loader for TypeScript support
register("ts-node/esm", pathToFileURL("./"));
