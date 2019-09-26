#[doc = "Reader of register SPI_FLASH_CFG"]
pub type R = crate::R<u32, super::SPI_FLASH_CFG>;
#[doc = "Writer for register SPI_FLASH_CFG"]
pub type W = crate::W<u32, super::SPI_FLASH_CFG>;
#[doc = "Register SPI_FLASH_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_FLASH_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_RECOVERY_BOOT_EN`"]
pub type SPI_RECOVERY_BOOT_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_RECOVERY_BOOT_EN`"]
pub struct SPI_RECOVERY_BOOT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RECOVERY_BOOT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
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
}
