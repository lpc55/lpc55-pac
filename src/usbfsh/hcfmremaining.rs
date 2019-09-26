#[doc = "Reader of register HCFMREMAINING"]
pub type R = crate::R<u32, super::HCFMREMAINING>;
#[doc = "Writer for register HCFMREMAINING"]
pub type W = crate::W<u32, super::HCFMREMAINING>;
#[doc = "Register HCFMREMAINING `reset()`'s with value 0"]
impl crate::ResetValue for super::HCFMREMAINING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FR`"]
pub type FR_R = crate::R<u16, u16>;
#[doc = "Reader of field `FRT`"]
pub type FRT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:13 - FrameRemaining This counter is decremented at each bit time."]
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - FrameRemainingToggle This bit is loaded from the FrameIntervalToggle field of HcFmInterval whenever FrameRemaining reaches 0."]
    #[inline(always)]
    pub fn frt(&self) -> FRT_R {
        FRT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {}
