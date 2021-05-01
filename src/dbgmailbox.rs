#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC mode register"]
    pub csw: crate::Reg<csw::CSW_SPEC>,
    #[doc = "0x04 - CRC seed register"]
    pub request: crate::Reg<request::REQUEST_SPEC>,
    #[doc = "0x08 - Return value from ROM."]
    pub return_: crate::Reg<return_::RETURN_SPEC>,
    _reserved3: [u8; 240usize],
    #[doc = "0xfc - Identification register"]
    pub id: crate::Reg<id::ID_SPEC>,
}
#[doc = "CSW register accessor: an alias for `Reg<CSW_SPEC>`"]
pub type CSW = crate::Reg<csw::CSW_SPEC>;
#[doc = "CRC mode register"]
pub mod csw;
#[doc = "REQUEST register accessor: an alias for `Reg<REQUEST_SPEC>`"]
pub type REQUEST = crate::Reg<request::REQUEST_SPEC>;
#[doc = "CRC seed register"]
pub mod request;
#[doc = "RETURN register accessor: an alias for `Reg<RETURN_SPEC>`"]
pub type RETURN = crate::Reg<return_::RETURN_SPEC>;
#[doc = "Return value from ROM."]
pub mod return_;
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Identification register"]
pub mod id;
