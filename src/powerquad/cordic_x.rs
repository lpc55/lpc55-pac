#[doc = "Register `CORDIC_X` reader"]
pub struct R(crate::R<CORDIC_X_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORDIC_X_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORDIC_X_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORDIC_X_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORDIC_X` writer"]
pub struct W(crate::W<CORDIC_X_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORDIC_X_SPEC>;
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
impl From<crate::W<CORDIC_X_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORDIC_X_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cordic_x` reader - Cordic input x"]
pub struct CORDIC_X_R(crate::FieldReader<u32, u32>);
impl CORDIC_X_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CORDIC_X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORDIC_X_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cordic_x` writer - Cordic input x"]
pub struct CORDIC_X_W<'a> {
    w: &'a mut W,
}
impl<'a> CORDIC_X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Cordic input x"]
    #[inline(always)]
    pub fn cordic_x(&self) -> CORDIC_X_R {
        CORDIC_X_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cordic input x"]
    #[inline(always)]
    pub fn cordic_x(&mut self) -> CORDIC_X_W {
        CORDIC_X_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cordic input X register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cordic_x](index.html) module"]
pub struct CORDIC_X_SPEC;
impl crate::RegisterSpec for CORDIC_X_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cordic_x::R](R) reader structure"]
impl crate::Readable for CORDIC_X_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cordic_x::W](W) writer structure"]
impl crate::Writable for CORDIC_X_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORDIC_X to value 0"]
impl crate::Resettable for CORDIC_X_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
