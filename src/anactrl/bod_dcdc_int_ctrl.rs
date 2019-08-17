#[doc = "Reader of register BOD_DCDC_INT_CTRL"]
pub type R = crate::R<u32, super::BOD_DCDC_INT_CTRL>;
#[doc = "Writer for register BOD_DCDC_INT_CTRL"]
pub type W = crate::W<u32, super::BOD_DCDC_INT_CTRL>;
#[doc = "Register BOD_DCDC_INT_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BOD_DCDC_INT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `BODVBAT_INT_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBAT_INT_ENABLE_A {
    #[doc = "BOD VBAT interrupt is disabled."]
    DISABLE,
    #[doc = "BOD VBAT interrupt is enabled."]
    ENABLE,
}
impl From<BODVBAT_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BODVBAT_INT_ENABLE_A) -> Self {
        match variant {
            BODVBAT_INT_ENABLE_A::DISABLE => false,
            BODVBAT_INT_ENABLE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `BODVBAT_INT_ENABLE`"]
pub type BODVBAT_INT_ENABLE_R = crate::R<bool, BODVBAT_INT_ENABLE_A>;
impl BODVBAT_INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODVBAT_INT_ENABLE_A {
        match self.bits {
            false => BODVBAT_INT_ENABLE_A::DISABLE,
            true => BODVBAT_INT_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODVBAT_INT_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODVBAT_INT_ENABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `BODVBAT_INT_ENABLE`"]
pub struct BODVBAT_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BODVBAT_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODVBAT_INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BOD VBAT interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODVBAT_INT_ENABLE_A::DISABLE)
    }
    #[doc = "BOD VBAT interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODVBAT_INT_ENABLE_A::ENABLE)
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
#[doc = "Reader of field `BODVBAT_INT_CLEAR`"]
pub type BODVBAT_INT_CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODVBAT_INT_CLEAR`"]
pub struct BODVBAT_INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> BODVBAT_INT_CLEAR_W<'a> {
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
#[doc = "Possible values of the field `BODCORE_INT_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODCORE_INT_ENABLE_A {
    #[doc = "BOD CORE interrupt is disabled."]
    DISABLE,
    #[doc = "BOD CORE interrupt is enabled."]
    ENABLE,
}
impl From<BODCORE_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BODCORE_INT_ENABLE_A) -> Self {
        match variant {
            BODCORE_INT_ENABLE_A::DISABLE => false,
            BODCORE_INT_ENABLE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `BODCORE_INT_ENABLE`"]
pub type BODCORE_INT_ENABLE_R = crate::R<bool, BODCORE_INT_ENABLE_A>;
impl BODCORE_INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODCORE_INT_ENABLE_A {
        match self.bits {
            false => BODCORE_INT_ENABLE_A::DISABLE,
            true => BODCORE_INT_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODCORE_INT_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODCORE_INT_ENABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `BODCORE_INT_ENABLE`"]
pub struct BODCORE_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BODCORE_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODCORE_INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BOD CORE interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODCORE_INT_ENABLE_A::DISABLE)
    }
    #[doc = "BOD CORE interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODCORE_INT_ENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `BODCORE_INT_CLEAR`"]
pub type BODCORE_INT_CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODCORE_INT_CLEAR`"]
pub struct BODCORE_INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> BODCORE_INT_CLEAR_W<'a> {
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
#[doc = "Possible values of the field `DCDC_INT_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_INT_ENABLE_A {
    #[doc = "DCDC interrupt is disabled."]
    DISABLE,
    #[doc = "DCDC interrupt is enabled."]
    ENABLE,
}
impl From<DCDC_INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_INT_ENABLE_A) -> Self {
        match variant {
            DCDC_INT_ENABLE_A::DISABLE => false,
            DCDC_INT_ENABLE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `DCDC_INT_ENABLE`"]
pub type DCDC_INT_ENABLE_R = crate::R<bool, DCDC_INT_ENABLE_A>;
impl DCDC_INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_INT_ENABLE_A {
        match self.bits {
            false => DCDC_INT_ENABLE_A::DISABLE,
            true => DCDC_INT_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DCDC_INT_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DCDC_INT_ENABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `DCDC_INT_ENABLE`"]
pub struct DCDC_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDC_INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DCDC interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DCDC_INT_ENABLE_A::DISABLE)
    }
    #[doc = "DCDC interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DCDC_INT_ENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DCDC_INT_CLEAR`"]
pub type DCDC_INT_CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDC_INT_CLEAR`"]
pub struct DCDC_INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_INT_CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BOD VBAT interrupt control."]
    #[inline(always)]
    pub fn bodvbat_int_enable(&self) -> BODVBAT_INT_ENABLE_R {
        BODVBAT_INT_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodvbat_int_clear(&self) -> BODVBAT_INT_CLEAR_R {
        BODVBAT_INT_CLEAR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BOD CORE interrupt control."]
    #[inline(always)]
    pub fn bodcore_int_enable(&self) -> BODCORE_INT_ENABLE_R {
        BODCORE_INT_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodcore_int_clear(&self) -> BODCORE_INT_CLEAR_R {
        BODCORE_INT_CLEAR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DCDC interrupt control."]
    #[inline(always)]
    pub fn dcdc_int_enable(&self) -> DCDC_INT_ENABLE_R {
        DCDC_INT_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn dcdc_int_clear(&self) -> DCDC_INT_CLEAR_R {
        DCDC_INT_CLEAR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOD VBAT interrupt control."]
    #[inline(always)]
    pub fn bodvbat_int_enable(&mut self) -> BODVBAT_INT_ENABLE_W {
        BODVBAT_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodvbat_int_clear(&mut self) -> BODVBAT_INT_CLEAR_W {
        BODVBAT_INT_CLEAR_W { w: self }
    }
    #[doc = "Bit 2 - BOD CORE interrupt control."]
    #[inline(always)]
    pub fn bodcore_int_enable(&mut self) -> BODCORE_INT_ENABLE_W {
        BODCORE_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 3 - BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn bodcore_int_clear(&mut self) -> BODCORE_INT_CLEAR_W {
        BODCORE_INT_CLEAR_W { w: self }
    }
    #[doc = "Bit 4 - DCDC interrupt control."]
    #[inline(always)]
    pub fn dcdc_int_enable(&mut self) -> DCDC_INT_ENABLE_W {
        DCDC_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn dcdc_int_clear(&mut self) -> DCDC_INT_CLEAR_W {
        DCDC_INT_CLEAR_W { w: self }
    }
}
