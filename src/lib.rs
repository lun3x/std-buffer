#![feature(maybe_uninit_slice)]
#![feature(read_buf)]
#![feature(core_io_borrowed_buf)]
#![feature(allocator_api)]
#![feature(io_const_error)]
mod buffer;
pub use buffer::Buffer;
