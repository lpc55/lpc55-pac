#[doc = "Reader of register INTREN"]
pub type R = crate::R<u32, super::INTREN>;
#[doc = "Writer for register INTREN"]
pub type W = crate::W<u32, super::INTREN>;
#[doc = "Register INTREN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTREN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `intr_oflow`"]
pub type INTR_OFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `intr_oflow`"]
pub struct INTR_OFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_OFLOW_W<'a> {
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
#[doc = "Reader of field `intr_nan`"]
pub type INTR_NAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `intr_nan`"]
pub struct INTR_NAN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_NAN_W<'a> {
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
#[doc = "Reader of field `intr_fixed`"]
pub type INTR_FIXED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `intr_fixed`"]
pub struct INTR_FIXED_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_FIXED_W<'a> {
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
#[doc = "Reader of field `intr_uflow`"]
pub type INTR_UFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `intr_uflow`"]
pub struct INTR_UFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_UFLOW_W<'a> {
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
#[doc = "Reader of field `intr_berr`"]
pub type INTR_BERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `intr_berr`"]
pub struct INTR_BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_BERR_W<'a> {
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
#[doc = "Reader of field `intr_comp`"]
pub type INTR_COMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `intr_comp`"]
pub struct INTR_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_COMP_W<'a> {
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
    #[doc = "Bit 0 - 1 : Enable interrupt on Floating point overflow"]
    #[inline(always)]
    pub fn intr_oflow(&self) -> INTR_OFLOW_R {
        INTR_OFLOW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1 : Enable interrupt on Floating point NaN"]
    #[inline(always)]
    pub fn intr_nan(&self) -> INTR_NAN_R {
        INTR_NAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1: Enable interrupt on Fixed point Overflow"]
    #[inline(always)]
    pub fn intr_fixed(&self) -> INTR_FIXED_R {
        INTR_FIXED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 : Enable interrupt on Subnormal truncation"]
    #[inline(always)]
    pub fn intr_uflow(&self) -> INTR_UFLOW_R {
        INTR_UFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1: Enable interrupt on AHBM Buss Error"]
    #[inline(always)]
    pub fn intr_berr(&self) -> INTR_BERR_R {
        INTR_BERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1: Enable interrupt on instruction completion"]
    #[inline(always)]
    pub fn intr_comp(&self) -> INTR_COMP_R {
        INTR_COMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 : Enable interrupt on Floating point overflow"]
    #[inline(always)]
    pub fn intr_oflow(&mut self) -> INTR_OFLOW_W {
        INTR_OFLOW_W { w: self }
    }
    #[doc = "Bit 1 - 1 : Enable interrupt on Floating point NaN"]
    #[inline(always)]
    pub fn intr_nan(&mut self) -> INTR_NAN_W {
        INTR_NAN_W { w: self }
    }
    #[doc = "Bit 2 - 1: Enable interrupt on Fixed point Overflow"]
    #[inline(always)]
    pub fn intr_fixed(&mut self) -> INTR_FIXED_W {
        INTR_FIXED_W { w: self }
    }
    #[doc = "Bit 3 - 1 : Enable interrupt on Subnormal truncation"]
    #[inline(always)]
    pub fn intr_uflow(&mut self) -> INTR_UFLOW_W {
        INTR_UFLOW_W { w: self }
    }
    #[doc = "Bit 4 - 1: Enable interrupt on AHBM Buss Error"]
    #[inline(always)]
    pub fn intr_berr(&mut self) -> INTR_BERR_W {
        INTR_BERR_W { w: self }
    }
    #[doc = "Bit 7 - 1: Enable interrupt on instruction completion"]
    #[inline(always)]
    pub fn intr_comp(&mut self) -> INTR_COMP_W {
        INTR_COMP_W { w: self }
    }
}
