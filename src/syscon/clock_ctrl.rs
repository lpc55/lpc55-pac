#[doc = "Register `CLOCK_CTRL` reader"]
pub struct R(crate::R<CLOCK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLOCK_CTRL_SPEC>> for R {
    fn from(reader: crate::R<CLOCK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_CTRL` writer"]
pub struct W(crate::W<CLOCK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<CLOCK_CTRL_SPEC>> for W {
    fn from(writer: crate::W<CLOCK_CTRL_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `XTAL32MHZ_FREQM_ENA` reader - Enable XTAL32MHz clock for Frequency Measure module."]
pub struct XTAL32MHZ_FREQM_ENA_R(crate::FieldReader<bool, XTAL32MHZ_FREQM_ENA_A>);
impl XTAL32MHZ_FREQM_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32MHZ_FREQM_ENA_R(crate::FieldReader::new(bits))
    }
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
        **self == XTAL32MHZ_FREQM_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == XTAL32MHZ_FREQM_ENA_A::ENABLE
    }
}
impl core::ops::Deref for XTAL32MHZ_FREQM_ENA_R {
    type Target = crate::FieldReader<bool, XTAL32MHZ_FREQM_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32MHZ_FREQM_ENA` writer - Enable XTAL32MHz clock for Frequency Measure module."]
pub struct XTAL32MHZ_FREQM_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32MHZ_FREQM_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTAL32MHZ_FREQM_ENA_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
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
#[doc = "Field `FRO1MHZ_UTICK_ENA` reader - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
pub struct FRO1MHZ_UTICK_ENA_R(crate::FieldReader<bool, FRO1MHZ_UTICK_ENA_A>);
impl FRO1MHZ_UTICK_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRO1MHZ_UTICK_ENA_R(crate::FieldReader::new(bits))
    }
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
        **self == FRO1MHZ_UTICK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FRO1MHZ_UTICK_ENA_A::ENABLE
    }
}
impl core::ops::Deref for FRO1MHZ_UTICK_ENA_R {
    type Target = crate::FieldReader<bool, FRO1MHZ_UTICK_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRO1MHZ_UTICK_ENA` writer - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
pub struct FRO1MHZ_UTICK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO1MHZ_UTICK_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRO1MHZ_UTICK_ENA_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
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
#[doc = "Field `FRO12MHZ_FREQM_ENA` reader - Enable FRO 12MHz clock for Frequency Measure module."]
pub struct FRO12MHZ_FREQM_ENA_R(crate::FieldReader<bool, FRO12MHZ_FREQM_ENA_A>);
impl FRO12MHZ_FREQM_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRO12MHZ_FREQM_ENA_R(crate::FieldReader::new(bits))
    }
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
        **self == FRO12MHZ_FREQM_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FRO12MHZ_FREQM_ENA_A::ENABLE
    }
}
impl core::ops::Deref for FRO12MHZ_FREQM_ENA_R {
    type Target = crate::FieldReader<bool, FRO12MHZ_FREQM_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRO12MHZ_FREQM_ENA` writer - Enable FRO 12MHz clock for Frequency Measure module."]
pub struct FRO12MHZ_FREQM_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO12MHZ_FREQM_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRO12MHZ_FREQM_ENA_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
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
#[doc = "Field `FRO_HF_FREQM_ENA` reader - Enable FRO 96MHz clock for Frequency Measure module."]
pub struct FRO_HF_FREQM_ENA_R(crate::FieldReader<bool, FRO_HF_FREQM_ENA_A>);
impl FRO_HF_FREQM_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRO_HF_FREQM_ENA_R(crate::FieldReader::new(bits))
    }
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
        **self == FRO_HF_FREQM_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FRO_HF_FREQM_ENA_A::ENABLE
    }
}
impl core::ops::Deref for FRO_HF_FREQM_ENA_R {
    type Target = crate::FieldReader<bool, FRO_HF_FREQM_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRO_HF_FREQM_ENA` writer - Enable FRO 96MHz clock for Frequency Measure module."]
pub struct FRO_HF_FREQM_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO_HF_FREQM_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRO_HF_FREQM_ENA_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
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
#[doc = "Field `CLKIN_ENA` reader - Enable clock_in clock for clock module."]
pub struct CLKIN_ENA_R(crate::FieldReader<bool, CLKIN_ENA_A>);
impl CLKIN_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKIN_ENA_R(crate::FieldReader::new(bits))
    }
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
        **self == CLKIN_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CLKIN_ENA_A::ENABLE
    }
}
impl core::ops::Deref for CLKIN_ENA_R {
    type Target = crate::FieldReader<bool, CLKIN_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKIN_ENA` writer - Enable clock_in clock for clock module."]
pub struct CLKIN_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKIN_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKIN_ENA_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
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
#[doc = "Field `FRO1MHZ_CLK_ENA` reader - Enable FRO 1MHz clock for clock muxing in clock gen."]
pub struct FRO1MHZ_CLK_ENA_R(crate::FieldReader<bool, FRO1MHZ_CLK_ENA_A>);
impl FRO1MHZ_CLK_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRO1MHZ_CLK_ENA_R(crate::FieldReader::new(bits))
    }
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
        **self == FRO1MHZ_CLK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FRO1MHZ_CLK_ENA_A::ENABLE
    }
}
impl core::ops::Deref for FRO1MHZ_CLK_ENA_R {
    type Target = crate::FieldReader<bool, FRO1MHZ_CLK_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRO1MHZ_CLK_ENA` writer - Enable FRO 1MHz clock for clock muxing in clock gen."]
pub struct FRO1MHZ_CLK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO1MHZ_CLK_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRO1MHZ_CLK_ENA_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
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
#[doc = "Field `ANA_FRO12M_CLK_ENA` reader - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
pub struct ANA_FRO12M_CLK_ENA_R(crate::FieldReader<bool, ANA_FRO12M_CLK_ENA_A>);
impl ANA_FRO12M_CLK_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANA_FRO12M_CLK_ENA_R(crate::FieldReader::new(bits))
    }
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
        **self == ANA_FRO12M_CLK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ANA_FRO12M_CLK_ENA_A::ENABLE
    }
}
impl core::ops::Deref for ANA_FRO12M_CLK_ENA_R {
    type Target = crate::FieldReader<bool, ANA_FRO12M_CLK_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANA_FRO12M_CLK_ENA` writer - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
pub struct ANA_FRO12M_CLK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA_FRO12M_CLK_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANA_FRO12M_CLK_ENA_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
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
#[doc = "Field `XO_CAL_CLK_ENA` reader - Enable clock for cristal oscilator calibration."]
pub struct XO_CAL_CLK_ENA_R(crate::FieldReader<bool, XO_CAL_CLK_ENA_A>);
impl XO_CAL_CLK_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        XO_CAL_CLK_ENA_R(crate::FieldReader::new(bits))
    }
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
        **self == XO_CAL_CLK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == XO_CAL_CLK_ENA_A::ENABLE
    }
}
impl core::ops::Deref for XO_CAL_CLK_ENA_R {
    type Target = crate::FieldReader<bool, XO_CAL_CLK_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XO_CAL_CLK_ENA` writer - Enable clock for cristal oscilator calibration."]
pub struct XO_CAL_CLK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> XO_CAL_CLK_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XO_CAL_CLK_ENA_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
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
#[doc = "Field `PLU_DEGLITCH_CLK_ENA` reader - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
pub struct PLU_DEGLITCH_CLK_ENA_R(crate::FieldReader<bool, PLU_DEGLITCH_CLK_ENA_A>);
impl PLU_DEGLITCH_CLK_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLU_DEGLITCH_CLK_ENA_R(crate::FieldReader::new(bits))
    }
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
        **self == PLU_DEGLITCH_CLK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PLU_DEGLITCH_CLK_ENA_A::ENABLE
    }
}
impl core::ops::Deref for PLU_DEGLITCH_CLK_ENA_R {
    type Target = crate::FieldReader<bool, PLU_DEGLITCH_CLK_ENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLU_DEGLITCH_CLK_ENA` writer - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
pub struct PLU_DEGLITCH_CLK_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PLU_DEGLITCH_CLK_ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLU_DEGLITCH_CLK_ENA_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_ctrl](index.html) module"]
pub struct CLOCK_CTRL_SPEC;
impl crate::RegisterSpec for CLOCK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_ctrl::R](R) reader structure"]
impl crate::Readable for CLOCK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_ctrl::W](W) writer structure"]
impl crate::Writable for CLOCK_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCK_CTRL to value 0x01"]
impl crate::Resettable for CLOCK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
