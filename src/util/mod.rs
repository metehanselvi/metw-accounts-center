/// Password utilities.
pub mod password;

/// Send templated mails.
pub mod templated_mails;

mod snowflake;

pub use snowflake::{EPOCH, snowflake};
