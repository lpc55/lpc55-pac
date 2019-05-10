#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTIMERCLKSEL1 {
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
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "Main clock."]
    ENUM_0X0,
    #[doc = "PLL0 clock."]
    ENUM_0X1,
    #[doc = "No clock."]
    ENUM_0X2,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4,
    #[doc = "MCLK clock."]
    ENUM_0X5,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6,
    #[doc = "No clock."]
    ENUM_0X7,
}
impl SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELR::ENUM_0X0 => 0,
            SELR::ENUM_0X1 => 1,
            SELR::ENUM_0X2 => 2,
            SELR::ENUM_0X3 => 3,
            SELR::ENUM_0X4 => 4,
            SELR::ENUM_0X5 => 5,
            SELR::ENUM_0X6 => 6,
            SELR::ENUM_0X7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELR {
        match value {
            0 => SELR::ENUM_0X0,
            1 => SELR::ENUM_0X1,
            2 => SELR::ENUM_0X2,
            3 => SELR::ENUM_0X3,
            4 => SELR::ENUM_0X4,
            5 => SELR::ENUM_0X5,
            6 => SELR::ENUM_0X6,
            7 => SELR::ENUM_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_0X0`"]
    #[inline]
    pub fn is_enum_0x0(&self) -> bool {
        *self == SELR::ENUM_0X0
    }
    #[doc = "Checks if the value of the field is `ENUM_0X1`"]
    #[inline]
    pub fn is_enum_0x1(&self) -> bool {
        *self == SELR::ENUM_0X1
    }
    #[doc = "Checks if the value of the field is `ENUM_0X2`"]
    #[inline]
    pub fn is_enum_0x2(&self) -> bool {
        *self == SELR::ENUM_0X2
    }
    #[doc = "Checks if the value of the field is `ENUM_0X3`"]
    #[inline]
    pub fn is_enum_0x3(&self) -> bool {
        *self == SELR::ENUM_0X3
    }
    #[doc = "Checks if the value of the field is `ENUM_0X4`"]
    #[inline]
    pub fn is_enum_0x4(&self) -> bool {
        *self == SELR::ENUM_0X4
    }
    #[doc = "Checks if the value of the field is `ENUM_0X5`"]
    #[inline]
    pub fn is_enum_0x5(&self) -> bool {
        *self == SELR::ENUM_0X5
    }
    #[doc = "Checks if the value of the field is `ENUM_0X6`"]
    #[inline]
    pub fn is_enum_0x6(&self) -> bool {
        *self == SELR::ENUM_0X6
    }
    #[doc = "Checks if the value of the field is `ENUM_0X7`"]
    #[inline]
    pub fn is_enum_0x7(&self) -> bool {
        *self == SELR::ENUM_0X7
    }
}
#[doc = "Values that can be written to the field `SEL`"]
pub enum SELW {
    #[doc = "Main clock."]
    ENUM_0X0,
    #[doc = "PLL0 clock."]
    ENUM_0X1,
    #[doc = "No clock."]
    ENUM_0X2,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0X3,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X4,
    #[doc = "MCLK clock."]
    ENUM_0X5,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X6,
    #[doc = "No clock."]
    ENUM_0X7,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::ENUM_0X0 => 0,
            SELW::ENUM_0X1 => 1,
            SELW::ENUM_0X2 => 2,
            SELW::ENUM_0X3 => 3,
            SELW::ENUM_0X4 => 4,
            SELW::ENUM_0X5 => 5,
            SELW::ENUM_0X6 => 6,
            SELW::ENUM_0X7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Main clock."]
    #[inline]
    pub fn enum_0x0(self) -> &'a mut W {
        self.variant(SELW::ENUM_0X0)
    }
    #[doc = "PLL0 clock."]
    #[inline]
    pub fn enum_0x1(self) -> &'a mut W {
        self.variant(SELW::ENUM_0X1)
    }
    #[doc = "No clock."]
    #[inline]
    pub fn enum_0x2(self) -> &'a mut W {
        self.variant(SELW::ENUM_0X2)
    }
    #[doc = "FRO 96 MHz clock."]
    #[inline]
    pub fn enum_0x3(self) -> &'a mut W {
        self.variant(SELW::ENUM_0X3)
    }
    #[doc = "FRO 1MHz clock."]
    #[inline]
    pub fn enum_0x4(self) -> &'a mut W {
        self.variant(SELW::ENUM_0X4)
    }
    #[doc = "MCLK clock."]
    #[inline]
    pub fn enum_0x5(self) -> &'a mut W {
        self.variant(SELW::ENUM_0X5)
    }
    #[doc = "Oscillator 32kHz clock."]
    #[inline]
    pub fn enum_0x6(self) -> &'a mut W {
        self.variant(SELW::ENUM_0X6)
    }
    #[doc = "No clock."]
    #[inline]
    pub fn enum_0x7(self) -> &'a mut W {
        self.variant(SELW::ENUM_0X7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - CTimer 1 clock source select."]
    #[inline]
    pub fn sel(&self) -> SELR {
        SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 7 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - CTimer 1 clock source select."]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
