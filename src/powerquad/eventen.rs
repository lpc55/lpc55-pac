#[doc = "Reader of register EVENTEN"]
pub type R = crate::R<u32, super::EVENTEN>;
#[doc = "Writer for register EVENTEN"]
pub type W = crate::W<u32, super::EVENTEN>;
#[doc = "Register EVENTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `event_oflow`"]
pub type EVENT_OFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `event_oflow`"]
pub struct EVENT_OFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_OFLOW_W<'a> {
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
#[doc = "Reader of field `event_nan`"]
pub type EVENT_NAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `event_nan`"]
pub struct EVENT_NAN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_NAN_W<'a> {
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
#[doc = "Reader of field `event_fixed`"]
pub type EVENT_FIXED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `event_fixed`"]
pub struct EVENT_FIXED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_FIXED_W<'a> {
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
#[doc = "Reader of field `event_uflow`"]
pub type EVENT_UFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `event_uflow`"]
pub struct EVENT_UFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_UFLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `event_berr`"]
pub type EVENT_BERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `event_berr`"]
pub struct EVENT_BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_BERR_W<'a> {
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
#[doc = "Reader of field `event_comp`"]
pub type EVENT_COMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `event_comp`"]
pub struct EVENT_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_COMP_W<'a> {
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
    #[doc = "Bit 0 - 1 : Enable event trigger on Floating point overflow"]
    #[inline(always)]
    pub fn event_oflow(&self) -> EVENT_OFLOW_R {
        EVENT_OFLOW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1 : Enable event trigger on Floating point NaN"]
    #[inline(always)]
    pub fn event_nan(&self) -> EVENT_NAN_R {
        EVENT_NAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1: Enable event trigger on Fixed point Overflow"]
    #[inline(always)]
    pub fn event_fixed(&self) -> EVENT_FIXED_R {
        EVENT_FIXED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 : Enable event trigger on Subnormal truncation"]
    #[inline(always)]
    pub fn event_uflow(&self) -> EVENT_UFLOW_R {
        EVENT_UFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1: Enable event trigger on AHBM Buss Error"]
    #[inline(always)]
    pub fn event_berr(&self) -> EVENT_BERR_R {
        EVENT_BERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1: Enable event trigger on instruction completion"]
    #[inline(always)]
    pub fn event_comp(&self) -> EVENT_COMP_R {
        EVENT_COMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 : Enable event trigger on Floating point overflow"]
    #[inline(always)]
    pub fn event_oflow(&mut self) -> EVENT_OFLOW_W {
        EVENT_OFLOW_W { w: self }
    }
    #[doc = "Bit 1 - 1 : Enable event trigger on Floating point NaN"]
    #[inline(always)]
    pub fn event_nan(&mut self) -> EVENT_NAN_W {
        EVENT_NAN_W { w: self }
    }
    #[doc = "Bit 2 - 1: Enable event trigger on Fixed point Overflow"]
    #[inline(always)]
    pub fn event_fixed(&mut self) -> EVENT_FIXED_W {
        EVENT_FIXED_W { w: self }
    }
    #[doc = "Bit 3 - 1 : Enable event trigger on Subnormal truncation"]
    #[inline(always)]
    pub fn event_uflow(&mut self) -> EVENT_UFLOW_W {
        EVENT_UFLOW_W { w: self }
    }
    #[doc = "Bit 4 - 1: Enable event trigger on AHBM Buss Error"]
    #[inline(always)]
    pub fn event_berr(&mut self) -> EVENT_BERR_W {
        EVENT_BERR_W { w: self }
    }
    #[doc = "Bit 7 - 1: Enable event trigger on instruction completion"]
    #[inline(always)]
    pub fn event_comp(&mut self) -> EVENT_COMP_W {
        EVENT_COMP_W { w: self }
    }
}
