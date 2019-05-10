#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PAUSE {
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
#[doc = r" Value of the field"]
pub struct PAUSEDLYR {
    bits: u16,
}
impl PAUSEDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `PAUSEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUSEENR {
    #[doc = "Pause operation disabled"]
    PAUSEEN_0,
    #[doc = "Pause operation enabled"]
    PAUSEEN_1,
}
impl PAUSEENR {
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
            PAUSEENR::PAUSEEN_0 => false,
            PAUSEENR::PAUSEEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAUSEENR {
        match value {
            false => PAUSEENR::PAUSEEN_0,
            true => PAUSEENR::PAUSEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PAUSEEN_0`"]
    #[inline]
    pub fn is_pauseen_0(&self) -> bool {
        *self == PAUSEENR::PAUSEEN_0
    }
    #[doc = "Checks if the value of the field is `PAUSEEN_1`"]
    #[inline]
    pub fn is_pauseen_1(&self) -> bool {
        *self == PAUSEENR::PAUSEEN_1
    }
}
#[doc = r" Proxy"]
pub struct _PAUSEDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _PAUSEDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PAUSEEN`"]
pub enum PAUSEENW {
    #[doc = "Pause operation disabled"]
    PAUSEEN_0,
    #[doc = "Pause operation enabled"]
    PAUSEEN_1,
}
impl PAUSEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PAUSEENW::PAUSEEN_0 => false,
            PAUSEENW::PAUSEEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAUSEENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAUSEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAUSEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pause operation disabled"]
    #[inline]
    pub fn pauseen_0(self) -> &'a mut W {
        self.variant(PAUSEENW::PAUSEEN_0)
    }
    #[doc = "Pause operation enabled"]
    #[inline]
    pub fn pauseen_1(self) -> &'a mut W {
        self.variant(PAUSEENW::PAUSEEN_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:8 - Pause Delay"]
    #[inline]
    pub fn pausedly(&self) -> PAUSEDLYR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PAUSEDLYR { bits }
    }
    #[doc = "Bit 31 - PAUSE Option Enable"]
    #[inline]
    pub fn pauseen(&self) -> PAUSEENR {
        PAUSEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:8 - Pause Delay"]
    #[inline]
    pub fn pausedly(&mut self) -> _PAUSEDLYW {
        _PAUSEDLYW { w: self }
    }
    #[doc = "Bit 31 - PAUSE Option Enable"]
    #[inline]
    pub fn pauseen(&mut self) -> _PAUSEENW {
        _PAUSEENW { w: self }
    }
}
