#[doc = "Reader of register POWERDOWN"]
pub type R = crate::R<u32, super::POWERDOWN>;
#[doc = "Writer for register POWERDOWN"]
pub type W = crate::W<u32, super::POWERDOWN>;
#[doc = "Register POWERDOWN `reset()`'s with value 0"]
impl crate::ResetValue for super::POWERDOWN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFT_RESET`"]
pub type SOFT_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFT_RESET`"]
pub struct SOFT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_RESET_W<'a> {
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
#[doc = "Reader of field `FORCE_SOFT_RESET`"]
pub type FORCE_SOFT_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_SOFT_RESET`"]
pub struct FORCE_SOFT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_SOFT_RESET_W<'a> {
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
#[doc = "Reader of field `POWERDOWN`"]
pub type POWERDOWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POWERDOWN`"]
pub struct POWERDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> POWERDOWN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Request softreset that will go low automaticaly after acknowledge from CORE."]
    #[inline(always)]
    pub fn soft_reset(&self) -> SOFT_RESET_R {
        SOFT_RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
    #[inline(always)]
    pub fn force_soft_reset(&self) -> FORCE_SOFT_RESET_R {
        FORCE_SOFT_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 31 - When set all accesses to standard registers are blocked."]
    #[inline(always)]
    pub fn powerdown(&self) -> POWERDOWN_R {
        POWERDOWN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Request softreset that will go low automaticaly after acknowledge from CORE."]
    #[inline(always)]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W {
        SOFT_RESET_W { w: self }
    }
    #[doc = "Bit 1 - When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
    #[inline(always)]
    pub fn force_soft_reset(&mut self) -> FORCE_SOFT_RESET_W {
        FORCE_SOFT_RESET_W { w: self }
    }
    #[doc = "Bit 31 - When set all accesses to standard registers are blocked."]
    #[inline(always)]
    pub fn powerdown(&mut self) -> POWERDOWN_W {
        POWERDOWN_W { w: self }
    }
}
