#[doc = "Register `PLL1MDEC` reader"]
pub struct R(crate::R<PLL1MDEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1MDEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PLL1MDEC_SPEC>> for R {
    fn from(reader: crate::R<PLL1MDEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL1MDEC` writer"]
pub struct W(crate::W<PLL1MDEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1MDEC_SPEC>;
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
impl core::convert::From<crate::W<PLL1MDEC_SPEC>> for W {
    fn from(writer: crate::W<PLL1MDEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDIV` reader - feedback divider divider ratio (M-divider)."]
pub struct MDIV_R(crate::FieldReader<u16, u16>);
impl MDIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        MDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDIV` writer - feedback divider divider ratio (M-divider)."]
pub struct MDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `MREQ` reader - feedback ratio change request."]
pub struct MREQ_R(crate::FieldReader<bool, bool>);
impl MREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        MREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MREQ` writer - feedback ratio change request."]
pub struct MREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    pub fn mdiv(&self) -> MDIV_R {
        MDIV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - feedback ratio change request."]
    #[inline(always)]
    pub fn mreq(&self) -> MREQ_R {
        MREQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - feedback divider divider ratio (M-divider)."]
    #[inline(always)]
    pub fn mdiv(&mut self) -> MDIV_W {
        MDIV_W { w: self }
    }
    #[doc = "Bit 16 - feedback ratio change request."]
    #[inline(always)]
    pub fn mreq(&mut self) -> MREQ_W {
        MREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL1 550m M divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1mdec](index.html) module"]
pub struct PLL1MDEC_SPEC;
impl crate::RegisterSpec for PLL1MDEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll1mdec::R](R) reader structure"]
impl crate::Readable for PLL1MDEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll1mdec::W](W) writer structure"]
impl crate::Writable for PLL1MDEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL1MDEC to value 0"]
impl crate::Resettable for PLL1MDEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
