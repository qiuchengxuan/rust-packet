extern crate byteorder;
extern crate hwaddr;
extern crate libc;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

#[cfg(target_os = "linux")]
pub mod afpacket;
pub mod eth;
