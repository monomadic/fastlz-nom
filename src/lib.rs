//! FastLZ compressor/decompressor for nom
//!
//! Native rust implementation of the FastLZ compression and decompression library, as a nom
//! parser/combinator. Currently only deflates.
//!

mod cb;
mod deflate;

pub use deflate::deflate;
