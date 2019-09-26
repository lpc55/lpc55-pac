#[doc = "Reader of register HCDONEHEAD"]
pub type R = crate::R<u32, super::HCDONEHEAD>;
#[doc = "Writer for register HCDONEHEAD"]
pub type W = crate::W<u32, super::HCDONEHEAD>;
#[doc = "Register HCDONEHEAD `reset()`'s with value 0"]
impl crate::ResetValue for super::HCDONEHEAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DH`"]
pub type DH_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 4:31 - DoneHead When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD."]
    #[inline(always)]
    pub fn dh(&self) -> DH_R {
        DH_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
impl W {}
