#[doc = "Reader of register PLL1NDEC"]
pub type R = crate::R<u32, super::PLL1NDEC>;
#[doc = "Writer for register PLL1NDEC"]
pub type W = crate::W<u32, super::PLL1NDEC>;
#[doc = "Register PLL1NDEC `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL1NDEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NDIV`"]
pub type NDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NDIV`"]
pub struct NDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> NDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `NREQ`"]
pub type NREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NREQ`"]
pub struct NREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> NREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - pre-divider ratio change request."]
    #[inline(always)]
    pub fn nreq(&self) -> NREQ_R {
        NREQ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    pub fn ndiv(&mut self) -> NDIV_W {
        NDIV_W { w: self }
    }
    #[doc = "Bit 8 - pre-divider ratio change request."]
    #[inline(always)]
    pub fn nreq(&mut self) -> NREQ_W {
        NREQ_W { w: self }
    }
}
