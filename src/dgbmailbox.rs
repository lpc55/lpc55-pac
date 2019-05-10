#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC mode register"]
    pub csw: CSW,
    #[doc = "0x04 - CRC seed register"]
    pub request: REQUEST,
    #[doc = "0x08 - Return value from ROM."]
    pub return_: RETURN,
    _reserved0: [u8; 240usize],
    #[doc = "0xfc - Identification register"]
    pub id: ID,
}
#[doc = "CRC mode register"]
pub struct CSW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC mode register"]
pub mod csw;
#[doc = "CRC seed register"]
pub struct REQUEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC seed register"]
pub mod request;
#[doc = "Return value from ROM."]
pub struct RETURN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Return value from ROM."]
pub mod return_;
#[doc = "Identification register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Identification register"]
pub mod id;
