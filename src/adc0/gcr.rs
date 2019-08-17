#[doc = "Reader of register GCR[%s]"]
pub type R = crate::R<u32, super::GCR>;
#[doc = "Writer for register GCR[%s]"]
pub type W = crate::W<u32, super::GCR>;
#[doc = "Register GCR[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::GCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GCALR`"]
pub type GCALR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GCALR`"]
pub struct GCALR_W<'a> {
    w: &'a mut W,
}
impl<'a> GCALR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Possible values of the field `RDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY_A {
    #[doc = "The gain offset calculation value is invalid."]
    RDY_0,
    #[doc = "The gain calibration value is valid."]
    RDY_1,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        match variant {
            RDY_A::RDY_0 => false,
            RDY_A::RDY_1 => true,
        }
    }
}
#[doc = "Reader of field `RDY`"]
pub type RDY_R = crate::R<bool, RDY_A>;
impl RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY_A {
        match self.bits {
            false => RDY_A::RDY_0,
            true => RDY_A::RDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDY_0`"]
    #[inline(always)]
    pub fn is_rdy_0(&self) -> bool {
        *self == RDY_A::RDY_0
    }
    #[doc = "Checks if the value of the field is `RDY_1`"]
    #[inline(always)]
    pub fn is_rdy_1(&self) -> bool {
        *self == RDY_A::RDY_1
    }
}
#[doc = "Write proxy for field `RDY`"]
pub struct RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The gain offset calculation value is invalid."]
    #[inline(always)]
    pub fn rdy_0(self) -> &'a mut W {
        self.variant(RDY_A::RDY_0)
    }
    #[doc = "The gain calibration value is valid."]
    #[inline(always)]
    pub fn rdy_1(self) -> &'a mut W {
        self.variant(RDY_A::RDY_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Gain Calculation Result"]
    #[inline(always)]
    pub fn gcalr(&self) -> GCALR_R {
        GCALR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Gain Calculation Ready"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Gain Calculation Result"]
    #[inline(always)]
    pub fn gcalr(&mut self) -> GCALR_W {
        GCALR_W { w: self }
    }
    #[doc = "Bit 24 - Gain Calculation Ready"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RDY_W {
        RDY_W { w: self }
    }
}
