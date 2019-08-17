#[doc = "Reader of register PLL1PDEC"]
pub type R = crate::R<u32, super::PLL1PDEC>;
#[doc = "Writer for register PLL1PDEC"]
pub type W = crate::W<u32, super::PLL1PDEC>;
#[doc = "Register PLL1PDEC `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL1PDEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDIV`"]
pub type PDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDIV`"]
pub struct PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `PREQ`"]
pub type PREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREQ`"]
pub struct PREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - post-divider divider ratio (P-divider)"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - feedback ratio change request."]
    #[inline(always)]
    pub fn preq(&self) -> PREQ_R {
        PREQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - post-divider divider ratio (P-divider)"]
    #[inline(always)]
    pub fn pdiv(&mut self) -> PDIV_W {
        PDIV_W { w: self }
    }
    #[doc = "Bit 5 - feedback ratio change request."]
    #[inline(always)]
    pub fn preq(&mut self) -> PREQ_W {
        PREQ_W { w: self }
    }
}
