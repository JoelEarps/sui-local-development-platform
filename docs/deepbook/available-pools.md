# Available Pools

As of a request made on the 31st of Jan 2025, these were the available market pairs when querying the available liquidity pools:

DEEP_SUI
DEEP_USDC
SUI_USDC
BETH_USDC
WUSDC_USDC
NS_SUI
NS_USDC
WUSDT_USDC
TYPUS_SUI
AUSD_USDC
SUI_AUSD
DRF_SUI

These are then directly fed into the `UrlPathPattern` value that enforces Regex matching on the WiremMock endpoint. This could have been made more generic for and matched for market pairs either being 3 or 4 capital letters long however this would not match the behaviour of the DeepBook indexer