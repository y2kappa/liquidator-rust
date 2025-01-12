#[macro_export]
macro_rules! readable {
    ($res:expr) => {
        ::anchor_lang::prelude::AccountMeta::new_readonly($res, false)
    };
}

#[macro_export]
macro_rules! signer {
    ($res:expr) => {
        ::anchor_lang::prelude::AccountMeta::new_readonly($res, true)
    };
}

#[macro_export]
macro_rules! writable {
    ($res:expr) => {
        ::anchor_lang::prelude::AccountMeta::new($res, false)
    };
}

#[macro_export]
macro_rules! writable_signer {
    ($res:expr) => {
        AccountMeta::new($res, true)
    };
}
