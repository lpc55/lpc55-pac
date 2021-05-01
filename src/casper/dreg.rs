#[doc = "Register `DREG` reader"]
pub struct R(crate::R<DREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DREG_SPEC>> for R {
    fn from(reader: crate::R<DREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DREG` writer"]
pub struct W(crate::W<DREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DREG_SPEC>;
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
impl core::convert::From<crate::W<DREG_SPEC>> for W {
    fn from(writer: crate::W<DREG_SPEC>) -> Self {
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
#[doc = "D register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dreg](index.html) module"]
pub struct DREG_SPEC;
impl crate::RegisterSpec for DREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dreg::R](R) reader structure"]
impl crate::Readable for DREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dreg::W](W) writer structure"]
impl crate::Writable for DREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DREG to value 0"]
impl crate::Resettable for DREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
