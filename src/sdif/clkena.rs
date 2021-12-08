#[doc = "Register `CLKENA` reader"]
pub struct R(crate::R<CLKENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKENA` writer"]
pub struct W(crate::W<CLKENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKENA_SPEC>;
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
impl From<crate::W<CLKENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCLK0_ENABLE` reader - Clock-enable control for SD card 0 clock."]
pub struct CCLK0_ENABLE_R(crate::FieldReader<bool, bool>);
impl CCLK0_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCLK0_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK0_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLK0_ENABLE` writer - Clock-enable control for SD card 0 clock."]
pub struct CCLK0_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK0_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CCLK1_ENABLE` reader - Clock-enable control for SD card 1 clock."]
pub struct CCLK1_ENABLE_R(crate::FieldReader<bool, bool>);
impl CCLK1_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCLK1_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK1_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLK1_ENABLE` writer - Clock-enable control for SD card 1 clock."]
pub struct CCLK1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK1_ENABLE_W<'a> {
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
#[doc = "Field `CCLK0_LOW_POWER` reader - Low-power control for SD card 0 clock."]
pub struct CCLK0_LOW_POWER_R(crate::FieldReader<bool, bool>);
impl CCLK0_LOW_POWER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCLK0_LOW_POWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK0_LOW_POWER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLK0_LOW_POWER` writer - Low-power control for SD card 0 clock."]
pub struct CCLK0_LOW_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK0_LOW_POWER_W<'a> {
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
#[doc = "Field `CCLK1_LOW_POWER` reader - Low-power control for SD card 1 clock."]
pub struct CCLK1_LOW_POWER_R(crate::FieldReader<bool, bool>);
impl CCLK1_LOW_POWER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCLK1_LOW_POWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK1_LOW_POWER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLK1_LOW_POWER` writer - Low-power control for SD card 1 clock."]
pub struct CCLK1_LOW_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK1_LOW_POWER_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Clock-enable control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_enable(&self) -> CCLK0_ENABLE_R {
        CCLK0_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock-enable control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_enable(&self) -> CCLK1_ENABLE_R {
        CCLK1_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Low-power control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_low_power(&self) -> CCLK0_LOW_POWER_R {
        CCLK0_LOW_POWER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Low-power control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_low_power(&self) -> CCLK1_LOW_POWER_R {
        CCLK1_LOW_POWER_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock-enable control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_enable(&mut self) -> CCLK0_ENABLE_W {
        CCLK0_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Clock-enable control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_enable(&mut self) -> CCLK1_ENABLE_W {
        CCLK1_ENABLE_W { w: self }
    }
    #[doc = "Bit 16 - Low-power control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_low_power(&mut self) -> CCLK0_LOW_POWER_W {
        CCLK0_LOW_POWER_W { w: self }
    }
    #[doc = "Bit 17 - Low-power control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_low_power(&mut self) -> CCLK1_LOW_POWER_W {
        CCLK1_LOW_POWER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkena](index.html) module"]
pub struct CLKENA_SPEC;
impl crate::RegisterSpec for CLKENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkena::R](R) reader structure"]
impl crate::Readable for CLKENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkena::W](W) writer structure"]
impl crate::Writable for CLKENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKENA to value 0"]
impl crate::Resettable for CLKENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
