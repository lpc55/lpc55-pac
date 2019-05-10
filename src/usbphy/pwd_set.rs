#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWD_SET {
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
#[doc = "Possible values of the field `TXPWDFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPWDFSR {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    VALUE1,
}
impl TXPWDFSR {
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
            TXPWDFSR::VALUE0 => false,
            TXPWDFSR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXPWDFSR {
        match value {
            false => TXPWDFSR::VALUE0,
            true => TXPWDFSR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == TXPWDFSR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TXPWDFSR::VALUE1
    }
}
#[doc = "Possible values of the field `TXPWDIBIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPWDIBIASR {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    VALUE1,
}
impl TXPWDIBIASR {
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
            TXPWDIBIASR::VALUE0 => false,
            TXPWDIBIASR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXPWDIBIASR {
        match value {
            false => TXPWDIBIASR::VALUE0,
            true => TXPWDIBIASR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == TXPWDIBIASR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TXPWDIBIASR::VALUE1
    }
}
#[doc = "Possible values of the field `TXPWDV2I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPWDV2IR {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    VALUE1,
}
impl TXPWDV2IR {
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
            TXPWDV2IR::VALUE0 => false,
            TXPWDV2IR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXPWDV2IR {
        match value {
            false => TXPWDV2IR::VALUE0,
            true => TXPWDV2IR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == TXPWDV2IR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TXPWDV2IR::VALUE1
    }
}
#[doc = "Possible values of the field `RXPWDENV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPWDENVR {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    VALUE1,
}
impl RXPWDENVR {
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
            RXPWDENVR::VALUE0 => false,
            RXPWDENVR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXPWDENVR {
        match value {
            false => RXPWDENVR::VALUE0,
            true => RXPWDENVR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == RXPWDENVR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXPWDENVR::VALUE1
    }
}
#[doc = "Possible values of the field `RXPWD1PT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPWD1PT1R {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    VALUE1,
}
impl RXPWD1PT1R {
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
            RXPWD1PT1R::VALUE0 => false,
            RXPWD1PT1R::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXPWD1PT1R {
        match value {
            false => RXPWD1PT1R::VALUE0,
            true => RXPWD1PT1R::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == RXPWD1PT1R::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXPWD1PT1R::VALUE1
    }
}
#[doc = "Possible values of the field `RXPWDDIFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPWDDIFFR {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the USB high-speed differential receive"]
    VALUE1,
}
impl RXPWDDIFFR {
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
            RXPWDDIFFR::VALUE0 => false,
            RXPWDDIFFR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXPWDDIFFR {
        match value {
            false => RXPWDDIFFR::VALUE0,
            true => RXPWDDIFFR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == RXPWDDIFFR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXPWDDIFFR::VALUE1
    }
}
#[doc = "Possible values of the field `RXPWDRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPWDRXR {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    VALUE1,
}
impl RXPWDRXR {
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
            RXPWDRXR::VALUE0 => false,
            RXPWDRXR::VALUE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXPWDRXR {
        match value {
            false => RXPWDRXR::VALUE0,
            true => RXPWDRXR::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline]
    pub fn is_value0(&self) -> bool {
        *self == RXPWDRXR::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXPWDRXR::VALUE1
    }
}
#[doc = "Values that can be written to the field `TXPWDFS`"]
pub enum TXPWDFSW {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    VALUE1,
}
impl TXPWDFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXPWDFSW::VALUE0 => false,
            TXPWDFSW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXPWDFSW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPWDFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXPWDFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(TXPWDFSW::VALUE0)
    }
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXPWDFSW::VALUE1)
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
#[doc = "Values that can be written to the field `TXPWDIBIAS`"]
pub enum TXPWDIBIASW {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    VALUE1,
}
impl TXPWDIBIASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXPWDIBIASW::VALUE0 => false,
            TXPWDIBIASW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXPWDIBIASW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPWDIBIASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXPWDIBIASW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(TXPWDIBIASW::VALUE0)
    }
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXPWDIBIASW::VALUE1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXPWDV2I`"]
pub enum TXPWDV2IW {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    VALUE1,
}
impl TXPWDV2IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXPWDV2IW::VALUE0 => false,
            TXPWDV2IW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXPWDV2IW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPWDV2IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXPWDV2IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(TXPWDV2IW::VALUE0)
    }
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXPWDV2IW::VALUE1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXPWDENV`"]
pub enum RXPWDENVW {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    VALUE1,
}
impl RXPWDENVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXPWDENVW::VALUE0 => false,
            RXPWDENVW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPWDENVW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPWDENVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPWDENVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXPWDENVW::VALUE0)
    }
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXPWDENVW::VALUE1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXPWD1PT1`"]
pub enum RXPWD1PT1W {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    VALUE1,
}
impl RXPWD1PT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXPWD1PT1W::VALUE0 => false,
            RXPWD1PT1W::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPWD1PT1W<'a> {
    w: &'a mut W,
}
impl<'a> _RXPWD1PT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPWD1PT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXPWD1PT1W::VALUE0)
    }
    #[doc = "Power-down the USB full-speed differential receiver."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXPWD1PT1W::VALUE1)
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
#[doc = "Values that can be written to the field `RXPWDDIFF`"]
pub enum RXPWDDIFFW {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the USB high-speed differential receive"]
    VALUE1,
}
impl RXPWDDIFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXPWDDIFFW::VALUE0 => false,
            RXPWDDIFFW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPWDDIFFW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPWDDIFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPWDDIFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXPWDDIFFW::VALUE0)
    }
    #[doc = "Power-down the USB high-speed differential receive"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXPWDDIFFW::VALUE1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXPWDRX`"]
pub enum RXPWDRXW {
    #[doc = "Normal operation."]
    VALUE0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    VALUE1,
}
impl RXPWDRXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXPWDRXW::VALUE0 => false,
            RXPWDRXW::VALUE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPWDRXW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPWDRXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPWDRXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXPWDRXW::VALUE0)
    }
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXPWDRXW::VALUE1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 10 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn txpwdfs(&self) -> TXPWDFSR {
        TXPWDFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn txpwdibias(&self) -> TXPWDIBIASR {
        TXPWDIBIASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn txpwdv2i(&self) -> TXPWDV2IR {
        TXPWDV2IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn rxpwdenv(&self) -> RXPWDENVR {
        RXPWDENVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn rxpwd1pt1(&self) -> RXPWD1PT1R {
        RXPWD1PT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn rxpwddiff(&self) -> RXPWDDIFFR {
        RXPWDDIFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn rxpwdrx(&self) -> RXPWDRXR {
        RXPWDRXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1973248 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 10 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn txpwdfs(&mut self) -> _TXPWDFSW {
        _TXPWDFSW { w: self }
    }
    #[doc = "Bit 11 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn txpwdibias(&mut self) -> _TXPWDIBIASW {
        _TXPWDIBIASW { w: self }
    }
    #[doc = "Bit 12 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn txpwdv2i(&mut self) -> _TXPWDV2IW {
        _TXPWDV2IW { w: self }
    }
    #[doc = "Bit 17 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn rxpwdenv(&mut self) -> _RXPWDENVW {
        _RXPWDENVW { w: self }
    }
    #[doc = "Bit 18 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn rxpwd1pt1(&mut self) -> _RXPWD1PT1W {
        _RXPWD1PT1W { w: self }
    }
    #[doc = "Bit 19 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn rxpwddiff(&mut self) -> _RXPWDDIFFW {
        _RXPWDDIFFW { w: self }
    }
    #[doc = "Bit 20 - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline]
    pub fn rxpwdrx(&mut self) -> _RXPWDRXW {
        _RXPWDRXW { w: self }
    }
}
