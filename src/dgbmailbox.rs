#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC mode register"]
    pub csw: CSW,
    #[doc = "0x04 - CRC seed register"]
    pub request: REQUEST,
    #[doc = "0x08 - Return value from ROM."]
    pub return_: RETURN,
    _reserved3: [u8; 240usize],
    #[doc = "0xfc - Identification register"]
    pub id: ID,
}
#[doc = "CRC mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csw](csw) module"]
pub type CSW = crate::Reg<u32, _CSW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSW;
#[doc = "`read()` method returns [csw::R](csw::R) reader structure"]
impl crate::Readable for CSW {}
#[doc = "`write(|w| ..)` method takes [csw::W](csw::W) writer structure"]
impl crate::Writable for CSW {}
#[doc = "CRC mode register"]
pub mod csw;
#[doc = "CRC seed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [request](request) module"]
pub type REQUEST = crate::Reg<u32, _REQUEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQUEST;
#[doc = "`read()` method returns [request::R](request::R) reader structure"]
impl crate::Readable for REQUEST {}
#[doc = "`write(|w| ..)` method takes [request::W](request::W) writer structure"]
impl crate::Writable for REQUEST {}
#[doc = "CRC seed register"]
pub mod request;
#[doc = "Return value from ROM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [return_](return_) module"]
pub type RETURN = crate::Reg<u32, _RETURN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RETURN;
#[doc = "`read()` method returns [return_::R](return_::R) reader structure"]
impl crate::Readable for RETURN {}
#[doc = "`write(|w| ..)` method takes [return_::W](return_::W) writer structure"]
impl crate::Writable for RETURN {}
#[doc = "Return value from ROM."]
pub mod return_;
#[doc = "Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "Identification register"]
pub mod id;
