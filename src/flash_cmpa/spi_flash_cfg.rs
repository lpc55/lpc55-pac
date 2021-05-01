#[doc = "Register `SPI_FLASH_CFG` reader"]
pub struct R(crate::R<SPI_FLASH_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FLASH_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPI_FLASH_CFG_SPEC>> for R {
    fn from(reader: crate::R<SPI_FLASH_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_FLASH_CFG` writer"]
pub struct W(crate::W<SPI_FLASH_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_FLASH_CFG_SPEC>;
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
impl core::convert::From<crate::W<SPI_FLASH_CFG_SPEC>> for W {
    fn from(writer: crate::W<SPI_FLASH_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_RECOVERY_BOOT_EN` reader - SPI flash recovery boot is enabled, if non-zero value is written to this field."]
pub struct SPI_RECOVERY_BOOT_EN_R(crate::FieldReader<u8, u8>);
impl SPI_RECOVERY_BOOT_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI_RECOVERY_BOOT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_RECOVERY_BOOT_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_RECOVERY_BOOT_EN` writer - SPI flash recovery boot is enabled, if non-zero value is written to this field."]
pub struct SPI_RECOVERY_BOOT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RECOVERY_BOOT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - SPI flash recovery boot is enabled, if non-zero value is written to this field."]
    #[inline(always)]
    pub fn spi_recovery_boot_en(&self) -> SPI_RECOVERY_BOOT_EN_R {
        SPI_RECOVERY_BOOT_EN_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SPI flash recovery boot is enabled, if non-zero value is written to this field."]
    #[inline(always)]
    pub fn spi_recovery_boot_en(&mut self) -> SPI_RECOVERY_BOOT_EN_W {
        SPI_RECOVERY_BOOT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_flash_cfg](index.html) module"]
pub struct SPI_FLASH_CFG_SPEC;
impl crate::RegisterSpec for SPI_FLASH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_flash_cfg::R](R) reader structure"]
impl crate::Readable for SPI_FLASH_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_flash_cfg::W](W) writer structure"]
impl crate::Writable for SPI_FLASH_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_FLASH_CFG to value 0"]
impl crate::Resettable for SPI_FLASH_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
