#![allow(clippy::collapsible_else_if)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_late_init)]

#[macro_use]
extern crate bitflags;

pub mod big_file;
pub mod broken_files;
pub mod duplicate;
pub mod empty_files;
pub mod empty_folder;
pub mod invalid_symlinks;
pub mod same_music;
pub mod similar_images;
pub mod similar_videos;
pub mod temporary;

pub mod common;
pub mod common_dir_traversal;
pub mod common_directory;
pub mod common_extensions;
pub mod common_items;
pub mod common_messages;
pub mod common_traits;
pub mod localizer_core;

pub const CZKAWKA_VERSION: &str = env!("CARGO_PKG_VERSION");
