pub mod core;

#[cfg(feature = "aladhan-provider")]
pub mod providers;

#[cfg(feature = "sqlite-storage")]
pub mod storage;

#[cfg(test)]
mod test;

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "sound")]
pub mod sound;
