#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FRO192M_STATUS {
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
#[doc = "Possible values of the field `CLK_VALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_VALIDR {
    #[doc = "No output clock present (None of 12 MHz, 48 MHz or 96 MHz clock is available)."]
    NOCLKOUT,
    #[doc = "Clock is present (12 MHz, 48 MHz or 96 MHz can be output if they are enable respectively by FRO192M_CTRL.ENA_12MHZCLK/ENA_48MHZCLK/ENA_96MHZCLK)."]
    CLKOUT,
}
impl CLK_VALIDR {
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
            CLK_VALIDR::NOCLKOUT => false,
            CLK_VALIDR::CLKOUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLK_VALIDR {
        match value {
            false => CLK_VALIDR::NOCLKOUT,
            true => CLK_VALIDR::CLKOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOCLKOUT`"]
    #[inline]
    pub fn is_noclkout(&self) -> bool {
        *self == CLK_VALIDR::NOCLKOUT
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline]
    pub fn is_clkout(&self) -> bool {
        *self == CLK_VALIDR::CLKOUT
    }
}
#[doc = r" Value of the field"]
pub struct ATB_VCTRLR {
    bits: bool,
}
impl ATB_VCTRLR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Output clock valid signal. Indicates that CCO clock has settled."]
    #[inline]
    pub fn clk_valid(&self) -> CLK_VALIDR {
        CLK_VALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CCO threshold voltage detector output (signal vcco_ok). Once the CCO voltage crosses the threshold voltage of a SLVT transistor, this output signal will go high. It is also possible to observe the clk_valid signal."]
    #[inline]
    pub fn atb_vctrl(&self) -> ATB_VCTRLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ATB_VCTRLR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
