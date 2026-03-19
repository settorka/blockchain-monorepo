import asyncio
from web3 import Web3

from config import (
    RPC_URL,
    WETH,
    USDC,
    UNISWAP_V2_ROUTER,
    SUSHISWAP_ROUTER,
)

from core.dex_uniswap import UniswapV2
from core.dex_sushi import SushiSwap
from core.pricing import detect_arb


AMOUNT_IN_WEI = 1 * 10**18         # 1 WETH
THRESHOLD = 5 * 10**6              # 5 USDC (6 decimals)


async def price_loop():
    w3 = Web3(Web3.HTTPProvider(RPC_URL))

    uni = UniswapV2(w3, UNISWAP_V2_ROUTER)
    sushi = SushiSwap(w3, SUSHISWAP_ROUTER)

    print("Arb bot started. Scanning Uniswap vs Sushi...\n")

    while True:
        price_uni  = uni.get_price(WETH, USDC, AMOUNT_IN_WEI)
        price_sushi = sushi.get_price(WETH, USDC, AMOUNT_IN_WEI)

        arb = detect_arb(price_uni, price_sushi, THRESHOLD)

        if arb:
            direction, profit = arb
            print(
                f"[ARB] {direction} | Profit ≈ {profit/1e6:.4f} USDC | "
                f"Uni: {price_uni/1e6:.4f} | Sushi: {price_sushi/1e6:.4f}"
            )

        await asyncio.sleep(1)


async def main():
    await price_loop()


if __name__ == "__main__":
    asyncio.run(main())
