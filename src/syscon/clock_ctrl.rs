#[doc = "Reader of register CLOCK_CTRL"]
pub type R = crate::R<u32, super::CLOCK_CTRL>;
#[doc = "Writer for register CLOCK_CTRL"]
pub type W = crate::W<u32, super::CLOCK_CTRL>;
#[doc = "Register CLOCK_CTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CLOCK_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Enable XTAL32MHz clock for Frequency Measure module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL32MHZ_FREQM_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<XTAL32MHZ_FREQM_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL32MHZ_FREQM_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTAL32MHZ_FREQM_ENA`"]
pub type XTAL32MHZ_FREQM_ENA_R = crate::R<bool, XTAL32MHZ_FREQM_ENA_A>;
impl XTAL32MHZ_FREQM_ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTAL32MHZ_FREQM_ENA_A {
        match self.bits {
            false => XTAL32MHZ_FREQM_ENA_A::DISABLE,
            true => XTAL32MHZ_FREQM_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == XTAL32MHZ_FREQM_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == XTAL32MHZ_FREQM_ENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `XTAL32MHZ_FREQM_ENA`"]
pub struct XTAL32MHZ_FREQM_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32MHZ_FREQM_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL32MHZ_FREQM_ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(XTAL32MHZ_FREQM_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(XTAL32MHZ_FREQM_ENA_A::ENABLE)
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
#[doc = "Enable FRO 1MHz clock for Frequency Measure module and for UTICK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO1MHZ_UTICK_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<FRO1MHZ_UTICK_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: FRO1MHZ_UTICK_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRO1MHZ_UTICK_ENA`"]
pub type FRO1MHZ_UTICK_ENA_R = crate::R<bool, FRO1MHZ_UTICK_ENA_A>;
impl FRO1MHZ_UTICK_ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO1MHZ_UTICK_ENA_A {
        match self.bits {
            false => FRO1MHZ_UTICK_ENA_A::DISABLE,
            true => FRO1MHZ_UTICK_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRO1MHZ_UTICK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRO1MHZ_UTICK_ENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `FRO1MHZ_UTICK_ENA`"]
pub struct FRO1MHZ_UTICK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO1MHZ_UTICK_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRO1MHZ_UTICK_ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRO1MHZ_UTICK_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRO1MHZ_UTICK_ENA_A::ENABLE)
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
#[doc = "Enable FRO 12MHz clock for Frequency Measure module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO12MHZ_FREQM_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<FRO12MHZ_FREQM_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: FRO12MHZ_FREQM_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRO12MHZ_FREQM_ENA`"]
pub type FRO12MHZ_FREQM_ENA_R = crate::R<bool, FRO12MHZ_FREQM_ENA_A>;
impl FRO12MHZ_FREQM_ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO12MHZ_FREQM_ENA_A {
        match self.bits {
            false => FRO12MHZ_FREQM_ENA_A::DISABLE,
            true => FRO12MHZ_FREQM_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRO12MHZ_FREQM_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRO12MHZ_FREQM_ENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `FRO12MHZ_FREQM_ENA`"]
pub struct FRO12MHZ_FREQM_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO12MHZ_FREQM_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRO12MHZ_FREQM_ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRO12MHZ_FREQM_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRO12MHZ_FREQM_ENA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Enable FRO 96MHz clock for Frequency Measure module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO_HF_FREQM_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<FRO_HF_FREQM_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: FRO_HF_FREQM_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRO_HF_FREQM_ENA`"]
pub type FRO_HF_FREQM_ENA_R = crate::R<bool, FRO_HF_FREQM_ENA_A>;
impl FRO_HF_FREQM_ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO_HF_FREQM_ENA_A {
        match self.bits {
            false => FRO_HF_FREQM_ENA_A::DISABLE,
            true => FRO_HF_FREQM_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRO_HF_FREQM_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRO_HF_FREQM_ENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `FRO_HF_FREQM_ENA`"]
pub struct FRO_HF_FREQM_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO_HF_FREQM_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRO_HF_FREQM_ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRO_HF_FREQM_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRO_HF_FREQM_ENA_A::ENABLE)
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
#[doc = "Enable clock_in clock for clock module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKIN_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<CLKIN_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: CLKIN_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKIN_ENA`"]
pub type CLKIN_ENA_R = crate::R<bool, CLKIN_ENA_A>;
impl CLKIN_ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKIN_ENA_A {
        match self.bits {
            false => CLKIN_ENA_A::DISABLE,
            true => CLKIN_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLKIN_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLKIN_ENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `CLKIN_ENA`"]
pub struct CLKIN_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKIN_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKIN_ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLKIN_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLKIN_ENA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Enable FRO 1MHz clock for clock muxing in clock gen.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO1MHZ_CLK_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<FRO1MHZ_CLK_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: FRO1MHZ_CLK_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRO1MHZ_CLK_ENA`"]
pub type FRO1MHZ_CLK_ENA_R = crate::R<bool, FRO1MHZ_CLK_ENA_A>;
impl FRO1MHZ_CLK_ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO1MHZ_CLK_ENA_A {
        match self.bits {
            false => FRO1MHZ_CLK_ENA_A::DISABLE,
            true => FRO1MHZ_CLK_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRO1MHZ_CLK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRO1MHZ_CLK_ENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `FRO1MHZ_CLK_ENA`"]
pub struct FRO1MHZ_CLK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO1MHZ_CLK_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRO1MHZ_CLK_ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRO1MHZ_CLK_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRO1MHZ_CLK_ENA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable FRO 12MHz clock for analog control of the FRO 192MHz.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANA_FRO12M_CLK_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<ANA_FRO12M_CLK_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: ANA_FRO12M_CLK_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ANA_FRO12M_CLK_ENA`"]
pub type ANA_FRO12M_CLK_ENA_R = crate::R<bool, ANA_FRO12M_CLK_ENA_A>;
impl ANA_FRO12M_CLK_ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANA_FRO12M_CLK_ENA_A {
        match self.bits {
            false => ANA_FRO12M_CLK_ENA_A::DISABLE,
            true => ANA_FRO12M_CLK_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ANA_FRO12M_CLK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ANA_FRO12M_CLK_ENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `ANA_FRO12M_CLK_ENA`"]
pub struct ANA_FRO12M_CLK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA_FRO12M_CLK_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANA_FRO12M_CLK_ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ANA_FRO12M_CLK_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ANA_FRO12M_CLK_ENA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Enable clock for cristal oscilator calibration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XO_CAL_CLK_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<XO_CAL_CLK_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: XO_CAL_CLK_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XO_CAL_CLK_ENA`"]
pub type XO_CAL_CLK_ENA_R = crate::R<bool, XO_CAL_CLK_ENA_A>;
impl XO_CAL_CLK_ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XO_CAL_CLK_ENA_A {
        match self.bits {
            false => XO_CAL_CLK_ENA_A::DISABLE,
            true => XO_CAL_CLK_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == XO_CAL_CLK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == XO_CAL_CLK_ENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `XO_CAL_CLK_ENA`"]
pub struct XO_CAL_CLK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> XO_CAL_CLK_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XO_CAL_CLK_ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(XO_CAL_CLK_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(XO_CAL_CLK_ENA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLU_DEGLITCH_CLK_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<PLU_DEGLITCH_CLK_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: PLU_DEGLITCH_CLK_ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLU_DEGLITCH_CLK_ENA`"]
pub type PLU_DEGLITCH_CLK_ENA_R = crate::R<bool, PLU_DEGLITCH_CLK_ENA_A>;
impl PLU_DEGLITCH_CLK_ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLU_DEGLITCH_CLK_ENA_A {
        match self.bits {
            false => PLU_DEGLITCH_CLK_ENA_A::DISABLE,
            true => PLU_DEGLITCH_CLK_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLU_DEGLITCH_CLK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLU_DEGLITCH_CLK_ENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `PLU_DEGLITCH_CLK_ENA`"]
pub struct PLU_DEGLITCH_CLK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PLU_DEGLITCH_CLK_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLU_DEGLITCH_CLK_ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLU_DEGLITCH_CLK_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLU_DEGLITCH_CLK_ENA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enable XTAL32MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn xtal32mhz_freqm_ena(&self) -> XTAL32MHZ_FREQM_ENA_R {
        XTAL32MHZ_FREQM_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
    #[inline(always)]
    pub fn fro1mhz_utick_ena(&self) -> FRO1MHZ_UTICK_ENA_R {
        FRO1MHZ_UTICK_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable FRO 12MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn fro12mhz_freqm_ena(&self) -> FRO12MHZ_FREQM_ENA_R {
        FRO12MHZ_FREQM_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable FRO 96MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn fro_hf_freqm_ena(&self) -> FRO_HF_FREQM_ENA_R {
        FRO_HF_FREQM_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable clock_in clock for clock module."]
    #[inline(always)]
    pub fn clkin_ena(&self) -> CLKIN_ENA_R {
        CLKIN_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable FRO 1MHz clock for clock muxing in clock gen."]
    #[inline(always)]
    pub fn fro1mhz_clk_ena(&self) -> FRO1MHZ_CLK_ENA_R {
        FRO1MHZ_CLK_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
    #[inline(always)]
    pub fn ana_fro12m_clk_ena(&self) -> ANA_FRO12M_CLK_ENA_R {
        ANA_FRO12M_CLK_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable clock for cristal oscilator calibration."]
    #[inline(always)]
    pub fn xo_cal_clk_ena(&self) -> XO_CAL_CLK_ENA_R {
        XO_CAL_CLK_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[inline(always)]
    pub fn plu_deglitch_clk_ena(&self) -> PLU_DEGLITCH_CLK_ENA_R {
        PLU_DEGLITCH_CLK_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable XTAL32MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn xtal32mhz_freqm_ena(&mut self) -> XTAL32MHZ_FREQM_ENA_W {
        XTAL32MHZ_FREQM_ENA_W { w: self }
    }
    #[doc = "Bit 2 - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
    #[inline(always)]
    pub fn fro1mhz_utick_ena(&mut self) -> FRO1MHZ_UTICK_ENA_W {
        FRO1MHZ_UTICK_ENA_W { w: self }
    }
    #[doc = "Bit 3 - Enable FRO 12MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn fro12mhz_freqm_ena(&mut self) -> FRO12MHZ_FREQM_ENA_W {
        FRO12MHZ_FREQM_ENA_W { w: self }
    }
    #[doc = "Bit 4 - Enable FRO 96MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn fro_hf_freqm_ena(&mut self) -> FRO_HF_FREQM_ENA_W {
        FRO_HF_FREQM_ENA_W { w: self }
    }
    #[doc = "Bit 5 - Enable clock_in clock for clock module."]
    #[inline(always)]
    pub fn clkin_ena(&mut self) -> CLKIN_ENA_W {
        CLKIN_ENA_W { w: self }
    }
    #[doc = "Bit 6 - Enable FRO 1MHz clock for clock muxing in clock gen."]
    #[inline(always)]
    pub fn fro1mhz_clk_ena(&mut self) -> FRO1MHZ_CLK_ENA_W {
        FRO1MHZ_CLK_ENA_W { w: self }
    }
    #[doc = "Bit 7 - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
    #[inline(always)]
    pub fn ana_fro12m_clk_ena(&mut self) -> ANA_FRO12M_CLK_ENA_W {
        ANA_FRO12M_CLK_ENA_W { w: self }
    }
    #[doc = "Bit 8 - Enable clock for cristal oscilator calibration."]
    #[inline(always)]
    pub fn xo_cal_clk_ena(&mut self) -> XO_CAL_CLK_ENA_W {
        XO_CAL_CLK_ENA_W { w: self }
    }
    #[doc = "Bit 9 - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[inline(always)]
    pub fn plu_deglitch_clk_ena(&mut self) -> PLU_DEGLITCH_CLK_ENA_W {
        PLU_DEGLITCH_CLK_ENA_W { w: self }
    }
}
