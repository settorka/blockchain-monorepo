import express from "express";
import dotenv from "dotenv";
import chainsRouter from "./routes/chains.js";
import messagesRouter from "./routes/messages.js";

dotenv.config();

const app = express();
const port = process.env.PORT || 3000;

app.use(express.json());

// routes
app.use("/chains", chainsRouter);
app.use("/messages", messagesRouter);

app.get("/", (_, res) => {
    res.send("Cross-chain API up and running 🚀");
});

app.listen(port, () => {
    console.log(` API running on http://localhost:${port}`);
});
