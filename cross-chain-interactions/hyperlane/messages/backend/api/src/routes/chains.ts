import { Router } from "express";
const router = Router();

router.get("/", (_, res) => {
    res.json({ message: "Chains endpoint placeholder" });
});

export default router;
