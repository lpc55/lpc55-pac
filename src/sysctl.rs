#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - update lock out control"]
    pub updatelckout: crate::Reg<updatelckout::UPDATELCKOUT_SPEC>,
    _reserved1: [u8; 60usize],
    #[doc = "0x40 - Selects the source for SCK going into Flexcomm index"]
    pub fcctrlsel: [crate::Reg<fcctrlsel::FCCTRLSEL_SPEC>; 8],
    _reserved2: [u8; 32usize],
    #[doc = "0x80 - Selects sources and data combinations for shared signal set index."]
    pub sharedctrlset: [crate::Reg<sharedctrlset::SHAREDCTRLSET_SPEC>; 2],
    _reserved3: [u8; 120usize],
    #[doc = "0x100 - Status register for USB HS"]
    pub usb_hs_status: crate::Reg<usb_hs_status::USB_HS_STATUS_SPEC>,
}
#[doc = "UPDATELCKOUT register accessor: an alias for `Reg<UPDATELCKOUT_SPEC>`"]
pub type UPDATELCKOUT = crate::Reg<updatelckout::UPDATELCKOUT_SPEC>;
#[doc = "update lock out control"]
pub mod updatelckout;
#[doc = "FCCTRLSEL register accessor: an alias for `Reg<FCCTRLSEL_SPEC>`"]
pub type FCCTRLSEL = crate::Reg<fcctrlsel::FCCTRLSEL_SPEC>;
#[doc = "Selects the source for SCK going into Flexcomm index"]
pub mod fcctrlsel;
#[doc = "SHAREDCTRLSET register accessor: an alias for `Reg<SHAREDCTRLSET_SPEC>`"]
pub type SHAREDCTRLSET = crate::Reg<sharedctrlset::SHAREDCTRLSET_SPEC>;
#[doc = "Selects sources and data combinations for shared signal set index."]
pub mod sharedctrlset;
#[doc = "USB_HS_STATUS register accessor: an alias for `Reg<USB_HS_STATUS_SPEC>`"]
pub type USB_HS_STATUS = crate::Reg<usb_hs_status::USB_HS_STATUS_SPEC>;
#[doc = "Status register for USB HS"]
pub mod usb_hs_status;
