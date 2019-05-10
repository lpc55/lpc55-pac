#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::VERID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESR {
    #[doc = "Up to 13-bit differential/12-bit single ended resolution supported."]
    RES_0,
    #[doc = "Up to 16-bit differential/16-bit single ended resolution supported."]
    RES_1,
}
impl RESR {
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
            RESR::RES_0 => false,
            RESR::RES_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESR {
        match value {
            false => RESR::RES_0,
            true => RESR::RES_1,
        }
    }
    #[doc = "Checks if the value of the field is `RES_0`"]
    #[inline]
    pub fn is_res_0(&self) -> bool {
        *self == RESR::RES_0
    }
    #[doc = "Checks if the value of the field is `RES_1`"]
    #[inline]
    pub fn is_res_1(&self) -> bool {
        *self == RESR::RES_1
    }
}
#[doc = "Possible values of the field `DIFFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFFENR {
    #[doc = "Differential operation not supported."]
    DIFFEN_0,
    #[doc = "Differential operation supported. CMDLa\\[CTYPE\\] controls fields implemented."]
    DIFFEN_1,
}
impl DIFFENR {
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
            DIFFENR::DIFFEN_0 => false,
            DIFFENR::DIFFEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIFFENR {
        match value {
            false => DIFFENR::DIFFEN_0,
            true => DIFFENR::DIFFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIFFEN_0`"]
    #[inline]
    pub fn is_diffen_0(&self) -> bool {
        *self == DIFFENR::DIFFEN_0
    }
    #[doc = "Checks if the value of the field is `DIFFEN_1`"]
    #[inline]
    pub fn is_diffen_1(&self) -> bool {
        *self == DIFFENR::DIFFEN_1
    }
}
#[doc = "Possible values of the field `MVI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MVIR {
    #[doc = "Single voltage reference high (VREFH) input supported."]
    MVI_0,
    #[doc = "Multiple voltage reference high (VREFH) inputs supported."]
    MVI_1,
}
impl MVIR {
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
            MVIR::MVI_0 => false,
            MVIR::MVI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MVIR {
        match value {
            false => MVIR::MVI_0,
            true => MVIR::MVI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MVI_0`"]
    #[inline]
    pub fn is_mvi_0(&self) -> bool {
        *self == MVIR::MVI_0
    }
    #[doc = "Checks if the value of the field is `MVI_1`"]
    #[inline]
    pub fn is_mvi_1(&self) -> bool {
        *self == MVIR::MVI_1
    }
}
#[doc = "Possible values of the field `CSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSWR {
    #[doc = "Channel scaling not supported."]
    CSW_0,
    #[doc = "Channel scaling supported. 1-bit CSCALE control field."]
    CSW_1,
    #[doc = "Channel scaling supported. 6-bit CSCALE control field."]
    CSW_6,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CSWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSWR::CSW_0 => 0,
            CSWR::CSW_1 => 1,
            CSWR::CSW_6 => 6,
            CSWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSWR {
        match value {
            0 => CSWR::CSW_0,
            1 => CSWR::CSW_1,
            6 => CSWR::CSW_6,
            i => CSWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CSW_0`"]
    #[inline]
    pub fn is_csw_0(&self) -> bool {
        *self == CSWR::CSW_0
    }
    #[doc = "Checks if the value of the field is `CSW_1`"]
    #[inline]
    pub fn is_csw_1(&self) -> bool {
        *self == CSWR::CSW_1
    }
    #[doc = "Checks if the value of the field is `CSW_6`"]
    #[inline]
    pub fn is_csw_6(&self) -> bool {
        *self == CSWR::CSW_6
    }
}
#[doc = "Possible values of the field `VR1RNGI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VR1RNGIR {
    #[doc = "Range control not required. CFG\\[VREF1RNG\\] is not implemented."]
    VR1RNGI_0,
    #[doc = "Range control required. CFG\\[VREF1RNG\\] is implemented."]
    VR1RNGI_1,
}
impl VR1RNGIR {
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
            VR1RNGIR::VR1RNGI_0 => false,
            VR1RNGIR::VR1RNGI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VR1RNGIR {
        match value {
            false => VR1RNGIR::VR1RNGI_0,
            true => VR1RNGIR::VR1RNGI_1,
        }
    }
    #[doc = "Checks if the value of the field is `VR1RNGI_0`"]
    #[inline]
    pub fn is_vr1rngi_0(&self) -> bool {
        *self == VR1RNGIR::VR1RNGI_0
    }
    #[doc = "Checks if the value of the field is `VR1RNGI_1`"]
    #[inline]
    pub fn is_vr1rngi_1(&self) -> bool {
        *self == VR1RNGIR::VR1RNGI_1
    }
}
#[doc = "Possible values of the field `IADCKI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IADCKIR {
    #[doc = "Internal clock source not implemented."]
    IADCKI_0,
    #[doc = "Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
    IADCKI_1,
}
impl IADCKIR {
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
            IADCKIR::IADCKI_0 => false,
            IADCKIR::IADCKI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IADCKIR {
        match value {
            false => IADCKIR::IADCKI_0,
            true => IADCKIR::IADCKI_1,
        }
    }
    #[doc = "Checks if the value of the field is `IADCKI_0`"]
    #[inline]
    pub fn is_iadcki_0(&self) -> bool {
        *self == IADCKIR::IADCKI_0
    }
    #[doc = "Checks if the value of the field is `IADCKI_1`"]
    #[inline]
    pub fn is_iadcki_1(&self) -> bool {
        *self == IADCKIR::IADCKI_1
    }
}
#[doc = "Possible values of the field `CALOFSI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALOFSIR {
    #[doc = "Calibration Not Implemented."]
    CALOFSI_0,
    #[doc = "Calibration Implemented."]
    CALOFSI_1,
}
impl CALOFSIR {
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
            CALOFSIR::CALOFSI_0 => false,
            CALOFSIR::CALOFSI_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CALOFSIR {
        match value {
            false => CALOFSIR::CALOFSI_0,
            true => CALOFSIR::CALOFSI_1,
        }
    }
    #[doc = "Checks if the value of the field is `CALOFSI_0`"]
    #[inline]
    pub fn is_calofsi_0(&self) -> bool {
        *self == CALOFSIR::CALOFSI_0
    }
    #[doc = "Checks if the value of the field is `CALOFSI_1`"]
    #[inline]
    pub fn is_calofsi_1(&self) -> bool {
        *self == CALOFSIR::CALOFSI_1
    }
}
#[doc = "Possible values of the field `NUM_SEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUM_SECR {
    #[doc = "This design supports one single ended conversion at a time."]
    NUM_SEC_0,
    #[doc = "This design supports two simultanious single ended conversions."]
    NUM_SEC_1,
}
impl NUM_SECR {
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
            NUM_SECR::NUM_SEC_0 => false,
            NUM_SECR::NUM_SEC_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NUM_SECR {
        match value {
            false => NUM_SECR::NUM_SEC_0,
            true => NUM_SECR::NUM_SEC_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUM_SEC_0`"]
    #[inline]
    pub fn is_num_sec_0(&self) -> bool {
        *self == NUM_SECR::NUM_SEC_0
    }
    #[doc = "Checks if the value of the field is `NUM_SEC_1`"]
    #[inline]
    pub fn is_num_sec_1(&self) -> bool {
        *self == NUM_SECR::NUM_SEC_1
    }
}
#[doc = "Possible values of the field `NUM_FIFO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUM_FIFOR {
    #[doc = "N/A"]
    NUM_FIFO_0,
    #[doc = "This design supports one result FIFO."]
    NUM_FIFO_1,
    #[doc = "This design supports two result FIFOs."]
    NUM_FIFO_2,
    #[doc = "This design supports three result FIFOs."]
    NUM_FIFO_3,
    #[doc = "This design supports four result FIFOs."]
    NUM_FIFO_4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NUM_FIFOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NUM_FIFOR::NUM_FIFO_0 => 0,
            NUM_FIFOR::NUM_FIFO_1 => 1,
            NUM_FIFOR::NUM_FIFO_2 => 2,
            NUM_FIFOR::NUM_FIFO_3 => 3,
            NUM_FIFOR::NUM_FIFO_4 => 4,
            NUM_FIFOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NUM_FIFOR {
        match value {
            0 => NUM_FIFOR::NUM_FIFO_0,
            1 => NUM_FIFOR::NUM_FIFO_1,
            2 => NUM_FIFOR::NUM_FIFO_2,
            3 => NUM_FIFOR::NUM_FIFO_3,
            4 => NUM_FIFOR::NUM_FIFO_4,
            i => NUM_FIFOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NUM_FIFO_0`"]
    #[inline]
    pub fn is_num_fifo_0(&self) -> bool {
        *self == NUM_FIFOR::NUM_FIFO_0
    }
    #[doc = "Checks if the value of the field is `NUM_FIFO_1`"]
    #[inline]
    pub fn is_num_fifo_1(&self) -> bool {
        *self == NUM_FIFOR::NUM_FIFO_1
    }
    #[doc = "Checks if the value of the field is `NUM_FIFO_2`"]
    #[inline]
    pub fn is_num_fifo_2(&self) -> bool {
        *self == NUM_FIFOR::NUM_FIFO_2
    }
    #[doc = "Checks if the value of the field is `NUM_FIFO_3`"]
    #[inline]
    pub fn is_num_fifo_3(&self) -> bool {
        *self == NUM_FIFOR::NUM_FIFO_3
    }
    #[doc = "Checks if the value of the field is `NUM_FIFO_4`"]
    #[inline]
    pub fn is_num_fifo_4(&self) -> bool {
        *self == NUM_FIFOR::NUM_FIFO_4
    }
}
#[doc = r" Value of the field"]
pub struct MINORR {
    bits: u8,
}
impl MINORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAJORR {
    bits: u8,
}
impl MAJORR {
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
    #[doc = "Bit 0 - Resolution"]
    #[inline]
    pub fn res(&self) -> RESR {
        RESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Differential Supported"]
    #[inline]
    pub fn diffen(&self) -> DIFFENR {
        DIFFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Multi Vref Implemented"]
    #[inline]
    pub fn mvi(&self) -> MVIR {
        MVIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Channel Scale Width"]
    #[inline]
    pub fn csw(&self) -> CSWR {
        CSWR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Voltage Reference 1 Range Control Bit Implemented"]
    #[inline]
    pub fn vr1rngi(&self) -> VR1RNGIR {
        VR1RNGIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Internal ADC Clock implemented"]
    #[inline]
    pub fn iadcki(&self) -> IADCKIR {
        IADCKIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Calibration Function Implemented"]
    #[inline]
    pub fn calofsi(&self) -> CALOFSIR {
        CALOFSIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Number of Single Ended Outputs Supported"]
    #[inline]
    pub fn num_sec(&self) -> NUM_SECR {
        NUM_SECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:14 - Number of FIFOs"]
    #[inline]
    pub fn num_fifo(&self) -> NUM_FIFOR {
        NUM_FIFOR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Minor Version Number"]
    #[inline]
    pub fn minor(&self) -> MINORR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MINORR { bits }
    }
    #[doc = "Bits 24:31 - Major Version Number"]
    #[inline]
    pub fn major(&self) -> MAJORR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAJORR { bits }
    }
}
