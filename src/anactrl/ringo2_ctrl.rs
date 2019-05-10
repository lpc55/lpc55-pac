#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RINGO2_CTRL {
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
#[doc = "Possible values of the field `S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR {
    #[doc = "Select short ringo (few elements)."]
    SHORT,
    #[doc = "Select long ringo (many elements)."]
    LONG,
}
impl SR {
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
            SR::SHORT => false,
            SR::LONG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SR {
        match value {
            false => SR::SHORT,
            true => SR::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline]
    pub fn is_short(&self) -> bool {
        *self == SR::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline]
    pub fn is_long(&self) -> bool {
        *self == SR::LONG
    }
}
#[doc = "Possible values of the field `FS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSR {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    FAST,
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    SLOW,
}
impl FSR {
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
            FSR::FAST => false,
            FSR::SLOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSR {
        match value {
            false => FSR::FAST,
            true => FSR::SLOW,
        }
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline]
    pub fn is_fast(&self) -> bool {
        *self == FSR::FAST
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline]
    pub fn is_slow(&self) -> bool {
        *self == FSR::SLOW
    }
}
#[doc = "Possible values of the field `PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDR {
    #[doc = "The Ringo module is enabled."]
    POWERED_ON,
    #[doc = "The Ringo module is disabled."]
    POWERED_DOWN,
}
impl PDR {
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
            PDR::POWERED_ON => false,
            PDR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDR {
        match value {
            false => PDR::POWERED_ON,
            true => PDR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_ON`"]
    #[inline]
    pub fn is_powered_on(&self) -> bool {
        *self == PDR::POWERED_ON
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == PDR::POWERED_DOWN
    }
}
#[doc = "Possible values of the field `E_R24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_R24R {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl E_R24R {
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
            E_R24R::DISABLE => false,
            E_R24R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_R24R {
        match value {
            false => E_R24R::DISABLE,
            true => E_R24R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_R24R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_R24R::ENABLE
    }
}
#[doc = "Possible values of the field `E_R35`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_R35R {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl E_R35R {
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
            E_R35R::DISABLE => false,
            E_R35R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_R35R {
        match value {
            false => E_R35R::DISABLE,
            true => E_R35R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_R35R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_R35R::ENABLE
    }
}
#[doc = "Possible values of the field `E_M2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_M2R {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl E_M2R {
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
            E_M2R::DISABLE => false,
            E_M2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_M2R {
        match value {
            false => E_M2R::DISABLE,
            true => E_M2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_M2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_M2R::ENABLE
    }
}
#[doc = "Possible values of the field `E_M3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_M3R {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl E_M3R {
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
            E_M3R::DISABLE => false,
            E_M3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_M3R {
        match value {
            false => E_M3R::DISABLE,
            true => E_M3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_M3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_M3R::ENABLE
    }
}
#[doc = "Possible values of the field `E_M4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_M4R {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl E_M4R {
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
            E_M4R::DISABLE => false,
            E_M4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_M4R {
        match value {
            false => E_M4R::DISABLE,
            true => E_M4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_M4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_M4R::ENABLE
    }
}
#[doc = "Possible values of the field `E_M5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_M5R {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl E_M5R {
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
            E_M5R::DISABLE => false,
            E_M5R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_M5R {
        match value {
            false => E_M5R::DISABLE,
            true => E_M5R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_M5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_M5R::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct DIVISORR {
    bits: u8,
}
impl DIVISORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIV_UPDATE_REQR {
    bits: bool,
}
impl DIV_UPDATE_REQR {
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
#[doc = "Values that can be written to the field `S`"]
pub enum SW {
    #[doc = "Select short ringo (few elements)."]
    SHORT,
    #[doc = "Select long ringo (many elements)."]
    LONG,
}
impl SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SW::SHORT => false,
            SW::LONG => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SW<'a> {
    w: &'a mut W,
}
impl<'a> _SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select short ringo (few elements)."]
    #[inline]
    pub fn short(self) -> &'a mut W {
        self.variant(SW::SHORT)
    }
    #[doc = "Select long ringo (many elements)."]
    #[inline]
    pub fn long(self) -> &'a mut W {
        self.variant(SW::LONG)
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
#[doc = "Values that can be written to the field `FS`"]
pub enum FSW {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    FAST,
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    SLOW,
}
impl FSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSW::FAST => false,
            FSW::SLOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSW<'a> {
    w: &'a mut W,
}
impl<'a> _FSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    #[inline]
    pub fn fast(self) -> &'a mut W {
        self.variant(FSW::FAST)
    }
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    #[inline]
    pub fn slow(self) -> &'a mut W {
        self.variant(FSW::SLOW)
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
#[doc = "Values that can be written to the field `PD`"]
pub enum PDW {
    #[doc = "The Ringo module is enabled."]
    POWERED_ON,
    #[doc = "The Ringo module is disabled."]
    POWERED_DOWN,
}
impl PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDW::POWERED_ON => false,
            PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDW<'a> {
    w: &'a mut W,
}
impl<'a> _PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Ringo module is enabled."]
    #[inline]
    pub fn powered_on(self) -> &'a mut W {
        self.variant(PDW::POWERED_ON)
    }
    #[doc = "The Ringo module is disabled."]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(PDW::POWERED_DOWN)
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
#[doc = "Values that can be written to the field `E_R24`"]
pub enum E_R24W {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl E_R24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_R24W::DISABLE => false,
            E_R24W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_R24W<'a> {
    w: &'a mut W,
}
impl<'a> _E_R24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_R24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_R24W::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_R24W::ENABLE)
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
#[doc = "Values that can be written to the field `E_R35`"]
pub enum E_R35W {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl E_R35W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_R35W::DISABLE => false,
            E_R35W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_R35W<'a> {
    w: &'a mut W,
}
impl<'a> _E_R35W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_R35W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_R35W::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_R35W::ENABLE)
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
#[doc = "Values that can be written to the field `E_M2`"]
pub enum E_M2W {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl E_M2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_M2W::DISABLE => false,
            E_M2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_M2W<'a> {
    w: &'a mut W,
}
impl<'a> _E_M2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_M2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_M2W::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_M2W::ENABLE)
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
#[doc = "Values that can be written to the field `E_M3`"]
pub enum E_M3W {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl E_M3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_M3W::DISABLE => false,
            E_M3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_M3W<'a> {
    w: &'a mut W,
}
impl<'a> _E_M3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_M3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_M3W::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_M3W::ENABLE)
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
#[doc = "Values that can be written to the field `E_M4`"]
pub enum E_M4W {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl E_M4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_M4W::DISABLE => false,
            E_M4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_M4W<'a> {
    w: &'a mut W,
}
impl<'a> _E_M4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_M4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_M4W::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_M4W::ENABLE)
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
#[doc = "Values that can be written to the field `E_M5`"]
pub enum E_M5W {
    #[doc = "Ringo is disabled."]
    DISABLE,
    #[doc = "Ringo is enabled."]
    ENABLE,
}
impl E_M5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_M5W::DISABLE => false,
            E_M5W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_M5W<'a> {
    w: &'a mut W,
}
impl<'a> _E_M5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_M5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_M5W::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_M5W::ENABLE)
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
#[doc = r" Proxy"]
pub struct _DIVISORW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVISORW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Select short or long ringo (for all ringos types)."]
    #[inline]
    pub fn s(&self) -> SR {
        SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline]
    pub fn fs(&self) -> FSR {
        FSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Ringo module Power control."]
    #[inline]
    pub fn pd(&self) -> PDR {
        PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ."]
    #[inline]
    pub fn e_r24(&self) -> E_R24R {
        E_R24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - ."]
    #[inline]
    pub fn e_r35(&self) -> E_R35R {
        E_R35R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Metal 2 (M2) monitor control."]
    #[inline]
    pub fn e_m2(&self) -> E_M2R {
        E_M2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Metal 3 (M3) monitor control."]
    #[inline]
    pub fn e_m3(&self) -> E_M3R {
        E_M3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Metal 4 (M4) monitor control."]
    #[inline]
    pub fn e_m4(&self) -> E_M4R {
        E_M4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Metal 5 (M5) monitor control."]
    #[inline]
    pub fn e_m5(&self) -> E_M5R {
        E_M5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline]
    pub fn divisor(&self) -> DIVISORR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIVISORR { bits }
    }
    #[doc = "Bit 31 - Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline]
    pub fn div_update_req(&self) -> DIV_UPDATE_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIV_UPDATE_REQR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 64 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Select short or long ringo (for all ringos types)."]
    #[inline]
    pub fn s(&mut self) -> _SW {
        _SW { w: self }
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline]
    pub fn fs(&mut self) -> _FSW {
        _FSW { w: self }
    }
    #[doc = "Bit 2 - Ringo module Power control."]
    #[inline]
    pub fn pd(&mut self) -> _PDW {
        _PDW { w: self }
    }
    #[doc = "Bit 3 - ."]
    #[inline]
    pub fn e_r24(&mut self) -> _E_R24W {
        _E_R24W { w: self }
    }
    #[doc = "Bit 4 - ."]
    #[inline]
    pub fn e_r35(&mut self) -> _E_R35W {
        _E_R35W { w: self }
    }
    #[doc = "Bit 5 - Metal 2 (M2) monitor control."]
    #[inline]
    pub fn e_m2(&mut self) -> _E_M2W {
        _E_M2W { w: self }
    }
    #[doc = "Bit 6 - Metal 3 (M3) monitor control."]
    #[inline]
    pub fn e_m3(&mut self) -> _E_M3W {
        _E_M3W { w: self }
    }
    #[doc = "Bit 7 - Metal 4 (M4) monitor control."]
    #[inline]
    pub fn e_m4(&mut self) -> _E_M4W {
        _E_M4W { w: self }
    }
    #[doc = "Bit 8 - Metal 5 (M5) monitor control."]
    #[inline]
    pub fn e_m5(&mut self) -> _E_M5W {
        _E_M5W { w: self }
    }
    #[doc = "Bits 16:19 - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline]
    pub fn divisor(&mut self) -> _DIVISORW {
        _DIVISORW { w: self }
    }
}
