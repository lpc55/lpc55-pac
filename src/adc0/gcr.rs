#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GCR {
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
pub struct GCALRR {
    bits: u16,
}
impl GCALRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `RDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDYR {
    #[doc = "The gain offset calculation value is invalid."]
    RDY_0,
    #[doc = "The gain calibration value is valid."]
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
#[doc = r" Proxy"]
pub struct _GCALRW<'a> {
    w: &'a mut W,
}
impl<'a> _GCALRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDY`"]
pub enum RDYW {
    #[doc = "The gain offset calculation value is invalid."]
    RDY_0,
    #[doc = "The gain calibration value is valid."]
    RDY_1,
}
impl RDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDYW::RDY_0 => false,
            RDYW::RDY_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDYW<'a> {
    w: &'a mut W,
}
impl<'a> _RDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The gain offset calculation value is invalid."]
    #[inline]
    pub fn rdy_0(self) -> &'a mut W {
        self.variant(RDYW::RDY_0)
    }
    #[doc = "The gain calibration value is valid."]
    #[inline]
    pub fn rdy_1(self) -> &'a mut W {
        self.variant(RDYW::RDY_1)
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:15 - Gain Calculation Result"]
    #[inline]
    pub fn gcalr(&self) -> GCALRR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        GCALRR { bits }
    }
    #[doc = "Bit 24 - Gain Calculation Ready"]
    #[inline]
    pub fn rdy(&self) -> RDYR {
        RDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:15 - Gain Calculation Result"]
    #[inline]
    pub fn gcalr(&mut self) -> _GCALRW {
        _GCALRW { w: self }
    }
    #[doc = "Bit 24 - Gain Calculation Ready"]
    #[inline]
    pub fn rdy(&mut self) -> _RDYW {
        _RDYW { w: self }
    }
}
