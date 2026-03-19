

from web3 import Web3


RPC_URL = "https://mainnet.infura.io/v3/e5a4c61d99244d6db298aed15d1fe52f"
WETH = Web3.to_checksum_address("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")
USDC = Web3.to_checksum_address("0xA0b86991c6218b36c1d19d4a2e9eb0ce3606eb48")

UNISWAP_V2_ROUTER = Web3.to_checksum_address("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D")
SUSHISWAP_ROUTER  = Web3.to_checksum_address("0xd9e1CE17F2641f24AE83637ab66a2CCa9C378B9F")
AMOUNT_IN_WEI = 1 * 10**18    # 1 WETH
THRESHOLD     = 5 * 10**6     # 5 USDC worth of spread