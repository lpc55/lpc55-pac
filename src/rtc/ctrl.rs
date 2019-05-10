#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `SWRESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRESETR {
    #[doc = "Not in reset. The RTC is not held in reset. This bit must be cleared prior to configuring or initiating any operation of the RTC."]
    NOT_IN_RESET,
    #[doc = "In reset. The RTC is held in reset. All register bits within the RTC will be forced to their reset value except the OFD bit. This bit must be cleared before writing to any register in the RTC - including writes to set any of the other bits within this register. Do not attempt to write to any bits of this register at the same time that the reset bit is being cleared."]
    IN_RESET,
}
impl SWRESETR {
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
            SWRESETR::NOT_IN_RESET => false,
            SWRESETR::IN_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRESETR {
        match value {
            false => SWRESETR::NOT_IN_RESET,
            true => SWRESETR::IN_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_IN_RESET`"]
    #[inline]
    pub fn is_not_in_reset(&self) -> bool {
        *self == SWRESETR::NOT_IN_RESET
    }
    #[doc = "Checks if the value of the field is `IN_RESET`"]
    #[inline]
    pub fn is_in_reset(&self) -> bool {
        *self == SWRESETR::IN_RESET
    }
}
#[doc = "Possible values of the field `ALARM1HZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM1HZR {
    #[doc = "No match. No match has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    NO_MATCH,
    #[doc = "Match. A match condition has occurred on the 1 Hz RTC timer. This flag generates an RTC alarm interrupt request RTC_ALARM which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    MATCH,
}
impl ALARM1HZR {
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
            ALARM1HZR::NO_MATCH => false,
            ALARM1HZR::MATCH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALARM1HZR {
        match value {
            false => ALARM1HZR::NO_MATCH,
            true => ALARM1HZR::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MATCH`"]
    #[inline]
    pub fn is_no_match(&self) -> bool {
        *self == ALARM1HZR::NO_MATCH
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == ALARM1HZR::MATCH
    }
}
#[doc = "Possible values of the field `WAKE1KHZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE1KHZR {
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    RUN,
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    TIMEOUT,
}
impl WAKE1KHZR {
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
            WAKE1KHZR::RUN => false,
            WAKE1KHZR::TIMEOUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKE1KHZR {
        match value {
            false => WAKE1KHZR::RUN,
            true => WAKE1KHZR::TIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline]
    pub fn is_run(&self) -> bool {
        *self == WAKE1KHZR::RUN
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline]
    pub fn is_timeout(&self) -> bool {
        *self == WAKE1KHZR::TIMEOUT
    }
}
#[doc = "Possible values of the field `ALARMDPD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARMDPD_ENR {
    #[doc = "Disable. A match on the 1 Hz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE,
    #[doc = "Enable. A match on the 1 Hz RTC timer bring the part out of Deep power-down mode."]
    ENABLE,
}
impl ALARMDPD_ENR {
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
            ALARMDPD_ENR::DISABLE => false,
            ALARMDPD_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALARMDPD_ENR {
        match value {
            false => ALARMDPD_ENR::DISABLE,
            true => ALARMDPD_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ALARMDPD_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ALARMDPD_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `WAKEDPD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEDPD_ENR {
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE,
    #[doc = "Enable. A match on the 1 kHz RTC timer bring the part out of Deep power-down mode."]
    ENABLE,
}
impl WAKEDPD_ENR {
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
            WAKEDPD_ENR::DISABLE => false,
            WAKEDPD_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEDPD_ENR {
        match value {
            false => WAKEDPD_ENR::DISABLE,
            true => WAKEDPD_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WAKEDPD_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WAKEDPD_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `RTC1KHZ_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC1KHZ_ENR {
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE,
    #[doc = "Enable. The 1 kHz RTC timer is enabled."]
    ENABLE,
}
impl RTC1KHZ_ENR {
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
            RTC1KHZ_ENR::DISABLE => false,
            RTC1KHZ_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC1KHZ_ENR {
        match value {
            false => RTC1KHZ_ENR::DISABLE,
            true => RTC1KHZ_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RTC1KHZ_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RTC1KHZ_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `RTC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ENR {
    #[doc = "Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. This bit should be 0 when writing to load a value in the RTC counter register."]
    DISABLE,
    #[doc = "Enable. The 1 Hz RTC clock is running and RTC operation is enabled. This bit must be set to initiate operation of the RTC. The first clock to the RTC counter occurs 1 s after this bit is set. To also enable the high-resolution, 1 kHz clock, set bit 6 in this register."]
    ENABLE,
}
impl RTC_ENR {
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
            RTC_ENR::DISABLE => false,
            RTC_ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_ENR {
        match value {
            false => RTC_ENR::DISABLE,
            true => RTC_ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RTC_ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RTC_ENR::ENABLE
    }
}
#[doc = "Possible values of the field `RTC_OSC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_OSC_PDR {
    #[doc = "See RTC_OSC_BYPASS"]
    POWER_UP,
    #[doc = "RTC oscillator is powered-down."]
    POWERED_DOWN,
}
impl RTC_OSC_PDR {
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
            RTC_OSC_PDR::POWER_UP => false,
            RTC_OSC_PDR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_OSC_PDR {
        match value {
            false => RTC_OSC_PDR::POWER_UP,
            true => RTC_OSC_PDR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_UP`"]
    #[inline]
    pub fn is_power_up(&self) -> bool {
        *self == RTC_OSC_PDR::POWER_UP
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == RTC_OSC_PDR::POWERED_DOWN
    }
}
#[doc = "Possible values of the field `RTC_OSC_BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_OSC_BYPASSR {
    #[doc = "The RTC Oscillator operates normally as a crystal oscillator with the crystal connected between the RTC_XTALIN and RTC_XTALOUT pins."]
    USED,
    #[doc = "The RTC Oscillator is in bypass mode. In this mode a clock can be directly input into the RTC_XTALIN pin."]
    BYPASS,
}
impl RTC_OSC_BYPASSR {
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
            RTC_OSC_BYPASSR::USED => false,
            RTC_OSC_BYPASSR::BYPASS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_OSC_BYPASSR {
        match value {
            false => RTC_OSC_BYPASSR::USED,
            true => RTC_OSC_BYPASSR::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline]
    pub fn is_used(&self) -> bool {
        *self == RTC_OSC_BYPASSR::USED
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline]
    pub fn is_bypass(&self) -> bool {
        *self == RTC_OSC_BYPASSR::BYPASS
    }
}
#[doc = "Possible values of the field `RTC_SUBSEC_ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_SUBSEC_ENAR {
    #[doc = "The sub-second counter (if implemented) is disabled. This bit is cleared by a system-level POR or BOD reset as well as a by the RTC_ENA bit (bit 7 in this register). On modules not equipped with a sub-second counter, this bit will always read-back as a '0'."]
    POWER_UP,
    #[doc = "The 32 KHz sub-second counter is enabled (if implemented). Counting commences on the start of the first one-second interval after this bit is set. Note: This bit can only be set after the RTC_ENA bit (bit 7) is set by a previous write operation. Note: The RTC sub-second counter must be re-enabled whenever the chip exits deep power-down mode."]
    POWERED_DOWN,
}
impl RTC_SUBSEC_ENAR {
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
            RTC_SUBSEC_ENAR::POWER_UP => false,
            RTC_SUBSEC_ENAR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_SUBSEC_ENAR {
        match value {
            false => RTC_SUBSEC_ENAR::POWER_UP,
            true => RTC_SUBSEC_ENAR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_UP`"]
    #[inline]
    pub fn is_power_up(&self) -> bool {
        *self == RTC_SUBSEC_ENAR::POWER_UP
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == RTC_SUBSEC_ENAR::POWERED_DOWN
    }
}
#[doc = "Values that can be written to the field `SWRESET`"]
pub enum SWRESETW {
    #[doc = "Not in reset. The RTC is not held in reset. This bit must be cleared prior to configuring or initiating any operation of the RTC."]
    NOT_IN_RESET,
    #[doc = "In reset. The RTC is held in reset. All register bits within the RTC will be forced to their reset value except the OFD bit. This bit must be cleared before writing to any register in the RTC - including writes to set any of the other bits within this register. Do not attempt to write to any bits of this register at the same time that the reset bit is being cleared."]
    IN_RESET,
}
impl SWRESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRESETW::NOT_IN_RESET => false,
            SWRESETW::IN_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not in reset. The RTC is not held in reset. This bit must be cleared prior to configuring or initiating any operation of the RTC."]
    #[inline]
    pub fn not_in_reset(self) -> &'a mut W {
        self.variant(SWRESETW::NOT_IN_RESET)
    }
    #[doc = "In reset. The RTC is held in reset. All register bits within the RTC will be forced to their reset value except the OFD bit. This bit must be cleared before writing to any register in the RTC - including writes to set any of the other bits within this register. Do not attempt to write to any bits of this register at the same time that the reset bit is being cleared."]
    #[inline]
    pub fn in_reset(self) -> &'a mut W {
        self.variant(SWRESETW::IN_RESET)
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
#[doc = "Values that can be written to the field `ALARM1HZ`"]
pub enum ALARM1HZW {
    #[doc = "No match. No match has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    NO_MATCH,
    #[doc = "Match. A match condition has occurred on the 1 Hz RTC timer. This flag generates an RTC alarm interrupt request RTC_ALARM which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    MATCH,
}
impl ALARM1HZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALARM1HZW::NO_MATCH => false,
            ALARM1HZW::MATCH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALARM1HZW<'a> {
    w: &'a mut W,
}
impl<'a> _ALARM1HZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALARM1HZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No match. No match has occurred on the 1 Hz RTC timer. Writing a 0 has no effect."]
    #[inline]
    pub fn no_match(self) -> &'a mut W {
        self.variant(ALARM1HZW::NO_MATCH)
    }
    #[doc = "Match. A match condition has occurred on the 1 Hz RTC timer. This flag generates an RTC alarm interrupt request RTC_ALARM which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(ALARM1HZW::MATCH)
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
#[doc = "Values that can be written to the field `WAKE1KHZ`"]
pub enum WAKE1KHZW {
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    RUN,
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    TIMEOUT,
}
impl WAKE1KHZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKE1KHZW::RUN => false,
            WAKE1KHZW::TIMEOUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKE1KHZW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKE1KHZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKE1KHZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Run. The RTC 1 kHz timer is running. Writing a 0 has no effect."]
    #[inline]
    pub fn run(self) -> &'a mut W {
        self.variant(WAKE1KHZW::RUN)
    }
    #[doc = "Time-out. The 1 kHz high-resolution/wake-up timer has timed out. This flag generates an RTC wake-up interrupt request RTC-WAKE which can also wake up the part from any low power mode. Writing a 1 clears this bit."]
    #[inline]
    pub fn timeout(self) -> &'a mut W {
        self.variant(WAKE1KHZW::TIMEOUT)
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
#[doc = "Values that can be written to the field `ALARMDPD_EN`"]
pub enum ALARMDPD_ENW {
    #[doc = "Disable. A match on the 1 Hz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE,
    #[doc = "Enable. A match on the 1 Hz RTC timer bring the part out of Deep power-down mode."]
    ENABLE,
}
impl ALARMDPD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALARMDPD_ENW::DISABLE => false,
            ALARMDPD_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALARMDPD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ALARMDPD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALARMDPD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. A match on the 1 Hz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ALARMDPD_ENW::DISABLE)
    }
    #[doc = "Enable. A match on the 1 Hz RTC timer bring the part out of Deep power-down mode."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ALARMDPD_ENW::ENABLE)
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
#[doc = "Values that can be written to the field `WAKEDPD_EN`"]
pub enum WAKEDPD_ENW {
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE,
    #[doc = "Enable. A match on the 1 kHz RTC timer bring the part out of Deep power-down mode."]
    ENABLE,
}
impl WAKEDPD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEDPD_ENW::DISABLE => false,
            WAKEDPD_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEDPD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEDPD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEDPD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WAKEDPD_ENW::DISABLE)
    }
    #[doc = "Enable. A match on the 1 kHz RTC timer bring the part out of Deep power-down mode."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WAKEDPD_ENW::ENABLE)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC1KHZ_EN`"]
pub enum RTC1KHZ_ENW {
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    DISABLE,
    #[doc = "Enable. The 1 kHz RTC timer is enabled."]
    ENABLE,
}
impl RTC1KHZ_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC1KHZ_ENW::DISABLE => false,
            RTC1KHZ_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC1KHZ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC1KHZ_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC1KHZ_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. A match on the 1 kHz RTC timer will not bring the part out of Deep power-down mode."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC1KHZ_ENW::DISABLE)
    }
    #[doc = "Enable. The 1 kHz RTC timer is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC1KHZ_ENW::ENABLE)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_EN`"]
pub enum RTC_ENW {
    #[doc = "Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. This bit should be 0 when writing to load a value in the RTC counter register."]
    DISABLE,
    #[doc = "Enable. The 1 Hz RTC clock is running and RTC operation is enabled. This bit must be set to initiate operation of the RTC. The first clock to the RTC counter occurs 1 s after this bit is set. To also enable the high-resolution, 1 kHz clock, set bit 6 in this register."]
    ENABLE,
}
impl RTC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_ENW::DISABLE => false,
            RTC_ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. The RTC 1 Hz and 1 kHz clocks are shut down and the RTC operation is disabled. This bit should be 0 when writing to load a value in the RTC counter register."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC_ENW::DISABLE)
    }
    #[doc = "Enable. The 1 Hz RTC clock is running and RTC operation is enabled. This bit must be set to initiate operation of the RTC. The first clock to the RTC counter occurs 1 s after this bit is set. To also enable the high-resolution, 1 kHz clock, set bit 6 in this register."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_ENW::ENABLE)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_OSC_PD`"]
pub enum RTC_OSC_PDW {
    #[doc = "See RTC_OSC_BYPASS"]
    POWER_UP,
    #[doc = "RTC oscillator is powered-down."]
    POWERED_DOWN,
}
impl RTC_OSC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_OSC_PDW::POWER_UP => false,
            RTC_OSC_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_OSC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_OSC_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_OSC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "See RTC_OSC_BYPASS"]
    #[inline]
    pub fn power_up(self) -> &'a mut W {
        self.variant(RTC_OSC_PDW::POWER_UP)
    }
    #[doc = "RTC oscillator is powered-down."]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(RTC_OSC_PDW::POWERED_DOWN)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_OSC_BYPASS`"]
pub enum RTC_OSC_BYPASSW {
    #[doc = "The RTC Oscillator operates normally as a crystal oscillator with the crystal connected between the RTC_XTALIN and RTC_XTALOUT pins."]
    USED,
    #[doc = "The RTC Oscillator is in bypass mode. In this mode a clock can be directly input into the RTC_XTALIN pin."]
    BYPASS,
}
impl RTC_OSC_BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_OSC_BYPASSW::USED => false,
            RTC_OSC_BYPASSW::BYPASS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_OSC_BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_OSC_BYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_OSC_BYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The RTC Oscillator operates normally as a crystal oscillator with the crystal connected between the RTC_XTALIN and RTC_XTALOUT pins."]
    #[inline]
    pub fn used(self) -> &'a mut W {
        self.variant(RTC_OSC_BYPASSW::USED)
    }
    #[doc = "The RTC Oscillator is in bypass mode. In this mode a clock can be directly input into the RTC_XTALIN pin."]
    #[inline]
    pub fn bypass(self) -> &'a mut W {
        self.variant(RTC_OSC_BYPASSW::BYPASS)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_SUBSEC_ENA`"]
pub enum RTC_SUBSEC_ENAW {
    #[doc = "The sub-second counter (if implemented) is disabled. This bit is cleared by a system-level POR or BOD reset as well as a by the RTC_ENA bit (bit 7 in this register). On modules not equipped with a sub-second counter, this bit will always read-back as a '0'."]
    POWER_UP,
    #[doc = "The 32 KHz sub-second counter is enabled (if implemented). Counting commences on the start of the first one-second interval after this bit is set. Note: This bit can only be set after the RTC_ENA bit (bit 7) is set by a previous write operation. Note: The RTC sub-second counter must be re-enabled whenever the chip exits deep power-down mode."]
    POWERED_DOWN,
}
impl RTC_SUBSEC_ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_SUBSEC_ENAW::POWER_UP => false,
            RTC_SUBSEC_ENAW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_SUBSEC_ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_SUBSEC_ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_SUBSEC_ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The sub-second counter (if implemented) is disabled. This bit is cleared by a system-level POR or BOD reset as well as a by the RTC_ENA bit (bit 7 in this register). On modules not equipped with a sub-second counter, this bit will always read-back as a '0'."]
    #[inline]
    pub fn power_up(self) -> &'a mut W {
        self.variant(RTC_SUBSEC_ENAW::POWER_UP)
    }
    #[doc = "The 32 KHz sub-second counter is enabled (if implemented). Counting commences on the start of the first one-second interval after this bit is set. Note: This bit can only be set after the RTC_ENA bit (bit 7) is set by a previous write operation. Note: The RTC sub-second counter must be re-enabled whenever the chip exits deep power-down mode."]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(RTC_SUBSEC_ENAW::POWERED_DOWN)
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
        const OFFSET: u8 = 10;
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
    #[doc = "Bit 0 - Software reset control"]
    #[inline]
    pub fn swreset(&self) -> SWRESETR {
        SWRESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RTC 1 Hz timer alarm flag status."]
    #[inline]
    pub fn alarm1hz(&self) -> ALARM1HZR {
        ALARM1HZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - RTC 1 kHz timer wake-up flag status."]
    #[inline]
    pub fn wake1khz(&self) -> WAKE1KHZR {
        WAKE1KHZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RTC 1 Hz timer alarm enable for Deep power-down."]
    #[inline]
    pub fn alarmdpd_en(&self) -> ALARMDPD_ENR {
        ALARMDPD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[inline]
    pub fn wakedpd_en(&self) -> WAKEDPD_ENR {
        WAKEDPD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[inline]
    pub fn rtc1khz_en(&self) -> RTC1KHZ_ENR {
        RTC1KHZ_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - RTC enable."]
    #[inline]
    pub fn rtc_en(&self) -> RTC_ENR {
        RTC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - RTC oscillator power-down control."]
    #[inline]
    pub fn rtc_osc_pd(&self) -> RTC_OSC_PDR {
        RTC_OSC_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - RTC oscillator bypass control."]
    #[inline]
    pub fn rtc_osc_bypass(&self) -> RTC_OSC_BYPASSR {
        RTC_OSC_BYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - RTC Sub-second counter control."]
    #[inline]
    pub fn rtc_subsec_ena(&self) -> RTC_SUBSEC_ENAR {
        RTC_SUBSEC_ENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 0 - Software reset control"]
    #[inline]
    pub fn swreset(&mut self) -> _SWRESETW {
        _SWRESETW { w: self }
    }
    #[doc = "Bit 2 - RTC 1 Hz timer alarm flag status."]
    #[inline]
    pub fn alarm1hz(&mut self) -> _ALARM1HZW {
        _ALARM1HZW { w: self }
    }
    #[doc = "Bit 3 - RTC 1 kHz timer wake-up flag status."]
    #[inline]
    pub fn wake1khz(&mut self) -> _WAKE1KHZW {
        _WAKE1KHZW { w: self }
    }
    #[doc = "Bit 4 - RTC 1 Hz timer alarm enable for Deep power-down."]
    #[inline]
    pub fn alarmdpd_en(&mut self) -> _ALARMDPD_ENW {
        _ALARMDPD_ENW { w: self }
    }
    #[doc = "Bit 5 - RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[inline]
    pub fn wakedpd_en(&mut self) -> _WAKEDPD_ENW {
        _WAKEDPD_ENW { w: self }
    }
    #[doc = "Bit 6 - RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[inline]
    pub fn rtc1khz_en(&mut self) -> _RTC1KHZ_ENW {
        _RTC1KHZ_ENW { w: self }
    }
    #[doc = "Bit 7 - RTC enable."]
    #[inline]
    pub fn rtc_en(&mut self) -> _RTC_ENW {
        _RTC_ENW { w: self }
    }
    #[doc = "Bit 8 - RTC oscillator power-down control."]
    #[inline]
    pub fn rtc_osc_pd(&mut self) -> _RTC_OSC_PDW {
        _RTC_OSC_PDW { w: self }
    }
    #[doc = "Bit 9 - RTC oscillator bypass control."]
    #[inline]
    pub fn rtc_osc_bypass(&mut self) -> _RTC_OSC_BYPASSW {
        _RTC_OSC_BYPASSW { w: self }
    }
    #[doc = "Bit 10 - RTC Sub-second counter control."]
    #[inline]
    pub fn rtc_subsec_ena(&mut self) -> _RTC_SUBSEC_ENAW {
        _RTC_SUBSEC_ENAW { w: self }
    }
}
