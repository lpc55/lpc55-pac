#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INFO {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FRAME_NRR {
    bits: u16,
}
impl FRAME_NRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ERR_CODER {
    bits: u8,
}
impl ERR_CODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MINREVR {
    bits: u8,
}
impl MINREVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAJREVR {
    bits: u8,
}
impl MAJREVR {
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
    #[doc = "Bits 0:10 - Frame number."]
    #[inline]
    pub fn frame_nr(&self) -> FRAME_NRR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FRAME_NRR { bits }
    }
    #[doc = "Bits 11:14 - The error code which last occurred:."]
    #[inline]
    pub fn err_code(&self) -> ERR_CODER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ERR_CODER { bits }
    }
    #[doc = "Bits 16:23 - Minor revision."]
    #[inline]
    pub fn minrev(&self) -> MINREVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MINREVR { bits }
    }
    #[doc = "Bits 24:31 - Major revision."]
    #[inline]
    pub fn majrev(&self) -> MAJREVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAJREVR { bits }
    }
}
