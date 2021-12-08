#[doc = "Register `PLL0PDEC` reader"]
pub struct R(crate::R<PLL0PDEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL0PDEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL0PDEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL0PDEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL0PDEC` writer"]
pub struct W(crate::W<PLL0PDEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL0PDEC_SPEC>;
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
impl From<crate::W<PLL0PDEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL0PDEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDIV` reader - post-divider divider ratio (P-divider)"]
pub struct PDIV_R(crate::FieldReader<u8, u8>);
impl PDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDIV` writer - post-divider divider ratio (P-divider)"]
pub struct PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `PREQ` reader - feedback ratio change request."]
pub struct PREQ_R(crate::FieldReader<bool, bool>);
impl PREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREQ` writer - feedback ratio change request."]
pub struct PREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - post-divider divider ratio (P-divider)"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - feedback ratio change request."]
    #[inline(always)]
    pub fn preq(&self) -> PREQ_R {
        PREQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - post-divider divider ratio (P-divider)"]
    #[inline(always)]
    pub fn pdiv(&mut self) -> PDIV_W {
        PDIV_W { w: self }
    }
    #[doc = "Bit 5 - feedback ratio change request."]
    #[inline(always)]
    pub fn preq(&mut self) -> PREQ_W {
        PREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL0 550m P divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll0pdec](index.html) module"]
pub struct PLL0PDEC_SPEC;
impl crate::RegisterSpec for PLL0PDEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll0pdec::R](R) reader structure"]
impl crate::Readable for PLL0PDEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll0pdec::W](W) writer structure"]
impl crate::Writable for PLL0PDEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL0PDEC to value 0"]
impl crate::Resettable for PLL0PDEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
