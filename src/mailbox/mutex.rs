#[doc = "Reader of register MUTEX"]
pub type R = crate::R<u32, super::MUTEX>;
#[doc = "Writer for register MUTEX"]
pub type W = crate::W<u32, super::MUTEX>;
#[doc = "Register MUTEX `reset()`'s with value 0x01"]
impl crate::ResetValue for super::MUTEX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `EX`"]
pub type EX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EX`"]
pub struct EX_W<'a> {
    w: &'a mut W,
}
impl<'a> EX_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Cleared when read, set when written. See usage description above."]
    #[inline(always)]
    pub fn ex(&self) -> EX_R {
        EX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cleared when read, set when written. See usage description above."]
    #[inline(always)]
    pub fn ex(&mut self) -> EX_W {
        EX_W { w: self }
    }
}
