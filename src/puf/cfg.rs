#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLOCKENROLL_SETKEY`"]
pub type BLOCKENROLL_SETKEY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLOCKENROLL_SETKEY`"]
pub struct BLOCKENROLL_SETKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCKENROLL_SETKEY_W<'a> {
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
#[doc = "Reader of field `BLOCKKEYOUTPUT`"]
pub type BLOCKKEYOUTPUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLOCKKEYOUTPUT`"]
pub struct BLOCKKEYOUTPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCKKEYOUTPUT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Block enroll operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockenroll_setkey(&self) -> BLOCKENROLL_SETKEY_R {
        BLOCKENROLL_SETKEY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Block set key operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockkeyoutput(&self) -> BLOCKKEYOUTPUT_R {
        BLOCKKEYOUTPUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Block enroll operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockenroll_setkey(&mut self) -> BLOCKENROLL_SETKEY_W {
        BLOCKENROLL_SETKEY_W { w: self }
    }
    #[doc = "Bit 1 - Block set key operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockkeyoutput(&mut self) -> BLOCKKEYOUTPUT_W {
        BLOCKKEYOUTPUT_W { w: self }
    }
}
