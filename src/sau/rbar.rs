#[doc = "Register `RBAR` reader"]
pub struct R(crate::R<RBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RBAR_SPEC>> for R {
    fn from(reader: crate::R<RBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBAR` writer"]
pub struct W(crate::W<RBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBAR_SPEC>;
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
impl core::convert::From<crate::W<RBAR_SPEC>> for W {
    fn from(writer: crate::W<RBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BADDR` reader - Base address. Holds bits\\[31:5\\]
of the base address for the selected SAU region. Bits\\[4:0\\]
of the base address are defined as 0x00."]
pub struct BADDR_R(crate::FieldReader<u32, u32>);
impl BADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        BADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BADDR` writer - Base address. Holds bits\\[31:5\\]
of the base address for the selected SAU region. Bits\\[4:0\\]
of the base address are defined as 0x00."]
pub struct BADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | ((value as u32 & 0x07ff_ffff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:31 - Base address. Holds bits\\[31:5\\]
of the base address for the selected SAU region. Bits\\[4:0\\]
of the base address are defined as 0x00."]
    #[inline(always)]
    pub fn baddr(&self) -> BADDR_R {
        BADDR_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 5:31 - Base address. Holds bits\\[31:5\\]
of the base address for the selected SAU region. Bits\\[4:0\\]
of the base address are defined as 0x00."]
    #[inline(always)]
    pub fn baddr(&mut self) -> BADDR_W {
        BADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Attribution Unit Region Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbar](index.html) module"]
pub struct RBAR_SPEC;
impl crate::RegisterSpec for RBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbar::R](R) reader structure"]
impl crate::Readable for RBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbar::W](W) writer structure"]
impl crate::Writable for RBAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RBAR to value 0"]
impl crate::Resettable for RBAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
