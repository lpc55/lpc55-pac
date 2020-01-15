#[doc = "Writer for register FMCFLUSH"]
pub type W = crate::W<u32, super::FMCFLUSH>;
#[doc = "Register FMCFLUSH `reset()`'s with value 0"]
impl crate::ResetValue for super::FMCFLUSH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Flush control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLUSH_AW {
    #[doc = "0: No action is performed."]
    NO_FLUSH = 0,
    #[doc = "1: Flush the FMC buffer contents."]
    FLUSH = 1,
}
impl From<FLUSH_AW> for bool {
    #[inline(always)]
    fn from(variant: FLUSH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FLUSH`"]
pub struct FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLUSH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action is performed."]
    #[inline(always)]
    pub fn no_flush(self) -> &'a mut W {
        self.variant(FLUSH_AW::NO_FLUSH)
    }
    #[doc = "Flush the FMC buffer contents."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(FLUSH_AW::FLUSH)
    }
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
impl W {
    #[doc = "Bit 0 - Flush control"]
    #[inline(always)]
    pub fn flush(&mut self) -> FLUSH_W {
        FLUSH_W { w: self }
    }
}
