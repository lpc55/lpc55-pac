#[doc = "Register `COUNTER_CFG` reader"]
pub struct R(crate::R<COUNTER_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNTER_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<COUNTER_CFG_SPEC>> for R {
    fn from(reader: crate::R<COUNTER_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNTER_CFG` writer"]
pub struct W(crate::W<COUNTER_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNTER_CFG_SPEC>;
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
impl core::convert::From<crate::W<COUNTER_CFG_SPEC>> for W {
    fn from(writer: crate::W<COUNTER_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - 00: disabled 01: update once."]
pub struct MODE_R(crate::FieldReader<u8, u8>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - 00: disabled 01: update once."]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CLOCK_SEL` reader - Selects the internal clock on which to compute statistics."]
pub struct CLOCK_SEL_R(crate::FieldReader<u8, u8>);
impl CLOCK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLOCK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOCK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOCK_SEL` writer - Selects the internal clock on which to compute statistics."]
pub struct CLOCK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `SHIFT4X` reader - To be used to add precision to clock_ratio and determine 'entropy refill'."]
pub struct SHIFT4X_R(crate::FieldReader<u8, u8>);
impl SHIFT4X_R {
    pub(crate) fn new(bits: u8) -> Self {
        SHIFT4X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFT4X_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFT4X` writer - To be used to add precision to clock_ratio and determine 'entropy refill'."]
pub struct SHIFT4X_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT4X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - 00: disabled 01: update once."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Selects the internal clock on which to compute statistics."]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:7 - To be used to add precision to clock_ratio and determine 'entropy refill'."]
    #[inline(always)]
    pub fn shift4x(&self) -> SHIFT4X_R {
        SHIFT4X_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00: disabled 01: update once."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 2:4 - Selects the internal clock on which to compute statistics."]
    #[inline(always)]
    pub fn clock_sel(&mut self) -> CLOCK_SEL_W {
        CLOCK_SEL_W { w: self }
    }
    #[doc = "Bits 5:7 - To be used to add precision to clock_ratio and determine 'entropy refill'."]
    #[inline(always)]
    pub fn shift4x(&mut self) -> SHIFT4X_W {
        SHIFT4X_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter_cfg](index.html) module"]
pub struct COUNTER_CFG_SPEC;
impl crate::RegisterSpec for COUNTER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [counter_cfg::R](R) reader structure"]
impl crate::Readable for COUNTER_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [counter_cfg::W](W) writer structure"]
impl crate::Writable for COUNTER_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COUNTER_CFG to value 0"]
impl crate::Resettable for COUNTER_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
