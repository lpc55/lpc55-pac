#[doc = "Register `PRESETCTRL2` reader"]
pub struct R(crate::R<PRESETCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PRESETCTRL2_SPEC>> for R {
    fn from(reader: crate::R<PRESETCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL2` writer"]
pub struct W(crate::W<PRESETCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL2_SPEC>;
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
impl core::convert::From<crate::W<PRESETCTRL2_SPEC>> for W {
    fn from(writer: crate::W<PRESETCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA1 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<DMA1_RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1_RST` reader - DMA1 reset control."]
pub struct DMA1_RST_R(crate::FieldReader<bool, DMA1_RST_A>);
impl DMA1_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1_RST_A {
        match self.bits {
            false => DMA1_RST_A::RELEASED,
            true => DMA1_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == DMA1_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == DMA1_RST_A::ASSERTED
    }
}
impl core::ops::Deref for DMA1_RST_R {
    type Target = crate::FieldReader<bool, DMA1_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1_RST` writer - DMA1 reset control."]
pub struct DMA1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(DMA1_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(DMA1_RST_A::ASSERTED)
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
#[doc = "Comparator reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<COMP_RST_A> for bool {
    #[inline(always)]
    fn from(variant: COMP_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP_RST` reader - Comparator reset control."]
pub struct COMP_RST_R(crate::FieldReader<bool, COMP_RST_A>);
impl COMP_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_RST_A {
        match self.bits {
            false => COMP_RST_A::RELEASED,
            true => COMP_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == COMP_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == COMP_RST_A::ASSERTED
    }
}
impl core::ops::Deref for COMP_RST_R {
    type Target = crate::FieldReader<bool, COMP_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP_RST` writer - Comparator reset control."]
pub struct COMP_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(COMP_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(COMP_RST_A::ASSERTED)
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
#[doc = "SDIO reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIO_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<SDIO_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO_RST` reader - SDIO reset control."]
pub struct SDIO_RST_R(crate::FieldReader<bool, SDIO_RST_A>);
impl SDIO_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO_RST_A {
        match self.bits {
            false => SDIO_RST_A::RELEASED,
            true => SDIO_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == SDIO_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == SDIO_RST_A::ASSERTED
    }
}
impl core::ops::Deref for SDIO_RST_R {
    type Target = crate::FieldReader<bool, SDIO_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_RST` writer - SDIO reset control."]
pub struct SDIO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIO_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SDIO_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SDIO_RST_A::ASSERTED)
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
#[doc = "USB1 Host reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_HOST_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB1_HOST_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_HOST_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_HOST_RST` reader - USB1 Host reset control."]
pub struct USB1_HOST_RST_R(crate::FieldReader<bool, USB1_HOST_RST_A>);
impl USB1_HOST_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB1_HOST_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_HOST_RST_A {
        match self.bits {
            false => USB1_HOST_RST_A::RELEASED,
            true => USB1_HOST_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == USB1_HOST_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == USB1_HOST_RST_A::ASSERTED
    }
}
impl core::ops::Deref for USB1_HOST_RST_R {
    type Target = crate::FieldReader<bool, USB1_HOST_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1_HOST_RST` writer - USB1 Host reset control."]
pub struct USB1_HOST_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_HOST_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_HOST_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB1_HOST_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB1_HOST_RST_A::ASSERTED)
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
#[doc = "USB1 dev reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_DEV_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB1_DEV_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_DEV_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_DEV_RST` reader - USB1 dev reset control."]
pub struct USB1_DEV_RST_R(crate::FieldReader<bool, USB1_DEV_RST_A>);
impl USB1_DEV_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB1_DEV_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_DEV_RST_A {
        match self.bits {
            false => USB1_DEV_RST_A::RELEASED,
            true => USB1_DEV_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == USB1_DEV_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == USB1_DEV_RST_A::ASSERTED
    }
}
impl core::ops::Deref for USB1_DEV_RST_R {
    type Target = crate::FieldReader<bool, USB1_DEV_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1_DEV_RST` writer - USB1 dev reset control."]
pub struct USB1_DEV_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_DEV_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_DEV_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB1_DEV_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB1_DEV_RST_A::ASSERTED)
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
#[doc = "USB1 RAM reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_RAM_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB1_RAM_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_RAM_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_RAM_RST` reader - USB1 RAM reset control."]
pub struct USB1_RAM_RST_R(crate::FieldReader<bool, USB1_RAM_RST_A>);
impl USB1_RAM_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB1_RAM_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_RAM_RST_A {
        match self.bits {
            false => USB1_RAM_RST_A::RELEASED,
            true => USB1_RAM_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == USB1_RAM_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == USB1_RAM_RST_A::ASSERTED
    }
}
impl core::ops::Deref for USB1_RAM_RST_R {
    type Target = crate::FieldReader<bool, USB1_RAM_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1_RAM_RST` writer - USB1 RAM reset control."]
pub struct USB1_RAM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_RAM_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_RAM_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB1_RAM_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB1_RAM_RST_A::ASSERTED)
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
#[doc = "USB1 PHY reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_PHY_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB1_PHY_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_PHY_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_PHY_RST` reader - USB1 PHY reset control."]
pub struct USB1_PHY_RST_R(crate::FieldReader<bool, USB1_PHY_RST_A>);
impl USB1_PHY_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB1_PHY_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_PHY_RST_A {
        match self.bits {
            false => USB1_PHY_RST_A::RELEASED,
            true => USB1_PHY_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == USB1_PHY_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == USB1_PHY_RST_A::ASSERTED
    }
}
impl core::ops::Deref for USB1_PHY_RST_R {
    type Target = crate::FieldReader<bool, USB1_PHY_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1_PHY_RST` writer - USB1 PHY reset control."]
pub struct USB1_PHY_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_PHY_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_PHY_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB1_PHY_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB1_PHY_RST_A::ASSERTED)
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
#[doc = "Frequency meter reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQME_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FREQME_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FREQME_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQME_RST` reader - Frequency meter reset control."]
pub struct FREQME_RST_R(crate::FieldReader<bool, FREQME_RST_A>);
impl FREQME_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREQME_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQME_RST_A {
        match self.bits {
            false => FREQME_RST_A::RELEASED,
            true => FREQME_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == FREQME_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == FREQME_RST_A::ASSERTED
    }
}
impl core::ops::Deref for FREQME_RST_R {
    type Target = crate::FieldReader<bool, FREQME_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQME_RST` writer - Frequency meter reset control."]
pub struct FREQME_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQME_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQME_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FREQME_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FREQME_RST_A::ASSERTED)
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
#[doc = "RNG reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<RNG_RST_A> for bool {
    #[inline(always)]
    fn from(variant: RNG_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNG_RST` reader - RNG reset control."]
pub struct RNG_RST_R(crate::FieldReader<bool, RNG_RST_A>);
impl RNG_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNG_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_RST_A {
        match self.bits {
            false => RNG_RST_A::RELEASED,
            true => RNG_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == RNG_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == RNG_RST_A::ASSERTED
    }
}
impl core::ops::Deref for RNG_RST_R {
    type Target = crate::FieldReader<bool, RNG_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNG_RST` writer - RNG reset control."]
pub struct RNG_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(RNG_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(RNG_RST_A::ASSERTED)
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
#[doc = "SYSCTL Block reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<SYSCTL_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCTL_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCTL_RST` reader - SYSCTL Block reset."]
pub struct SYSCTL_RST_R(crate::FieldReader<bool, SYSCTL_RST_A>);
impl SYSCTL_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCTL_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTL_RST_A {
        match self.bits {
            false => SYSCTL_RST_A::RELEASED,
            true => SYSCTL_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == SYSCTL_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == SYSCTL_RST_A::ASSERTED
    }
}
impl core::ops::Deref for SYSCTL_RST_R {
    type Target = crate::FieldReader<bool, SYSCTL_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCTL_RST` writer - SYSCTL Block reset."]
pub struct SYSCTL_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTL_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SYSCTL_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SYSCTL_RST_A::ASSERTED)
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
#[doc = "USB0 Host Master reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_HOSTM_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB0_HOSTM_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_HOSTM_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_HOSTM_RST` reader - USB0 Host Master reset control."]
pub struct USB0_HOSTM_RST_R(crate::FieldReader<bool, USB0_HOSTM_RST_A>);
impl USB0_HOSTM_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0_HOSTM_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_HOSTM_RST_A {
        match self.bits {
            false => USB0_HOSTM_RST_A::RELEASED,
            true => USB0_HOSTM_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == USB0_HOSTM_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == USB0_HOSTM_RST_A::ASSERTED
    }
}
impl core::ops::Deref for USB0_HOSTM_RST_R {
    type Target = crate::FieldReader<bool, USB0_HOSTM_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0_HOSTM_RST` writer - USB0 Host Master reset control."]
pub struct USB0_HOSTM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_HOSTM_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_HOSTM_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB0_HOSTM_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB0_HOSTM_RST_A::ASSERTED)
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
#[doc = "USB0 Host Slave reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_HOSTS_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<USB0_HOSTS_RST_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_HOSTS_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_HOSTS_RST` reader - USB0 Host Slave reset control."]
pub struct USB0_HOSTS_RST_R(crate::FieldReader<bool, USB0_HOSTS_RST_A>);
impl USB0_HOSTS_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0_HOSTS_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_HOSTS_RST_A {
        match self.bits {
            false => USB0_HOSTS_RST_A::RELEASED,
            true => USB0_HOSTS_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == USB0_HOSTS_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == USB0_HOSTS_RST_A::ASSERTED
    }
}
impl core::ops::Deref for USB0_HOSTS_RST_R {
    type Target = crate::FieldReader<bool, USB0_HOSTS_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0_HOSTS_RST` writer - USB0 Host Slave reset control."]
pub struct USB0_HOSTS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_HOSTS_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_HOSTS_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(USB0_HOSTS_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(USB0_HOSTS_RST_A::ASSERTED)
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
#[doc = "HASH_AES reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH_AES_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<HASH_AES_RST_A> for bool {
    #[inline(always)]
    fn from(variant: HASH_AES_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASH_AES_RST` reader - HASH_AES reset control."]
pub struct HASH_AES_RST_R(crate::FieldReader<bool, HASH_AES_RST_A>);
impl HASH_AES_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        HASH_AES_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH_AES_RST_A {
        match self.bits {
            false => HASH_AES_RST_A::RELEASED,
            true => HASH_AES_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == HASH_AES_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == HASH_AES_RST_A::ASSERTED
    }
}
impl core::ops::Deref for HASH_AES_RST_R {
    type Target = crate::FieldReader<bool, HASH_AES_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASH_AES_RST` writer - HASH_AES reset control."]
pub struct HASH_AES_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_AES_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASH_AES_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(HASH_AES_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(HASH_AES_RST_A::ASSERTED)
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
#[doc = "Power Quad reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PQ_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<PQ_RST_A> for bool {
    #[inline(always)]
    fn from(variant: PQ_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PQ_RST` reader - Power Quad reset control."]
pub struct PQ_RST_R(crate::FieldReader<bool, PQ_RST_A>);
impl PQ_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PQ_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PQ_RST_A {
        match self.bits {
            false => PQ_RST_A::RELEASED,
            true => PQ_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == PQ_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == PQ_RST_A::ASSERTED
    }
}
impl core::ops::Deref for PQ_RST_R {
    type Target = crate::FieldReader<bool, PQ_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PQ_RST` writer - Power Quad reset control."]
pub struct PQ_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PQ_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PQ_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(PQ_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PQ_RST_A::ASSERTED)
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
#[doc = "PLU LUT reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLULUT_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<PLULUT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: PLULUT_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLULUT_RST` reader - PLU LUT reset control."]
pub struct PLULUT_RST_R(crate::FieldReader<bool, PLULUT_RST_A>);
impl PLULUT_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLULUT_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLULUT_RST_A {
        match self.bits {
            false => PLULUT_RST_A::RELEASED,
            true => PLULUT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == PLULUT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == PLULUT_RST_A::ASSERTED
    }
}
impl core::ops::Deref for PLULUT_RST_R {
    type Target = crate::FieldReader<bool, PLULUT_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLULUT_RST` writer - PLU LUT reset control."]
pub struct PLULUT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PLULUT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLULUT_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(PLULUT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PLULUT_RST_A::ASSERTED)
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
#[doc = "Timer 3 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER3_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<TIMER3_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER3_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER3_RST` reader - Timer 3 reset control."]
pub struct TIMER3_RST_R(crate::FieldReader<bool, TIMER3_RST_A>);
impl TIMER3_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER3_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER3_RST_A {
        match self.bits {
            false => TIMER3_RST_A::RELEASED,
            true => TIMER3_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == TIMER3_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == TIMER3_RST_A::ASSERTED
    }
}
impl core::ops::Deref for TIMER3_RST_R {
    type Target = crate::FieldReader<bool, TIMER3_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER3_RST` writer - Timer 3 reset control."]
pub struct TIMER3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER3_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER3_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER3_RST_A::ASSERTED)
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
#[doc = "Timer 4 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER4_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<TIMER4_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER4_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER4_RST` reader - Timer 4 reset control."]
pub struct TIMER4_RST_R(crate::FieldReader<bool, TIMER4_RST_A>);
impl TIMER4_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER4_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER4_RST_A {
        match self.bits {
            false => TIMER4_RST_A::RELEASED,
            true => TIMER4_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == TIMER4_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == TIMER4_RST_A::ASSERTED
    }
}
impl core::ops::Deref for TIMER4_RST_R {
    type Target = crate::FieldReader<bool, TIMER4_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER4_RST` writer - Timer 4 reset control."]
pub struct TIMER4_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER4_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER4_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER4_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER4_RST_A::ASSERTED)
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
#[doc = "PUF reset control reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUF_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<PUF_RST_A> for bool {
    #[inline(always)]
    fn from(variant: PUF_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUF_RST` reader - PUF reset control reset control."]
pub struct PUF_RST_R(crate::FieldReader<bool, PUF_RST_A>);
impl PUF_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PUF_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUF_RST_A {
        match self.bits {
            false => PUF_RST_A::RELEASED,
            true => PUF_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == PUF_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == PUF_RST_A::ASSERTED
    }
}
impl core::ops::Deref for PUF_RST_R {
    type Target = crate::FieldReader<bool, PUF_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUF_RST` writer - PUF reset control reset control."]
pub struct PUF_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PUF_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUF_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(PUF_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PUF_RST_A::ASSERTED)
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
#[doc = "Casper reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASPER_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<CASPER_RST_A> for bool {
    #[inline(always)]
    fn from(variant: CASPER_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER_RST` reader - Casper reset control."]
pub struct CASPER_RST_R(crate::FieldReader<bool, CASPER_RST_A>);
impl CASPER_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CASPER_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_RST_A {
        match self.bits {
            false => CASPER_RST_A::RELEASED,
            true => CASPER_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == CASPER_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == CASPER_RST_A::ASSERTED
    }
}
impl core::ops::Deref for CASPER_RST_R {
    type Target = crate::FieldReader<bool, CASPER_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CASPER_RST` writer - Casper reset control."]
pub struct CASPER_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CASPER_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CASPER_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(CASPER_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(CASPER_RST_A::ASSERTED)
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
#[doc = "analog control reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANALOG_CTRL_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<ANALOG_CTRL_RST_A> for bool {
    #[inline(always)]
    fn from(variant: ANALOG_CTRL_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANALOG_CTRL_RST` reader - analog control reset control."]
pub struct ANALOG_CTRL_RST_R(crate::FieldReader<bool, ANALOG_CTRL_RST_A>);
impl ANALOG_CTRL_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_CTRL_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANALOG_CTRL_RST_A {
        match self.bits {
            false => ANALOG_CTRL_RST_A::RELEASED,
            true => ANALOG_CTRL_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == ANALOG_CTRL_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == ANALOG_CTRL_RST_A::ASSERTED
    }
}
impl core::ops::Deref for ANALOG_CTRL_RST_R {
    type Target = crate::FieldReader<bool, ANALOG_CTRL_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_CTRL_RST` writer - analog control reset control."]
pub struct ANALOG_CTRL_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_CTRL_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANALOG_CTRL_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_RST_A::ASSERTED)
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
#[doc = "HS LSPI reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_LSPI_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<HS_LSPI_RST_A> for bool {
    #[inline(always)]
    fn from(variant: HS_LSPI_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS_LSPI_RST` reader - HS LSPI reset control."]
pub struct HS_LSPI_RST_R(crate::FieldReader<bool, HS_LSPI_RST_A>);
impl HS_LSPI_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        HS_LSPI_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_LSPI_RST_A {
        match self.bits {
            false => HS_LSPI_RST_A::RELEASED,
            true => HS_LSPI_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == HS_LSPI_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == HS_LSPI_RST_A::ASSERTED
    }
}
impl core::ops::Deref for HS_LSPI_RST_R {
    type Target = crate::FieldReader<bool, HS_LSPI_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS_LSPI_RST` writer - HS LSPI reset control."]
pub struct HS_LSPI_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_LSPI_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_LSPI_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(HS_LSPI_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(HS_LSPI_RST_A::ASSERTED)
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
#[doc = "GPIO secure reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<GPIO_SEC_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_SEC_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_SEC_RST` reader - GPIO secure reset control."]
pub struct GPIO_SEC_RST_R(crate::FieldReader<bool, GPIO_SEC_RST_A>);
impl GPIO_SEC_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_SEC_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_SEC_RST_A {
        match self.bits {
            false => GPIO_SEC_RST_A::RELEASED,
            true => GPIO_SEC_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == GPIO_SEC_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == GPIO_SEC_RST_A::ASSERTED
    }
}
impl core::ops::Deref for GPIO_SEC_RST_R {
    type Target = crate::FieldReader<bool, GPIO_SEC_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_SEC_RST` writer - GPIO secure reset control."]
pub struct GPIO_SEC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SEC_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_SEC_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO_SEC_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO_SEC_RST_A::ASSERTED)
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
#[doc = "GPIO secure int reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_INT_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<GPIO_SEC_INT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_SEC_INT_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_SEC_INT_RST` reader - GPIO secure int reset control."]
pub struct GPIO_SEC_INT_RST_R(crate::FieldReader<bool, GPIO_SEC_INT_RST_A>);
impl GPIO_SEC_INT_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_SEC_INT_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_SEC_INT_RST_A {
        match self.bits {
            false => GPIO_SEC_INT_RST_A::RELEASED,
            true => GPIO_SEC_INT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        **self == GPIO_SEC_INT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        **self == GPIO_SEC_INT_RST_A::ASSERTED
    }
}
impl core::ops::Deref for GPIO_SEC_INT_RST_R {
    type Target = crate::FieldReader<bool, GPIO_SEC_INT_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_SEC_INT_RST` writer - GPIO secure int reset control."]
pub struct GPIO_SEC_INT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_SEC_INT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_SEC_INT_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_RST_A::ASSERTED)
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
    #[doc = "Bit 1 - DMA1 reset control."]
    #[inline(always)]
    pub fn dma1_rst(&self) -> DMA1_RST_R {
        DMA1_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator reset control."]
    #[inline(always)]
    pub fn comp_rst(&self) -> COMP_RST_R {
        COMP_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline(always)]
    pub fn sdio_rst(&self) -> SDIO_RST_R {
        SDIO_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline(always)]
    pub fn usb1_host_rst(&self) -> USB1_HOST_RST_R {
        USB1_HOST_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB1 dev reset control."]
    #[inline(always)]
    pub fn usb1_dev_rst(&self) -> USB1_DEV_RST_R {
        USB1_DEV_RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline(always)]
    pub fn usb1_ram_rst(&self) -> USB1_RAM_RST_R {
        USB1_RAM_RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB1 PHY reset control."]
    #[inline(always)]
    pub fn usb1_phy_rst(&self) -> USB1_PHY_RST_R {
        USB1_PHY_RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Frequency meter reset control."]
    #[inline(always)]
    pub fn freqme_rst(&self) -> FREQME_RST_R {
        FREQME_RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&self) -> RNG_RST_R {
        RNG_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SYSCTL Block reset."]
    #[inline(always)]
    pub fn sysctl_rst(&self) -> SYSCTL_RST_R {
        SYSCTL_RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB0 Host Master reset control."]
    #[inline(always)]
    pub fn usb0_hostm_rst(&self) -> USB0_HOSTM_RST_R {
        USB0_HOSTM_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USB0 Host Slave reset control."]
    #[inline(always)]
    pub fn usb0_hosts_rst(&self) -> USB0_HOSTS_RST_R {
        USB0_HOSTS_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HASH_AES reset control."]
    #[inline(always)]
    pub fn hash_aes_rst(&self) -> HASH_AES_RST_R {
        HASH_AES_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Power Quad reset control."]
    #[inline(always)]
    pub fn pq_rst(&self) -> PQ_RST_R {
        PQ_RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PLU LUT reset control."]
    #[inline(always)]
    pub fn plulut_rst(&self) -> PLULUT_RST_R {
        PLULUT_RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Timer 3 reset control."]
    #[inline(always)]
    pub fn timer3_rst(&self) -> TIMER3_RST_R {
        TIMER3_RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timer 4 reset control."]
    #[inline(always)]
    pub fn timer4_rst(&self) -> TIMER4_RST_R {
        TIMER4_RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PUF reset control reset control."]
    #[inline(always)]
    pub fn puf_rst(&self) -> PUF_RST_R {
        PUF_RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Casper reset control."]
    #[inline(always)]
    pub fn casper_rst(&self) -> CASPER_RST_R {
        CASPER_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27 - analog control reset control."]
    #[inline(always)]
    pub fn analog_ctrl_rst(&self) -> ANALOG_CTRL_RST_R {
        ANALOG_CTRL_RST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - HS LSPI reset control."]
    #[inline(always)]
    pub fn hs_lspi_rst(&self) -> HS_LSPI_RST_R {
        HS_LSPI_RST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - GPIO secure reset control."]
    #[inline(always)]
    pub fn gpio_sec_rst(&self) -> GPIO_SEC_RST_R {
        GPIO_SEC_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - GPIO secure int reset control."]
    #[inline(always)]
    pub fn gpio_sec_int_rst(&self) -> GPIO_SEC_INT_RST_R {
        GPIO_SEC_INT_RST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DMA1 reset control."]
    #[inline(always)]
    pub fn dma1_rst(&mut self) -> DMA1_RST_W {
        DMA1_RST_W { w: self }
    }
    #[doc = "Bit 2 - Comparator reset control."]
    #[inline(always)]
    pub fn comp_rst(&mut self) -> COMP_RST_W {
        COMP_RST_W { w: self }
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline(always)]
    pub fn sdio_rst(&mut self) -> SDIO_RST_W {
        SDIO_RST_W { w: self }
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline(always)]
    pub fn usb1_host_rst(&mut self) -> USB1_HOST_RST_W {
        USB1_HOST_RST_W { w: self }
    }
    #[doc = "Bit 5 - USB1 dev reset control."]
    #[inline(always)]
    pub fn usb1_dev_rst(&mut self) -> USB1_DEV_RST_W {
        USB1_DEV_RST_W { w: self }
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline(always)]
    pub fn usb1_ram_rst(&mut self) -> USB1_RAM_RST_W {
        USB1_RAM_RST_W { w: self }
    }
    #[doc = "Bit 7 - USB1 PHY reset control."]
    #[inline(always)]
    pub fn usb1_phy_rst(&mut self) -> USB1_PHY_RST_W {
        USB1_PHY_RST_W { w: self }
    }
    #[doc = "Bit 8 - Frequency meter reset control."]
    #[inline(always)]
    pub fn freqme_rst(&mut self) -> FREQME_RST_W {
        FREQME_RST_W { w: self }
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&mut self) -> RNG_RST_W {
        RNG_RST_W { w: self }
    }
    #[doc = "Bit 15 - SYSCTL Block reset."]
    #[inline(always)]
    pub fn sysctl_rst(&mut self) -> SYSCTL_RST_W {
        SYSCTL_RST_W { w: self }
    }
    #[doc = "Bit 16 - USB0 Host Master reset control."]
    #[inline(always)]
    pub fn usb0_hostm_rst(&mut self) -> USB0_HOSTM_RST_W {
        USB0_HOSTM_RST_W { w: self }
    }
    #[doc = "Bit 17 - USB0 Host Slave reset control."]
    #[inline(always)]
    pub fn usb0_hosts_rst(&mut self) -> USB0_HOSTS_RST_W {
        USB0_HOSTS_RST_W { w: self }
    }
    #[doc = "Bit 18 - HASH_AES reset control."]
    #[inline(always)]
    pub fn hash_aes_rst(&mut self) -> HASH_AES_RST_W {
        HASH_AES_RST_W { w: self }
    }
    #[doc = "Bit 19 - Power Quad reset control."]
    #[inline(always)]
    pub fn pq_rst(&mut self) -> PQ_RST_W {
        PQ_RST_W { w: self }
    }
    #[doc = "Bit 20 - PLU LUT reset control."]
    #[inline(always)]
    pub fn plulut_rst(&mut self) -> PLULUT_RST_W {
        PLULUT_RST_W { w: self }
    }
    #[doc = "Bit 21 - Timer 3 reset control."]
    #[inline(always)]
    pub fn timer3_rst(&mut self) -> TIMER3_RST_W {
        TIMER3_RST_W { w: self }
    }
    #[doc = "Bit 22 - Timer 4 reset control."]
    #[inline(always)]
    pub fn timer4_rst(&mut self) -> TIMER4_RST_W {
        TIMER4_RST_W { w: self }
    }
    #[doc = "Bit 23 - PUF reset control reset control."]
    #[inline(always)]
    pub fn puf_rst(&mut self) -> PUF_RST_W {
        PUF_RST_W { w: self }
    }
    #[doc = "Bit 24 - Casper reset control."]
    #[inline(always)]
    pub fn casper_rst(&mut self) -> CASPER_RST_W {
        CASPER_RST_W { w: self }
    }
    #[doc = "Bit 27 - analog control reset control."]
    #[inline(always)]
    pub fn analog_ctrl_rst(&mut self) -> ANALOG_CTRL_RST_W {
        ANALOG_CTRL_RST_W { w: self }
    }
    #[doc = "Bit 28 - HS LSPI reset control."]
    #[inline(always)]
    pub fn hs_lspi_rst(&mut self) -> HS_LSPI_RST_W {
        HS_LSPI_RST_W { w: self }
    }
    #[doc = "Bit 29 - GPIO secure reset control."]
    #[inline(always)]
    pub fn gpio_sec_rst(&mut self) -> GPIO_SEC_RST_W {
        GPIO_SEC_RST_W { w: self }
    }
    #[doc = "Bit 30 - GPIO secure int reset control."]
    #[inline(always)]
    pub fn gpio_sec_int_rst(&mut self) -> GPIO_SEC_INT_RST_W {
        GPIO_SEC_INT_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl2](index.html) module"]
pub struct PRESETCTRL2_SPEC;
impl crate::RegisterSpec for PRESETCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl2::R](R) reader structure"]
impl crate::Readable for PRESETCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl2::W](W) writer structure"]
impl crate::Writable for PRESETCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL2 to value 0"]
impl crate::Resettable for PRESETCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
