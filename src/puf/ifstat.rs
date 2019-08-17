#[doc = "Reader of register IFSTAT"]
pub type R = crate::R<u32, super::IFSTAT>;
#[doc = "Writer for register IFSTAT"]
pub type W = crate::W<u32, super::IFSTAT>;
#[doc = "Register IFSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::IFSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
}
