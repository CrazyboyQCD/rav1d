use crate::include::stdint::uint16_t;
use crate::include::stdint::uint32_t;
use crate::include::stdint::uint64_t;
use crate::include::stdint::uint8_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias8 {
    pub u8_0: uint8_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias16 {
    pub u16_0: uint16_t,
    pub u8_0: [uint8_t; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias32 {
    pub u32_0: uint32_t,
    pub u8_0: [uint8_t; 4],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union alias64 {
    pub u64_0: uint64_t,
    pub u8_0: [uint8_t; 8],
}