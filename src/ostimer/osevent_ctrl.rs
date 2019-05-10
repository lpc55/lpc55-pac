#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSEVENT_CTRL {
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
pub struct OSTIMER_INTRFLAGR {
    bits: bool,
}
impl OSTIMER_INTRFLAGR {
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
pub struct OSTIMER_INTENAR {
    bits: bool,
}
impl OSTIMER_INTENAR {
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
#[doc = r" Proxy"]
pub struct _OSTIMER_INTRFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _OSTIMER_INTRFLAGW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSTIMER_INTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _OSTIMER_INTENAW<'a> {
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
    #[doc = "Bit 0 - This bit is set when a match occurs between the central 64-bit EVTIMER and the value programmed in the Match-register pair for the associated CPU This bit is cleared by writing a '1'. Writes to clear this bit are asynchronous. This should be done before a new match value is written into the MATCH_L/H registers"]
    #[inline]
    pub fn ostimer_intrflag(&self) -> OSTIMER_INTRFLAGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OSTIMER_INTRFLAGR { bits }
    }
    #[doc = "Bit 1 - When this bit is '1' an interrupt/wakeup request to the Domainn processor will be asserted when the OSTIMER_INTR flag is set. When this bit is '0', interrupt/wakeup requests due to the OSTIMER_INTR flag are blocked.A separate OSEVENT_CTRL register is implemented for each CPU. Each CPU reads its own local value at the same address."]
    #[inline]
    pub fn ostimer_intena(&self) -> OSTIMER_INTENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OSTIMER_INTENAR { bits }
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
    #[doc = "Bit 0 - This bit is set when a match occurs between the central 64-bit EVTIMER and the value programmed in the Match-register pair for the associated CPU This bit is cleared by writing a '1'. Writes to clear this bit are asynchronous. This should be done before a new match value is written into the MATCH_L/H registers"]
    #[inline]
    pub fn ostimer_intrflag(&mut self) -> _OSTIMER_INTRFLAGW {
        _OSTIMER_INTRFLAGW { w: self }
    }
    #[doc = "Bit 1 - When this bit is '1' an interrupt/wakeup request to the Domainn processor will be asserted when the OSTIMER_INTR flag is set. When this bit is '0', interrupt/wakeup requests due to the OSTIMER_INTR flag are blocked.A separate OSEVENT_CTRL register is implemented for each CPU. Each CPU reads its own local value at the same address."]
    #[inline]
    pub fn ostimer_intena(&mut self) -> _OSTIMER_INTENAW {
        _OSTIMER_INTENAW { w: self }
    }
}
