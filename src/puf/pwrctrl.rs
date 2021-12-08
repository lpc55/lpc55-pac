#[doc = "Register `PWRCTRL` reader"]
pub struct R(crate::R<PWRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCTRL` writer"]
pub struct W(crate::W<PWRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCTRL_SPEC>;
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
impl From<crate::W<PWRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMON` reader - Power on the PUF RAM."]
pub struct RAMON_R(crate::FieldReader<bool, bool>);
impl RAMON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMON` writer - Power on the PUF RAM."]
pub struct RAMON_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMON_W<'a> {
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
#[doc = "Field `RAMSTAT` reader - PUF RAM status."]
pub struct RAMSTAT_R(crate::FieldReader<bool, bool>);
impl RAMSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMSTAT` writer - PUF RAM status."]
pub struct RAMSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMSTAT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Power on the PUF RAM."]
    #[inline(always)]
    pub fn ramon(&self) -> RAMON_R {
        RAMON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PUF RAM status."]
    #[inline(always)]
    pub fn ramstat(&self) -> RAMSTAT_R {
        RAMSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power on the PUF RAM."]
    #[inline(always)]
    pub fn ramon(&mut self) -> RAMON_W {
        RAMON_W { w: self }
    }
    #[doc = "Bit 1 - PUF RAM status."]
    #[inline(always)]
    pub fn ramstat(&mut self) -> RAMSTAT_W {
        RAMSTAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF RAM Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrctrl](index.html) module"]
pub struct PWRCTRL_SPEC;
impl crate::RegisterSpec for PWRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrctrl::R](R) reader structure"]
impl crate::Readable for PWRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrctrl::W](W) writer structure"]
impl crate::Writable for PWRCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRCTRL to value 0xf8"]
impl crate::Resettable for PWRCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf8
    }
}
