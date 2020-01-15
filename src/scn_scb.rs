#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 12usize],
    #[doc = "0x0c - Coprocessor Power Control Register"]
    pub cppwr: CPPWR,
}
#[doc = "Coprocessor Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cppwr](cppwr) module"]
pub type CPPWR = crate::Reg<u32, _CPPWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPPWR;
#[doc = "`read()` method returns [cppwr::R](cppwr::R) reader structure"]
impl crate::Readable for CPPWR {}
#[doc = "`write(|w| ..)` method takes [cppwr::W](cppwr::W) writer structure"]
impl crate::Writable for CPPWR {}
#[doc = "Coprocessor Power Control Register"]
pub mod cppwr;
