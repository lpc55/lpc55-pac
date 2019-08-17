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
#[doc = "Reader of field `DBG_VENDOR_USAGE`"]
pub type DBG_VENDOR_USAGE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DBG_VENDOR_USAGE`"]
pub struct DBG_VENDOR_USAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_VENDOR_USAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `INVERSE_VALUE`"]
pub type INVERSE_VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INVERSE_VALUE`"]
pub struct INVERSE_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> INVERSE_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DBG_VENDOR_USAGE."]
    #[inline(always)]
    pub fn dbg_vendor_usage(&self) -> DBG_VENDOR_USAGE_R {
        DBG_VENDOR_USAGE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&self) -> INVERSE_VALUE_R {
        INVERSE_VALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DBG_VENDOR_USAGE."]
    #[inline(always)]
    pub fn dbg_vendor_usage(&mut self) -> DBG_VENDOR_USAGE_W {
        DBG_VENDOR_USAGE_W { w: self }
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&mut self) -> INVERSE_VALUE_W {
        INVERSE_VALUE_W { w: self }
    }
}
