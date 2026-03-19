from web3 import Web3
from .abi_router_v2 import ABI_ROUTER_V2

class UniswapV2:
    def __init__(self, w3: Web3, router_address: str):
        self.w3 = w3
        self.router = w3.eth.contract(address=router_address, abi=ABI_ROUTER_V2)

    def get_price(self, token_in: str, token_out: str, amount_in_wei: int):
        """Returns the output amount for a swap token_in -> token_out."""
        try:
            amounts = self.router.functions.getAmountsOut(
                amount_in_wei,
                [token_in, token_out]
            ).call()
            return amounts[-1]
        except Exception:
            return None
