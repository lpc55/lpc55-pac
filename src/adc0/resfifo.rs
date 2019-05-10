#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RESFIFO {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DR {
    bits: u16,
}
impl DR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `TSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSRCR {
    #[doc = "Trigger source 0 initiated this conversion."]
    TSRC_0,
    #[doc = "Trigger source 1 initiated this conversion."]
    TSRC_1,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_2,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_3,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_4,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_5,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_6,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_7,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_8,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_9,
    #[doc = "Trigger source 15 initiated this conversion."]
    TSRC_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSRCR::TSRC_0 => 0,
            TSRCR::TSRC_1 => 1,
            TSRCR::TSRC_2 => 2,
            TSRCR::TSRC_3 => 3,
            TSRCR::TSRC_4 => 4,
            TSRCR::TSRC_5 => 5,
            TSRCR::TSRC_6 => 6,
            TSRCR::TSRC_7 => 7,
            TSRCR::TSRC_8 => 8,
            TSRCR::TSRC_9 => 9,
            TSRCR::TSRC_15 => 15,
            TSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSRCR {
        match value {
            0 => TSRCR::TSRC_0,
            1 => TSRCR::TSRC_1,
            2 => TSRCR::TSRC_2,
            3 => TSRCR::TSRC_3,
            4 => TSRCR::TSRC_4,
            5 => TSRCR::TSRC_5,
            6 => TSRCR::TSRC_6,
            7 => TSRCR::TSRC_7,
            8 => TSRCR::TSRC_8,
            9 => TSRCR::TSRC_9,
            15 => TSRCR::TSRC_15,
            i => TSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TSRC_0`"]
    #[inline]
    pub fn is_tsrc_0(&self) -> bool {
        *self == TSRCR::TSRC_0
    }
    #[doc = "Checks if the value of the field is `TSRC_1`"]
    #[inline]
    pub fn is_tsrc_1(&self) -> bool {
        *self == TSRCR::TSRC_1
    }
    #[doc = "Checks if the value of the field is `TSRC_2`"]
    #[inline]
    pub fn is_tsrc_2(&self) -> bool {
        *self == TSRCR::TSRC_2
    }
    #[doc = "Checks if the value of the field is `TSRC_3`"]
    #[inline]
    pub fn is_tsrc_3(&self) -> bool {
        *self == TSRCR::TSRC_3
    }
    #[doc = "Checks if the value of the field is `TSRC_4`"]
    #[inline]
    pub fn is_tsrc_4(&self) -> bool {
        *self == TSRCR::TSRC_4
    }
    #[doc = "Checks if the value of the field is `TSRC_5`"]
    #[inline]
    pub fn is_tsrc_5(&self) -> bool {
        *self == TSRCR::TSRC_5
    }
    #[doc = "Checks if the value of the field is `TSRC_6`"]
    #[inline]
    pub fn is_tsrc_6(&self) -> bool {
        *self == TSRCR::TSRC_6
    }
    #[doc = "Checks if the value of the field is `TSRC_7`"]
    #[inline]
    pub fn is_tsrc_7(&self) -> bool {
        *self == TSRCR::TSRC_7
    }
    #[doc = "Checks if the value of the field is `TSRC_8`"]
    #[inline]
    pub fn is_tsrc_8(&self) -> bool {
        *self == TSRCR::TSRC_8
    }
    #[doc = "Checks if the value of the field is `TSRC_9`"]
    #[inline]
    pub fn is_tsrc_9(&self) -> bool {
        *self == TSRCR::TSRC_9
    }
    #[doc = "Checks if the value of the field is `TSRC_15`"]
    #[inline]
    pub fn is_tsrc_15(&self) -> bool {
        *self == TSRCR::TSRC_15
    }
}
#[doc = "Possible values of the field `LOOPCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPCNTR {
    #[doc = "Result is from initial conversion in command."]
    LOOPCNT_0,
    #[doc = "Result is from second conversion in command."]
    LOOPCNT_1,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_2,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_3,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_4,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_5,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_6,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_7,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_8,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_9,
    #[doc = "Result is from 16th conversion in command."]
    LOOPCNT_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LOOPCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LOOPCNTR::LOOPCNT_0 => 0,
            LOOPCNTR::LOOPCNT_1 => 1,
            LOOPCNTR::LOOPCNT_2 => 2,
            LOOPCNTR::LOOPCNT_3 => 3,
            LOOPCNTR::LOOPCNT_4 => 4,
            LOOPCNTR::LOOPCNT_5 => 5,
            LOOPCNTR::LOOPCNT_6 => 6,
            LOOPCNTR::LOOPCNT_7 => 7,
            LOOPCNTR::LOOPCNT_8 => 8,
            LOOPCNTR::LOOPCNT_9 => 9,
            LOOPCNTR::LOOPCNT_15 => 15,
            LOOPCNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LOOPCNTR {
        match value {
            0 => LOOPCNTR::LOOPCNT_0,
            1 => LOOPCNTR::LOOPCNT_1,
            2 => LOOPCNTR::LOOPCNT_2,
            3 => LOOPCNTR::LOOPCNT_3,
            4 => LOOPCNTR::LOOPCNT_4,
            5 => LOOPCNTR::LOOPCNT_5,
            6 => LOOPCNTR::LOOPCNT_6,
            7 => LOOPCNTR::LOOPCNT_7,
            8 => LOOPCNTR::LOOPCNT_8,
            9 => LOOPCNTR::LOOPCNT_9,
            15 => LOOPCNTR::LOOPCNT_15,
            i => LOOPCNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_0`"]
    #[inline]
    pub fn is_loopcnt_0(&self) -> bool {
        *self == LOOPCNTR::LOOPCNT_0
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_1`"]
    #[inline]
    pub fn is_loopcnt_1(&self) -> bool {
        *self == LOOPCNTR::LOOPCNT_1
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_2`"]
    #[inline]
    pub fn is_loopcnt_2(&self) -> bool {
        *self == LOOPCNTR::LOOPCNT_2
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_3`"]
    #[inline]
    pub fn is_loopcnt_3(&self) -> bool {
        *self == LOOPCNTR::LOOPCNT_3
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_4`"]
    #[inline]
    pub fn is_loopcnt_4(&self) -> bool {
        *self == LOOPCNTR::LOOPCNT_4
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_5`"]
    #[inline]
    pub fn is_loopcnt_5(&self) -> bool {
        *self == LOOPCNTR::LOOPCNT_5
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_6`"]
    #[inline]
    pub fn is_loopcnt_6(&self) -> bool {
        *self == LOOPCNTR::LOOPCNT_6
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_7`"]
    #[inline]
    pub fn is_loopcnt_7(&self) -> bool {
        *self == LOOPCNTR::LOOPCNT_7
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_8`"]
    #[inline]
    pub fn is_loopcnt_8(&self) -> bool {
        *self == LOOPCNTR::LOOPCNT_8
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_9`"]
    #[inline]
    pub fn is_loopcnt_9(&self) -> bool {
        *self == LOOPCNTR::LOOPCNT_9
    }
    #[doc = "Checks if the value of the field is `LOOPCNT_15`"]
    #[inline]
    pub fn is_loopcnt_15(&self) -> bool {
        *self == LOOPCNTR::LOOPCNT_15
    }
}
#[doc = "Possible values of the field `CMDSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDSRCR {
    #[doc = "Not a valid value CMDSRC value for a dataword in RESFIFO. 0x0 is only found in initial FIFO state prior to an ADC conversion result dataword being stored to a RESFIFO buffer."]
    CMDSRC_0,
    #[doc = "CMD1 buffer used as control settings for this conversion."]
    CMDSRC_1,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_2,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_3,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_4,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_5,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_6,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_7,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_8,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_9,
    #[doc = "CMD15 buffer used as control settings for this conversion."]
    CMDSRC_15,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDSRCR::CMDSRC_0 => 0,
            CMDSRCR::CMDSRC_1 => 1,
            CMDSRCR::CMDSRC_2 => 2,
            CMDSRCR::CMDSRC_3 => 3,
            CMDSRCR::CMDSRC_4 => 4,
            CMDSRCR::CMDSRC_5 => 5,
            CMDSRCR::CMDSRC_6 => 6,
            CMDSRCR::CMDSRC_7 => 7,
            CMDSRCR::CMDSRC_8 => 8,
            CMDSRCR::CMDSRC_9 => 9,
            CMDSRCR::CMDSRC_15 => 15,
            CMDSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDSRCR {
        match value {
            0 => CMDSRCR::CMDSRC_0,
            1 => CMDSRCR::CMDSRC_1,
            2 => CMDSRCR::CMDSRC_2,
            3 => CMDSRCR::CMDSRC_3,
            4 => CMDSRCR::CMDSRC_4,
            5 => CMDSRCR::CMDSRC_5,
            6 => CMDSRCR::CMDSRC_6,
            7 => CMDSRCR::CMDSRC_7,
            8 => CMDSRCR::CMDSRC_8,
            9 => CMDSRCR::CMDSRC_9,
            15 => CMDSRCR::CMDSRC_15,
            i => CMDSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CMDSRC_0`"]
    #[inline]
    pub fn is_cmdsrc_0(&self) -> bool {
        *self == CMDSRCR::CMDSRC_0
    }
    #[doc = "Checks if the value of the field is `CMDSRC_1`"]
    #[inline]
    pub fn is_cmdsrc_1(&self) -> bool {
        *self == CMDSRCR::CMDSRC_1
    }
    #[doc = "Checks if the value of the field is `CMDSRC_2`"]
    #[inline]
    pub fn is_cmdsrc_2(&self) -> bool {
        *self == CMDSRCR::CMDSRC_2
    }
    #[doc = "Checks if the value of the field is `CMDSRC_3`"]
    #[inline]
    pub fn is_cmdsrc_3(&self) -> bool {
        *self == CMDSRCR::CMDSRC_3
    }
    #[doc = "Checks if the value of the field is `CMDSRC_4`"]
    #[inline]
    pub fn is_cmdsrc_4(&self) -> bool {
        *self == CMDSRCR::CMDSRC_4
    }
    #[doc = "Checks if the value of the field is `CMDSRC_5`"]
    #[inline]
    pub fn is_cmdsrc_5(&self) -> bool {
        *self == CMDSRCR::CMDSRC_5
    }
    #[doc = "Checks if the value of the field is `CMDSRC_6`"]
    #[inline]
    pub fn is_cmdsrc_6(&self) -> bool {
        *self == CMDSRCR::CMDSRC_6
    }
    #[doc = "Checks if the value of the field is `CMDSRC_7`"]
    #[inline]
    pub fn is_cmdsrc_7(&self) -> bool {
        *self == CMDSRCR::CMDSRC_7
    }
    #[doc = "Checks if the value of the field is `CMDSRC_8`"]
    #[inline]
    pub fn is_cmdsrc_8(&self) -> bool {
        *self == CMDSRCR::CMDSRC_8
    }
    #[doc = "Checks if the value of the field is `CMDSRC_9`"]
    #[inline]
    pub fn is_cmdsrc_9(&self) -> bool {
        *self == CMDSRCR::CMDSRC_9
    }
    #[doc = "Checks if the value of the field is `CMDSRC_15`"]
    #[inline]
    pub fn is_cmdsrc_15(&self) -> bool {
        *self == CMDSRCR::CMDSRC_15
    }
}
#[doc = "Possible values of the field `VALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALIDR {
    #[doc = "FIFO is empty. Discard any read from RESFIFO."]
    VALID_0,
    #[doc = "FIFO record read from RESFIFO is valid."]
    VALID_1,
}
impl VALIDR {
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
            VALIDR::VALID_0 => false,
            VALIDR::VALID_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VALIDR {
        match value {
            false => VALIDR::VALID_0,
            true => VALIDR::VALID_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALID_0`"]
    #[inline]
    pub fn is_valid_0(&self) -> bool {
        *self == VALIDR::VALID_0
    }
    #[doc = "Checks if the value of the field is `VALID_1`"]
    #[inline]
    pub fn is_valid_1(&self) -> bool {
        *self == VALIDR::VALID_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Data result"]
    #[inline]
    pub fn d(&self) -> DR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DR { bits }
    }
    #[doc = "Bits 16:19 - Trigger Source"]
    #[inline]
    pub fn tsrc(&self) -> TSRCR {
        TSRCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Loop count value"]
    #[inline]
    pub fn loopcnt(&self) -> LOOPCNTR {
        LOOPCNTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Command Buffer Source"]
    #[inline]
    pub fn cmdsrc(&self) -> CMDSRCR {
        CMDSRCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - FIFO entry is valid"]
    #[inline]
    pub fn valid(&self) -> VALIDR {
        VALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
