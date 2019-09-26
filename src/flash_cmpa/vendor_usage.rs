#[doc = "Reader of register VENDOR_USAGE"]
pub type R = crate::R<u32, super::VENDOR_USAGE>;
#[doc = "Writer for register VENDOR_USAGE"]
pub type W = crate::W<u32, super::VENDOR_USAGE>;
#[doc = "Register VENDOR_USAGE `reset()`'s with value 0"]
impl crate::ResetValue for super::VENDOR_USAGE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VENDOR_USAGE`"]
pub type VENDOR_USAGE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VENDOR_USAGE`"]
pub struct VENDOR_USAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> VENDOR_USAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
    #[inline(always)]
    pub fn vendor_usage(&self) -> VENDOR_USAGE_R {
        VENDOR_USAGE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
    #[inline(always)]
    pub fn vendor_usage(&mut self) -> VENDOR_USAGE_W {
        VENDOR_USAGE_W { w: self }
    }
}
