#[doc = "Reader of register DEVICE_ID0"]
pub type R = crate::R<u32, super::DEVICE_ID0>;
#[doc = "Reader of field `PARTCONFIG`"]
pub type PARTCONFIG_R = crate::R<u8, u8>;
#[doc = "Reader of field `SRAM_SIZE`"]
pub type SRAM_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `FLASH_SIZE`"]
pub type FLASH_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `ROM_REV_MINOR`"]
pub type ROM_REV_MINOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `MODELNUM_EXTENTION`"]
pub type MODELNUM_EXTENTION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn partconfig(&self) -> PARTCONFIG_R {
        PARTCONFIG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - no description available"]
    #[inline(always)]
    pub fn sram_size(&self) -> SRAM_SIZE_R {
        SRAM_SIZE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - no description available"]
    #[inline(always)]
    pub fn flash_size(&self) -> FLASH_SIZE_R {
        FLASH_SIZE_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - no description available"]
    #[inline(always)]
    pub fn rom_rev_minor(&self) -> ROM_REV_MINOR_R {
        ROM_REV_MINOR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - no description available"]
    #[inline(always)]
    pub fn modelnum_extention(&self) -> MODELNUM_EXTENTION_R {
        MODELNUM_EXTENTION_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
