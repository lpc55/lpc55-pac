#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RESETCAUSE {
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
pub struct PORR {
    bits: bool,
}
impl PORR {
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
pub struct PADRESETR {
    bits: bool,
}
impl PADRESETR {
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
pub struct BODRESETR {
    bits: bool,
}
impl BODRESETR {
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
pub struct SYSTEMRESETR {
    bits: bool,
}
impl SYSTEMRESETR {
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
pub struct WDTRESETR {
    bits: bool,
}
impl WDTRESETR {
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
pub struct SWRRESETR {
    bits: bool,
}
impl SWRRESETR {
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
pub struct DPDRESET_WAKEUPIOR {
    bits: bool,
}
impl DPDRESET_WAKEUPIOR {
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
pub struct DPDRESET_RTCR {
    bits: bool,
}
impl DPDRESET_RTCR {
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
pub struct DPDRESET_OSTIMERR {
    bits: bool,
}
impl DPDRESET_OSTIMERR {
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
pub struct _PORW<'a> {
    w: &'a mut W,
}
impl<'a> _PORW<'a> {
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
pub struct _PADRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _PADRESETW<'a> {
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
#[doc = r" Proxy"]
pub struct _BODRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRESETW<'a> {
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
#[doc = r" Proxy"]
pub struct _SYSTEMRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSTEMRESETW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WDTRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTRESETW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SWRRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRRESETW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPDRESET_WAKEUPIOW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDRESET_WAKEUPIOW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPDRESET_RTCW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDRESET_RTCW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPDRESET_OSTIMERW<'a> {
    w: &'a mut W,
}
impl<'a> _DPDRESET_OSTIMERW<'a> {
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
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - 1 : The last chip reset was caused by a Power On Reset. Write '1' to clear this bit."]
    #[inline]
    pub fn por(&self) -> PORR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PORR { bits }
    }
    #[doc = "Bit 1 - 1 : The last chip reset was caused by a Pin Reset. Write '1' to clear this bit."]
    #[inline]
    pub fn padreset(&self) -> PADRESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PADRESETR { bits }
    }
    #[doc = "Bit 2 - 1 : The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD. Write '1' to clear this bit."]
    #[inline]
    pub fn bodreset(&self) -> BODRESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BODRESETR { bits }
    }
    #[doc = "Bit 3 - 1 : The last chip reset was caused by a System Reset requested by the ARM CPU. Write '1' to clear this bit."]
    #[inline]
    pub fn systemreset(&self) -> SYSTEMRESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSTEMRESETR { bits }
    }
    #[doc = "Bit 4 - 1 : The last chip reset was caused by the Watchdog Timer. Write '1' to clear this bit."]
    #[inline]
    pub fn wdtreset(&self) -> WDTRESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WDTRESETR { bits }
    }
    #[doc = "Bit 5 - 1 : The last chip reset was caused by a Software. Write '1' to clear this bit."]
    #[inline]
    pub fn swrreset(&self) -> SWRRESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWRRESETR { bits }
    }
    #[doc = "Bit 6 - 1 : The last chip reset was caused by a Wake-up I/O reset event during DEEP POWER DOWN mode. Write '1' to clear this bit."]
    #[inline]
    pub fn dpdreset_wakeupio(&self) -> DPDRESET_WAKEUPIOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DPDRESET_WAKEUPIOR { bits }
    }
    #[doc = "Bit 7 - 1 : The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during DEEP POWER DOWN mode. Write '1' to clear this bit."]
    #[inline]
    pub fn dpdreset_rtc(&self) -> DPDRESET_RTCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DPDRESET_RTCR { bits }
    }
    #[doc = "Bit 8 - 1 : The last chip reset was caused by a OS Event Timer reset eventduring DEEP POWER DOWN mode. Write '1' to clear this bit."]
    #[inline]
    pub fn dpdreset_ostimer(&self) -> DPDRESET_OSTIMERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DPDRESET_OSTIMERR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - 1 : The last chip reset was caused by a Power On Reset. Write '1' to clear this bit."]
    #[inline]
    pub fn por(&mut self) -> _PORW {
        _PORW { w: self }
    }
    #[doc = "Bit 1 - 1 : The last chip reset was caused by a Pin Reset. Write '1' to clear this bit."]
    #[inline]
    pub fn padreset(&mut self) -> _PADRESETW {
        _PADRESETW { w: self }
    }
    #[doc = "Bit 2 - 1 : The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD. Write '1' to clear this bit."]
    #[inline]
    pub fn bodreset(&mut self) -> _BODRESETW {
        _BODRESETW { w: self }
    }
    #[doc = "Bit 3 - 1 : The last chip reset was caused by a System Reset requested by the ARM CPU. Write '1' to clear this bit."]
    #[inline]
    pub fn systemreset(&mut self) -> _SYSTEMRESETW {
        _SYSTEMRESETW { w: self }
    }
    #[doc = "Bit 4 - 1 : The last chip reset was caused by the Watchdog Timer. Write '1' to clear this bit."]
    #[inline]
    pub fn wdtreset(&mut self) -> _WDTRESETW {
        _WDTRESETW { w: self }
    }
    #[doc = "Bit 5 - 1 : The last chip reset was caused by a Software. Write '1' to clear this bit."]
    #[inline]
    pub fn swrreset(&mut self) -> _SWRRESETW {
        _SWRRESETW { w: self }
    }
    #[doc = "Bit 6 - 1 : The last chip reset was caused by a Wake-up I/O reset event during DEEP POWER DOWN mode. Write '1' to clear this bit."]
    #[inline]
    pub fn dpdreset_wakeupio(&mut self) -> _DPDRESET_WAKEUPIOW {
        _DPDRESET_WAKEUPIOW { w: self }
    }
    #[doc = "Bit 7 - 1 : The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during DEEP POWER DOWN mode. Write '1' to clear this bit."]
    #[inline]
    pub fn dpdreset_rtc(&mut self) -> _DPDRESET_RTCW {
        _DPDRESET_RTCW { w: self }
    }
    #[doc = "Bit 8 - 1 : The last chip reset was caused by a OS Event Timer reset eventduring DEEP POWER DOWN mode. Write '1' to clear this bit."]
    #[inline]
    pub fn dpdreset_ostimer(&mut self) -> _DPDRESET_OSTIMERW {
        _DPDRESET_OSTIMERW { w: self }
    }
}
