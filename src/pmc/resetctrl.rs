#[doc = "Register `RESETCTRL` reader"]
pub struct R(crate::R<RESETCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESETCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RESETCTRL_SPEC>> for R {
    fn from(reader: crate::R<RESETCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESETCTRL` writer"]
pub struct W(crate::W<RESETCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESETCTRL_SPEC>;
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
impl core::convert::From<crate::W<RESETCTRL_SPEC>> for W {
    fn from(writer: crate::W<RESETCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDWAKEUPRESETENABLE_A {
    #[doc = "0: Reset event from DEEP POWER DOWN mode is disable."]
    DISABLE = 0,
    #[doc = "1: Reset event from DEEP POWER DOWN mode is enable."]
    ENABLE = 1,
}
impl From<DPDWAKEUPRESETENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DPDWAKEUPRESETENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPDWAKEUPRESETENABLE` reader - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
pub struct DPDWAKEUPRESETENABLE_R(crate::FieldReader<bool, DPDWAKEUPRESETENABLE_A>);
impl DPDWAKEUPRESETENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPDWAKEUPRESETENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPDWAKEUPRESETENABLE_A {
        match self.bits {
            false => DPDWAKEUPRESETENABLE_A::DISABLE,
            true => DPDWAKEUPRESETENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DPDWAKEUPRESETENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DPDWAKEUPRESETENABLE_A::ENABLE
    }
}
impl core::ops::Deref for DPDWAKEUPRESETENABLE_R {
    type Target = crate::FieldReader<bool, DPDWAKEUPRESETENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPDWAKEUPRESETENABLE` writer - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
pub struct DPDWAKEUPRESETENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPDWAKEUPRESETENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPDWAKEUPRESETENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset event from DEEP POWER DOWN mode is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DPDWAKEUPRESETENABLE_A::DISABLE)
    }
    #[doc = "Reset event from DEEP POWER DOWN mode is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DPDWAKEUPRESETENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "BOD VBAT reset enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBATRESETENABLE_A {
    #[doc = "0: BOD VBAT reset is disable."]
    DISABLE = 0,
    #[doc = "1: BOD VBAT reset is enable."]
    ENABLE = 1,
}
impl From<BODVBATRESETENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BODVBATRESETENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODVBATRESETENABLE` reader - BOD VBAT reset enable."]
pub struct BODVBATRESETENABLE_R(crate::FieldReader<bool, BODVBATRESETENABLE_A>);
impl BODVBATRESETENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODVBATRESETENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODVBATRESETENABLE_A {
        match self.bits {
            false => BODVBATRESETENABLE_A::DISABLE,
            true => BODVBATRESETENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BODVBATRESETENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BODVBATRESETENABLE_A::ENABLE
    }
}
impl core::ops::Deref for BODVBATRESETENABLE_R {
    type Target = crate::FieldReader<bool, BODVBATRESETENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODVBATRESETENABLE` writer - BOD VBAT reset enable."]
pub struct BODVBATRESETENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BODVBATRESETENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODVBATRESETENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BOD VBAT reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODVBATRESETENABLE_A::DISABLE)
    }
    #[doc = "BOD VBAT reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODVBATRESETENABLE_A::ENABLE)
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
#[doc = "Software reset enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRRESETENABLE_A {
    #[doc = "0: Software reset is disable."]
    DISABLE = 0,
    #[doc = "1: Software reset is enable."]
    ENABLE = 1,
}
impl From<SWRRESETENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: SWRRESETENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRRESETENABLE` reader - Software reset enable."]
pub struct SWRRESETENABLE_R(crate::FieldReader<bool, SWRRESETENABLE_A>);
impl SWRRESETENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRRESETENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRRESETENABLE_A {
        match self.bits {
            false => SWRRESETENABLE_A::DISABLE,
            true => SWRRESETENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SWRRESETENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SWRRESETENABLE_A::ENABLE
    }
}
impl core::ops::Deref for SWRRESETENABLE_R {
    type Target = crate::FieldReader<bool, SWRRESETENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRRESETENABLE` writer - Software reset enable."]
pub struct SWRRESETENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRRESETENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRRESETENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Software reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SWRRESETENABLE_A::DISABLE)
    }
    #[doc = "Software reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SWRRESETENABLE_A::ENABLE)
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
impl R {
    #[doc = "Bit 0 - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline(always)]
    pub fn dpdwakeupresetenable(&self) -> DPDWAKEUPRESETENABLE_R {
        DPDWAKEUPRESETENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BOD VBAT reset enable."]
    #[inline(always)]
    pub fn bodvbatresetenable(&self) -> BODVBATRESETENABLE_R {
        BODVBATRESETENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software reset enable."]
    #[inline(always)]
    pub fn swrresetenable(&self) -> SWRRESETENABLE_R {
        SWRRESETENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline(always)]
    pub fn dpdwakeupresetenable(&mut self) -> DPDWAKEUPRESETENABLE_W {
        DPDWAKEUPRESETENABLE_W { w: self }
    }
    #[doc = "Bit 1 - BOD VBAT reset enable."]
    #[inline(always)]
    pub fn bodvbatresetenable(&mut self) -> BODVBATRESETENABLE_W {
        BODVBATRESETENABLE_W { w: self }
    }
    #[doc = "Bit 3 - Software reset enable."]
    #[inline(always)]
    pub fn swrresetenable(&mut self) -> SWRRESETENABLE_W {
        SWRRESETENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetctrl](index.html) module"]
pub struct RESETCTRL_SPEC;
impl crate::RegisterSpec for RESETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resetctrl::R](R) reader structure"]
impl crate::Readable for RESETCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resetctrl::W](W) writer structure"]
impl crate::Writable for RESETCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESETCTRL to value 0"]
impl crate::Resettable for RESETCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
