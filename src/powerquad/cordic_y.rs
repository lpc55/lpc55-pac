#[doc = "Register `CORDIC_Y` reader"]
pub struct R(crate::R<CORDIC_Y_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORDIC_Y_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORDIC_Y_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORDIC_Y_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORDIC_Y` writer"]
pub struct W(crate::W<CORDIC_Y_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORDIC_Y_SPEC>;
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
impl From<crate::W<CORDIC_Y_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORDIC_Y_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cordic_y` reader - Cordic input y"]
pub struct CORDIC_Y_R(crate::FieldReader<u32, u32>);
impl CORDIC_Y_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CORDIC_Y_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORDIC_Y_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cordic_y` writer - Cordic input y"]
pub struct CORDIC_Y_W<'a> {
    w: &'a mut W,
}
impl<'a> CORDIC_Y_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Cordic input y"]
    #[inline(always)]
    pub fn cordic_y(&self) -> CORDIC_Y_R {
        CORDIC_Y_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cordic input y"]
    #[inline(always)]
    pub fn cordic_y(&mut self) -> CORDIC_Y_W {
        CORDIC_Y_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cordic input Y register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cordic_y](index.html) module"]
pub struct CORDIC_Y_SPEC;
impl crate::RegisterSpec for CORDIC_Y_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cordic_y::R](R) reader structure"]
impl crate::Readable for CORDIC_Y_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cordic_y::W](W) writer structure"]
impl crate::Writable for CORDIC_Y_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORDIC_Y to value 0"]
impl crate::Resettable for CORDIC_Y_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
