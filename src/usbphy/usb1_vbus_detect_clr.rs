#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USB1_VBUS_DETECT_CLR {
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
#[doc = "Possible values of the field `VBUSVALID_THRESH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSVALID_THRESHR {
    #[doc = "4.0V"]
    VALUE0,
    #[doc = "4.1V"]
    VALUE1,
    #[doc = "4.2V"]
    VALUE2,
    #[doc = "4.3V"]
    VALUE3,
    #[doc = "4.4V(Default)"]
    VALUE4,
    #[doc = "4.5V"]
    VALUE5,
    #[doc = "4.6V"]
    VALUE6,
    #[doc = "4.7V"]
    VALUE7,
}
impl VBUSVALID_THRESHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VBUSVALID_THRESHR::VALUE0 => 0,
            VBUSVALID_THRESHR::VALUE1 => 1,
            VBUSVALID_THRESHR::VALUE2 => 2,
            VBUSVALID_THRESHR::VALUE3 => 3,
            VBUSVALID_THRESHR::VALUE4 => 4,
            VBUSVALID_THRESHR::VALUE5 => 5,
            VBUSVALID_THRESHR::VALUE6 => 6,
            VBUSVALID_THRESHR::VALUE7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VBUSVALID_THRESHR {
        match value {
            0 => VBUSVALID_THRESHR::VALUE0,
            1 => VBUSVALID_THRESHR::VALUE1,
            2 => VBUSVALID_THRESHR::VALUE2,
            3 => VBUSVALID_THRESHR::VALUE3,
            4 => VBUSVALID_THRESHR::VALUE4,
            5 => VBUSVALID_THRESHR::VALUE5,
            6 => VBUSVALID_THRESHR::VALUE6,
            7 => VBUSVALID_THRESHR::VALUE7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == VBUSVALID_THRESHR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VBUSVALID_THRESHR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VBUSVALID_THRESHR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == VBUSVALID_THRESHR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == VBUSVALID_THRESHR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == VBUSVALID_THRESHR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == VBUSVALID_THRESHR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == VBUSVALID_THRESHR::VALUE7
    }
}
#[doc = "Possible values of the field `VBUS_OVERRIDE_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_OVERRIDE_ENR {
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    VALUE0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    VALUE1,
}
impl VBUS_OVERRIDE_ENR {
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
            VBUS_OVERRIDE_ENR::VALUE0 => false,
            VBUS_OVERRIDE_ENR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBUS_OVERRIDE_ENR {
        match value {
            false => VBUS_OVERRIDE_ENR::VALUE0,
            true => VBUS_OVERRIDE_ENR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == VBUS_OVERRIDE_ENR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VBUS_OVERRIDE_ENR::VALUE1
    }
}
#[doc = r" Value of the field"]
pub struct SESSEND_OVERRIDER {
    bits: bool,
}
impl SESSEND_OVERRIDER {
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
pub struct BVALID_OVERRIDER {
    bits: bool,
}
impl BVALID_OVERRIDER {
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
pub struct AVALID_OVERRIDER {
    bits: bool,
}
impl AVALID_OVERRIDER {
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
pub struct VBUSVALID_OVERRIDER {
    bits: bool,
}
impl VBUSVALID_OVERRIDER {
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
#[doc = "Possible values of the field `VBUSVALID_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSVALID_SELR {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    VALUE1,
}
impl VBUSVALID_SELR {
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
            VBUSVALID_SELR::VALUE0 => false,
            VBUSVALID_SELR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBUSVALID_SELR {
        match value {
            false => VBUSVALID_SELR::VALUE0,
            true => VBUSVALID_SELR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == VBUSVALID_SELR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VBUSVALID_SELR::VALUE1
    }
}
#[doc = "Possible values of the field `VBUS_SOURCE_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_SOURCE_SELR {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE1,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VBUS_SOURCE_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VBUS_SOURCE_SELR::VALUE0 => 0,
            VBUS_SOURCE_SELR::VALUE1 => 1,
            VBUS_SOURCE_SELR::VALUE2 => 2,
            VBUS_SOURCE_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VBUS_SOURCE_SELR {
        match value {
            0 => VBUS_SOURCE_SELR::VALUE0,
            1 => VBUS_SOURCE_SELR::VALUE1,
            2 => VBUS_SOURCE_SELR::VALUE2,
            i => VBUS_SOURCE_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == VBUS_SOURCE_SELR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VBUS_SOURCE_SELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VBUS_SOURCE_SELR::VALUE2
    }
}
#[doc = "Possible values of the field `VBUSVALID_TO_SESSVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSVALID_TO_SESSVALIDR {
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    VALUE0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    VALUE1,
}
impl VBUSVALID_TO_SESSVALIDR {
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
            VBUSVALID_TO_SESSVALIDR::VALUE0 => false,
            VBUSVALID_TO_SESSVALIDR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VBUSVALID_TO_SESSVALIDR {
        match value {
            false => VBUSVALID_TO_SESSVALIDR::VALUE0,
            true => VBUSVALID_TO_SESSVALIDR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == VBUSVALID_TO_SESSVALIDR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VBUSVALID_TO_SESSVALIDR::VALUE1
    }
}
#[doc = "Possible values of the field `PWRUP_CMPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRUP_CMPSR {
    #[doc = "Powers down the VBUS_VALID comparator"]
    VALUE0,
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    VALUE1,
}
impl PWRUP_CMPSR {
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
            PWRUP_CMPSR::VALUE0 => false,
            PWRUP_CMPSR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWRUP_CMPSR {
        match value {
            false => PWRUP_CMPSR::VALUE0,
            true => PWRUP_CMPSR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == PWRUP_CMPSR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PWRUP_CMPSR::VALUE1
    }
}
#[doc = "Possible values of the field `DISCHARGE_VBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCHARGE_VBUSR {
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    VALUE0,
    #[doc = "VBUS discharge resistor is enabled"]
    VALUE1,
}
impl DISCHARGE_VBUSR {
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
            DISCHARGE_VBUSR::VALUE0 => false,
            DISCHARGE_VBUSR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISCHARGE_VBUSR {
        match value {
            false => DISCHARGE_VBUSR::VALUE0,
            true => DISCHARGE_VBUSR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == DISCHARGE_VBUSR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DISCHARGE_VBUSR::VALUE1
    }
}
#[doc = "Possible values of the field `EN_CHARGER_RESISTOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_CHARGER_RESISTORR {
    #[doc = "Disable resistive charger detection resistors on USB_DP and USB_DP"]
    VALUE0,
    #[doc = "Enable resistive charger detection resistors on USB_DP and USB_DP"]
    VALUE1,
}
impl EN_CHARGER_RESISTORR {
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
            EN_CHARGER_RESISTORR::VALUE0 => false,
            EN_CHARGER_RESISTORR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_CHARGER_RESISTORR {
        match value {
            false => EN_CHARGER_RESISTORR::VALUE0,
            true => EN_CHARGER_RESISTORR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == EN_CHARGER_RESISTORR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EN_CHARGER_RESISTORR::VALUE1
    }
}
#[doc = "Values that can be written to the field `VBUSVALID_THRESH`"]
pub enum VBUSVALID_THRESHW {
    #[doc = "4.0V"]
    VALUE0,
    #[doc = "4.1V"]
    VALUE1,
    #[doc = "4.2V"]
    VALUE2,
    #[doc = "4.3V"]
    VALUE3,
    #[doc = "4.4V(Default)"]
    VALUE4,
    #[doc = "4.5V"]
    VALUE5,
    #[doc = "4.6V"]
    VALUE6,
    #[doc = "4.7V"]
    VALUE7,
}
impl VBUSVALID_THRESHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VBUSVALID_THRESHW::VALUE0 => 0,
            VBUSVALID_THRESHW::VALUE1 => 1,
            VBUSVALID_THRESHW::VALUE2 => 2,
            VBUSVALID_THRESHW::VALUE3 => 3,
            VBUSVALID_THRESHW::VALUE4 => 4,
            VBUSVALID_THRESHW::VALUE5 => 5,
            VBUSVALID_THRESHW::VALUE6 => 6,
            VBUSVALID_THRESHW::VALUE7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBUSVALID_THRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSVALID_THRESHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBUSVALID_THRESHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4.0V"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::VALUE0)
    }
    #[doc = "4.1V"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::VALUE1)
    }
    #[doc = "4.2V"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::VALUE2)
    }
    #[doc = "4.3V"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::VALUE3)
    }
    #[doc = "4.4V(Default)"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::VALUE4)
    }
    #[doc = "4.5V"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::VALUE5)
    }
    #[doc = "4.6V"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::VALUE6)
    }
    #[doc = "4.7V"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESHW::VALUE7)
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
#[doc = "Values that can be written to the field `VBUS_OVERRIDE_EN`"]
pub enum VBUS_OVERRIDE_ENW {
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    VALUE0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    VALUE1,
}
impl VBUS_OVERRIDE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VBUS_OVERRIDE_ENW::VALUE0 => false,
            VBUS_OVERRIDE_ENW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBUS_OVERRIDE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUS_OVERRIDE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBUS_OVERRIDE_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(VBUS_OVERRIDE_ENW::VALUE0)
    }
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBUS_OVERRIDE_ENW::VALUE1)
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
#[doc = r" Proxy"]
pub struct _SESSEND_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _SESSEND_OVERRIDEW<'a> {
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
pub struct _BVALID_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _BVALID_OVERRIDEW<'a> {
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
pub struct _AVALID_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _AVALID_OVERRIDEW<'a> {
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
pub struct _VBUSVALID_OVERRIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSVALID_OVERRIDEW<'a> {
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
#[doc = "Values that can be written to the field `VBUSVALID_SEL`"]
pub enum VBUSVALID_SELW {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    VALUE1,
}
impl VBUSVALID_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VBUSVALID_SELW::VALUE0 => false,
            VBUSVALID_SELW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBUSVALID_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSVALID_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBUSVALID_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(VBUSVALID_SELW::VALUE0)
    }
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBUSVALID_SELW::VALUE1)
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
#[doc = "Values that can be written to the field `VBUS_SOURCE_SEL`"]
pub enum VBUS_SOURCE_SELW {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE1,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE2,
}
impl VBUS_SOURCE_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VBUS_SOURCE_SELW::VALUE0 => 0,
            VBUS_SOURCE_SELW::VALUE1 => 1,
            VBUS_SOURCE_SELW::VALUE2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBUS_SOURCE_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUS_SOURCE_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBUS_SOURCE_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(VBUS_SOURCE_SELW::VALUE0)
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBUS_SOURCE_SELW::VALUE1)
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBUS_SOURCE_SELW::VALUE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VBUSVALID_TO_SESSVALID`"]
pub enum VBUSVALID_TO_SESSVALIDW {
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    VALUE0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    VALUE1,
}
impl VBUSVALID_TO_SESSVALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VBUSVALID_TO_SESSVALIDW::VALUE0 => false,
            VBUSVALID_TO_SESSVALIDW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBUSVALID_TO_SESSVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSVALID_TO_SESSVALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBUSVALID_TO_SESSVALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(VBUSVALID_TO_SESSVALIDW::VALUE0)
    }
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBUSVALID_TO_SESSVALIDW::VALUE1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWRUP_CMPS`"]
pub enum PWRUP_CMPSW {
    #[doc = "Powers down the VBUS_VALID comparator"]
    VALUE0,
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    VALUE1,
}
impl PWRUP_CMPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWRUP_CMPSW::VALUE0 => false,
            PWRUP_CMPSW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRUP_CMPSW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRUP_CMPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRUP_CMPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powers down the VBUS_VALID comparator"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(PWRUP_CMPSW::VALUE0)
    }
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PWRUP_CMPSW::VALUE1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DISCHARGE_VBUS`"]
pub enum DISCHARGE_VBUSW {
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    VALUE0,
    #[doc = "VBUS discharge resistor is enabled"]
    VALUE1,
}
impl DISCHARGE_VBUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISCHARGE_VBUSW::VALUE0 => false,
            DISCHARGE_VBUSW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISCHARGE_VBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCHARGE_VBUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISCHARGE_VBUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(DISCHARGE_VBUSW::VALUE0)
    }
    #[doc = "VBUS discharge resistor is enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DISCHARGE_VBUSW::VALUE1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN_CHARGER_RESISTOR`"]
pub enum EN_CHARGER_RESISTORW {
    #[doc = "Disable resistive charger detection resistors on USB_DP and USB_DP"]
    VALUE0,
    #[doc = "Enable resistive charger detection resistors on USB_DP and USB_DP"]
    VALUE1,
}
impl EN_CHARGER_RESISTORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_CHARGER_RESISTORW::VALUE0 => false,
            EN_CHARGER_RESISTORW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_CHARGER_RESISTORW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_CHARGER_RESISTORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_CHARGER_RESISTORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable resistive charger detection resistors on USB_DP and USB_DP"]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(EN_CHARGER_RESISTORW::VALUE0)
    }
    #[doc = "Enable resistive charger detection resistors on USB_DP and USB_DP"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EN_CHARGER_RESISTORW::VALUE1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:2 - Sets the threshold for the VBUSVALID comparator"]
    #[inline]
    pub fn vbusvalid_thresh(&self) -> VBUSVALID_THRESHR {
        VBUSVALID_THRESHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - VBUS detect signal override enable"]
    #[inline]
    pub fn vbus_override_en(&self) -> VBUS_OVERRIDE_ENR {
        VBUS_OVERRIDE_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline]
    pub fn sessend_override(&self) -> SESSEND_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SESSEND_OVERRIDER { bits }
    }
    #[doc = "Bit 5 - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline]
    pub fn bvalid_override(&self) -> BVALID_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BVALID_OVERRIDER { bits }
    }
    #[doc = "Bit 6 - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline]
    pub fn avalid_override(&self) -> AVALID_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AVALID_OVERRIDER { bits }
    }
    #[doc = "Bit 7 - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
    #[inline]
    pub fn vbusvalid_override(&self) -> VBUSVALID_OVERRIDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBUSVALID_OVERRIDER { bits }
    }
    #[doc = "Bit 8 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline]
    pub fn vbusvalid_sel(&self) -> VBUSVALID_SELR {
        VBUSVALID_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:10 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline]
    pub fn vbus_source_sel(&self) -> VBUS_SOURCE_SELR {
        VBUS_SOURCE_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
    #[inline]
    pub fn vbusvalid_to_sessvalid(&self) -> VBUSVALID_TO_SESSVALIDR {
        VBUSVALID_TO_SESSVALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[inline]
    pub fn pwrup_cmps(&self) -> PWRUP_CMPSR {
        PWRUP_CMPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[inline]
    pub fn discharge_vbus(&self) -> DISCHARGE_VBUSR {
        DISCHARGE_VBUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Enables resistors used for an older method of resistive battery charger detection"]
    #[inline]
    pub fn en_charger_resistor(&self) -> EN_CHARGER_RESISTORR {
        EN_CHARGER_RESISTORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 7340036 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Sets the threshold for the VBUSVALID comparator"]
    #[inline]
    pub fn vbusvalid_thresh(&mut self) -> _VBUSVALID_THRESHW {
        _VBUSVALID_THRESHW { w: self }
    }
    #[doc = "Bit 3 - VBUS detect signal override enable"]
    #[inline]
    pub fn vbus_override_en(&mut self) -> _VBUS_OVERRIDE_ENW {
        _VBUS_OVERRIDE_ENW { w: self }
    }
    #[doc = "Bit 4 - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline]
    pub fn sessend_override(&mut self) -> _SESSEND_OVERRIDEW {
        _SESSEND_OVERRIDEW { w: self }
    }
    #[doc = "Bit 5 - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline]
    pub fn bvalid_override(&mut self) -> _BVALID_OVERRIDEW {
        _BVALID_OVERRIDEW { w: self }
    }
    #[doc = "Bit 6 - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline]
    pub fn avalid_override(&mut self) -> _AVALID_OVERRIDEW {
        _AVALID_OVERRIDEW { w: self }
    }
    #[doc = "Bit 7 - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
    #[inline]
    pub fn vbusvalid_override(&mut self) -> _VBUSVALID_OVERRIDEW {
        _VBUSVALID_OVERRIDEW { w: self }
    }
    #[doc = "Bit 8 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline]
    pub fn vbusvalid_sel(&mut self) -> _VBUSVALID_SELW {
        _VBUSVALID_SELW { w: self }
    }
    #[doc = "Bits 9:10 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline]
    pub fn vbus_source_sel(&mut self) -> _VBUS_SOURCE_SELW {
        _VBUS_SOURCE_SELW { w: self }
    }
    #[doc = "Bit 18 - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
    #[inline]
    pub fn vbusvalid_to_sessvalid(&mut self) -> _VBUSVALID_TO_SESSVALIDW {
        _VBUSVALID_TO_SESSVALIDW { w: self }
    }
    #[doc = "Bit 20 - Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[inline]
    pub fn pwrup_cmps(&mut self) -> _PWRUP_CMPSW {
        _PWRUP_CMPSW { w: self }
    }
    #[doc = "Bit 26 - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[inline]
    pub fn discharge_vbus(&mut self) -> _DISCHARGE_VBUSW {
        _DISCHARGE_VBUSW { w: self }
    }
    #[doc = "Bit 31 - Enables resistors used for an older method of resistive battery charger detection"]
    #[inline]
    pub fn en_charger_resistor(&mut self) -> _EN_CHARGER_RESISTORW {
        _EN_CHARGER_RESISTORW { w: self }
    }
}
