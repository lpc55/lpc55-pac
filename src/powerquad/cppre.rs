#[doc = "Reader of register CPPRE"]
pub type R = crate::R<u32, super::CPPRE>;
#[doc = "Writer for register CPPRE"]
pub type W = crate::W<u32, super::CPPRE>;
#[doc = "Register CPPRE `reset()`'s with value 0"]
impl crate::ResetValue for super::CPPRE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cppre_in`"]
pub type CPPRE_IN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cppre_in`"]
pub struct CPPRE_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPPRE_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `cppre_out`"]
pub type CPPRE_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cppre_out`"]
pub struct CPPRE_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPPRE_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `cppre_sat`"]
pub type CPPRE_SAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cppre_sat`"]
pub struct CPPRE_SAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPPRE_SAT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `cppre_sat8`"]
pub type CPPRE_SAT8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cppre_sat8`"]
pub struct CPPRE_SAT8_W<'a> {
    w: &'a mut W,
}
impl<'a> CPPRE_SAT8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - co-processor scaling of input"]
    #[inline(always)]
    pub fn cppre_in(&self) -> CPPRE_IN_R {
        CPPRE_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - co-processor fixed point output"]
    #[inline(always)]
    pub fn cppre_out(&self) -> CPPRE_OUT_R {
        CPPRE_OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 1 : forces sub-32 bit saturation"]
    #[inline(always)]
    pub fn cppre_sat(&self) -> CPPRE_SAT_R {
        CPPRE_SAT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 0 = 8bits, 1 = 16bits"]
    #[inline(always)]
    pub fn cppre_sat8(&self) -> CPPRE_SAT8_R {
        CPPRE_SAT8_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - co-processor scaling of input"]
    #[inline(always)]
    pub fn cppre_in(&mut self) -> CPPRE_IN_W {
        CPPRE_IN_W { w: self }
    }
    #[doc = "Bits 8:15 - co-processor fixed point output"]
    #[inline(always)]
    pub fn cppre_out(&mut self) -> CPPRE_OUT_W {
        CPPRE_OUT_W { w: self }
    }
    #[doc = "Bit 16 - 1 : forces sub-32 bit saturation"]
    #[inline(always)]
    pub fn cppre_sat(&mut self) -> CPPRE_SAT_W {
        CPPRE_SAT_W { w: self }
    }
    #[doc = "Bit 17 - 0 = 8bits, 1 = 16bits"]
    #[inline(always)]
    pub fn cppre_sat8(&mut self) -> CPPRE_SAT8_W {
        CPPRE_SAT8_W { w: self }
    }
}
