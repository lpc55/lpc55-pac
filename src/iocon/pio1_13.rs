#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO1_13 {
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
#[doc = "Possible values of the field `FUNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUNCR {
    #[doc = "Alternative connection 0."]
    ALT0,
    #[doc = "Alternative connection 1."]
    ALT1,
    #[doc = "Alternative connection 2."]
    ALT2,
    #[doc = "Alternative connection 3."]
    ALT3,
    #[doc = "Alternative connection 4."]
    ALT4,
    #[doc = "Alternative connection 5."]
    ALT5,
    #[doc = "Alternative connection 6."]
    ALT6,
    #[doc = "Alternative connection 7."]
    ALT7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FUNCR::ALT0 => 0,
            FUNCR::ALT1 => 1,
            FUNCR::ALT2 => 2,
            FUNCR::ALT3 => 3,
            FUNCR::ALT4 => 4,
            FUNCR::ALT5 => 5,
            FUNCR::ALT6 => 6,
            FUNCR::ALT7 => 7,
            FUNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FUNCR {
        match value {
            0 => FUNCR::ALT0,
            1 => FUNCR::ALT1,
            2 => FUNCR::ALT2,
            3 => FUNCR::ALT3,
            4 => FUNCR::ALT4,
            5 => FUNCR::ALT5,
            6 => FUNCR::ALT6,
            7 => FUNCR::ALT7,
            i => FUNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALT0`"]
    #[inline]
    pub fn is_alt0(&self) -> bool {
        *self == FUNCR::ALT0
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline]
    pub fn is_alt1(&self) -> bool {
        *self == FUNCR::ALT1
    }
    #[doc = "Checks if the value of the field is `ALT2`"]
    #[inline]
    pub fn is_alt2(&self) -> bool {
        *self == FUNCR::ALT2
    }
    #[doc = "Checks if the value of the field is `ALT3`"]
    #[inline]
    pub fn is_alt3(&self) -> bool {
        *self == FUNCR::ALT3
    }
    #[doc = "Checks if the value of the field is `ALT4`"]
    #[inline]
    pub fn is_alt4(&self) -> bool {
        *self == FUNCR::ALT4
    }
    #[doc = "Checks if the value of the field is `ALT5`"]
    #[inline]
    pub fn is_alt5(&self) -> bool {
        *self == FUNCR::ALT5
    }
    #[doc = "Checks if the value of the field is `ALT6`"]
    #[inline]
    pub fn is_alt6(&self) -> bool {
        *self == FUNCR::ALT6
    }
    #[doc = "Checks if the value of the field is `ALT7`"]
    #[inline]
    pub fn is_alt7(&self) -> bool {
        *self == FUNCR::ALT7
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. Repeater mode."]
    REPEATER,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::INACTIVE => 0,
            MODER::PULL_DOWN => 1,
            MODER::PULL_UP => 2,
            MODER::REPEATER => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::INACTIVE,
            1 => MODER::PULL_DOWN,
            2 => MODER::PULL_UP,
            3 => MODER::REPEATER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == MODER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == MODER::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == MODER::REPEATER
    }
}
#[doc = "Possible values of the field `SLEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEWR {
    #[doc = "Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    STANDARD,
    #[doc = "Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    FAST,
}
impl SLEWR {
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
            SLEWR::STANDARD => false,
            SLEWR::FAST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEWR {
        match value {
            false => SLEWR::STANDARD,
            true => SLEWR::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == SLEWR::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline]
    pub fn is_fast(&self) -> bool {
        *self == SLEWR::FAST
    }
}
#[doc = "Possible values of the field `INVERT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVERTR {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED,
}
impl INVERTR {
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
            INVERTR::DISABLED => false,
            INVERTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVERTR {
        match value {
            false => INVERTR::DISABLED,
            true => INVERTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == INVERTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == INVERTR::ENABLED
    }
}
#[doc = "Possible values of the field `DIGIMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGIMODER {
    #[doc = "Analog mode, digital input is disabled."]
    ANALOG,
    #[doc = "Digital mode, digital input is enabled."]
    DIGITAL,
}
impl DIGIMODER {
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
            DIGIMODER::ANALOG => false,
            DIGIMODER::DIGITAL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIGIMODER {
        match value {
            false => DIGIMODER::ANALOG,
            true => DIGIMODER::DIGITAL,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG`"]
    #[inline]
    pub fn is_analog(&self) -> bool {
        *self == DIGIMODER::ANALOG
    }
    #[doc = "Checks if the value of the field is `DIGITAL`"]
    #[inline]
    pub fn is_digital(&self) -> bool {
        *self == DIGIMODER::DIGITAL
    }
}
#[doc = "Possible values of the field `OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODR {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN,
}
impl ODR {
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
            ODR::NORMAL => false,
            ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ODR {
        match value {
            false => ODR::NORMAL,
            true => ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
pub enum FUNCW {
    #[doc = "Alternative connection 0."]
    ALT0,
    #[doc = "Alternative connection 1."]
    ALT1,
    #[doc = "Alternative connection 2."]
    ALT2,
    #[doc = "Alternative connection 3."]
    ALT3,
    #[doc = "Alternative connection 4."]
    ALT4,
    #[doc = "Alternative connection 5."]
    ALT5,
    #[doc = "Alternative connection 6."]
    ALT6,
    #[doc = "Alternative connection 7."]
    ALT7,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::ALT0 => 0,
            FUNCW::ALT1 => 1,
            FUNCW::ALT2 => 2,
            FUNCW::ALT3 => 3,
            FUNCW::ALT4 => 4,
            FUNCW::ALT5 => 5,
            FUNCW::ALT6 => 6,
            FUNCW::ALT7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FUNCW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FUNCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternative connection 0."]
    #[inline]
    pub fn alt0(self) -> &'a mut W {
        self.variant(FUNCW::ALT0)
    }
    #[doc = "Alternative connection 1."]
    #[inline]
    pub fn alt1(self) -> &'a mut W {
        self.variant(FUNCW::ALT1)
    }
    #[doc = "Alternative connection 2."]
    #[inline]
    pub fn alt2(self) -> &'a mut W {
        self.variant(FUNCW::ALT2)
    }
    #[doc = "Alternative connection 3."]
    #[inline]
    pub fn alt3(self) -> &'a mut W {
        self.variant(FUNCW::ALT3)
    }
    #[doc = "Alternative connection 4."]
    #[inline]
    pub fn alt4(self) -> &'a mut W {
        self.variant(FUNCW::ALT4)
    }
    #[doc = "Alternative connection 5."]
    #[inline]
    pub fn alt5(self) -> &'a mut W {
        self.variant(FUNCW::ALT5)
    }
    #[doc = "Alternative connection 6."]
    #[inline]
    pub fn alt6(self) -> &'a mut W {
        self.variant(FUNCW::ALT6)
    }
    #[doc = "Alternative connection 7."]
    #[inline]
    pub fn alt7(self) -> &'a mut W {
        self.variant(FUNCW::ALT7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. Repeater mode."]
    REPEATER,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::INACTIVE => 0,
            MODEW::PULL_DOWN => 1,
            MODEW::PULL_UP => 2,
            MODEW::REPEATER => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline]
    pub fn inactive(self) -> &'a mut W {
        self.variant(MODEW::INACTIVE)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODEW::PULL_DOWN)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODEW::PULL_UP)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(MODEW::REPEATER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLEW`"]
pub enum SLEWW {
    #[doc = "Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    STANDARD,
    #[doc = "Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    FAST,
}
impl SLEWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEWW::STANDARD => false,
            SLEWW::FAST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEWW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(SLEWW::STANDARD)
    }
    #[doc = "Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    #[inline]
    pub fn fast(self) -> &'a mut W {
        self.variant(SLEWW::FAST)
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
#[doc = "Values that can be written to the field `INVERT`"]
pub enum INVERTW {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED,
}
impl INVERTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVERTW::DISABLED => false,
            INVERTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVERTW<'a> {
    w: &'a mut W,
}
impl<'a> _INVERTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVERTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Input function is not inverted."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INVERTW::DISABLED)
    }
    #[doc = "Enabled. Input is function inverted."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INVERTW::ENABLED)
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
#[doc = "Values that can be written to the field `DIGIMODE`"]
pub enum DIGIMODEW {
    #[doc = "Analog mode, digital input is disabled."]
    ANALOG,
    #[doc = "Digital mode, digital input is enabled."]
    DIGITAL,
}
impl DIGIMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIGIMODEW::ANALOG => false,
            DIGIMODEW::DIGITAL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIGIMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DIGIMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIGIMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog mode, digital input is disabled."]
    #[inline]
    pub fn analog(self) -> &'a mut W {
        self.variant(DIGIMODEW::ANALOG)
    }
    #[doc = "Digital mode, digital input is enabled."]
    #[inline]
    pub fn digital(self) -> &'a mut W {
        self.variant(DIGIMODEW::DIGITAL)
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
#[doc = "Values that can be written to the field `OD`"]
pub enum ODW {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN,
}
impl ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ODW::NORMAL => false,
            ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ODW<'a> {
    w: &'a mut W,
}
impl<'a> _ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. Normal push-pull output"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(ODW::NORMAL)
    }
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(ODW::OPEN_DRAIN)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Selects pin function."]
    #[inline]
    pub fn func(&self) -> FUNCR {
        FUNCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Driver slew rate."]
    #[inline]
    pub fn slew(&self) -> SLEWR {
        SLEWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Input polarity."]
    #[inline]
    pub fn invert(&self) -> INVERTR {
        INVERTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Select Digital mode."]
    #[inline]
    pub fn digimode(&self) -> DIGIMODER {
        DIGIMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Controls open-drain mode."]
    #[inline]
    pub fn od(&self) -> ODR {
        ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    #[doc = "Bits 0:3 - Selects pin function."]
    #[inline]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
    #[doc = "Bits 4:5 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 6 - Driver slew rate."]
    #[inline]
    pub fn slew(&mut self) -> _SLEWW {
        _SLEWW { w: self }
    }
    #[doc = "Bit 7 - Input polarity."]
    #[inline]
    pub fn invert(&mut self) -> _INVERTW {
        _INVERTW { w: self }
    }
    #[doc = "Bit 8 - Select Digital mode."]
    #[inline]
    pub fn digimode(&mut self) -> _DIGIMODEW {
        _DIGIMODEW { w: self }
    }
    #[doc = "Bit 9 - Controls open-drain mode."]
    #[inline]
    pub fn od(&mut self) -> _ODW {
        _ODW { w: self }
    }
}
