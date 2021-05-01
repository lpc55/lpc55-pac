#[doc = "Register `PLL1NDEC` reader"]
pub struct R(crate::R<PLL1NDEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1NDEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PLL1NDEC_SPEC>> for R {
    fn from(reader: crate::R<PLL1NDEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL1NDEC` writer"]
pub struct W(crate::W<PLL1NDEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1NDEC_SPEC>;
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
impl core::convert::From<crate::W<PLL1NDEC_SPEC>> for W {
    fn from(writer: crate::W<PLL1NDEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDIV` reader - pre-divider divider ratio (N-divider)."]
pub struct NDIV_R(crate::FieldReader<u8, u8>);
impl NDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        NDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDIV` writer - pre-divider divider ratio (N-divider)."]
pub struct NDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> NDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `NREQ` reader - pre-divider ratio change request."]
pub struct NREQ_R(crate::FieldReader<bool, bool>);
impl NREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        NREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NREQ` writer - pre-divider ratio change request."]
pub struct NREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> NREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - pre-divider ratio change request."]
    #[inline(always)]
    pub fn nreq(&self) -> NREQ_R {
        NREQ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    pub fn ndiv(&mut self) -> NDIV_W {
        NDIV_W { w: self }
    }
    #[doc = "Bit 8 - pre-divider ratio change request."]
    #[inline(always)]
    pub fn nreq(&mut self) -> NREQ_W {
        NREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL1 550m N divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1ndec](index.html) module"]
pub struct PLL1NDEC_SPEC;
impl crate::RegisterSpec for PLL1NDEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll1ndec::R](R) reader structure"]
impl crate::Readable for PLL1NDEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll1ndec::W](W) writer structure"]
impl crate::Writable for PLL1NDEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL1NDEC to value 0"]
impl crate::Resettable for PLL1NDEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
