#[doc = "Reader of register ADC_CTRL"]
pub type R = crate::R<u32, super::ADC_CTRL>;
#[doc = "Writer for register ADC_CTRL"]
pub type W = crate::W<u32, super::ADC_CTRL>;
#[doc = "Register ADC_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `VBATDIVENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATDIVENABLE_A {
    #[doc = "VBAT divider branch is disabled."]
    DISABLE,
    #[doc = "VBAT divider branch is enabled."]
    ENABLE,
}
impl From<VBATDIVENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: VBATDIVENABLE_A) -> Self {
        match variant {
            VBATDIVENABLE_A::DISABLE => false,
            VBATDIVENABLE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `VBATDIVENABLE`"]
pub type VBATDIVENABLE_R = crate::R<bool, VBATDIVENABLE_A>;
impl VBATDIVENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATDIVENABLE_A {
        match self.bits {
            false => VBATDIVENABLE_A::DISABLE,
            true => VBATDIVENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VBATDIVENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VBATDIVENABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `VBATDIVENABLE`"]
pub struct VBATDIVENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATDIVENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBATDIVENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VBAT divider branch is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VBATDIVENABLE_A::DISABLE)
    }
    #[doc = "VBAT divider branch is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VBATDIVENABLE_A::ENABLE)
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
impl R {
    #[doc = "Bit 0 - Switch On/Off VBAT divider branch."]
    #[inline(always)]
    pub fn vbatdivenable(&self) -> VBATDIVENABLE_R {
        VBATDIVENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Switch On/Off VBAT divider branch."]
    #[inline(always)]
    pub fn vbatdivenable(&mut self) -> VBATDIVENABLE_W {
        VBATDIVENABLE_W { w: self }
    }
}
