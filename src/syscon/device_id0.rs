#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DEVICE_ID0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PARTCONFIGR {
    bits: u8,
}
impl PARTCONFIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SRAM_SIZER {
    bits: u8,
}
impl SRAM_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FLASH_SIZER {
    bits: u8,
}
impl FLASH_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ROM_REV_MINORR {
    bits: u8,
}
impl ROM_REV_MINORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MODELNUM_EXTENTIONR {
    bits: u8,
}
impl MODELNUM_EXTENTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - no description available"]
    #[inline]
    pub fn partconfig(&self) -> PARTCONFIGR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PARTCONFIGR { bits }
    }
    #[doc = "Bits 8:11 - no description available"]
    #[inline]
    pub fn sram_size(&self) -> SRAM_SIZER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SRAM_SIZER { bits }
    }
    #[doc = "Bits 12:14 - no description available"]
    #[inline]
    pub fn flash_size(&self) -> FLASH_SIZER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FLASH_SIZER { bits }
    }
    #[doc = "Bits 20:23 - no description available"]
    #[inline]
    pub fn rom_rev_minor(&self) -> ROM_REV_MINORR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ROM_REV_MINORR { bits }
    }
    #[doc = "Bits 24:26 - no description available"]
    #[inline]
    pub fn modelnum_extention(&self) -> MODELNUM_EXTENTIONR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MODELNUM_EXTENTIONR { bits }
    }
}
