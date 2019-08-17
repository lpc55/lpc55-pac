#[doc = "Reader of register PLL1MDEC"]
pub type R = crate::R<u32, super::PLL1MDEC>;
#[doc = "Writer for register PLL1MDEC"]
pub type W = crate::W<u32, super::PLL1MDEC>;
#[doc = "Register PLL1MDEC `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL1MDEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MDIV`"]
pub type MDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MDIV`"]
pub struct MDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `MREQ`"]
pub type MREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MREQ`"]
pub struct MREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MREQ_W<'a> {
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
impl R {
    #[doc = "Bits 0:15 - feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    pub fn mdiv(&self) -> MDIV_R {
        MDIV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - feedback ratio change request."]
    #[inline(always)]
    pub fn mreq(&self) -> MREQ_R {
        MREQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    pub fn mdiv(&mut self) -> MDIV_W {
        MDIV_W { w: self }
    }
    #[doc = "Bit 16 - feedback ratio change request."]
    #[inline(always)]
    pub fn mreq(&mut self) -> MREQ_W {
        MREQ_W { w: self }
    }
}
