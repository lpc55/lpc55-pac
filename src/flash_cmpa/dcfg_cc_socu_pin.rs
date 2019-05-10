#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCFG_CC_SOCU_PIN {
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
#[doc = "Possible values of the field `NIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NIDENR {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl NIDENR {
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
            NIDENR::VALUE_0 => false,
            NIDENR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NIDENR {
        match value {
            false => NIDENR::VALUE_0,
            true => NIDENR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == NIDENR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == NIDENR::VALUE_1
    }
}
#[doc = "Possible values of the field `DBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGENR {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl DBGENR {
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
            DBGENR::VALUE_0 => false,
            DBGENR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGENR {
        match value {
            false => DBGENR::VALUE_0,
            true => DBGENR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == DBGENR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == DBGENR::VALUE_1
    }
}
#[doc = "Possible values of the field `SPNIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPNIDENR {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl SPNIDENR {
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
            SPNIDENR::VALUE_0 => false,
            SPNIDENR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPNIDENR {
        match value {
            false => SPNIDENR::VALUE_0,
            true => SPNIDENR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == SPNIDENR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == SPNIDENR::VALUE_1
    }
}
#[doc = "Possible values of the field `SPIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIDENR {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl SPIDENR {
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
            SPIDENR::VALUE_0 => false,
            SPIDENR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPIDENR {
        match value {
            false => SPIDENR::VALUE_0,
            true => SPIDENR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == SPIDENR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == SPIDENR::VALUE_1
    }
}
#[doc = "Possible values of the field `TAPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAPENR {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl TAPENR {
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
            TAPENR::VALUE_0 => false,
            TAPENR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAPENR {
        match value {
            false => TAPENR::VALUE_0,
            true => TAPENR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == TAPENR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == TAPENR::VALUE_1
    }
}
#[doc = "Possible values of the field `MCM33_DBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCM33_DBGENR {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl MCM33_DBGENR {
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
            MCM33_DBGENR::VALUE_0 => false,
            MCM33_DBGENR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCM33_DBGENR {
        match value {
            false => MCM33_DBGENR::VALUE_0,
            true => MCM33_DBGENR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == MCM33_DBGENR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == MCM33_DBGENR::VALUE_1
    }
}
#[doc = "Possible values of the field `ISP_CMD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISP_CMD_ENR {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl ISP_CMD_ENR {
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
            ISP_CMD_ENR::VALUE_0 => false,
            ISP_CMD_ENR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISP_CMD_ENR {
        match value {
            false => ISP_CMD_ENR::VALUE_0,
            true => ISP_CMD_ENR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == ISP_CMD_ENR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == ISP_CMD_ENR::VALUE_1
    }
}
#[doc = "Possible values of the field `FA_CMD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FA_CMD_ENR {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl FA_CMD_ENR {
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
            FA_CMD_ENR::VALUE_0 => false,
            FA_CMD_ENR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FA_CMD_ENR {
        match value {
            false => FA_CMD_ENR::VALUE_0,
            true => FA_CMD_ENR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == FA_CMD_ENR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == FA_CMD_ENR::VALUE_1
    }
}
#[doc = "Possible values of the field `ME_CMD_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ME_CMD_ENR {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl ME_CMD_ENR {
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
            ME_CMD_ENR::VALUE_0 => false,
            ME_CMD_ENR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ME_CMD_ENR {
        match value {
            false => ME_CMD_ENR::VALUE_0,
            true => ME_CMD_ENR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == ME_CMD_ENR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == ME_CMD_ENR::VALUE_1
    }
}
#[doc = "Possible values of the field `MCM33_NIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCM33_NIDENR {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl MCM33_NIDENR {
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
            MCM33_NIDENR::VALUE_0 => false,
            MCM33_NIDENR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCM33_NIDENR {
        match value {
            false => MCM33_NIDENR::VALUE_0,
            true => MCM33_NIDENR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == MCM33_NIDENR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == MCM33_NIDENR::VALUE_1
    }
}
#[doc = r" Value of the field"]
pub struct UUID_CHECKR {
    bits: bool,
}
impl UUID_CHECKR {
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
pub struct INVERSE_VALUER {
    bits: u16,
}
impl INVERSE_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `NIDEN`"]
pub enum NIDENW {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl NIDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NIDENW::VALUE_0 => false,
            NIDENW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NIDENW<'a> {
    w: &'a mut W,
}
impl<'a> _NIDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NIDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(NIDENW::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(NIDENW::VALUE_1)
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
#[doc = "Values that can be written to the field `DBGEN`"]
pub enum DBGENW {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl DBGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGENW::VALUE_0 => false,
            DBGENW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(DBGENW::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(DBGENW::VALUE_1)
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
#[doc = "Values that can be written to the field `SPNIDEN`"]
pub enum SPNIDENW {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl SPNIDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPNIDENW::VALUE_0 => false,
            SPNIDENW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPNIDENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPNIDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPNIDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SPNIDENW::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(SPNIDENW::VALUE_1)
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
#[doc = "Values that can be written to the field `SPIDEN`"]
pub enum SPIDENW {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl SPIDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPIDENW::VALUE_0 => false,
            SPIDENW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPIDENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPIDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SPIDENW::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(SPIDENW::VALUE_1)
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
#[doc = "Values that can be written to the field `TAPEN`"]
pub enum TAPENW {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl TAPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAPENW::VALUE_0 => false,
            TAPENW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TAPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(TAPENW::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(TAPENW::VALUE_1)
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
#[doc = "Values that can be written to the field `MCM33_DBGEN`"]
pub enum MCM33_DBGENW {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl MCM33_DBGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCM33_DBGENW::VALUE_0 => false,
            MCM33_DBGENW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCM33_DBGENW<'a> {
    w: &'a mut W,
}
impl<'a> _MCM33_DBGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCM33_DBGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(MCM33_DBGENW::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(MCM33_DBGENW::VALUE_1)
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
#[doc = "Values that can be written to the field `ISP_CMD_EN`"]
pub enum ISP_CMD_ENW {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl ISP_CMD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISP_CMD_ENW::VALUE_0 => false,
            ISP_CMD_ENW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISP_CMD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ISP_CMD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISP_CMD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(ISP_CMD_ENW::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(ISP_CMD_ENW::VALUE_1)
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
#[doc = "Values that can be written to the field `FA_CMD_EN`"]
pub enum FA_CMD_ENW {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl FA_CMD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FA_CMD_ENW::VALUE_0 => false,
            FA_CMD_ENW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FA_CMD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FA_CMD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FA_CMD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(FA_CMD_ENW::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(FA_CMD_ENW::VALUE_1)
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
#[doc = "Values that can be written to the field `ME_CMD_EN`"]
pub enum ME_CMD_ENW {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl ME_CMD_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ME_CMD_ENW::VALUE_0 => false,
            ME_CMD_ENW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ME_CMD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ME_CMD_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ME_CMD_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(ME_CMD_ENW::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(ME_CMD_ENW::VALUE_1)
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
#[doc = "Values that can be written to the field `MCM33_NIDEN`"]
pub enum MCM33_NIDENW {
    #[doc = "Use DAP to enable"]
    VALUE_0,
    #[doc = "Fixed state"]
    VALUE_1,
}
impl MCM33_NIDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCM33_NIDENW::VALUE_0 => false,
            MCM33_NIDENW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCM33_NIDENW<'a> {
    w: &'a mut W,
}
impl<'a> _MCM33_NIDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCM33_NIDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use DAP to enable"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(MCM33_NIDENW::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(MCM33_NIDENW::VALUE_1)
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
#[doc = r" Proxy"]
pub struct _UUID_CHECKW<'a> {
    w: &'a mut W,
}
impl<'a> _UUID_CHECKW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INVERSE_VALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _INVERSE_VALUEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bit 0 - Non Secure non-invasive debug enable"]
    #[inline]
    pub fn niden(&self) -> NIDENR {
        NIDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Non Secure debug enable"]
    #[inline]
    pub fn dbgen(&self) -> DBGENR {
        DBGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Secure non-invasive debug enable"]
    #[inline]
    pub fn spniden(&self) -> SPNIDENR {
        SPNIDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Secure invasive debug enable"]
    #[inline]
    pub fn spiden(&self) -> SPIDENR {
        SPIDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - JTAG TAP enable"]
    #[inline]
    pub fn tapen(&self) -> TAPENR {
        TAPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Micro CM33 invasive debug enable"]
    #[inline]
    pub fn mcm33_dbgen(&self) -> MCM33_DBGENR {
        MCM33_DBGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - ISP Boot Command enable"]
    #[inline]
    pub fn isp_cmd_en(&self) -> ISP_CMD_ENR {
        ISP_CMD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - FA Command enable"]
    #[inline]
    pub fn fa_cmd_en(&self) -> FA_CMD_ENR {
        FA_CMD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Flash Mass Erase Command enable"]
    #[inline]
    pub fn me_cmd_en(&self) -> ME_CMD_ENR {
        ME_CMD_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Micro CM33 non-invasive debug enable"]
    #[inline]
    pub fn mcm33_niden(&self) -> MCM33_NIDENR {
        MCM33_NIDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enforce UUID match during Debug authentication."]
    #[inline]
    pub fn uuid_check(&self) -> UUID_CHECKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UUID_CHECKR { bits }
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline]
    pub fn inverse_value(&self) -> INVERSE_VALUER {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        INVERSE_VALUER { bits }
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
    #[doc = "Bit 0 - Non Secure non-invasive debug enable"]
    #[inline]
    pub fn niden(&mut self) -> _NIDENW {
        _NIDENW { w: self }
    }
    #[doc = "Bit 1 - Non Secure debug enable"]
    #[inline]
    pub fn dbgen(&mut self) -> _DBGENW {
        _DBGENW { w: self }
    }
    #[doc = "Bit 2 - Secure non-invasive debug enable"]
    #[inline]
    pub fn spniden(&mut self) -> _SPNIDENW {
        _SPNIDENW { w: self }
    }
    #[doc = "Bit 3 - Secure invasive debug enable"]
    #[inline]
    pub fn spiden(&mut self) -> _SPIDENW {
        _SPIDENW { w: self }
    }
    #[doc = "Bit 4 - JTAG TAP enable"]
    #[inline]
    pub fn tapen(&mut self) -> _TAPENW {
        _TAPENW { w: self }
    }
    #[doc = "Bit 5 - Micro CM33 invasive debug enable"]
    #[inline]
    pub fn mcm33_dbgen(&mut self) -> _MCM33_DBGENW {
        _MCM33_DBGENW { w: self }
    }
    #[doc = "Bit 6 - ISP Boot Command enable"]
    #[inline]
    pub fn isp_cmd_en(&mut self) -> _ISP_CMD_ENW {
        _ISP_CMD_ENW { w: self }
    }
    #[doc = "Bit 7 - FA Command enable"]
    #[inline]
    pub fn fa_cmd_en(&mut self) -> _FA_CMD_ENW {
        _FA_CMD_ENW { w: self }
    }
    #[doc = "Bit 8 - Flash Mass Erase Command enable"]
    #[inline]
    pub fn me_cmd_en(&mut self) -> _ME_CMD_ENW {
        _ME_CMD_ENW { w: self }
    }
    #[doc = "Bit 9 - Micro CM33 non-invasive debug enable"]
    #[inline]
    pub fn mcm33_niden(&mut self) -> _MCM33_NIDENW {
        _MCM33_NIDENW { w: self }
    }
    #[doc = "Bit 15 - Enforce UUID match during Debug authentication."]
    #[inline]
    pub fn uuid_check(&mut self) -> _UUID_CHECKW {
        _UUID_CHECKW { w: self }
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline]
    pub fn inverse_value(&mut self) -> _INVERSE_VALUEW {
        _INVERSE_VALUEW { w: self }
    }
}
