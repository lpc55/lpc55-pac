#[doc = "Register `AHBCLKCTRL2` reader"]
pub struct R(crate::R<AHBCLKCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKCTRL2` writer"]
pub struct W(crate::W<AHBCLKCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRL2_SPEC>;
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
impl From<crate::W<AHBCLKCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enables the clock for the DMA1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<DMA1_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1` reader - Enables the clock for the DMA1."]
pub struct DMA1_R(crate::FieldReader<bool, DMA1_A>);
impl DMA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1_A {
        match self.bits {
            false => DMA1_A::DISABLE,
            true => DMA1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DMA1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DMA1_A::ENABLE
    }
}
impl core::ops::Deref for DMA1_R {
    type Target = crate::FieldReader<bool, DMA1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1` writer - Enables the clock for the DMA1."]
pub struct DMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA1_A::ENABLE)
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
#[doc = "Enables the clock for the Comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<COMP_A> for bool {
    #[inline(always)]
    fn from(variant: COMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP` reader - Enables the clock for the Comparator."]
pub struct COMP_R(crate::FieldReader<bool, COMP_A>);
impl COMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_A {
        match self.bits {
            false => COMP_A::DISABLE,
            true => COMP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == COMP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == COMP_A::ENABLE
    }
}
impl core::ops::Deref for COMP_R {
    type Target = crate::FieldReader<bool, COMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP` writer - Enables the clock for the Comparator."]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMP_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMP_A::ENABLE)
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
#[doc = "Enables the clock for the SDIO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIO_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SDIO_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO` reader - Enables the clock for the SDIO."]
pub struct SDIO_R(crate::FieldReader<bool, SDIO_A>);
impl SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO_A {
        match self.bits {
            false => SDIO_A::DISABLE,
            true => SDIO_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SDIO_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SDIO_A::ENABLE
    }
}
impl core::ops::Deref for SDIO_R {
    type Target = crate::FieldReader<bool, SDIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO` writer - Enables the clock for the SDIO."]
pub struct SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDIO_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDIO_A::ENABLE)
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
#[doc = "Enables the clock for the USB1 Host.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_HOST_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<USB1_HOST_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_HOST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_HOST` reader - Enables the clock for the USB1 Host."]
pub struct USB1_HOST_R(crate::FieldReader<bool, USB1_HOST_A>);
impl USB1_HOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB1_HOST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_HOST_A {
        match self.bits {
            false => USB1_HOST_A::DISABLE,
            true => USB1_HOST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == USB1_HOST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == USB1_HOST_A::ENABLE
    }
}
impl core::ops::Deref for USB1_HOST_R {
    type Target = crate::FieldReader<bool, USB1_HOST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1_HOST` writer - Enables the clock for the USB1 Host."]
pub struct USB1_HOST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_HOST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_HOST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_HOST_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_HOST_A::ENABLE)
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
#[doc = "Enables the clock for the USB1 dev.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_DEV_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<USB1_DEV_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_DEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_DEV` reader - Enables the clock for the USB1 dev."]
pub struct USB1_DEV_R(crate::FieldReader<bool, USB1_DEV_A>);
impl USB1_DEV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB1_DEV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_DEV_A {
        match self.bits {
            false => USB1_DEV_A::DISABLE,
            true => USB1_DEV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == USB1_DEV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == USB1_DEV_A::ENABLE
    }
}
impl core::ops::Deref for USB1_DEV_R {
    type Target = crate::FieldReader<bool, USB1_DEV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1_DEV` writer - Enables the clock for the USB1 dev."]
pub struct USB1_DEV_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_DEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_DEV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_DEV_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_DEV_A::ENABLE)
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
#[doc = "Enables the clock for the USB1 RAM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_RAM_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<USB1_RAM_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_RAM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_RAM` reader - Enables the clock for the USB1 RAM."]
pub struct USB1_RAM_R(crate::FieldReader<bool, USB1_RAM_A>);
impl USB1_RAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB1_RAM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_RAM_A {
        match self.bits {
            false => USB1_RAM_A::DISABLE,
            true => USB1_RAM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == USB1_RAM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == USB1_RAM_A::ENABLE
    }
}
impl core::ops::Deref for USB1_RAM_R {
    type Target = crate::FieldReader<bool, USB1_RAM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1_RAM` writer - Enables the clock for the USB1 RAM."]
pub struct USB1_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_RAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_RAM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_RAM_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_RAM_A::ENABLE)
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
#[doc = "Enables the clock for the USB1 PHY.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_PHY_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<USB1_PHY_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_PHY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_PHY` reader - Enables the clock for the USB1 PHY."]
pub struct USB1_PHY_R(crate::FieldReader<bool, USB1_PHY_A>);
impl USB1_PHY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB1_PHY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_PHY_A {
        match self.bits {
            false => USB1_PHY_A::DISABLE,
            true => USB1_PHY_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == USB1_PHY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == USB1_PHY_A::ENABLE
    }
}
impl core::ops::Deref for USB1_PHY_R {
    type Target = crate::FieldReader<bool, USB1_PHY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1_PHY` writer - Enables the clock for the USB1 PHY."]
pub struct USB1_PHY_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_PHY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_PHY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1_PHY_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1_PHY_A::ENABLE)
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
#[doc = "Enables the clock for the Frequency meter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQME_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FREQME_A> for bool {
    #[inline(always)]
    fn from(variant: FREQME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQME` reader - Enables the clock for the Frequency meter."]
pub struct FREQME_R(crate::FieldReader<bool, FREQME_A>);
impl FREQME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FREQME_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQME_A {
        match self.bits {
            false => FREQME_A::DISABLE,
            true => FREQME_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FREQME_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FREQME_A::ENABLE
    }
}
impl core::ops::Deref for FREQME_R {
    type Target = crate::FieldReader<bool, FREQME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQME` writer - Enables the clock for the Frequency meter."]
pub struct FREQME_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQME_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FREQME_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FREQME_A::ENABLE)
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
#[doc = "Enables the clock for the RNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<RNG_A> for bool {
    #[inline(always)]
    fn from(variant: RNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG` reader - Enables the clock for the RNG."]
pub struct RNG_R(crate::FieldReader<bool, RNG_A>);
impl RNG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_A {
        match self.bits {
            false => RNG_A::DISABLE,
            true => RNG_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RNG_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RNG_A::ENABLE
    }
}
impl core::ops::Deref for RNG_R {
    type Target = crate::FieldReader<bool, RNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG` writer - Enables the clock for the RNG."]
pub struct RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RNG_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RNG_A::ENABLE)
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
#[doc = "SYSCTL block clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SYSCTL_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCTL` reader - SYSCTL block clock."]
pub struct SYSCTL_R(crate::FieldReader<bool, SYSCTL_A>);
impl SYSCTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSCTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTL_A {
        match self.bits {
            false => SYSCTL_A::DISABLE,
            true => SYSCTL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SYSCTL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SYSCTL_A::ENABLE
    }
}
impl core::ops::Deref for SYSCTL_R {
    type Target = crate::FieldReader<bool, SYSCTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCTL` writer - SYSCTL block clock."]
pub struct SYSCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYSCTL_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYSCTL_A::ENABLE)
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
#[doc = "Enables the clock for the USB0 Host Master.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_HOSTM_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<USB0_HOSTM_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_HOSTM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_HOSTM` reader - Enables the clock for the USB0 Host Master."]
pub struct USB0_HOSTM_R(crate::FieldReader<bool, USB0_HOSTM_A>);
impl USB0_HOSTM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB0_HOSTM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_HOSTM_A {
        match self.bits {
            false => USB0_HOSTM_A::DISABLE,
            true => USB0_HOSTM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == USB0_HOSTM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == USB0_HOSTM_A::ENABLE
    }
}
impl core::ops::Deref for USB0_HOSTM_R {
    type Target = crate::FieldReader<bool, USB0_HOSTM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0_HOSTM` writer - Enables the clock for the USB0 Host Master."]
pub struct USB0_HOSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_HOSTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_HOSTM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_HOSTM_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_HOSTM_A::ENABLE)
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
#[doc = "Enables the clock for the USB0 Host Slave.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_HOSTS_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<USB0_HOSTS_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_HOSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_HOSTS` reader - Enables the clock for the USB0 Host Slave."]
pub struct USB0_HOSTS_R(crate::FieldReader<bool, USB0_HOSTS_A>);
impl USB0_HOSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB0_HOSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_HOSTS_A {
        match self.bits {
            false => USB0_HOSTS_A::DISABLE,
            true => USB0_HOSTS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == USB0_HOSTS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == USB0_HOSTS_A::ENABLE
    }
}
impl core::ops::Deref for USB0_HOSTS_R {
    type Target = crate::FieldReader<bool, USB0_HOSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0_HOSTS` writer - Enables the clock for the USB0 Host Slave."]
pub struct USB0_HOSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_HOSTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_HOSTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_HOSTS_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_HOSTS_A::ENABLE)
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
#[doc = "Enables the clock for the HASH_AES.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH_AES_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<HASH_AES_A> for bool {
    #[inline(always)]
    fn from(variant: HASH_AES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASH_AES` reader - Enables the clock for the HASH_AES."]
pub struct HASH_AES_R(crate::FieldReader<bool, HASH_AES_A>);
impl HASH_AES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HASH_AES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH_AES_A {
        match self.bits {
            false => HASH_AES_A::DISABLE,
            true => HASH_AES_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == HASH_AES_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == HASH_AES_A::ENABLE
    }
}
impl core::ops::Deref for HASH_AES_R {
    type Target = crate::FieldReader<bool, HASH_AES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASH_AES` writer - Enables the clock for the HASH_AES."]
pub struct HASH_AES_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_AES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASH_AES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HASH_AES_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HASH_AES_A::ENABLE)
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
#[doc = "Enables the clock for the Power Quad.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PQ_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<PQ_A> for bool {
    #[inline(always)]
    fn from(variant: PQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PQ` reader - Enables the clock for the Power Quad."]
pub struct PQ_R(crate::FieldReader<bool, PQ_A>);
impl PQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PQ_A {
        match self.bits {
            false => PQ_A::DISABLE,
            true => PQ_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PQ_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PQ_A::ENABLE
    }
}
impl core::ops::Deref for PQ_R {
    type Target = crate::FieldReader<bool, PQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PQ` writer - Enables the clock for the Power Quad."]
pub struct PQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PQ_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PQ_A::ENABLE)
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
#[doc = "Enables the clock for the PLU LUT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLULUT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<PLULUT_A> for bool {
    #[inline(always)]
    fn from(variant: PLULUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLULUT` reader - Enables the clock for the PLU LUT."]
pub struct PLULUT_R(crate::FieldReader<bool, PLULUT_A>);
impl PLULUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLULUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLULUT_A {
        match self.bits {
            false => PLULUT_A::DISABLE,
            true => PLULUT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PLULUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PLULUT_A::ENABLE
    }
}
impl core::ops::Deref for PLULUT_R {
    type Target = crate::FieldReader<bool, PLULUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLULUT` writer - Enables the clock for the PLU LUT."]
pub struct PLULUT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLULUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLULUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLULUT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLULUT_A::ENABLE)
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
#[doc = "Enables the clock for the Timer 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER3_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<TIMER3_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER3` reader - Enables the clock for the Timer 3."]
pub struct TIMER3_R(crate::FieldReader<bool, TIMER3_A>);
impl TIMER3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER3_A {
        match self.bits {
            false => TIMER3_A::DISABLE,
            true => TIMER3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TIMER3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TIMER3_A::ENABLE
    }
}
impl core::ops::Deref for TIMER3_R {
    type Target = crate::FieldReader<bool, TIMER3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER3` writer - Enables the clock for the Timer 3."]
pub struct TIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER3_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER3_A::ENABLE)
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
#[doc = "Enables the clock for the Timer 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER4_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<TIMER4_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER4` reader - Enables the clock for the Timer 4."]
pub struct TIMER4_R(crate::FieldReader<bool, TIMER4_A>);
impl TIMER4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER4_A {
        match self.bits {
            false => TIMER4_A::DISABLE,
            true => TIMER4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TIMER4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TIMER4_A::ENABLE
    }
}
impl core::ops::Deref for TIMER4_R {
    type Target = crate::FieldReader<bool, TIMER4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER4` writer - Enables the clock for the Timer 4."]
pub struct TIMER4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER4_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER4_A::ENABLE)
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
#[doc = "Enables the clock for the PUF reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUF_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<PUF_A> for bool {
    #[inline(always)]
    fn from(variant: PUF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF` reader - Enables the clock for the PUF reset control."]
pub struct PUF_R(crate::FieldReader<bool, PUF_A>);
impl PUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PUF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUF_A {
        match self.bits {
            false => PUF_A::DISABLE,
            true => PUF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PUF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PUF_A::ENABLE
    }
}
impl core::ops::Deref for PUF_R {
    type Target = crate::FieldReader<bool, PUF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUF` writer - Enables the clock for the PUF reset control."]
pub struct PUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PUF_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PUF_A::ENABLE)
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
#[doc = "Enables the clock for the Casper.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASPER_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<CASPER_A> for bool {
    #[inline(always)]
    fn from(variant: CASPER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER` reader - Enables the clock for the Casper."]
pub struct CASPER_R(crate::FieldReader<bool, CASPER_A>);
impl CASPER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CASPER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_A {
        match self.bits {
            false => CASPER_A::DISABLE,
            true => CASPER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CASPER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CASPER_A::ENABLE
    }
}
impl core::ops::Deref for CASPER_R {
    type Target = crate::FieldReader<bool, CASPER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CASPER` writer - Enables the clock for the Casper."]
pub struct CASPER_W<'a> {
    w: &'a mut W,
}
impl<'a> CASPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CASPER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CASPER_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CASPER_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Enables the clock for the analog control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANALOG_CTRL_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<ANALOG_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: ANALOG_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANALOG_CTRL` reader - Enables the clock for the analog control."]
pub struct ANALOG_CTRL_R(crate::FieldReader<bool, ANALOG_CTRL_A>);
impl ANALOG_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANALOG_CTRL_A {
        match self.bits {
            false => ANALOG_CTRL_A::DISABLE,
            true => ANALOG_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ANALOG_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ANALOG_CTRL_A::ENABLE
    }
}
impl core::ops::Deref for ANALOG_CTRL_R {
    type Target = crate::FieldReader<bool, ANALOG_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_CTRL` writer - Enables the clock for the analog control."]
pub struct ANALOG_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANALOG_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_A::ENABLE)
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
#[doc = "Enables the clock for the HS LSPI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_LSPI_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<HS_LSPI_A> for bool {
    #[inline(always)]
    fn from(variant: HS_LSPI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_LSPI` reader - Enables the clock for the HS LSPI."]
pub struct HS_LSPI_R(crate::FieldReader<bool, HS_LSPI_A>);
impl HS_LSPI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HS_LSPI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_LSPI_A {
        match self.bits {
            false => HS_LSPI_A::DISABLE,
            true => HS_LSPI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == HS_LSPI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == HS_LSPI_A::ENABLE
    }
}
impl core::ops::Deref for HS_LSPI_R {
    type Target = crate::FieldReader<bool, HS_LSPI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS_LSPI` writer - Enables the clock for the HS LSPI."]
pub struct HS_LSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_LSPI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_LSPI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HS_LSPI_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HS_LSPI_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Enables the clock for the GPIO secure.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GPIO_SEC_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_SEC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_SEC` reader - Enables the clock for the GPIO secure."]
pub struct GPIO_SEC_R(crate::FieldReader<bool, GPIO_SEC_A>);
impl GPIO_SEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_SEC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_SEC_A {
        match self.bits {
            false => GPIO_SEC_A::DISABLE,
            true => GPIO_SEC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == GPIO_SEC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == GPIO_SEC_A::ENABLE
    }
}
impl core::ops::Deref for GPIO_SEC_R {
    type Target = crate::FieldReader<bool, GPIO_SEC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_SEC` writer - Enables the clock for the GPIO secure."]
pub struct GPIO_SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_SEC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_SEC_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_SEC_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Enables the clock for the GPIO secure int.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_INT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GPIO_SEC_INT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_SEC_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_SEC_INT` reader - Enables the clock for the GPIO secure int."]
pub struct GPIO_SEC_INT_R(crate::FieldReader<bool, GPIO_SEC_INT_A>);
impl GPIO_SEC_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_SEC_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_SEC_INT_A {
        match self.bits {
            false => GPIO_SEC_INT_A::DISABLE,
            true => GPIO_SEC_INT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == GPIO_SEC_INT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == GPIO_SEC_INT_A::ENABLE
    }
}
impl core::ops::Deref for GPIO_SEC_INT_R {
    type Target = crate::FieldReader<bool, GPIO_SEC_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_SEC_INT` writer - Enables the clock for the GPIO secure int."]
pub struct GPIO_SEC_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SEC_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_SEC_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enables the clock for the DMA1."]
    #[inline(always)]
    pub fn dma1(&self) -> DMA1_R {
        DMA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the clock for the Comparator."]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO."]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 Host."]
    #[inline(always)]
    pub fn usb1_host(&self) -> USB1_HOST_R {
        USB1_HOST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 dev."]
    #[inline(always)]
    pub fn usb1_dev(&self) -> USB1_DEV_R {
        USB1_DEV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM."]
    #[inline(always)]
    pub fn usb1_ram(&self) -> USB1_RAM_R {
        USB1_RAM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the USB1 PHY."]
    #[inline(always)]
    pub fn usb1_phy(&self) -> USB1_PHY_R {
        USB1_PHY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the Frequency meter."]
    #[inline(always)]
    pub fn freqme(&self) -> FREQME_R {
        FREQME_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the RNG."]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SYSCTL block clock."]
    #[inline(always)]
    pub fn sysctl(&self) -> SYSCTL_R {
        SYSCTL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the USB0 Host Master."]
    #[inline(always)]
    pub fn usb0_hostm(&self) -> USB0_HOSTM_R {
        USB0_HOSTM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the USB0 Host Slave."]
    #[inline(always)]
    pub fn usb0_hosts(&self) -> USB0_HOSTS_R {
        USB0_HOSTS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the HASH_AES."]
    #[inline(always)]
    pub fn hash_aes(&self) -> HASH_AES_R {
        HASH_AES_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the Power Quad."]
    #[inline(always)]
    pub fn pq(&self) -> PQ_R {
        PQ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the PLU LUT."]
    #[inline(always)]
    pub fn plulut(&self) -> PLULUT_R {
        PLULUT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enables the clock for the Timer 3."]
    #[inline(always)]
    pub fn timer3(&self) -> TIMER3_R {
        TIMER3_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 4."]
    #[inline(always)]
    pub fn timer4(&self) -> TIMER4_R {
        TIMER4_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enables the clock for the PUF reset control."]
    #[inline(always)]
    pub fn puf(&self) -> PUF_R {
        PUF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enables the clock for the Casper."]
    #[inline(always)]
    pub fn casper(&self) -> CASPER_R {
        CASPER_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the analog control."]
    #[inline(always)]
    pub fn analog_ctrl(&self) -> ANALOG_CTRL_R {
        ANALOG_CTRL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enables the clock for the HS LSPI."]
    #[inline(always)]
    pub fn hs_lspi(&self) -> HS_LSPI_R {
        HS_LSPI_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enables the clock for the GPIO secure."]
    #[inline(always)]
    pub fn gpio_sec(&self) -> GPIO_SEC_R {
        GPIO_SEC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enables the clock for the GPIO secure int."]
    #[inline(always)]
    pub fn gpio_sec_int(&self) -> GPIO_SEC_INT_R {
        GPIO_SEC_INT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enables the clock for the DMA1."]
    #[inline(always)]
    pub fn dma1(&mut self) -> DMA1_W {
        DMA1_W { w: self }
    }
    #[doc = "Bit 2 - Enables the clock for the Comparator."]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO."]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W {
        SDIO_W { w: self }
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 Host."]
    #[inline(always)]
    pub fn usb1_host(&mut self) -> USB1_HOST_W {
        USB1_HOST_W { w: self }
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 dev."]
    #[inline(always)]
    pub fn usb1_dev(&mut self) -> USB1_DEV_W {
        USB1_DEV_W { w: self }
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM."]
    #[inline(always)]
    pub fn usb1_ram(&mut self) -> USB1_RAM_W {
        USB1_RAM_W { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for the USB1 PHY."]
    #[inline(always)]
    pub fn usb1_phy(&mut self) -> USB1_PHY_W {
        USB1_PHY_W { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for the Frequency meter."]
    #[inline(always)]
    pub fn freqme(&mut self) -> FREQME_W {
        FREQME_W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the RNG."]
    #[inline(always)]
    pub fn rng(&mut self) -> RNG_W {
        RNG_W { w: self }
    }
    #[doc = "Bit 15 - SYSCTL block clock."]
    #[inline(always)]
    pub fn sysctl(&mut self) -> SYSCTL_W {
        SYSCTL_W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the USB0 Host Master."]
    #[inline(always)]
    pub fn usb0_hostm(&mut self) -> USB0_HOSTM_W {
        USB0_HOSTM_W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the USB0 Host Slave."]
    #[inline(always)]
    pub fn usb0_hosts(&mut self) -> USB0_HOSTS_W {
        USB0_HOSTS_W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the HASH_AES."]
    #[inline(always)]
    pub fn hash_aes(&mut self) -> HASH_AES_W {
        HASH_AES_W { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the Power Quad."]
    #[inline(always)]
    pub fn pq(&mut self) -> PQ_W {
        PQ_W { w: self }
    }
    #[doc = "Bit 20 - Enables the clock for the PLU LUT."]
    #[inline(always)]
    pub fn plulut(&mut self) -> PLULUT_W {
        PLULUT_W { w: self }
    }
    #[doc = "Bit 21 - Enables the clock for the Timer 3."]
    #[inline(always)]
    pub fn timer3(&mut self) -> TIMER3_W {
        TIMER3_W { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 4."]
    #[inline(always)]
    pub fn timer4(&mut self) -> TIMER4_W {
        TIMER4_W { w: self }
    }
    #[doc = "Bit 23 - Enables the clock for the PUF reset control."]
    #[inline(always)]
    pub fn puf(&mut self) -> PUF_W {
        PUF_W { w: self }
    }
    #[doc = "Bit 24 - Enables the clock for the Casper."]
    #[inline(always)]
    pub fn casper(&mut self) -> CASPER_W {
        CASPER_W { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for the analog control."]
    #[inline(always)]
    pub fn analog_ctrl(&mut self) -> ANALOG_CTRL_W {
        ANALOG_CTRL_W { w: self }
    }
    #[doc = "Bit 28 - Enables the clock for the HS LSPI."]
    #[inline(always)]
    pub fn hs_lspi(&mut self) -> HS_LSPI_W {
        HS_LSPI_W { w: self }
    }
    #[doc = "Bit 29 - Enables the clock for the GPIO secure."]
    #[inline(always)]
    pub fn gpio_sec(&mut self) -> GPIO_SEC_W {
        GPIO_SEC_W { w: self }
    }
    #[doc = "Bit 30 - Enables the clock for the GPIO secure int."]
    #[inline(always)]
    pub fn gpio_sec_int(&mut self) -> GPIO_SEC_INT_W {
        GPIO_SEC_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Clock control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl2](index.html) module"]
pub struct AHBCLKCTRL2_SPEC;
impl crate::RegisterSpec for AHBCLKCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclkctrl2::R](R) reader structure"]
impl crate::Readable for AHBCLKCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl2::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKCTRL2 to value 0"]
impl crate::Resettable for AHBCLKCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
