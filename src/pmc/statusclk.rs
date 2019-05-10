#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATUSCLK {
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
pub struct XTAL32KOKR {
    bits: bool,
}
impl XTAL32KOKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct FRO1MCLKVALIDR {
    bits: bool,
}
impl FRO1MCLKVALIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `XTAL32KOSCFAILURE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL32KOSCFAILURER {
    #[doc = "No oscillation failure has been detetced since the last time this bit has been cleared.."]
    NOFAIL,
    #[doc = "At least one oscillation failure has been detetced since the last time this bit has been cleared.."]
    FAILURE,
}
impl XTAL32KOSCFAILURER {
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
            XTAL32KOSCFAILURER::NOFAIL => false,
            XTAL32KOSCFAILURER::FAILURE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTAL32KOSCFAILURER {
        match value {
            false => XTAL32KOSCFAILURER::NOFAIL,
            true => XTAL32KOSCFAILURER::FAILURE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAIL`"]
    #[inline]
    pub fn is_nofail(&self) -> bool {
        *self == XTAL32KOSCFAILURER::NOFAIL
    }
    #[doc = "Checks if the value of the field is `FAILURE`"]
    #[inline]
    pub fn is_failure(&self) -> bool {
        *self == XTAL32KOSCFAILURER::FAILURE
    }
}
#[doc = "Values that can be written to the field `XTAL32KOSCFAILURE`"]
pub enum XTAL32KOSCFAILUREW {
    #[doc = "No oscillation failure has been detetced since the last time this bit has been cleared.."]
    NOFAIL,
    #[doc = "At least one oscillation failure has been detetced since the last time this bit has been cleared.."]
    FAILURE,
}
impl XTAL32KOSCFAILUREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTAL32KOSCFAILUREW::NOFAIL => false,
            XTAL32KOSCFAILUREW::FAILURE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTAL32KOSCFAILUREW<'a> {
    w: &'a mut W,
}
impl<'a> _XTAL32KOSCFAILUREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTAL32KOSCFAILUREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No oscillation failure has been detetced since the last time this bit has been cleared.."]
    #[inline]
    pub fn nofail(self) -> &'a mut W {
        self.variant(XTAL32KOSCFAILUREW::NOFAIL)
    }
    #[doc = "At least one oscillation failure has been detetced since the last time this bit has been cleared.."]
    #[inline]
    pub fn failure(self) -> &'a mut W {
        self.variant(XTAL32KOSCFAILUREW::FAILURE)
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
        const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - XTAL oscillator 32 K OK signal."]
    #[inline]
    pub fn xtal32kok(&self) -> XTAL32KOKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XTAL32KOKR { bits }
    }
    #[doc = "Bit 1 - FRO 1 MHz CCO voltage detector output."]
    #[inline]
    pub fn fro1mclkvalid(&self) -> FRO1MCLKVALIDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRO1MCLKVALIDR { bits }
    }
    #[doc = "Bit 2 - XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[inline]
    pub fn xtal32koscfailure(&self) -> XTAL32KOSCFAILURER {
        XTAL32KOSCFAILURER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 6 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - XTAL32 KHZ oscillator oscillation failure detection indicator."]
    #[inline]
    pub fn xtal32koscfailure(&mut self) -> _XTAL32KOSCFAILUREW {
        _XTAL32KOSCFAILUREW { w: self }
    }
}
