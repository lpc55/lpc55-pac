#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - update lock out control"]
    pub updatelckout: UPDATELCKOUT,
    _reserved0: [u8; 60usize],
    #[doc = "0x40 - Selects the source for SCK going into Flexcomm 0"]
    pub fcctrlsel: [FCCTRLSEL; 8],
    _reserved1: [u8; 32usize],
    #[doc = "0x80 - Selects sources and data combinations for shared signal set 0."]
    pub sharedctrlset: [SHAREDCTRLSET; 2],
    _reserved2: [u8; 120usize],
    #[doc = "0x100 - Status register for USB HS"]
    pub usb_hs_status: USB_HS_STATUS,
}
#[doc = "update lock out control"]
pub struct UPDATELCKOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "update lock out control"]
pub mod updatelckout;
#[doc = "Selects the source for SCK going into Flexcomm 0"]
pub struct FCCTRLSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects the source for SCK going into Flexcomm 0"]
pub mod fcctrlsel;
#[doc = "Selects sources and data combinations for shared signal set 0."]
pub struct SHAREDCTRLSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects sources and data combinations for shared signal set 0."]
pub mod sharedctrlset;
#[doc = "Status register for USB HS"]
pub struct USB_HS_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register for USB HS"]
pub mod usb_hs_status;
