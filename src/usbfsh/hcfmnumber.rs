#[doc = "Reader of register HCFMNUMBER"]
pub type R = crate::R<u32, super::HCFMNUMBER>;
#[doc = "Writer for register HCFMNUMBER"]
pub type W = crate::W<u32, super::HCFMNUMBER>;
#[doc = "Register HCFMNUMBER `reset()`'s with value 0"]
impl crate::ResetValue for super::HCFMNUMBER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FN`"]
pub type FN_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - FrameNumber This is incremented when HcFmRemaining is re-loaded."]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
