//! This module handles the serialization and deserialization of 
//! RTMP chunks.

#[macro_use] extern crate quick_error;
extern crate byteorder;
extern crate rtmp_time;
extern crate rtmp_message;

pub mod serialization;
pub mod deserialization;
mod chunk_header;
mod utils;