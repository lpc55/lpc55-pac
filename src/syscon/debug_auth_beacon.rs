#[doc = "Register `DEBUG_AUTH_BEACON` reader"]
pub struct R(crate::R<DEBUG_AUTH_BEACON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_AUTH_BEACON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DEBUG_AUTH_BEACON_SPEC>> for R {
    fn from(reader: crate::R<DEBUG_AUTH_BEACON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_AUTH_BEACON` writer"]
pub struct W(crate::W<DEBUG_AUTH_BEACON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_AUTH_BEACON_SPEC>;
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
impl core::convert::From<crate::W<DEBUG_AUTH_BEACON_SPEC>> for W {
    fn from(writer: crate::W<DEBUG_AUTH_BEACON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BEACON` reader - Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
pub struct BEACON_R(crate::FieldReader<u32, u32>);
impl BEACON_R {
    pub(crate) fn new(bits: u32) -> Self {
        BEACON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEACON_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEACON` writer - Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
pub struct BEACON_W<'a> {
    w: &'a mut W,
}
impl<'a> BEACON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
    #[inline(always)]
    pub fn beacon(&self) -> BEACON_R {
        BEACON_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
    #[inline(always)]
    pub fn beacon(&mut self) -> BEACON_W {
        BEACON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug authentication BEACON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_auth_beacon](index.html) module"]
pub struct DEBUG_AUTH_BEACON_SPEC;
impl crate::RegisterSpec for DEBUG_AUTH_BEACON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_auth_beacon::R](R) reader structure"]
impl crate::Readable for DEBUG_AUTH_BEACON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_auth_beacon::W](W) writer structure"]
impl crate::Writable for DEBUG_AUTH_BEACON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG_AUTH_BEACON to value 0"]
impl crate::Resettable for DEBUG_AUTH_BEACON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
