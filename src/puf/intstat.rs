#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Writer for register INTSTAT"]
pub type W = crate::W<u32, super::INTSTAT>;
#[doc = "Register INTSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READY`"]
pub struct READY_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SUCCESS`"]
pub type SUCCESS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUCCESS`"]
pub struct SUCCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> SUCCESS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROR`"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `KEYINREQ`"]
pub type KEYINREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEYINREQ`"]
pub struct KEYINREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYINREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `KEYOUTAVAIL`"]
pub type KEYOUTAVAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEYOUTAVAIL`"]
pub struct KEYOUTAVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYOUTAVAIL_W<'a> {
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
#[doc = "Reader of field `CODEINREQ`"]
pub type CODEINREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CODEINREQ`"]
pub struct CODEINREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CODEINREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CODEOUTAVAIL`"]
pub type CODEOUTAVAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CODEOUTAVAIL`"]
pub struct CODEOUTAVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> CODEOUTAVAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Triggers on falling edge of busy, write 1 to clear"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn success(&self) -> SUCCESS_R {
        SUCCESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn keyinreq(&self) -> KEYINREQ_R {
        KEYINREQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn keyoutavail(&self) -> KEYOUTAVAIL_R {
        KEYOUTAVAIL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn codeinreq(&self) -> CODEINREQ_R {
        CODEINREQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn codeoutavail(&self) -> CODEOUTAVAIL_R {
        CODEOUTAVAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Triggers on falling edge of busy, write 1 to clear"]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W {
        READY_W { w: self }
    }
    #[doc = "Bit 1 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn success(&mut self) -> SUCCESS_W {
        SUCCESS_W { w: self }
    }
    #[doc = "Bit 2 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Bit 4 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn keyinreq(&mut self) -> KEYINREQ_W {
        KEYINREQ_W { w: self }
    }
    #[doc = "Bit 5 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn keyoutavail(&mut self) -> KEYOUTAVAIL_W {
        KEYOUTAVAIL_W { w: self }
    }
    #[doc = "Bit 6 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn codeinreq(&mut self) -> CODEINREQ_W {
        CODEINREQ_W { w: self }
    }
    #[doc = "Bit 7 - Level sensitive interrupt, cleared when interrupt source clears"]
    #[inline(always)]
    pub fn codeoutavail(&mut self) -> CODEOUTAVAIL_W {
        CODEOUTAVAIL_W { w: self }
    }
}
