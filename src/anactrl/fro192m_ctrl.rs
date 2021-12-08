#[doc = "Register `FRO192M_CTRL` reader"]
pub struct R(crate::R<FRO192M_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRO192M_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRO192M_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRO192M_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRO192M_CTRL` writer"]
pub struct W(crate::W<FRO192M_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRO192M_CTRL_SPEC>;
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
impl From<crate::W<FRO192M_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRO192M_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "12 MHz clock control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_12MHZCLK_A {
    #[doc = "0: 12 MHz clock is disabled."]
    DISABLE = 0,
    #[doc = "1: 12 MHz clock is enabled."]
    ENABLE = 1,
}
impl From<ENA_12MHZCLK_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_12MHZCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA_12MHZCLK` reader - 12 MHz clock control."]
pub struct ENA_12MHZCLK_R(crate::FieldReader<bool, ENA_12MHZCLK_A>);
impl ENA_12MHZCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_12MHZCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_12MHZCLK_A {
        match self.bits {
            false => ENA_12MHZCLK_A::DISABLE,
            true => ENA_12MHZCLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ENA_12MHZCLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ENA_12MHZCLK_A::ENABLE
    }
}
impl core::ops::Deref for ENA_12MHZCLK_R {
    type Target = crate::FieldReader<bool, ENA_12MHZCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_12MHZCLK` writer - 12 MHz clock control."]
pub struct ENA_12MHZCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_12MHZCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_12MHZCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "12 MHz clock is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENA_12MHZCLK_A::DISABLE)
    }
    #[doc = "12 MHz clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_12MHZCLK_A::ENABLE)
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
#[doc = "48 MHz clock control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_48MHZCLK_A {
    #[doc = "1: 48 MHz clock is enabled."]
    ENABLE = 1,
}
impl From<ENA_48MHZCLK_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_48MHZCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA_48MHZCLK` reader - 48 MHz clock control."]
pub struct ENA_48MHZCLK_R(crate::FieldReader<bool, ENA_48MHZCLK_A>);
impl ENA_48MHZCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_48MHZCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENA_48MHZCLK_A> {
        match self.bits {
            true => Some(ENA_48MHZCLK_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ENA_48MHZCLK_A::ENABLE
    }
}
impl core::ops::Deref for ENA_48MHZCLK_R {
    type Target = crate::FieldReader<bool, ENA_48MHZCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_48MHZCLK` writer - 48 MHz clock control."]
pub struct ENA_48MHZCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_48MHZCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_48MHZCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "48 MHz clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_48MHZCLK_A::ENABLE)
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
#[doc = "Field `DAC_TRIM` reader - Frequency trim."]
pub struct DAC_TRIM_R(crate::FieldReader<u8, u8>);
impl DAC_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_TRIM` writer - Frequency trim."]
pub struct DAC_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `USBCLKADJ` reader - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
pub struct USBCLKADJ_R(crate::FieldReader<bool, bool>);
impl USBCLKADJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBCLKADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBCLKADJ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBCLKADJ` writer - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
pub struct USBCLKADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCLKADJ_W<'a> {
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
#[doc = "Field `USBMODCHG` reader - If it reads as 1 when reading the DAC_TRIM field and USBCLKADJ=1, it should be re-read until it is 0."]
pub struct USBMODCHG_R(crate::FieldReader<bool, bool>);
impl USBMODCHG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBMODCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBMODCHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "96 MHz clock control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_96MHZCLK_A {
    #[doc = "0: 96 MHz clock is disabled."]
    DISABLE = 0,
    #[doc = "1: 96 MHz clock is enabled."]
    ENABLE = 1,
}
impl From<ENA_96MHZCLK_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_96MHZCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA_96MHZCLK` reader - 96 MHz clock control."]
pub struct ENA_96MHZCLK_R(crate::FieldReader<bool, ENA_96MHZCLK_A>);
impl ENA_96MHZCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_96MHZCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_96MHZCLK_A {
        match self.bits {
            false => ENA_96MHZCLK_A::DISABLE,
            true => ENA_96MHZCLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ENA_96MHZCLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ENA_96MHZCLK_A::ENABLE
    }
}
impl core::ops::Deref for ENA_96MHZCLK_R {
    type Target = crate::FieldReader<bool, ENA_96MHZCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_96MHZCLK` writer - 96 MHz clock control."]
pub struct ENA_96MHZCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_96MHZCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_96MHZCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "96 MHz clock is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENA_96MHZCLK_A::DISABLE)
    }
    #[doc = "96 MHz clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_96MHZCLK_A::ENABLE)
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
    #[doc = "Bit 14 - 12 MHz clock control."]
    #[inline(always)]
    pub fn ena_12mhzclk(&self) -> ENA_12MHZCLK_R {
        ENA_12MHZCLK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 48 MHz clock control."]
    #[inline(always)]
    pub fn ena_48mhzclk(&self) -> ENA_48MHZCLK_R {
        ENA_48MHZCLK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn dac_trim(&self) -> DAC_TRIM_R {
        DAC_TRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline(always)]
    pub fn usbclkadj(&self) -> USBCLKADJ_R {
        USBCLKADJ_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - If it reads as 1 when reading the DAC_TRIM field and USBCLKADJ=1, it should be re-read until it is 0."]
    #[inline(always)]
    pub fn usbmodchg(&self) -> USBMODCHG_R {
        USBMODCHG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline(always)]
    pub fn ena_96mhzclk(&self) -> ENA_96MHZCLK_R {
        ENA_96MHZCLK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - 12 MHz clock control."]
    #[inline(always)]
    pub fn ena_12mhzclk(&mut self) -> ENA_12MHZCLK_W {
        ENA_12MHZCLK_W { w: self }
    }
    #[doc = "Bit 15 - 48 MHz clock control."]
    #[inline(always)]
    pub fn ena_48mhzclk(&mut self) -> ENA_48MHZCLK_W {
        ENA_48MHZCLK_W { w: self }
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn dac_trim(&mut self) -> DAC_TRIM_W {
        DAC_TRIM_W { w: self }
    }
    #[doc = "Bit 24 - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline(always)]
    pub fn usbclkadj(&mut self) -> USBCLKADJ_W {
        USBCLKADJ_W { w: self }
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline(always)]
    pub fn ena_96mhzclk(&mut self) -> ENA_96MHZCLK_W {
        ENA_96MHZCLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "192MHz Free Running OScillator (FRO) Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fro192m_ctrl](index.html) module"]
pub struct FRO192M_CTRL_SPEC;
impl crate::RegisterSpec for FRO192M_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fro192m_ctrl::R](R) reader structure"]
impl crate::Readable for FRO192M_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fro192m_ctrl::W](W) writer structure"]
impl crate::Writable for FRO192M_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRO192M_CTRL to value 0x0080_d01a"]
impl crate::Resettable for FRO192M_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_d01a
    }
}
