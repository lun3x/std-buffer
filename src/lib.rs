#![feature(maybe_uninit_slice)]
#![feature(new_uninit)]
#![feature(read_buf)]
#![feature(core_io_borrowed_buf)]
mod buffer;
pub use buffer::Buffer;
