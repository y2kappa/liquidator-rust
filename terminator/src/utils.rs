use std::collections::HashMap;

use anchor_lang::prelude::Pubkey;
use kamino_lending::Reserve;

use crate::accounts::MarketAccounts;

pub fn get_all_reserve_mints(
    market_accounts: &HashMap<Pubkey, MarketAccounts>,
) -> (HashMap<Pubkey, Reserve>, Vec<Pubkey>, Vec<Pubkey>) {
    let mut all_reserves = HashMap::new();
    let mut c_mints = Vec::new();
    let mut l_mints = Vec::new();
    for (_, market_account) in market_accounts.iter() {
        for (_, reserve) in market_account.reserves.iter() {
            all_reserves.insert(reserve.liquidity.mint_pubkey, *reserve);
            c_mints.push(reserve.collateral.mint_pubkey);
            l_mints.push(reserve.liquidity.mint_pubkey);
        }
    }
    (all_reserves, c_mints, l_mints)
}

pub mod px {
    use std::{collections::HashMap, str::FromStr};

    use anchor_lang::prelude::Pubkey;
    use anyhow::Result;
    use juno::SwapPrice;
    use tracing::info;

    pub struct Prices {
        pub prices: HashMap<Pubkey, f64>,
    }

    impl Prices {
        pub fn a_to_b(&self, a: &Pubkey, b: &Pubkey) -> f64 {
            info!("Getting price of {} to {}", a, b);
            let a_price = self.prices.get(a).unwrap();
            let b_price = self.prices.get(b).unwrap();

            a_price / b_price
        }
    }

    pub async fn fetch_jup_prices(
        input_mints: &[Pubkey],
        output_mint: &Pubkey,
        amount: f32,
    ) -> Result<Prices> {
        let raw_prices = juno::get_prices(input_mints, output_mint, amount).await?;
        let mut prices: HashMap<Pubkey, f64> = HashMap::new();
        for (mint, SwapPrice { price, .. }) in raw_prices {
            prices.insert(Pubkey::from_str(&mint).unwrap(), price as f64);
        }
        Ok(Prices { prices })
    }
}

pub mod sysvars {
    use anchor_lang::{prelude::Clock, solana_program::sysvar::SysvarId};
    use anyhow::Result;

    use crate::client::KlendClient;

    /// Get current clock
    pub async fn get_clock(client: &KlendClient) -> Result<Clock> {
        let clock = crate::client::rpc::get_account(client, &Clock::id())
            .await?
            .deserialize_data()?;

        Ok(clock)
    }

    /// Get current clock
    pub async fn clock(rpc: &KlendClient) -> Clock {
        get_clock(rpc).await.unwrap()
    }
}

pub mod lookup_tables {
    use std::collections::{HashMap, HashSet};

    use anchor_lang::prelude::Pubkey;
    use kamino_lending::{LendingMarket, Reserve};

    use crate::liquidator::Liquidator;

    pub fn collect_keys(
        reserves: &HashMap<Pubkey, Reserve>,
        liquidator: &Liquidator,
        lending_market: &LendingMarket,
    ) -> HashSet<Pubkey> {
        let mut lending_markets = HashSet::new();
        let mut keys = HashSet::new();
        for (pubkey, reserve) in reserves {
            keys.insert(*pubkey);
            keys.insert(reserve.collateral.supply_vault);
            keys.insert(reserve.collateral.mint_pubkey);
            keys.insert(reserve.liquidity.supply_vault);
            keys.insert(reserve.liquidity.fee_vault);
            keys.insert(reserve.liquidity.mint_pubkey);
            lending_markets.insert(reserve.lending_market);
        }

        for (mint, ata) in liquidator.atas.iter() {
            keys.insert(*mint);
            keys.insert(*ata);
        }

        keys.insert(lending_market.lending_market_owner);
        keys.insert(lending_market.risk_council);

        for lending_market in lending_markets.iter() {
            let lending_market_authority =
                kamino_lending::utils::seeds::pda::lending_market_auth(lending_market);
            keys.insert(*lending_market);
            keys.insert(lending_market_authority);
        }

        keys
    }
}

pub mod model {
    use std::{
        cell::{Ref, RefCell, RefMut},
        rc::Rc,
    };

    use anchor_lang::prelude::Pubkey;
    use kamino_lending::utils::AnyAccountLoader;

    #[derive(Default, Clone)]
    pub struct StateWithKey<T> {
        pub key: Pubkey,
        pub state: Rc<RefCell<T>>,
    }

    impl<T> StateWithKey<T> {
        pub fn new(state: T, key: Pubkey) -> Self {
            Self {
                key,
                state: Rc::new(RefCell::new(state)),
            }
        }
    }

    impl<T> AnyAccountLoader<'_, T> for StateWithKey<T> {
        fn get_mut(&self) -> anchor_lang::Result<RefMut<T>> {
            Ok(RefMut::map(self.state.borrow_mut(), |state| state))
        }
        fn get(&self) -> anchor_lang::Result<Ref<T>> {
            Ok(Ref::map(self.state.borrow(), |state| state))
        }

        fn get_pubkey(&self) -> Pubkey {
            self.key
        }
    }
}
