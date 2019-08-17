#[doc = "Reader of register LDO_XO32M"]
pub type R = crate::R<u32, super::LDO_XO32M>;
#[doc = "Writer for register LDO_XO32M"]
pub type W = crate::W<u32, super::LDO_XO32M>;
#[doc = "Register LDO_XO32M `reset()`'s with value 0x03a0"]
impl crate::ResetValue for super::LDO_XO32M {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03a0
    }
}
#[doc = "Possible values of the field `BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "Disable bypass mode (for normal operations)."]
    DISABLE,
    #[doc = "Activate LDO bypass."]
    ENABLE,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        match variant {
            BYPASS_A::DISABLE => false,
            BYPASS_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, BYPASS_A>;
impl BYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLE,
            true => BYPASS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BYPASS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BYPASS_A::ENABLE
    }
}
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable bypass mode (for normal operations)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLE)
    }
    #[doc = "Activate LDO bypass."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `HIGHZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIGHZ_A {
    #[doc = "Output in High normal state."]
    NORMALMPEDANCE,
    #[doc = "Output in High Impedance state."]
    HIGHIMPEDANCE,
}
impl From<HIGHZ_A> for bool {
    #[inline(always)]
    fn from(variant: HIGHZ_A) -> Self {
        match variant {
            HIGHZ_A::NORMALMPEDANCE => false,
            HIGHZ_A::HIGHIMPEDANCE => true,
        }
    }
}
#[doc = "Reader of field `HIGHZ`"]
pub type HIGHZ_R = crate::R<bool, HIGHZ_A>;
impl HIGHZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIGHZ_A {
        match self.bits {
            false => HIGHZ_A::NORMALMPEDANCE,
            true => HIGHZ_A::HIGHIMPEDANCE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMALMPEDANCE`"]
    #[inline(always)]
    pub fn is_normalmpedance(&self) -> bool {
        *self == HIGHZ_A::NORMALMPEDANCE
    }
    #[doc = "Checks if the value of the field is `HIGHIMPEDANCE`"]
    #[inline(always)]
    pub fn is_highimpedance(&self) -> bool {
        *self == HIGHZ_A::HIGHIMPEDANCE
    }
}
#[doc = "Write proxy for field `HIGHZ`"]
pub struct HIGHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGHZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIGHZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output in High normal state."]
    #[inline(always)]
    pub fn normalmpedance(self) -> &'a mut W {
        self.variant(HIGHZ_A::NORMALMPEDANCE)
    }
    #[doc = "Output in High Impedance state."]
    #[inline(always)]
    pub fn highimpedance(self) -> &'a mut W {
        self.variant(HIGHZ_A::HIGHIMPEDANCE)
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
#[doc = "Possible values of the field `VOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOUT_A {
    #[doc = "0.750 V."]
    V_0P750,
    #[doc = "0.775 V."]
    V_0P775,
    #[doc = "0.800 V."]
    V_0P800,
    #[doc = "0.825 V."]
    V_0P825,
    #[doc = "0.850 V."]
    V_0P850,
    #[doc = "0.875 V."]
    V_0P875,
    #[doc = "0.900 V."]
    V_0P900,
    #[doc = "0.925 V."]
    V_0P925,
}
impl From<VOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: VOUT_A) -> Self {
        match variant {
            VOUT_A::V_0P750 => 0,
            VOUT_A::V_0P775 => 1,
            VOUT_A::V_0P800 => 2,
            VOUT_A::V_0P825 => 3,
            VOUT_A::V_0P850 => 4,
            VOUT_A::V_0P875 => 5,
            VOUT_A::V_0P900 => 6,
            VOUT_A::V_0P925 => 7,
        }
    }
}
#[doc = "Reader of field `VOUT`"]
pub type VOUT_R = crate::R<u8, VOUT_A>;
impl VOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOUT_A {
        match self.bits {
            0 => VOUT_A::V_0P750,
            1 => VOUT_A::V_0P775,
            2 => VOUT_A::V_0P800,
            3 => VOUT_A::V_0P825,
            4 => VOUT_A::V_0P850,
            5 => VOUT_A::V_0P875,
            6 => VOUT_A::V_0P900,
            7 => VOUT_A::V_0P925,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V_0P750`"]
    #[inline(always)]
    pub fn is_v_0p750(&self) -> bool {
        *self == VOUT_A::V_0P750
    }
    #[doc = "Checks if the value of the field is `V_0P775`"]
    #[inline(always)]
    pub fn is_v_0p775(&self) -> bool {
        *self == VOUT_A::V_0P775
    }
    #[doc = "Checks if the value of the field is `V_0P800`"]
    #[inline(always)]
    pub fn is_v_0p800(&self) -> bool {
        *self == VOUT_A::V_0P800
    }
    #[doc = "Checks if the value of the field is `V_0P825`"]
    #[inline(always)]
    pub fn is_v_0p825(&self) -> bool {
        *self == VOUT_A::V_0P825
    }
    #[doc = "Checks if the value of the field is `V_0P850`"]
    #[inline(always)]
    pub fn is_v_0p850(&self) -> bool {
        *self == VOUT_A::V_0P850
    }
    #[doc = "Checks if the value of the field is `V_0P875`"]
    #[inline(always)]
    pub fn is_v_0p875(&self) -> bool {
        *self == VOUT_A::V_0P875
    }
    #[doc = "Checks if the value of the field is `V_0P900`"]
    #[inline(always)]
    pub fn is_v_0p900(&self) -> bool {
        *self == VOUT_A::V_0P900
    }
    #[doc = "Checks if the value of the field is `V_0P925`"]
    #[inline(always)]
    pub fn is_v_0p925(&self) -> bool {
        *self == VOUT_A::V_0P925
    }
}
#[doc = "Write proxy for field `VOUT`"]
pub struct VOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> VOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VOUT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "0.750 V."]
    #[inline(always)]
    pub fn v_0p750(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P750)
    }
    #[doc = "0.775 V."]
    #[inline(always)]
    pub fn v_0p775(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P775)
    }
    #[doc = "0.800 V."]
    #[inline(always)]
    pub fn v_0p800(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P800)
    }
    #[doc = "0.825 V."]
    #[inline(always)]
    pub fn v_0p825(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P825)
    }
    #[doc = "0.850 V."]
    #[inline(always)]
    pub fn v_0p850(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P850)
    }
    #[doc = "0.875 V."]
    #[inline(always)]
    pub fn v_0p875(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P875)
    }
    #[doc = "0.900 V."]
    #[inline(always)]
    pub fn v_0p900(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P900)
    }
    #[doc = "0.925 V."]
    #[inline(always)]
    pub fn v_0p925(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P925)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `IBIAS`"]
pub type IBIAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBIAS`"]
pub struct IBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `STABMODE`"]
pub type STABMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STABMODE`"]
pub struct STABMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> STABMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Activate LDO bypass."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    pub fn highz(&self) -> HIGHZ_R {
        HIGHZ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Sets the LDO output level."]
    #[inline(always)]
    pub fn vout(&self) -> VOUT_R {
        VOUT_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Adjust the biasing current."]
    #[inline(always)]
    pub fn ibias(&self) -> IBIAS_R {
        IBIAS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Stability configuration."]
    #[inline(always)]
    pub fn stabmode(&self) -> STABMODE_R {
        STABMODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Activate LDO bypass."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    pub fn highz(&mut self) -> HIGHZ_W {
        HIGHZ_W { w: self }
    }
    #[doc = "Bits 3:5 - Sets the LDO output level."]
    #[inline(always)]
    pub fn vout(&mut self) -> VOUT_W {
        VOUT_W { w: self }
    }
    #[doc = "Bits 6:7 - Adjust the biasing current."]
    #[inline(always)]
    pub fn ibias(&mut self) -> IBIAS_W {
        IBIAS_W { w: self }
    }
    #[doc = "Bits 8:9 - Stability configuration."]
    #[inline(always)]
    pub fn stabmode(&mut self) -> STABMODE_W {
        STABMODE_W { w: self }
    }
}
