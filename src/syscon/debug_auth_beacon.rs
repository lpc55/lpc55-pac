#[doc = "Reader of register DEBUG_AUTH_BEACON"]
pub type R = crate::R<u32, super::DEBUG_AUTH_BEACON>;
#[doc = "Writer for register DEBUG_AUTH_BEACON"]
pub type W = crate::W<u32, super::DEBUG_AUTH_BEACON>;
#[doc = "Register DEBUG_AUTH_BEACON `reset()`'s with value 0"]
impl crate::ResetValue for super::DEBUG_AUTH_BEACON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BEACON`"]
pub type BEACON_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BEACON`"]
pub struct BEACON_W<'a> {
    w: &'a mut W,
}
impl<'a> BEACON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
    #[inline(always)]
    pub fn beacon(&self) -> BEACON_R {
        BEACON_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
    #[inline(always)]
    pub fn beacon(&mut self) -> BEACON_W {
        BEACON_W { w: self }
    }
}
