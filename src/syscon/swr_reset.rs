#[doc = "Writer for register SWR_RESET"]
pub type W = crate::W<u32, super::SWR_RESET>;
#[doc = "Register SWR_RESET `reset()`'s with value 0"]
impl crate::ResetValue for super::SWR_RESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write 0x5A00_0001 to generate a software_reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SWR_RESET_AW {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1509949441: Generate a software reset."]
    ASSERTED = 1509949441,
}
impl From<SWR_RESET_AW> for u32 {
    #[inline(always)]
    fn from(variant: SWR_RESET_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `SWR_RESET`"]
pub struct SWR_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWR_RESET_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SWR_RESET_AW::RELEASED)
    }
    #[doc = "Generate a software reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SWR_RESET_AW::ASSERTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Write 0x5A00_0001 to generate a software_reset."]
    #[inline(always)]
    pub fn swr_reset(&mut self) -> SWR_RESET_W {
        SWR_RESET_W { w: self }
    }
}
