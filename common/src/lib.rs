//! Contains shared code for the id3tools family of programs and is not intended to be used directly.

mod file_types;
mod log;
mod main_cli;
mod shared;

// Define the file types supported by the id3tools family of programs.
pub use crate::file_types::FileTypes;

// Builds the main CLI for the `id3tag` application and also the `id3cli-gen` application.
pub use crate::main_cli::build_cli;

// Builds the log config
pub use crate::log::build_log;

// Misc utility functions
pub use crate::shared::count_files;
pub use crate::shared::file_rename_pattern_validate;
pub use crate::shared::get_extension;
pub use crate::shared::get_file_type;
pub use crate::shared::get_mime_type;
pub use crate::shared::get_unique_value;
pub use crate::shared::need_split;
pub use crate::shared::roman_to_decimal;
pub use crate::shared::split_val;
pub use crate::shared::thousand_separated;
