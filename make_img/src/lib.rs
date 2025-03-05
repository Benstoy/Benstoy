mod compile_source;
pub mod disk;
pub mod format;
mod partition;

pub const BRANDING: &str = "Benstoy";

pub use compile_source::compile;
pub use partition::partition_file;
