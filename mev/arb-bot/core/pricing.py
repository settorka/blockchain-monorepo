def calc_spread(amount_out_a: int, amount_out_b: int):
    """
    Spread = absolute difference in token_out between DEX A and DEX B.
    """
    if amount_out_a is None or amount_out_b is None:
        return None
    return amount_out_a - amount_out_b


def detect_arb(price_uni: int, price_sushi: int, threshold: int):
    """
    Returns a tuple describing the direction of arbitrage if profitable.
    threshold: minimum difference in output tokens to consider as profit.
    """
    if price_uni is None or price_sushi is None:
        return None

    diff = price_uni - price_sushi

    if diff > threshold:
        return ("BUY_SUSHI_SELL_UNI", diff)

    if diff < -threshold:
        return ("BUY_UNI_SELL_SUSHI", abs(diff))

    return None
