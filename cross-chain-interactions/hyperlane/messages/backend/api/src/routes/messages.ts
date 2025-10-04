import { Router } from "express";
const router = Router();

router.get("/", (_, res) => {
    res.json({ message: "Messages endpoint placeholder" });
});

export default router;
