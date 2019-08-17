#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - update lock out control"]
    pub updatelckout: UPDATELCKOUT,
    _reserved1: [u8; 60usize],
    #[doc = "0x40 - Selects the source for SCK going into Flexcomm 0"]
    pub fcctrlsel: [FCCTRLSEL; 8],
    _reserved2: [u8; 32usize],
    #[doc = "0x80 - Selects sources and data combinations for shared signal set 0."]
    pub sharedctrlset: [SHAREDCTRLSET; 2],
    _reserved3: [u8; 120usize],
    #[doc = "0x100 - Status register for USB HS"]
    pub usb_hs_status: USB_HS_STATUS,
}
#[doc = "update lock out control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [updatelckout](updatelckout) module"]
pub type UPDATELCKOUT = crate::Reg<u32, _UPDATELCKOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPDATELCKOUT;
#[doc = "`read()` method returns [updatelckout::R](updatelckout::R) reader structure"]
impl crate::Readable for UPDATELCKOUT {}
#[doc = "`write(|w| ..)` method takes [updatelckout::W](updatelckout::W) writer structure"]
impl crate::Writable for UPDATELCKOUT {}
#[doc = "update lock out control"]
pub mod updatelckout;
#[doc = "Selects the source for SCK going into Flexcomm 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcctrlsel](fcctrlsel) module"]
pub type FCCTRLSEL = crate::Reg<u32, _FCCTRLSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCTRLSEL;
#[doc = "`read()` method returns [fcctrlsel::R](fcctrlsel::R) reader structure"]
impl crate::Readable for FCCTRLSEL {}
#[doc = "`write(|w| ..)` method takes [fcctrlsel::W](fcctrlsel::W) writer structure"]
impl crate::Writable for FCCTRLSEL {}
#[doc = "Selects the source for SCK going into Flexcomm 0"]
pub mod fcctrlsel;
#[doc = "Selects sources and data combinations for shared signal set 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sharedctrlset](sharedctrlset) module"]
pub type SHAREDCTRLSET = crate::Reg<u32, _SHAREDCTRLSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHAREDCTRLSET;
#[doc = "`read()` method returns [sharedctrlset::R](sharedctrlset::R) reader structure"]
impl crate::Readable for SHAREDCTRLSET {}
#[doc = "`write(|w| ..)` method takes [sharedctrlset::W](sharedctrlset::W) writer structure"]
impl crate::Writable for SHAREDCTRLSET {}
#[doc = "Selects sources and data combinations for shared signal set 0."]
pub mod sharedctrlset;
#[doc = "Status register for USB HS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_hs_status](usb_hs_status) module"]
pub type USB_HS_STATUS = crate::Reg<u32, _USB_HS_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_HS_STATUS;
#[doc = "`read()` method returns [usb_hs_status::R](usb_hs_status::R) reader structure"]
impl crate::Readable for USB_HS_STATUS {}
#[doc = "`write(|w| ..)` method takes [usb_hs_status::W](usb_hs_status::W) writer structure"]
impl crate::Writable for USB_HS_STATUS {}
#[doc = "Status register for USB HS"]
pub mod usb_hs_status;
