#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `READYEN`"]
pub type READYEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READYEN`"]
pub struct READYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> READYEN_W<'a> {
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
#[doc = "Reader of field `SUCCESEN`"]
pub type SUCCESEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUCCESEN`"]
pub struct SUCCESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SUCCESEN_W<'a> {
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
#[doc = "Reader of field `ERROREN`"]
pub type ERROREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERROREN`"]
pub struct ERROREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROREN_W<'a> {
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
#[doc = "Reader of field `KEYINREQEN`"]
pub type KEYINREQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEYINREQEN`"]
pub struct KEYINREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYINREQEN_W<'a> {
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
#[doc = "Reader of field `KEYOUTAVAILEN`"]
pub type KEYOUTAVAILEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEYOUTAVAILEN`"]
pub struct KEYOUTAVAILEN_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYOUTAVAILEN_W<'a> {
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
#[doc = "Reader of field `CODEINREQEN`"]
pub type CODEINREQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CODEINREQEN`"]
pub struct CODEINREQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CODEINREQEN_W<'a> {
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
#[doc = "Reader of field `CODEOUTAVAILEN`"]
pub type CODEOUTAVAILEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CODEOUTAVAILEN`"]
pub struct CODEOUTAVAILEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CODEOUTAVAILEN_W<'a> {
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
    #[doc = "Bit 0 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn readyen(&self) -> READYEN_R {
        READYEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn succesen(&self) -> SUCCESEN_R {
        SUCCESEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn erroren(&self) -> ERROREN_R {
        ERROREN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn keyinreqen(&self) -> KEYINREQEN_R {
        KEYINREQEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn keyoutavailen(&self) -> KEYOUTAVAILEN_R {
        KEYOUTAVAILEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn codeinreqen(&self) -> CODEINREQEN_R {
        CODEINREQEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn codeoutavailen(&self) -> CODEOUTAVAILEN_R {
        CODEOUTAVAILEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn readyen(&mut self) -> READYEN_W {
        READYEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn succesen(&mut self) -> SUCCESEN_W {
        SUCCESEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn erroren(&mut self) -> ERROREN_W {
        ERROREN_W { w: self }
    }
    #[doc = "Bit 4 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn keyinreqen(&mut self) -> KEYINREQEN_W {
        KEYINREQEN_W { w: self }
    }
    #[doc = "Bit 5 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn keyoutavailen(&mut self) -> KEYOUTAVAILEN_W {
        KEYOUTAVAILEN_W { w: self }
    }
    #[doc = "Bit 6 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn codeinreqen(&mut self) -> CODEINREQEN_W {
        CODEINREQEN_W { w: self }
    }
    #[doc = "Bit 7 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn codeoutavailen(&mut self) -> CODEOUTAVAILEN_W {
        CODEOUTAVAILEN_W { w: self }
    }
}
