#[doc = "Register `BREG` reader"]
pub struct R(crate::R<BREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BREG_SPEC>> for R {
    fn from(reader: crate::R<BREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BREG` writer"]
pub struct W(crate::W<BREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BREG_SPEC>;
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
impl core::convert::From<crate::W<BREG_SPEC>> for W {
    fn from(writer: crate::W<BREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_VALUE` reader - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
pub struct REG_VALUE_R(crate::FieldReader<u32, u32>);
impl REG_VALUE_R {
    pub(crate) fn new(bits: u32) -> Self {
        REG_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_VALUE` writer - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
pub struct REG_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub fn reg_value(&self) -> REG_VALUE_R {
        REG_VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub fn reg_value(&mut self) -> REG_VALUE_W {
        REG_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "B register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [breg](index.html) module"]
pub struct BREG_SPEC;
impl crate::RegisterSpec for BREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [breg::R](R) reader structure"]
impl crate::Readable for BREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [breg::W](W) writer structure"]
impl crate::Writable for BREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BREG to value 0"]
impl crate::Resettable for BREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
