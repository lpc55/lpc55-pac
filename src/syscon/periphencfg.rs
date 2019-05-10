#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PERIPHENCFG {
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
#[doc = "Possible values of the field `SCTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCTENR {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl SCTENR {
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
            SCTENR::DISABLE => false,
            SCTENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCTENR {
        match value {
            false => SCTENR::DISABLE,
            true => SCTENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SCTENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SCTENR::ENABLE
    }
}
#[doc = "Possible values of the field `ADCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCENR {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl ADCENR {
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
            ADCENR::DISABLE => false,
            ADCENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCENR {
        match value {
            false => ADCENR::DISABLE,
            true => ADCENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADCENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADCENR::ENABLE
    }
}
#[doc = "Possible values of the field `USB0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0ENR {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl USB0ENR {
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
            USB0ENR::DISABLE => false,
            USB0ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0ENR {
        match value {
            false => USB0ENR::DISABLE,
            true => USB0ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USB0ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USB0ENR::ENABLE
    }
}
#[doc = "Possible values of the field `PUFFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUFFENR {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl PUFFENR {
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
            PUFFENR::DISABLE => false,
            PUFFENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PUFFENR {
        match value {
            false => PUFFENR::DISABLE,
            true => PUFFENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PUFFENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PUFFENR::ENABLE
    }
}
#[doc = "Possible values of the field `USB1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1ENR {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl USB1ENR {
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
            USB1ENR::DISABLE => false,
            USB1ENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1ENR {
        match value {
            false => USB1ENR::DISABLE,
            true => USB1ENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == USB1ENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == USB1ENR::ENABLE
    }
}
#[doc = "Possible values of the field `SDIOEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOENR {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl SDIOENR {
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
            SDIOENR::DISABLE => false,
            SDIOENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDIOENR {
        match value {
            false => SDIOENR::DISABLE,
            true => SDIOENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SDIOENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SDIOENR::ENABLE
    }
}
#[doc = "Possible values of the field `HASHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASHENR {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl HASHENR {
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
            HASHENR::DISABLE => false,
            HASHENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HASHENR {
        match value {
            false => HASHENR::DISABLE,
            true => HASHENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == HASHENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == HASHENR::ENABLE
    }
}
#[doc = "Possible values of the field `PRINCEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRINCEENR {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl PRINCEENR {
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
            PRINCEENR::DISABLE => false,
            PRINCEENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRINCEENR {
        match value {
            false => PRINCEENR::DISABLE,
            true => PRINCEENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == PRINCEENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == PRINCEENR::ENABLE
    }
}
#[doc = "Values that can be written to the field `SCTEN`"]
pub enum SCTENW {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl SCTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCTENW::DISABLE => false,
            SCTENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCTENW::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCTENW::ENABLE)
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
#[doc = "Values that can be written to the field `ADCEN`"]
pub enum ADCENW {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl ADCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCENW::DISABLE => false,
            ADCENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADCENW::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADCENW::ENABLE)
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
#[doc = "Values that can be written to the field `USB0EN`"]
pub enum USB0ENW {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl USB0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0ENW::DISABLE => false,
            USB0ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0ENW::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0ENW::ENABLE)
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
#[doc = "Values that can be written to the field `PUFFEN`"]
pub enum PUFFENW {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl PUFFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PUFFENW::DISABLE => false,
            PUFFENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PUFFENW<'a> {
    w: &'a mut W,
}
impl<'a> _PUFFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUFFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PUFFENW::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PUFFENW::ENABLE)
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
#[doc = "Values that can be written to the field `USB1EN`"]
pub enum USB1ENW {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl USB1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1ENW::DISABLE => false,
            USB1ENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB1ENW::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB1ENW::ENABLE)
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
#[doc = "Values that can be written to the field `SDIOEN`"]
pub enum SDIOENW {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl SDIOENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDIOENW::DISABLE => false,
            SDIOENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDIOENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIOENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDIOENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDIOENW::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDIOENW::ENABLE)
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
#[doc = "Values that can be written to the field `HASHEN`"]
pub enum HASHENW {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl HASHENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HASHENW::DISABLE => false,
            HASHENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HASHENW<'a> {
    w: &'a mut W,
}
impl<'a> _HASHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HASHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(HASHENW::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(HASHENW::ENABLE)
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
#[doc = "Values that can be written to the field `PRINCEEN`"]
pub enum PRINCEENW {
    #[doc = "peripheral is disable."]
    DISABLE,
    #[doc = "peripheral is enable."]
    ENABLE,
}
impl PRINCEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRINCEENW::DISABLE => false,
            PRINCEENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRINCEENW<'a> {
    w: &'a mut W,
}
impl<'a> _PRINCEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRINCEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "peripheral is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRINCEENW::DISABLE)
    }
    #[doc = "peripheral is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRINCEENW::ENABLE)
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bit 0 - SCT enable."]
    #[inline]
    pub fn scten(&self) -> SCTENR {
        SCTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - ADC enable."]
    #[inline]
    pub fn adcen(&self) -> ADCENR {
        ADCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - USB0 enable."]
    #[inline]
    pub fn usb0en(&self) -> USB0ENR {
        USB0ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Puff enable."]
    #[inline]
    pub fn puffen(&self) -> PUFFENR {
        PUFFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - USB1 enable."]
    #[inline]
    pub fn usb1en(&self) -> USB1ENR {
        USB1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - SDIO enable."]
    #[inline]
    pub fn sdioen(&self) -> SDIOENR {
        SDIOENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - HASH enable."]
    #[inline]
    pub fn hashen(&self) -> HASHENR {
        HASHENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - PRINCE enable."]
    #[inline]
    pub fn princeen(&self) -> PRINCEENR {
        PRINCEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 23623 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SCT enable."]
    #[inline]
    pub fn scten(&mut self) -> _SCTENW {
        _SCTENW { w: self }
    }
    #[doc = "Bit 1 - ADC enable."]
    #[inline]
    pub fn adcen(&mut self) -> _ADCENW {
        _ADCENW { w: self }
    }
    #[doc = "Bit 2 - USB0 enable."]
    #[inline]
    pub fn usb0en(&mut self) -> _USB0ENW {
        _USB0ENW { w: self }
    }
    #[doc = "Bit 6 - Puff enable."]
    #[inline]
    pub fn puffen(&mut self) -> _PUFFENW {
        _PUFFENW { w: self }
    }
    #[doc = "Bit 10 - USB1 enable."]
    #[inline]
    pub fn usb1en(&mut self) -> _USB1ENW {
        _USB1ENW { w: self }
    }
    #[doc = "Bit 11 - SDIO enable."]
    #[inline]
    pub fn sdioen(&mut self) -> _SDIOENW {
        _SDIOENW { w: self }
    }
    #[doc = "Bit 12 - HASH enable."]
    #[inline]
    pub fn hashen(&mut self) -> _HASHENW {
        _HASHENW { w: self }
    }
    #[doc = "Bit 14 - PRINCE enable."]
    #[inline]
    pub fn princeen(&mut self) -> _PRINCEENW {
        _PRINCEENW { w: self }
    }
}
