#[doc = "Register `AHBCLKCTRL0` reader"]
pub struct R(crate::R<AHBCLKCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKCTRL0` writer"]
pub struct W(crate::W<AHBCLKCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRL0_SPEC>;
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
impl From<crate::W<AHBCLKCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enables the clock for the ROM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROM_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<ROM_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM` reader - Enables the clock for the ROM."]
pub struct ROM_R(crate::FieldReader<bool, ROM_A>);
impl ROM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_A {
        match self.bits {
            false => ROM_A::DISABLE,
            true => ROM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ROM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ROM_A::ENABLE
    }
}
impl core::ops::Deref for ROM_R {
    type Target = crate::FieldReader<bool, ROM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM` writer - Enables the clock for the ROM."]
pub struct ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROM_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROM_A::ENABLE)
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
#[doc = "Enables the clock for the SRAM Controller 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL1_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SRAM_CTRL1_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_CTRL1` reader - Enables the clock for the SRAM Controller 1."]
pub struct SRAM_CTRL1_R(crate::FieldReader<bool, SRAM_CTRL1_A>);
impl SRAM_CTRL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_CTRL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL1_A {
        match self.bits {
            false => SRAM_CTRL1_A::DISABLE,
            true => SRAM_CTRL1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SRAM_CTRL1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SRAM_CTRL1_A::ENABLE
    }
}
impl core::ops::Deref for SRAM_CTRL1_R {
    type Target = crate::FieldReader<bool, SRAM_CTRL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_CTRL1` writer - Enables the clock for the SRAM Controller 1."]
pub struct SRAM_CTRL1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CTRL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CTRL1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL1_A::ENABLE)
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
#[doc = "Enables the clock for the SRAM Controller 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL2_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SRAM_CTRL2_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_CTRL2` reader - Enables the clock for the SRAM Controller 2."]
pub struct SRAM_CTRL2_R(crate::FieldReader<bool, SRAM_CTRL2_A>);
impl SRAM_CTRL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_CTRL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL2_A {
        match self.bits {
            false => SRAM_CTRL2_A::DISABLE,
            true => SRAM_CTRL2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SRAM_CTRL2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SRAM_CTRL2_A::ENABLE
    }
}
impl core::ops::Deref for SRAM_CTRL2_R {
    type Target = crate::FieldReader<bool, SRAM_CTRL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_CTRL2` writer - Enables the clock for the SRAM Controller 2."]
pub struct SRAM_CTRL2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CTRL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CTRL2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL2_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL2_A::ENABLE)
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
#[doc = "Enables the clock for the SRAM Controller 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL3_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SRAM_CTRL3_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_CTRL3` reader - Enables the clock for the SRAM Controller 3."]
pub struct SRAM_CTRL3_R(crate::FieldReader<bool, SRAM_CTRL3_A>);
impl SRAM_CTRL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_CTRL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL3_A {
        match self.bits {
            false => SRAM_CTRL3_A::DISABLE,
            true => SRAM_CTRL3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SRAM_CTRL3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SRAM_CTRL3_A::ENABLE
    }
}
impl core::ops::Deref for SRAM_CTRL3_R {
    type Target = crate::FieldReader<bool, SRAM_CTRL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_CTRL3` writer - Enables the clock for the SRAM Controller 3."]
pub struct SRAM_CTRL3_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CTRL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CTRL3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_A::ENABLE)
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
#[doc = "Enables the clock for the SRAM Controller 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL4_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SRAM_CTRL4_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_CTRL4` reader - Enables the clock for the SRAM Controller 4."]
pub struct SRAM_CTRL4_R(crate::FieldReader<bool, SRAM_CTRL4_A>);
impl SRAM_CTRL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_CTRL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL4_A {
        match self.bits {
            false => SRAM_CTRL4_A::DISABLE,
            true => SRAM_CTRL4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SRAM_CTRL4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SRAM_CTRL4_A::ENABLE
    }
}
impl core::ops::Deref for SRAM_CTRL4_R {
    type Target = crate::FieldReader<bool, SRAM_CTRL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM_CTRL4` writer - Enables the clock for the SRAM Controller 4."]
pub struct SRAM_CTRL4_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CTRL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CTRL4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL4_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL4_A::ENABLE)
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
#[doc = "Enables the clock for the Flash controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FLASH_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH` reader - Enables the clock for the Flash controller."]
pub struct FLASH_R(crate::FieldReader<bool, FLASH_A>);
impl FLASH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_A {
        match self.bits {
            false => FLASH_A::DISABLE,
            true => FLASH_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FLASH_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FLASH_A::ENABLE
    }
}
impl core::ops::Deref for FLASH_R {
    type Target = crate::FieldReader<bool, FLASH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH` writer - Enables the clock for the Flash controller."]
pub struct FLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLASH_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLASH_A::ENABLE)
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
#[doc = "Enables the clock for the FMC controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMC_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FMC_A> for bool {
    #[inline(always)]
    fn from(variant: FMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMC` reader - Enables the clock for the FMC controller."]
pub struct FMC_R(crate::FieldReader<bool, FMC_A>);
impl FMC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FMC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMC_A {
        match self.bits {
            false => FMC_A::DISABLE,
            true => FMC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FMC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FMC_A::ENABLE
    }
}
impl core::ops::Deref for FMC_R {
    type Target = crate::FieldReader<bool, FMC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMC` writer - Enables the clock for the FMC controller."]
pub struct FMC_W<'a> {
    w: &'a mut W,
}
impl<'a> FMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FMC_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FMC_A::ENABLE)
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
#[doc = "Enables the clock for the Input Mux.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<MUX_A> for bool {
    #[inline(always)]
    fn from(variant: MUX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUX` reader - Enables the clock for the Input Mux."]
pub struct MUX_R(crate::FieldReader<bool, MUX_A>);
impl MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MUX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_A {
        match self.bits {
            false => MUX_A::DISABLE,
            true => MUX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == MUX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == MUX_A::ENABLE
    }
}
impl core::ops::Deref for MUX_R {
    type Target = crate::FieldReader<bool, MUX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX` writer - Enables the clock for the Input Mux."]
pub struct MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MUX_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MUX_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Enables the clock for the I/O controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<IOCON_A> for bool {
    #[inline(always)]
    fn from(variant: IOCON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCON` reader - Enables the clock for the I/O controller."]
pub struct IOCON_R(crate::FieldReader<bool, IOCON_A>);
impl IOCON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IOCON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_A {
        match self.bits {
            false => IOCON_A::DISABLE,
            true => IOCON_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IOCON_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IOCON_A::ENABLE
    }
}
impl core::ops::Deref for IOCON_R {
    type Target = crate::FieldReader<bool, IOCON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOCON` writer - Enables the clock for the I/O controller."]
pub struct IOCON_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IOCON_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IOCON_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Enables the clock for the GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GPIO0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO0` reader - Enables the clock for the GPIO0."]
pub struct GPIO0_R(crate::FieldReader<bool, GPIO0_A>);
impl GPIO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0_A {
        match self.bits {
            false => GPIO0_A::DISABLE,
            true => GPIO0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == GPIO0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == GPIO0_A::ENABLE
    }
}
impl core::ops::Deref for GPIO0_R {
    type Target = crate::FieldReader<bool, GPIO0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO0` writer - Enables the clock for the GPIO0."]
pub struct GPIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Enables the clock for the GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GPIO1_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO1` reader - Enables the clock for the GPIO1."]
pub struct GPIO1_R(crate::FieldReader<bool, GPIO1_A>);
impl GPIO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1_A {
        match self.bits {
            false => GPIO1_A::DISABLE,
            true => GPIO1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == GPIO1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == GPIO1_A::ENABLE
    }
}
impl core::ops::Deref for GPIO1_R {
    type Target = crate::FieldReader<bool, GPIO1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO1` writer - Enables the clock for the GPIO1."]
pub struct GPIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Enables the clock for the GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GPIO2_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO2` reader - Enables the clock for the GPIO2."]
pub struct GPIO2_R(crate::FieldReader<bool, GPIO2_A>);
impl GPIO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2_A {
        match self.bits {
            false => GPIO2_A::DISABLE,
            true => GPIO2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == GPIO2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == GPIO2_A::ENABLE
    }
}
impl core::ops::Deref for GPIO2_R {
    type Target = crate::FieldReader<bool, GPIO2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO2` writer - Enables the clock for the GPIO2."]
pub struct GPIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO2_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO2_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Enables the clock for the GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GPIO3_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO3` reader - Enables the clock for the GPIO3."]
pub struct GPIO3_R(crate::FieldReader<bool, GPIO3_A>);
impl GPIO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3_A {
        match self.bits {
            false => GPIO3_A::DISABLE,
            true => GPIO3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == GPIO3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == GPIO3_A::ENABLE
    }
}
impl core::ops::Deref for GPIO3_R {
    type Target = crate::FieldReader<bool, GPIO3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO3` writer - Enables the clock for the GPIO3."]
pub struct GPIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO3_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO3_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Enables the clock for the Pin interrupt (PINT).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<PINT_A> for bool {
    #[inline(always)]
    fn from(variant: PINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINT` reader - Enables the clock for the Pin interrupt (PINT)."]
pub struct PINT_R(crate::FieldReader<bool, PINT_A>);
impl PINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT_A {
        match self.bits {
            false => PINT_A::DISABLE,
            true => PINT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PINT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PINT_A::ENABLE
    }
}
impl core::ops::Deref for PINT_R {
    type Target = crate::FieldReader<bool, PINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINT` writer - Enables the clock for the Pin interrupt (PINT)."]
pub struct PINT_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PINT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PINT_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Enables the clock for the Group interrupt (GINT).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GINT_A> for bool {
    #[inline(always)]
    fn from(variant: GINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GINT` reader - Enables the clock for the Group interrupt (GINT)."]
pub struct GINT_R(crate::FieldReader<bool, GINT_A>);
impl GINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GINT_A {
        match self.bits {
            false => GINT_A::DISABLE,
            true => GINT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == GINT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == GINT_A::ENABLE
    }
}
impl core::ops::Deref for GINT_R {
    type Target = crate::FieldReader<bool, GINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINT` writer - Enables the clock for the Group interrupt (GINT)."]
pub struct GINT_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GINT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GINT_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Enables the clock for the DMA0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<DMA0_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0` reader - Enables the clock for the DMA0."]
pub struct DMA0_R(crate::FieldReader<bool, DMA0_A>);
impl DMA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0_A {
        match self.bits {
            false => DMA0_A::DISABLE,
            true => DMA0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DMA0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DMA0_A::ENABLE
    }
}
impl core::ops::Deref for DMA0_R {
    type Target = crate::FieldReader<bool, DMA0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA0` writer - Enables the clock for the DMA0."]
pub struct DMA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Enables the clock for the CRCGEN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCGEN_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<CRCGEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCGEN` reader - Enables the clock for the CRCGEN."]
pub struct CRCGEN_R(crate::FieldReader<bool, CRCGEN_A>);
impl CRCGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRCGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCGEN_A {
        match self.bits {
            false => CRCGEN_A::DISABLE,
            true => CRCGEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CRCGEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CRCGEN_A::ENABLE
    }
}
impl core::ops::Deref for CRCGEN_R {
    type Target = crate::FieldReader<bool, CRCGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCGEN` writer - Enables the clock for the CRCGEN."]
pub struct CRCGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRCGEN_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CRCGEN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Enables the clock for the Watchdog Timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<WWDT_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT` reader - Enables the clock for the Watchdog Timer."]
pub struct WWDT_R(crate::FieldReader<bool, WWDT_A>);
impl WWDT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WWDT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT_A {
        match self.bits {
            false => WWDT_A::DISABLE,
            true => WWDT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WWDT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WWDT_A::ENABLE
    }
}
impl core::ops::Deref for WWDT_R {
    type Target = crate::FieldReader<bool, WWDT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDT` writer - Enables the clock for the Watchdog Timer."]
pub struct WWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WWDT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WWDT_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Enables the clock for the Real Time Clock (RTC).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<RTC_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC` reader - Enables the clock for the Real Time Clock (RTC)."]
pub struct RTC_R(crate::FieldReader<bool, RTC_A>);
impl RTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            false => RTC_A::DISABLE,
            true => RTC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RTC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RTC_A::ENABLE
    }
}
impl core::ops::Deref for RTC_R {
    type Target = crate::FieldReader<bool, RTC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC` writer - Enables the clock for the Real Time Clock (RTC)."]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Enables the clock for the Inter CPU communication Mailbox.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAILBOX_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<MAILBOX_A> for bool {
    #[inline(always)]
    fn from(variant: MAILBOX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAILBOX` reader - Enables the clock for the Inter CPU communication Mailbox."]
pub struct MAILBOX_R(crate::FieldReader<bool, MAILBOX_A>);
impl MAILBOX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAILBOX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAILBOX_A {
        match self.bits {
            false => MAILBOX_A::DISABLE,
            true => MAILBOX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == MAILBOX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == MAILBOX_A::ENABLE
    }
}
impl core::ops::Deref for MAILBOX_R {
    type Target = crate::FieldReader<bool, MAILBOX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAILBOX` writer - Enables the clock for the Inter CPU communication Mailbox."]
pub struct MAILBOX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAILBOX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAILBOX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MAILBOX_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MAILBOX_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Enables the clock for the ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC` reader - Enables the clock for the ADC."]
pub struct ADC_R(crate::FieldReader<bool, ADC_A>);
impl ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::DISABLE,
            true => ADC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ADC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ADC_A::ENABLE
    }
}
impl core::ops::Deref for ADC_R {
    type Target = crate::FieldReader<bool, ADC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC` writer - Enables the clock for the ADC."]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enables the clock for the ROM."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the clock for the SRAM Controller 1."]
    #[inline(always)]
    pub fn sram_ctrl1(&self) -> SRAM_CTRL1_R {
        SRAM_CTRL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for the SRAM Controller 2."]
    #[inline(always)]
    pub fn sram_ctrl2(&self) -> SRAM_CTRL2_R {
        SRAM_CTRL2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables the clock for the SRAM Controller 3."]
    #[inline(always)]
    pub fn sram_ctrl3(&self) -> SRAM_CTRL3_R {
        SRAM_CTRL3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables the clock for the SRAM Controller 4."]
    #[inline(always)]
    pub fn sram_ctrl4(&self) -> SRAM_CTRL4_R {
        SRAM_CTRL4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the Flash controller."]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the FMC controller."]
    #[inline(always)]
    pub fn fmc(&self) -> FMC_R {
        FMC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the Input Mux."]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the I/O controller."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the Pin interrupt (PINT)."]
    #[inline(always)]
    pub fn pint(&self) -> PINT_R {
        PINT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the Group interrupt (GINT)."]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the DMA0."]
    #[inline(always)]
    pub fn dma0(&self) -> DMA0_R {
        DMA0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enables the clock for the CRCGEN."]
    #[inline(always)]
    pub fn crcgen(&self) -> CRCGEN_R {
        CRCGEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer."]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enables the clock for the Real Time Clock (RTC)."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enables the clock for the Inter CPU communication Mailbox."]
    #[inline(always)]
    pub fn mailbox(&self) -> MAILBOX_R {
        MAILBOX_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the ADC."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enables the clock for the ROM."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 3 - Enables the clock for the SRAM Controller 1."]
    #[inline(always)]
    pub fn sram_ctrl1(&mut self) -> SRAM_CTRL1_W {
        SRAM_CTRL1_W { w: self }
    }
    #[doc = "Bit 4 - Enables the clock for the SRAM Controller 2."]
    #[inline(always)]
    pub fn sram_ctrl2(&mut self) -> SRAM_CTRL2_W {
        SRAM_CTRL2_W { w: self }
    }
    #[doc = "Bit 5 - Enables the clock for the SRAM Controller 3."]
    #[inline(always)]
    pub fn sram_ctrl3(&mut self) -> SRAM_CTRL3_W {
        SRAM_CTRL3_W { w: self }
    }
    #[doc = "Bit 6 - Enables the clock for the SRAM Controller 4."]
    #[inline(always)]
    pub fn sram_ctrl4(&mut self) -> SRAM_CTRL4_W {
        SRAM_CTRL4_W { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for the Flash controller."]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W {
        FLASH_W { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for the FMC controller."]
    #[inline(always)]
    pub fn fmc(&mut self) -> FMC_W {
        FMC_W { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for the Input Mux."]
    #[inline(always)]
    pub fn mux(&mut self) -> MUX_W {
        MUX_W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the I/O controller."]
    #[inline(always)]
    pub fn iocon(&mut self) -> IOCON_W {
        IOCON_W { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> GPIO0_W {
        GPIO0_W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1."]
    #[inline(always)]
    pub fn gpio1(&mut self) -> GPIO1_W {
        GPIO1_W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2."]
    #[inline(always)]
    pub fn gpio2(&mut self) -> GPIO2_W {
        GPIO2_W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3."]
    #[inline(always)]
    pub fn gpio3(&mut self) -> GPIO3_W {
        GPIO3_W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the Pin interrupt (PINT)."]
    #[inline(always)]
    pub fn pint(&mut self) -> PINT_W {
        PINT_W { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the Group interrupt (GINT)."]
    #[inline(always)]
    pub fn gint(&mut self) -> GINT_W {
        GINT_W { w: self }
    }
    #[doc = "Bit 20 - Enables the clock for the DMA0."]
    #[inline(always)]
    pub fn dma0(&mut self) -> DMA0_W {
        DMA0_W { w: self }
    }
    #[doc = "Bit 21 - Enables the clock for the CRCGEN."]
    #[inline(always)]
    pub fn crcgen(&mut self) -> CRCGEN_W {
        CRCGEN_W { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer."]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WWDT_W {
        WWDT_W { w: self }
    }
    #[doc = "Bit 23 - Enables the clock for the Real Time Clock (RTC)."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 26 - Enables the clock for the Inter CPU communication Mailbox."]
    #[inline(always)]
    pub fn mailbox(&mut self) -> MAILBOX_W {
        MAILBOX_W { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for the ADC."]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Clock control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl0](index.html) module"]
pub struct AHBCLKCTRL0_SPEC;
impl crate::RegisterSpec for AHBCLKCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclkctrl0::R](R) reader structure"]
impl crate::Readable for AHBCLKCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl0::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKCTRL0 to value 0x0180"]
impl crate::Resettable for AHBCLKCTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0180
    }
}
