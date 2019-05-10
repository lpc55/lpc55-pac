#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSYR {
    #[doc = "The transmitter/receiver for channel pair is currently idle."]
    IDLE,
    #[doc = "The transmitter/receiver for channel pair is currently processing data."]
    BUSY,
}
impl BUSYR {
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
            BUSYR::IDLE => false,
            BUSYR::BUSY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSYR {
        match value {
            false => BUSYR::IDLE,
            true => BUSYR::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == BUSYR::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline]
    pub fn is_busy(&self) -> bool {
        *self == BUSYR::BUSY
    }
}
#[doc = "Possible values of the field `LR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRR {
    #[doc = "Left channel."]
    LEFT_CHANNEL,
    #[doc = "Right channel."]
    RIGHT_CHANNEL,
}
impl LRR {
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
            LRR::LEFT_CHANNEL => false,
            LRR::RIGHT_CHANNEL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRR {
        match value {
            false => LRR::LEFT_CHANNEL,
            true => LRR::RIGHT_CHANNEL,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT_CHANNEL`"]
    #[inline]
    pub fn is_left_channel(&self) -> bool {
        *self == LRR::LEFT_CHANNEL
    }
    #[doc = "Checks if the value of the field is `RIGHT_CHANNEL`"]
    #[inline]
    pub fn is_right_channel(&self) -> bool {
        *self == LRR::RIGHT_CHANNEL
    }
}
#[doc = "Possible values of the field `DATAPAUSED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAPAUSEDR {
    #[doc = "Data is not currently paused. A data pause may have been requested but is not yet in force, waiting for an allowed pause point. Refer to the description of the DATAPAUSE control bit in the CFG1 register."]
    NOT_PAUSED,
    #[doc = "A data pause has been requested and is now in force."]
    PAUSED,
}
impl DATAPAUSEDR {
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
            DATAPAUSEDR::NOT_PAUSED => false,
            DATAPAUSEDR::PAUSED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATAPAUSEDR {
        match value {
            false => DATAPAUSEDR::NOT_PAUSED,
            true => DATAPAUSEDR::PAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PAUSED`"]
    #[inline]
    pub fn is_not_paused(&self) -> bool {
        *self == DATAPAUSEDR::NOT_PAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline]
    pub fn is_paused(&self) -> bool {
        *self == DATAPAUSEDR::PAUSED
    }
}
#[doc = "Values that can be written to the field `SLVFRMERR`"]
pub enum SLVFRMERRW {
    #[doc = "No error has been recorded."]
    NO_ERROR,
    #[doc = "An error has been recorded for some channel pair that is operating in slave mode. ERROR is cleared by writing a 1 to this bit position."]
    ERROR,
}
impl SLVFRMERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVFRMERRW::NO_ERROR => false,
            SLVFRMERRW::ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVFRMERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVFRMERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVFRMERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error has been recorded."]
    #[inline]
    pub fn no_error(self) -> &'a mut W {
        self.variant(SLVFRMERRW::NO_ERROR)
    }
    #[doc = "An error has been recorded for some channel pair that is operating in slave mode. ERROR is cleared by writing a 1 to this bit position."]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(SLVFRMERRW::ERROR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Busy status for the primary channel pair. Other BUSY flags may be found in the STAT register for each channel pair."]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Left/Right indication. This flag is considered to be a debugging aid and is not expected to be used by an I2S driver. Valid when one channel pair is busy. Indicates left or right data being processed for the currently busy channel pair."]
    #[inline]
    pub fn lr(&self) -> LRR {
        LRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Data Paused status flag. Applies to all I2S channels"]
    #[inline]
    pub fn datapaused(&self) -> DATAPAUSEDR {
        DATAPAUSEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Slave Frame Error flag. This applies when at least one channel pair is operating as a slave. An error indicates that the incoming WS signal did not transition as expected due to a mismatch between FRAMELEN and the actual incoming I2S stream."]
    #[inline]
    pub fn slvfrmerr(&mut self) -> _SLVFRMERRW {
        _SLVFRMERRW { w: self }
    }
}
