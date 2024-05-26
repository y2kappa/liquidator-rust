# Kamino lending liquidations bot rust

## How to run

* `.env` file:
```
CLUSTER=https://api.mainnet-beta.solana.com
KEYPAIR=./liquidator.json
RUST_BACKTRACE=1
PROGRAM_ID=KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD
RUST_LOG=info
LIQUIDATOR_LOOKUP_TABLE_FILE=.lookuptable.rust.mainnet-beta
BASE_CURRENCY=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
MIN_SOL_BALANCE=1.0
USDC_MINT=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
REBALANCE_SLIPPAGE_PCT=0.3
NON_SWAPPABLE_DUST_USD_VALUE=0.1
BASE_TOKEN=USDC
``` 

* `ENV=.env.mainnet-beta cargo run -- crank`

