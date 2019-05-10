#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PARAM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TRIG_NUMR {
    bits: u8,
}
impl TRIG_NUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FIFOSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOSIZER {
    #[doc = "Result FIFO depth = 1 dataword."]
    FIFOSIZE_1,
    #[doc = "Result FIFO depth = 4 datawords."]
    FIFOSIZE_4,
    #[doc = "Result FIFO depth = 8 datawords."]
    FIFOSIZE_8,
    #[doc = "Result FIFO depth = 16 datawords."]
    FIFOSIZE_16,
    #[doc = "Result FIFO depth = 32 datawords."]
    FIFOSIZE_32,
    #[doc = "Result FIFO depth = 64 datawords."]
    FIFOSIZE_64,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FIFOSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FIFOSIZER::FIFOSIZE_1 => 1,
            FIFOSIZER::FIFOSIZE_4 => 4,
            FIFOSIZER::FIFOSIZE_8 => 8,
            FIFOSIZER::FIFOSIZE_16 => 16,
            FIFOSIZER::FIFOSIZE_32 => 32,
            FIFOSIZER::FIFOSIZE_64 => 64,
            FIFOSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FIFOSIZER {
        match value {
            1 => FIFOSIZER::FIFOSIZE_1,
            4 => FIFOSIZER::FIFOSIZE_4,
            8 => FIFOSIZER::FIFOSIZE_8,
            16 => FIFOSIZER::FIFOSIZE_16,
            32 => FIFOSIZER::FIFOSIZE_32,
            64 => FIFOSIZER::FIFOSIZE_64,
            i => FIFOSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_1`"]
    #[inline]
    pub fn is_fifosize_1(&self) -> bool {
        *self == FIFOSIZER::FIFOSIZE_1
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_4`"]
    #[inline]
    pub fn is_fifosize_4(&self) -> bool {
        *self == FIFOSIZER::FIFOSIZE_4
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_8`"]
    #[inline]
    pub fn is_fifosize_8(&self) -> bool {
        *self == FIFOSIZER::FIFOSIZE_8
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_16`"]
    #[inline]
    pub fn is_fifosize_16(&self) -> bool {
        *self == FIFOSIZER::FIFOSIZE_16
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_32`"]
    #[inline]
    pub fn is_fifosize_32(&self) -> bool {
        *self == FIFOSIZER::FIFOSIZE_32
    }
    #[doc = "Checks if the value of the field is `FIFOSIZE_64`"]
    #[inline]
    pub fn is_fifosize_64(&self) -> bool {
        *self == FIFOSIZER::FIFOSIZE_64
    }
}
#[doc = r" Value of the field"]
pub struct CV_NUMR {
    bits: u8,
}
impl CV_NUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMD_NUMR {
    bits: u8,
}
impl CMD_NUMR {
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
    #[doc = "Bits 0:7 - Trigger Number"]
    #[inline]
    pub fn trig_num(&self) -> TRIG_NUMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIG_NUMR { bits }
    }
    #[doc = "Bits 8:15 - Result FIFO Depth"]
    #[inline]
    pub fn fifosize(&self) -> FIFOSIZER {
        FIFOSIZER::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Compare Value Number"]
    #[inline]
    pub fn cv_num(&self) -> CV_NUMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CV_NUMR { bits }
    }
    #[doc = "Bits 24:31 - Command Buffer Number"]
    #[inline]
    pub fn cmd_num(&self) -> CMD_NUMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMD_NUMR { bits }
    }
}
