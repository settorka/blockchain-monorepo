// backend/api/src/db/index.ts
import pkg from "pg";
import dotenv from "dotenv";
dotenv.config();

const { Pool } = pkg;

export const pool = new Pool({
    host: process.env.DB_HOST || "db",
    port: Number(process.env.DB_PORT) || 5432,
    user: process.env.DB_USER || "postgres",
    password: process.env.DB_PASSWORD || "postgres",
    database: process.env.DB_NAME || "crosschain",
});

// helpful debug
pool.on("connect", () => console.log("✅ Connected to Postgres"));
pool.on("error", (err) => console.error("❌ Postgres connection error", err));
