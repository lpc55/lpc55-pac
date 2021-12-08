#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUAL` reader - 1 if 2 x 512 bit buffers, 0 if only 1 x 512 bit"]
pub struct DUAL_R(crate::FieldReader<bool, bool>);
impl DUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA` reader - 1 if DMA is connected"]
pub struct DMA_R(crate::FieldReader<bool, bool>);
impl DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB` reader - 1 if AHB Master is enabled"]
pub struct AHB_R(crate::FieldReader<bool, bool>);
impl AHB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES` reader - 1 if AES 128 included"]
pub struct AES_R(crate::FieldReader<bool, bool>);
impl AES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESKEY` reader - 1 if AES 192 and 256 also included"]
pub struct AESKEY_R(crate::FieldReader<bool, bool>);
impl AESKEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AESKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESKEY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECRET` reader - 1 if AES Secret key available"]
pub struct SECRET_R(crate::FieldReader<bool, bool>);
impl SECRET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SECRET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECRET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICB` reader - 1 if ICB over AES included"]
pub struct ICB_R(crate::FieldReader<bool, bool>);
impl ICB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - 1 if 2 x 512 bit buffers, 0 if only 1 x 512 bit"]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1 if DMA is connected"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 if AHB Master is enabled"]
    #[inline(always)]
    pub fn ahb(&self) -> AHB_R {
        AHB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1 if AES 128 included"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1 if AES 192 and 256 also included"]
    #[inline(always)]
    pub fn aeskey(&self) -> AESKEY_R {
        AESKEY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1 if AES Secret key available"]
    #[inline(always)]
    pub fn secret(&self) -> SECRET_R {
        SECRET_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 1 if ICB over AES included"]
    #[inline(always)]
    pub fn icb(&self) -> ICB_R {
        ICB_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Returns the configuration of this block in this chip - indicates what services are available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
