use super::buffer::Buffer;
use super::core::Core;
use super::file::*;

use super::core_e::{CoreError, CoreResult};
use super::output::*;

mod buffer_info;
mod buffer_set_byte;
mod buffer_write_bytes;
mod create_file;
mod fill_buffer;
mod from_buffer_to_file;
mod from_file_to_buffer;
mod make_buffer;
mod merge_buffers;
mod open_file;
mod pull_out_slice;
mod set_variable;
mod show_buffer;
mod show_file;
mod turn_buffer_to_file;
mod turn_file_to_buffer;

pub use buffer_info::*;
pub use buffer_set_byte::*;
pub use buffer_write_bytes::*;
pub use create_file::*;
pub use fill_buffer::*;
pub use from_buffer_to_file::*;
pub use from_file_to_buffer::*;
pub use make_buffer::*;
pub use merge_buffers::*;
pub use open_file::*;
pub use pull_out_slice::*;
pub use set_variable::*;
pub use show_buffer::*;
pub use show_file::*;
pub use turn_buffer_to_file::*;
pub use turn_file_to_buffer::*;

// Inner ones

mod make_table;

use make_table::make_table;
