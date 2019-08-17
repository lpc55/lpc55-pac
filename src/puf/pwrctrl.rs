#[doc = "Reader of register PWRCTRL"]
pub type R = crate::R<u32, super::PWRCTRL>;
#[doc = "Writer for register PWRCTRL"]
pub type W = crate::W<u32, super::PWRCTRL>;
#[doc = "Register PWRCTRL `reset()`'s with value 0xf8"]
impl crate::ResetValue for super::PWRCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf8
    }
}
#[doc = "Reader of field `RAMON`"]
pub type RAMON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMON`"]
pub struct RAMON_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMON_W<'a> {
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
#[doc = "Reader of field `RAMSTAT`"]
pub type RAMSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMSTAT`"]
pub struct RAMSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMSTAT_W<'a> {
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
    #[doc = "Bit 0 - Power on the PUF RAM."]
    #[inline(always)]
    pub fn ramon(&self) -> RAMON_R {
        RAMON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PUF RAM status."]
    #[inline(always)]
    pub fn ramstat(&self) -> RAMSTAT_R {
        RAMSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power on the PUF RAM."]
    #[inline(always)]
    pub fn ramon(&mut self) -> RAMON_W {
        RAMON_W { w: self }
    }
    #[doc = "Bit 1 - PUF RAM status."]
    #[inline(always)]
    pub fn ramstat(&mut self) -> RAMSTAT_W {
        RAMSTAT_W { w: self }
    }
}
