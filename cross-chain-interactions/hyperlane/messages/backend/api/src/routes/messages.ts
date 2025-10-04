import { Router } from "express";
import { sendMessage } from "../services/messages.service.js";

const router = Router();

router.post("/", async (req, res) => {
    try {
        const { fromChain, toChain, payload } = req.body;
        if (!fromChain || !toChain || !payload) {
            return res.status(400).json({ error: "Missing fields" });
        }

        const result = await sendMessage(fromChain, toChain, payload);
        res.json({ status: "sent", tx: result });
    } catch (err) {
        console.error("Message error:", err);
        res.status(500).json({ error: "Message dispatch failed" });
    }
});

export default router;
