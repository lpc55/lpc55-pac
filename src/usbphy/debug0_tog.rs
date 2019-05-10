#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEBUG0_TOG {
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
pub struct OTGIDPIOLOCKR {
    bits: bool,
}
impl OTGIDPIOLOCKR {
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
pub struct DEBUG_INTERFACE_HOLDR {
    bits: bool,
}
impl DEBUG_INTERFACE_HOLDR {
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
pub struct HSTPULLDOWNR {
    bits: u8,
}
impl HSTPULLDOWNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENHSTPULLDOWNR {
    bits: u8,
}
impl ENHSTPULLDOWNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TX2RXCOUNTR {
    bits: u8,
}
impl TX2RXCOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENTX2RXCOUNTR {
    bits: bool,
}
impl ENTX2RXCOUNTR {
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
pub struct SQUELCHRESETCOUNTR {
    bits: u8,
}
impl SQUELCHRESETCOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENSQUELCHRESETR {
    bits: bool,
}
impl ENSQUELCHRESETR {
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
pub struct SQUELCHRESETLENGTHR {
    bits: u8,
}
impl SQUELCHRESETLENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HOST_RESUME_DEBUGR {
    bits: bool,
}
impl HOST_RESUME_DEBUGR {
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
pub struct CLKGATER {
    bits: bool,
}
impl CLKGATER {
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
pub struct _OTGIDPIOLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGIDPIOLOCKW<'a> {
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
pub struct _DEBUG_INTERFACE_HOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUG_INTERFACE_HOLDW<'a> {
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
pub struct _HSTPULLDOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _HSTPULLDOWNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENHSTPULLDOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _ENHSTPULLDOWNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX2RXCOUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _TX2RXCOUNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENTX2RXCOUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENTX2RXCOUNTW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SQUELCHRESETCOUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _SQUELCHRESETCOUNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENSQUELCHRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _ENSQUELCHRESETW<'a> {
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
#[doc = r" Proxy"]
pub struct _SQUELCHRESETLENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _SQUELCHRESETLENGTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HOST_RESUME_DEBUGW<'a> {
    w: &'a mut W,
}
impl<'a> _HOST_RESUME_DEBUGW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATEW<'a> {
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 0 - Once OTG ID from STATUS_OTGID_STATUS is sampled, use this to hold the value"]
    #[inline]
    pub fn otgidpiolock(&self) -> OTGIDPIOLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTGIDPIOLOCKR { bits }
    }
    #[doc = "Bit 1 - Use holding registers to assist in timing for external UTMI interface."]
    #[inline]
    pub fn debug_interface_hold(&self) -> DEBUG_INTERFACE_HOLDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEBUG_INTERFACE_HOLDR { bits }
    }
    #[doc = "Bits 2:3 - This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
    #[inline]
    pub fn hstpulldown(&self) -> HSTPULLDOWNR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSTPULLDOWNR { bits }
    }
    #[doc = "Bits 4:5 - This bit field selects host pulldown overdrive mode"]
    #[inline]
    pub fn enhstpulldown(&self) -> ENHSTPULLDOWNR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ENHSTPULLDOWNR { bits }
    }
    #[doc = "Bits 8:11 - Delay in between the end of transmit to the beginning of receive"]
    #[inline]
    pub fn tx2rxcount(&self) -> TX2RXCOUNTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TX2RXCOUNTR { bits }
    }
    #[doc = "Bit 12 - Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline]
    pub fn entx2rxcount(&self) -> ENTX2RXCOUNTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENTX2RXCOUNTR { bits }
    }
    #[doc = "Bits 16:20 - Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline]
    pub fn squelchresetcount(&self) -> SQUELCHRESETCOUNTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SQUELCHRESETCOUNTR { bits }
    }
    #[doc = "Bit 24 - Set bit to allow squelch to reset high-speed receive."]
    #[inline]
    pub fn ensquelchreset(&self) -> ENSQUELCHRESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENSQUELCHRESETR { bits }
    }
    #[doc = "Bits 25:28 - Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline]
    pub fn squelchresetlength(&self) -> SQUELCHRESETLENGTHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SQUELCHRESETLENGTHR { bits }
    }
    #[doc = "Bit 29 - Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline]
    pub fn host_resume_debug(&self) -> HOST_RESUME_DEBUGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HOST_RESUME_DEBUGR { bits }
    }
    #[doc = "Bit 30 - Gate Test Clocks"]
    #[inline]
    pub fn clkgate(&self) -> CLKGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKGATER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2132279296 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Once OTG ID from STATUS_OTGID_STATUS is sampled, use this to hold the value"]
    #[inline]
    pub fn otgidpiolock(&mut self) -> _OTGIDPIOLOCKW {
        _OTGIDPIOLOCKW { w: self }
    }
    #[doc = "Bit 1 - Use holding registers to assist in timing for external UTMI interface."]
    #[inline]
    pub fn debug_interface_hold(&mut self) -> _DEBUG_INTERFACE_HOLDW {
        _DEBUG_INTERFACE_HOLDW { w: self }
    }
    #[doc = "Bits 2:3 - This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
    #[inline]
    pub fn hstpulldown(&mut self) -> _HSTPULLDOWNW {
        _HSTPULLDOWNW { w: self }
    }
    #[doc = "Bits 4:5 - This bit field selects host pulldown overdrive mode"]
    #[inline]
    pub fn enhstpulldown(&mut self) -> _ENHSTPULLDOWNW {
        _ENHSTPULLDOWNW { w: self }
    }
    #[doc = "Bits 8:11 - Delay in between the end of transmit to the beginning of receive"]
    #[inline]
    pub fn tx2rxcount(&mut self) -> _TX2RXCOUNTW {
        _TX2RXCOUNTW { w: self }
    }
    #[doc = "Bit 12 - Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline]
    pub fn entx2rxcount(&mut self) -> _ENTX2RXCOUNTW {
        _ENTX2RXCOUNTW { w: self }
    }
    #[doc = "Bits 16:20 - Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline]
    pub fn squelchresetcount(&mut self) -> _SQUELCHRESETCOUNTW {
        _SQUELCHRESETCOUNTW { w: self }
    }
    #[doc = "Bit 24 - Set bit to allow squelch to reset high-speed receive."]
    #[inline]
    pub fn ensquelchreset(&mut self) -> _ENSQUELCHRESETW {
        _ENSQUELCHRESETW { w: self }
    }
    #[doc = "Bits 25:28 - Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline]
    pub fn squelchresetlength(&mut self) -> _SQUELCHRESETLENGTHW {
        _SQUELCHRESETLENGTHW { w: self }
    }
    #[doc = "Bit 29 - Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline]
    pub fn host_resume_debug(&mut self) -> _HOST_RESUME_DEBUGW {
        _HOST_RESUME_DEBUGW { w: self }
    }
    #[doc = "Bit 30 - Gate Test Clocks"]
    #[inline]
    pub fn clkgate(&mut self) -> _CLKGATEW {
        _CLKGATEW { w: self }
    }
}
