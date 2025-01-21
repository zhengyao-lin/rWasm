pub mod parser;
pub mod wasm;

pub static DEBUG_PRINT_LEVEL: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
type Maybe<T> = color_eyre::eyre::Result<T>;
