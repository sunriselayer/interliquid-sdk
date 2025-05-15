pub mod core;
pub mod merkle;
pub mod state;
pub mod trie;
pub mod types;
pub mod utils;
pub mod x;
pub mod zkp;

#[cfg(not(feature = "sp1"))]
pub mod runner;

#[cfg(not(feature = "sp1"))]
use sha2;
#[cfg(feature = "sp1")]
use sha2_sp1 as sha2;

#[cfg(not(feature = "sp1"))]
use sha3;
#[cfg(feature = "sp1")]
use sha3_sp1 as sha3;

#[cfg(not(feature = "sp1"))]
use crypto_bigint;
#[cfg(feature = "sp1")]
use crypto_bigint_sp1 as crypto_bigint;

#[cfg(not(feature = "sp1"))]
use p256;
#[cfg(feature = "sp1")]
use p256_sp1 as p256;
