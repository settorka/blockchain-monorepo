import { Router } from "express";
import { loadChains } from "../services/chains.service.js";

const router = Router();

router.get("/", (_, res) => {
    const chains = loadChains();
    res.json(chains);
});

export default router;
