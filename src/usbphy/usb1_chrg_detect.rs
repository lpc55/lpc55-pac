#[doc = "Reader of register USB1_CHRG_DETECT"]
pub type R = crate::R<u32, super::USB1_CHRG_DETECT>;
#[doc = "Writer for register USB1_CHRG_DETECT"]
pub type W = crate::W<u32, super::USB1_CHRG_DETECT>;
#[doc = "Register USB1_CHRG_DETECT `reset()`'s with value 0x8018_0000"]
impl crate::ResetValue for super::USB1_CHRG_DETECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8018_0000
    }
}
#[doc = "Reader of field `PULLUP_DP`"]
pub type PULLUP_DP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PULLUP_DP`"]
pub struct PULLUP_DP_W<'a> {
    w: &'a mut W,
}
impl<'a> PULLUP_DP_W<'a> {
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
#[doc = "Possible values of the field `BGR_IBIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGR_IBIAS_A {
    #[doc = "Bias current is derived from the USB PHY internal current generator."]
    VALUE0,
    #[doc = "Bias current is derived from the reference generator of the bandgap."]
    VALUE1,
}
impl From<BGR_IBIAS_A> for bool {
    #[inline(always)]
    fn from(variant: BGR_IBIAS_A) -> Self {
        match variant {
            BGR_IBIAS_A::VALUE0 => false,
            BGR_IBIAS_A::VALUE1 => true,
        }
    }
}
#[doc = "Reader of field `BGR_IBIAS`"]
pub type BGR_IBIAS_R = crate::R<bool, BGR_IBIAS_A>;
impl BGR_IBIAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGR_IBIAS_A {
        match self.bits {
            false => BGR_IBIAS_A::VALUE0,
            true => BGR_IBIAS_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == BGR_IBIAS_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BGR_IBIAS_A::VALUE1
    }
}
#[doc = "Write proxy for field `BGR_IBIAS`"]
pub struct BGR_IBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> BGR_IBIAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BGR_IBIAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bias current is derived from the USB PHY internal current generator."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(BGR_IBIAS_A::VALUE0)
    }
    #[doc = "Bias current is derived from the reference generator of the bandgap."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BGR_IBIAS_A::VALUE1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - This bit is used to pull up DP, for digital charge detect."]
    #[inline(always)]
    pub fn pullup_dp(&self) -> PULLUP_DP_R {
        PULLUP_DP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 23 - USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[inline(always)]
    pub fn bgr_ibias(&self) -> BGR_IBIAS_R {
        BGR_IBIAS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit is used to pull up DP, for digital charge detect."]
    #[inline(always)]
    pub fn pullup_dp(&mut self) -> PULLUP_DP_W {
        PULLUP_DP_W { w: self }
    }
    #[doc = "Bit 23 - USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[inline(always)]
    pub fn bgr_ibias(&mut self) -> BGR_IBIAS_W {
        BGR_IBIAS_W { w: self }
    }
}
