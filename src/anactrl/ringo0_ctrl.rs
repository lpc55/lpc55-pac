#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RINGO0_CTRL {
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
#[doc = "Possible values of the field `SL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLR {
    #[doc = "Select short ringo (few elements)."]
    SHORT,
    #[doc = "Select long ringo (many elements)."]
    LONG,
}
impl SLR {
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
            SLR::SHORT => false,
            SLR::LONG => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLR {
        match value {
            false => SLR::SHORT,
            true => SLR::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline]
    pub fn is_short(&self) -> bool {
        *self == SLR::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline]
    pub fn is_long(&self) -> bool {
        *self == SLR::LONG
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
#[doc = "Possible values of the field `SWN_SWP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWN_SWPR {
    #[doc = "Normal mode."]
    NORMAL,
    #[doc = "P-Monitor mode. Measure with weak P transistor."]
    P_MONITOR,
    #[doc = "P-Monitor mode. Measure with weak N transistor."]
    N_MONITOR,
    #[doc = "Don't use."]
    FORBIDDEN,
}
impl SWN_SWPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SWN_SWPR::NORMAL => 0,
            SWN_SWPR::P_MONITOR => 1,
            SWN_SWPR::N_MONITOR => 2,
            SWN_SWPR::FORBIDDEN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SWN_SWPR {
        match value {
            0 => SWN_SWPR::NORMAL,
            1 => SWN_SWPR::P_MONITOR,
            2 => SWN_SWPR::N_MONITOR,
            3 => SWN_SWPR::FORBIDDEN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == SWN_SWPR::NORMAL
    }
    #[doc = "Checks if the value of the field is `P_MONITOR`"]
    #[inline]
    pub fn is_p_monitor(&self) -> bool {
        *self == SWN_SWPR::P_MONITOR
    }
    #[doc = "Checks if the value of the field is `N_MONITOR`"]
    #[inline]
    pub fn is_n_monitor(&self) -> bool {
        *self == SWN_SWPR::N_MONITOR
    }
    #[doc = "Checks if the value of the field is `FORBIDDEN`"]
    #[inline]
    pub fn is_forbidden(&self) -> bool {
        *self == SWN_SWPR::FORBIDDEN
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
#[doc = "Possible values of the field `E_ND0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_ND0R {
    #[doc = "First NAND2-based ringo is disabled."]
    DISABLE,
    #[doc = "First NAND2-based ringo is enabled."]
    ENABLE,
}
impl E_ND0R {
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
            E_ND0R::DISABLE => false,
            E_ND0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_ND0R {
        match value {
            false => E_ND0R::DISABLE,
            true => E_ND0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_ND0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_ND0R::ENABLE
    }
}
#[doc = "Possible values of the field `E_ND1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_ND1R {
    #[doc = "Second NAND2-based ringo is disabled."]
    DISABLE,
    #[doc = "Second NAND2-based ringo is enabled."]
    ENABLE,
}
impl E_ND1R {
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
            E_ND1R::DISABLE => false,
            E_ND1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_ND1R {
        match value {
            false => E_ND1R::DISABLE,
            true => E_ND1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_ND1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_ND1R::ENABLE
    }
}
#[doc = "Possible values of the field `E_NR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_NR0R {
    #[doc = "First NOR2-based ringo is disabled."]
    DISABLE,
    #[doc = "First NOR2-based ringo is enabled."]
    ENABLE,
}
impl E_NR0R {
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
            E_NR0R::DISABLE => false,
            E_NR0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_NR0R {
        match value {
            false => E_NR0R::DISABLE,
            true => E_NR0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_NR0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_NR0R::ENABLE
    }
}
#[doc = "Possible values of the field `E_NR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_NR1R {
    #[doc = "Second NORD2-based ringo is disabled."]
    DISABLE,
    #[doc = "Second NORD2-based ringo is enabled."]
    ENABLE,
}
impl E_NR1R {
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
            E_NR1R::DISABLE => false,
            E_NR1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_NR1R {
        match value {
            false => E_NR1R::DISABLE,
            true => E_NR1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_NR1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_NR1R::ENABLE
    }
}
#[doc = "Possible values of the field `E_IV0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_IV0R {
    #[doc = "First INV-based ringo is disabled."]
    DISABLE,
    #[doc = "First INV-based ringo is enabled."]
    ENABLE,
}
impl E_IV0R {
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
            E_IV0R::DISABLE => false,
            E_IV0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_IV0R {
        match value {
            false => E_IV0R::DISABLE,
            true => E_IV0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_IV0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_IV0R::ENABLE
    }
}
#[doc = "Possible values of the field `E_IV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_IV1R {
    #[doc = "Second INV-based ringo is disabled."]
    DISABLE,
    #[doc = "Second INV-based ringo is enabled."]
    ENABLE,
}
impl E_IV1R {
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
            E_IV1R::DISABLE => false,
            E_IV1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_IV1R {
        match value {
            false => E_IV1R::DISABLE,
            true => E_IV1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_IV1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_IV1R::ENABLE
    }
}
#[doc = "Possible values of the field `E_PN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_PN0R {
    #[doc = "First PN-based ringo is disabled."]
    DISABLE,
    #[doc = "First PN-based ringo is enabled."]
    ENABLE,
}
impl E_PN0R {
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
            E_PN0R::DISABLE => false,
            E_PN0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_PN0R {
        match value {
            false => E_PN0R::DISABLE,
            true => E_PN0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_PN0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_PN0R::ENABLE
    }
}
#[doc = "Possible values of the field `E_PN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum E_PN1R {
    #[doc = "Second PN-based ringo is disabled."]
    DISABLE,
    #[doc = "Second PN-based ringo is enabled."]
    ENABLE,
}
impl E_PN1R {
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
            E_PN1R::DISABLE => false,
            E_PN1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> E_PN1R {
        match value {
            false => E_PN1R::DISABLE,
            true => E_PN1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == E_PN1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == E_PN1R::ENABLE
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
#[doc = "Values that can be written to the field `SL`"]
pub enum SLW {
    #[doc = "Select short ringo (few elements)."]
    SHORT,
    #[doc = "Select long ringo (many elements)."]
    LONG,
}
impl SLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLW::SHORT => false,
            SLW::LONG => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLW<'a> {
    w: &'a mut W,
}
impl<'a> _SLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Select short ringo (few elements)."]
    #[inline]
    pub fn short(self) -> &'a mut W {
        self.variant(SLW::SHORT)
    }
    #[doc = "Select long ringo (many elements)."]
    #[inline]
    pub fn long(self) -> &'a mut W {
        self.variant(SLW::LONG)
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
#[doc = "Values that can be written to the field `SWN_SWP`"]
pub enum SWN_SWPW {
    #[doc = "Normal mode."]
    NORMAL,
    #[doc = "P-Monitor mode. Measure with weak P transistor."]
    P_MONITOR,
    #[doc = "P-Monitor mode. Measure with weak N transistor."]
    N_MONITOR,
    #[doc = "Don't use."]
    FORBIDDEN,
}
impl SWN_SWPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SWN_SWPW::NORMAL => 0,
            SWN_SWPW::P_MONITOR => 1,
            SWN_SWPW::N_MONITOR => 2,
            SWN_SWPW::FORBIDDEN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWN_SWPW<'a> {
    w: &'a mut W,
}
impl<'a> _SWN_SWPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWN_SWPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(SWN_SWPW::NORMAL)
    }
    #[doc = "P-Monitor mode. Measure with weak P transistor."]
    #[inline]
    pub fn p_monitor(self) -> &'a mut W {
        self.variant(SWN_SWPW::P_MONITOR)
    }
    #[doc = "P-Monitor mode. Measure with weak N transistor."]
    #[inline]
    pub fn n_monitor(self) -> &'a mut W {
        self.variant(SWN_SWPW::N_MONITOR)
    }
    #[doc = "Don't use."]
    #[inline]
    pub fn forbidden(self) -> &'a mut W {
        self.variant(SWN_SWPW::FORBIDDEN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `E_ND0`"]
pub enum E_ND0W {
    #[doc = "First NAND2-based ringo is disabled."]
    DISABLE,
    #[doc = "First NAND2-based ringo is enabled."]
    ENABLE,
}
impl E_ND0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_ND0W::DISABLE => false,
            E_ND0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_ND0W<'a> {
    w: &'a mut W,
}
impl<'a> _E_ND0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_ND0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "First NAND2-based ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_ND0W::DISABLE)
    }
    #[doc = "First NAND2-based ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_ND0W::ENABLE)
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
#[doc = "Values that can be written to the field `E_ND1`"]
pub enum E_ND1W {
    #[doc = "Second NAND2-based ringo is disabled."]
    DISABLE,
    #[doc = "Second NAND2-based ringo is enabled."]
    ENABLE,
}
impl E_ND1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_ND1W::DISABLE => false,
            E_ND1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_ND1W<'a> {
    w: &'a mut W,
}
impl<'a> _E_ND1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_ND1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Second NAND2-based ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_ND1W::DISABLE)
    }
    #[doc = "Second NAND2-based ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_ND1W::ENABLE)
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
#[doc = "Values that can be written to the field `E_NR0`"]
pub enum E_NR0W {
    #[doc = "First NOR2-based ringo is disabled."]
    DISABLE,
    #[doc = "First NOR2-based ringo is enabled."]
    ENABLE,
}
impl E_NR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_NR0W::DISABLE => false,
            E_NR0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_NR0W<'a> {
    w: &'a mut W,
}
impl<'a> _E_NR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_NR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "First NOR2-based ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_NR0W::DISABLE)
    }
    #[doc = "First NOR2-based ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_NR0W::ENABLE)
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
#[doc = "Values that can be written to the field `E_NR1`"]
pub enum E_NR1W {
    #[doc = "Second NORD2-based ringo is disabled."]
    DISABLE,
    #[doc = "Second NORD2-based ringo is enabled."]
    ENABLE,
}
impl E_NR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_NR1W::DISABLE => false,
            E_NR1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_NR1W<'a> {
    w: &'a mut W,
}
impl<'a> _E_NR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_NR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Second NORD2-based ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_NR1W::DISABLE)
    }
    #[doc = "Second NORD2-based ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_NR1W::ENABLE)
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
#[doc = "Values that can be written to the field `E_IV0`"]
pub enum E_IV0W {
    #[doc = "First INV-based ringo is disabled."]
    DISABLE,
    #[doc = "First INV-based ringo is enabled."]
    ENABLE,
}
impl E_IV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_IV0W::DISABLE => false,
            E_IV0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_IV0W<'a> {
    w: &'a mut W,
}
impl<'a> _E_IV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_IV0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "First INV-based ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_IV0W::DISABLE)
    }
    #[doc = "First INV-based ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_IV0W::ENABLE)
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
#[doc = "Values that can be written to the field `E_IV1`"]
pub enum E_IV1W {
    #[doc = "Second INV-based ringo is disabled."]
    DISABLE,
    #[doc = "Second INV-based ringo is enabled."]
    ENABLE,
}
impl E_IV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_IV1W::DISABLE => false,
            E_IV1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_IV1W<'a> {
    w: &'a mut W,
}
impl<'a> _E_IV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_IV1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Second INV-based ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_IV1W::DISABLE)
    }
    #[doc = "Second INV-based ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_IV1W::ENABLE)
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
#[doc = "Values that can be written to the field `E_PN0`"]
pub enum E_PN0W {
    #[doc = "First PN-based ringo is disabled."]
    DISABLE,
    #[doc = "First PN-based ringo is enabled."]
    ENABLE,
}
impl E_PN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_PN0W::DISABLE => false,
            E_PN0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_PN0W<'a> {
    w: &'a mut W,
}
impl<'a> _E_PN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_PN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "First PN-based ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_PN0W::DISABLE)
    }
    #[doc = "First PN-based ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_PN0W::ENABLE)
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
#[doc = "Values that can be written to the field `E_PN1`"]
pub enum E_PN1W {
    #[doc = "Second PN-based ringo is disabled."]
    DISABLE,
    #[doc = "Second PN-based ringo is enabled."]
    ENABLE,
}
impl E_PN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            E_PN1W::DISABLE => false,
            E_PN1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _E_PN1W<'a> {
    w: &'a mut W,
}
impl<'a> _E_PN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: E_PN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Second PN-based ringo is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_PN1W::DISABLE)
    }
    #[doc = "Second PN-based ringo is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_PN1W::ENABLE)
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
    pub fn sl(&self) -> SLR {
        SLR::_from({
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
    #[doc = "Bits 2:3 - PN-Ringos (P-Transistor and N-Transistor processing) control."]
    #[inline]
    pub fn swn_swp(&self) -> SWN_SWPR {
        SWN_SWPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Ringo module Power control."]
    #[inline]
    pub fn pd(&self) -> PDR {
        PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - First NAND2-based ringo control."]
    #[inline]
    pub fn e_nd0(&self) -> E_ND0R {
        E_ND0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Second NAND2-based ringo control."]
    #[inline]
    pub fn e_nd1(&self) -> E_ND1R {
        E_ND1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - First NOR2-based ringo control."]
    #[inline]
    pub fn e_nr0(&self) -> E_NR0R {
        E_NR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Second NOR2-based ringo control."]
    #[inline]
    pub fn e_nr1(&self) -> E_NR1R {
        E_NR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - First Inverter-based ringo control."]
    #[inline]
    pub fn e_iv0(&self) -> E_IV0R {
        E_IV0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Second Inverter-based ringo control."]
    #[inline]
    pub fn e_iv1(&self) -> E_IV1R {
        E_IV1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - First PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline]
    pub fn e_pn0(&self) -> E_PN0R {
        E_PN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Second PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline]
    pub fn e_pn1(&self) -> E_PN1R {
        E_PN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
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
    pub fn sl(&mut self) -> _SLW {
        _SLW { w: self }
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline]
    pub fn fs(&mut self) -> _FSW {
        _FSW { w: self }
    }
    #[doc = "Bits 2:3 - PN-Ringos (P-Transistor and N-Transistor processing) control."]
    #[inline]
    pub fn swn_swp(&mut self) -> _SWN_SWPW {
        _SWN_SWPW { w: self }
    }
    #[doc = "Bit 4 - Ringo module Power control."]
    #[inline]
    pub fn pd(&mut self) -> _PDW {
        _PDW { w: self }
    }
    #[doc = "Bit 5 - First NAND2-based ringo control."]
    #[inline]
    pub fn e_nd0(&mut self) -> _E_ND0W {
        _E_ND0W { w: self }
    }
    #[doc = "Bit 6 - Second NAND2-based ringo control."]
    #[inline]
    pub fn e_nd1(&mut self) -> _E_ND1W {
        _E_ND1W { w: self }
    }
    #[doc = "Bit 7 - First NOR2-based ringo control."]
    #[inline]
    pub fn e_nr0(&mut self) -> _E_NR0W {
        _E_NR0W { w: self }
    }
    #[doc = "Bit 8 - Second NOR2-based ringo control."]
    #[inline]
    pub fn e_nr1(&mut self) -> _E_NR1W {
        _E_NR1W { w: self }
    }
    #[doc = "Bit 9 - First Inverter-based ringo control."]
    #[inline]
    pub fn e_iv0(&mut self) -> _E_IV0W {
        _E_IV0W { w: self }
    }
    #[doc = "Bit 10 - Second Inverter-based ringo control."]
    #[inline]
    pub fn e_iv1(&mut self) -> _E_IV1W {
        _E_IV1W { w: self }
    }
    #[doc = "Bit 11 - First PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline]
    pub fn e_pn0(&mut self) -> _E_PN0W {
        _E_PN0W { w: self }
    }
    #[doc = "Bit 12 - Second PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline]
    pub fn e_pn1(&mut self) -> _E_PN1W {
        _E_PN1W { w: self }
    }
    #[doc = "Bits 16:19 - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline]
    pub fn divisor(&mut self) -> _DIVISORW {
        _DIVISORW { w: self }
    }
}
