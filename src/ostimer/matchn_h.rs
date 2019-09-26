#[doc = "Reader of register MATCHn_H"]
pub type R = crate::R<u32, super::MATCHN_H>;
#[doc = "Writer for register MATCHn_H"]
pub type W = crate::W<u32, super::MATCHN_H>;
#[doc = "Register MATCHn_H `reset()`'s with value 0"]
impl crate::ResetValue for super::MATCHN_H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MATCHn_VALUE`"]
pub type MATCHN_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MATCHn_VALUE`"]
pub struct MATCHN_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCHN_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - The value written to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled. A separate pair of MATCH registers are implemented for each CPU. Each CPU reads its own local value at the same pair of addresses."]
    #[inline(always)]
    pub fn matchn_value(&self) -> MATCHN_VALUE_R {
        MATCHN_VALUE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The value written to the MATCH (L/H) register pair is compared against the central EVTIMER. When a match occurs, an interrupt request is generated if enabled. A separate pair of MATCH registers are implemented for each CPU. Each CPU reads its own local value at the same pair of addresses."]
    #[inline(always)]
    pub fn matchn_value(&mut self) -> MATCHN_VALUE_W {
        MATCHN_VALUE_W { w: self }
    }
}
