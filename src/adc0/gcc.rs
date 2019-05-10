#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::GCC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct GAIN_CALR {
    bits: u16,
}
impl GAIN_CALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `RDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDYR {
    #[doc = "The gain calibration value is invalid. Run the auto-calibration routine for this value to be written."]
    RDY_0,
    #[doc = "The gain calibration value is valid. It should be used to update the GCRa\\[GCALR\\] register field."]
    RDY_1,
}
impl RDYR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RDYR::RDY_0 => false,
            RDYR::RDY_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDYR {
        match value {
            false => RDYR::RDY_0,
            true => RDYR::RDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDY_0`"]
    #[inline]
    pub fn is_rdy_0(&self) -> bool {
        *self == RDYR::RDY_0
    }
    #[doc = "Checks if the value of the field is `RDY_1`"]
    #[inline]
    pub fn is_rdy_1(&self) -> bool {
        *self == RDYR::RDY_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Gain Calibration Value"]
    #[inline]
    pub fn gain_cal(&self) -> GAIN_CALR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        GAIN_CALR { bits }
    }
    #[doc = "Bit 24 - Gain Calibration Value Valid"]
    #[inline]
    pub fn rdy(&self) -> RDYR {
        RDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
