#[doc = "Register `USB1NEEDCLKSTAT` reader"]
pub struct R(crate::R<USB1NEEDCLKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1NEEDCLKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<USB1NEEDCLKSTAT_SPEC>> for R {
    fn from(reader: crate::R<USB1NEEDCLKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1NEEDCLKSTAT` writer"]
pub struct W(crate::W<USB1NEEDCLKSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1NEEDCLKSTAT_SPEC>;
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
impl core::convert::From<crate::W<USB1NEEDCLKSTAT_SPEC>> for W {
    fn from(writer: crate::W<USB1NEEDCLKSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB1 Device need_clock signal status:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_NEEDCLK_A {
    #[doc = "0: DEV_NEEDCLK is low."]
    LOW = 0,
    #[doc = "1: DEV_NEEDCLK is high."]
    HIGH = 1,
}
impl From<DEV_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_NEEDCLK` reader - USB1 Device need_clock signal status:."]
pub struct DEV_NEEDCLK_R(crate::FieldReader<bool, DEV_NEEDCLK_A>);
impl DEV_NEEDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEV_NEEDCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEV_NEEDCLK_A {
        match self.bits {
            false => DEV_NEEDCLK_A::LOW,
            true => DEV_NEEDCLK_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == DEV_NEEDCLK_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == DEV_NEEDCLK_A::HIGH
    }
}
impl core::ops::Deref for DEV_NEEDCLK_R {
    type Target = crate::FieldReader<bool, DEV_NEEDCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USB1 Host need_clock signal status:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_NEEDCLK_A {
    #[doc = "0: HOST_NEEDCLK is low."]
    LOW = 0,
    #[doc = "1: HOST_NEEDCLK is high."]
    HIGH = 1,
}
impl From<HOST_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST_NEEDCLK` reader - USB1 Host need_clock signal status:."]
pub struct HOST_NEEDCLK_R(crate::FieldReader<bool, HOST_NEEDCLK_A>);
impl HOST_NEEDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        HOST_NEEDCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOST_NEEDCLK_A {
        match self.bits {
            false => HOST_NEEDCLK_A::LOW,
            true => HOST_NEEDCLK_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == HOST_NEEDCLK_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == HOST_NEEDCLK_A::HIGH
    }
}
impl core::ops::Deref for HOST_NEEDCLK_R {
    type Target = crate::FieldReader<bool, HOST_NEEDCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - USB1 Device need_clock signal status:."]
    #[inline(always)]
    pub fn dev_needclk(&self) -> DEV_NEEDCLK_R {
        DEV_NEEDCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB1 Host need_clock signal status:."]
    #[inline(always)]
    pub fn host_needclk(&self) -> HOST_NEEDCLK_R {
        HOST_NEEDCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB1 need clock status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1needclkstat](index.html) module"]
pub struct USB1NEEDCLKSTAT_SPEC;
impl crate::RegisterSpec for USB1NEEDCLKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1needclkstat::R](R) reader structure"]
impl crate::Readable for USB1NEEDCLKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1needclkstat::W](W) writer structure"]
impl crate::Writable for USB1NEEDCLKSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB1NEEDCLKSTAT to value 0"]
impl crate::Resettable for USB1NEEDCLKSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
