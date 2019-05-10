#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USB1CLKCTRL {
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
#[doc = "Possible values of the field `AP_HS_DEV_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_HS_DEV_CLKR {
    #[doc = "Under hardware control."]
    HW_CTRL,
    #[doc = "Forced high."]
    FORCED,
}
impl AP_HS_DEV_CLKR {
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
            AP_HS_DEV_CLKR::HW_CTRL => false,
            AP_HS_DEV_CLKR::FORCED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AP_HS_DEV_CLKR {
        match value {
            false => AP_HS_DEV_CLKR::HW_CTRL,
            true => AP_HS_DEV_CLKR::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `HW_CTRL`"]
    #[inline]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == AP_HS_DEV_CLKR::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline]
    pub fn is_forced(&self) -> bool {
        *self == AP_HS_DEV_CLKR::FORCED
    }
}
#[doc = "Possible values of the field `POL_HS_DEV_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_HS_DEV_CLKR {
    #[doc = "Falling edge of device need_clock triggers wake-up."]
    FALLING,
    #[doc = "Rising edge of device need_clock triggers wake-up."]
    RISING,
}
impl POL_HS_DEV_CLKR {
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
            POL_HS_DEV_CLKR::FALLING => false,
            POL_HS_DEV_CLKR::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POL_HS_DEV_CLKR {
        match value {
            false => POL_HS_DEV_CLKR::FALLING,
            true => POL_HS_DEV_CLKR::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == POL_HS_DEV_CLKR::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == POL_HS_DEV_CLKR::RISING
    }
}
#[doc = "Possible values of the field `AP_HS_HOST_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_HS_HOST_CLKR {
    #[doc = "Under hardware control."]
    HW_CTRL,
    #[doc = "Forced high."]
    FORCED,
}
impl AP_HS_HOST_CLKR {
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
            AP_HS_HOST_CLKR::HW_CTRL => false,
            AP_HS_HOST_CLKR::FORCED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AP_HS_HOST_CLKR {
        match value {
            false => AP_HS_HOST_CLKR::HW_CTRL,
            true => AP_HS_HOST_CLKR::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `HW_CTRL`"]
    #[inline]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == AP_HS_HOST_CLKR::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline]
    pub fn is_forced(&self) -> bool {
        *self == AP_HS_HOST_CLKR::FORCED
    }
}
#[doc = "Possible values of the field `POL_HS_HOST_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_HS_HOST_CLKR {
    #[doc = "Falling edge of device need_clock triggers wake-up."]
    FALLING,
    #[doc = "Rising edge of device need_clock triggers wake-up."]
    RISING,
}
impl POL_HS_HOST_CLKR {
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
            POL_HS_HOST_CLKR::FALLING => false,
            POL_HS_HOST_CLKR::RISING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POL_HS_HOST_CLKR {
        match value {
            false => POL_HS_HOST_CLKR::FALLING,
            true => POL_HS_HOST_CLKR::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == POL_HS_HOST_CLKR::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == POL_HS_HOST_CLKR::RISING
    }
}
#[doc = "Possible values of the field `HS_DEV_WAKEUP_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_DEV_WAKEUP_NR {
    #[doc = "Forces USB1 PHY to wake-up."]
    FORCE_WUP,
    #[doc = "Normal USB1 PHY behavior."]
    NORMAL_WUP,
}
impl HS_DEV_WAKEUP_NR {
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
            HS_DEV_WAKEUP_NR::FORCE_WUP => false,
            HS_DEV_WAKEUP_NR::NORMAL_WUP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HS_DEV_WAKEUP_NR {
        match value {
            false => HS_DEV_WAKEUP_NR::FORCE_WUP,
            true => HS_DEV_WAKEUP_NR::NORMAL_WUP,
        }
    }
    #[doc = "Checks if the value of the field is `FORCE_WUP`"]
    #[inline]
    pub fn is_force_wup(&self) -> bool {
        *self == HS_DEV_WAKEUP_NR::FORCE_WUP
    }
    #[doc = "Checks if the value of the field is `NORMAL_WUP`"]
    #[inline]
    pub fn is_normal_wup(&self) -> bool {
        *self == HS_DEV_WAKEUP_NR::NORMAL_WUP
    }
}
#[doc = "Values that can be written to the field `AP_HS_DEV_CLK`"]
pub enum AP_HS_DEV_CLKW {
    #[doc = "Under hardware control."]
    HW_CTRL,
    #[doc = "Forced high."]
    FORCED,
}
impl AP_HS_DEV_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AP_HS_DEV_CLKW::HW_CTRL => false,
            AP_HS_DEV_CLKW::FORCED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AP_HS_DEV_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _AP_HS_DEV_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AP_HS_DEV_CLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Under hardware control."]
    #[inline]
    pub fn hw_ctrl(self) -> &'a mut W {
        self.variant(AP_HS_DEV_CLKW::HW_CTRL)
    }
    #[doc = "Forced high."]
    #[inline]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_HS_DEV_CLKW::FORCED)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `POL_HS_DEV_CLK`"]
pub enum POL_HS_DEV_CLKW {
    #[doc = "Falling edge of device need_clock triggers wake-up."]
    FALLING,
    #[doc = "Rising edge of device need_clock triggers wake-up."]
    RISING,
}
impl POL_HS_DEV_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POL_HS_DEV_CLKW::FALLING => false,
            POL_HS_DEV_CLKW::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POL_HS_DEV_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _POL_HS_DEV_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POL_HS_DEV_CLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge of device need_clock triggers wake-up."]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(POL_HS_DEV_CLKW::FALLING)
    }
    #[doc = "Rising edge of device need_clock triggers wake-up."]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(POL_HS_DEV_CLKW::RISING)
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
#[doc = "Values that can be written to the field `AP_HS_HOST_CLK`"]
pub enum AP_HS_HOST_CLKW {
    #[doc = "Under hardware control."]
    HW_CTRL,
    #[doc = "Forced high."]
    FORCED,
}
impl AP_HS_HOST_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AP_HS_HOST_CLKW::HW_CTRL => false,
            AP_HS_HOST_CLKW::FORCED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AP_HS_HOST_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _AP_HS_HOST_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AP_HS_HOST_CLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Under hardware control."]
    #[inline]
    pub fn hw_ctrl(self) -> &'a mut W {
        self.variant(AP_HS_HOST_CLKW::HW_CTRL)
    }
    #[doc = "Forced high."]
    #[inline]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_HS_HOST_CLKW::FORCED)
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
#[doc = "Values that can be written to the field `POL_HS_HOST_CLK`"]
pub enum POL_HS_HOST_CLKW {
    #[doc = "Falling edge of device need_clock triggers wake-up."]
    FALLING,
    #[doc = "Rising edge of device need_clock triggers wake-up."]
    RISING,
}
impl POL_HS_HOST_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POL_HS_HOST_CLKW::FALLING => false,
            POL_HS_HOST_CLKW::RISING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POL_HS_HOST_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _POL_HS_HOST_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POL_HS_HOST_CLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge of device need_clock triggers wake-up."]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(POL_HS_HOST_CLKW::FALLING)
    }
    #[doc = "Rising edge of device need_clock triggers wake-up."]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(POL_HS_HOST_CLKW::RISING)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HS_DEV_WAKEUP_N`"]
pub enum HS_DEV_WAKEUP_NW {
    #[doc = "Forces USB1 PHY to wake-up."]
    FORCE_WUP,
    #[doc = "Normal USB1 PHY behavior."]
    NORMAL_WUP,
}
impl HS_DEV_WAKEUP_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HS_DEV_WAKEUP_NW::FORCE_WUP => false,
            HS_DEV_WAKEUP_NW::NORMAL_WUP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HS_DEV_WAKEUP_NW<'a> {
    w: &'a mut W,
}
impl<'a> _HS_DEV_WAKEUP_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HS_DEV_WAKEUP_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Forces USB1 PHY to wake-up."]
    #[inline]
    pub fn force_wup(self) -> &'a mut W {
        self.variant(HS_DEV_WAKEUP_NW::FORCE_WUP)
    }
    #[doc = "Normal USB1 PHY behavior."]
    #[inline]
    pub fn normal_wup(self) -> &'a mut W {
        self.variant(HS_DEV_WAKEUP_NW::NORMAL_WUP)
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - USB1 Device need_clock signal control:."]
    #[inline]
    pub fn ap_hs_dev_clk(&self) -> AP_HS_DEV_CLKR {
        AP_HS_DEV_CLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - USB1 Device need_clock polarity for triggering the USB1 wake-up interrupt:."]
    #[inline]
    pub fn pol_hs_dev_clk(&self) -> POL_HS_DEV_CLKR {
        POL_HS_DEV_CLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - USB1 Host need_clock signal control:."]
    #[inline]
    pub fn ap_hs_host_clk(&self) -> AP_HS_HOST_CLKR {
        AP_HS_HOST_CLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - USB1 Host need_clock polarity for triggering the USB1 wake-up interrupt: 0 Falling edge of device need_clock triggers wake-up."]
    #[inline]
    pub fn pol_hs_host_clk(&self) -> POL_HS_HOST_CLKR {
        POL_HS_HOST_CLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to synchronous control logic:."]
    #[inline]
    pub fn hs_dev_wakeup_n(&self) -> HS_DEV_WAKEUP_NR {
        HS_DEV_WAKEUP_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - USB1 Device need_clock signal control:."]
    #[inline]
    pub fn ap_hs_dev_clk(&mut self) -> _AP_HS_DEV_CLKW {
        _AP_HS_DEV_CLKW { w: self }
    }
    #[doc = "Bit 1 - USB1 Device need_clock polarity for triggering the USB1 wake-up interrupt:."]
    #[inline]
    pub fn pol_hs_dev_clk(&mut self) -> _POL_HS_DEV_CLKW {
        _POL_HS_DEV_CLKW { w: self }
    }
    #[doc = "Bit 2 - USB1 Host need_clock signal control:."]
    #[inline]
    pub fn ap_hs_host_clk(&mut self) -> _AP_HS_HOST_CLKW {
        _AP_HS_HOST_CLKW { w: self }
    }
    #[doc = "Bit 3 - USB1 Host need_clock polarity for triggering the USB1 wake-up interrupt: 0 Falling edge of device need_clock triggers wake-up."]
    #[inline]
    pub fn pol_hs_host_clk(&mut self) -> _POL_HS_HOST_CLKW {
        _POL_HS_HOST_CLKW { w: self }
    }
    #[doc = "Bit 4 - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to synchronous control logic:."]
    #[inline]
    pub fn hs_dev_wakeup_n(&mut self) -> _HS_DEV_WAKEUP_NW {
        _HS_DEV_WAKEUP_NW { w: self }
    }
}
