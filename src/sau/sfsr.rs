#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SFSR {
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
#[doc = "Possible values of the field `INVEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVEPR {
    #[doc = "Error has not occurred."]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl INVEPR {
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
            INVEPR::NO_ERROR => false,
            INVEPR::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVEPR {
        match value {
            false => INVEPR::NO_ERROR,
            true => INVEPR::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == INVEPR::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == INVEPR::ERROR
    }
}
#[doc = "Possible values of the field `INVIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVISR {
    #[doc = "Error has not occurred."]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl INVISR {
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
            INVISR::NO_ERROR => false,
            INVISR::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVISR {
        match value {
            false => INVISR::NO_ERROR,
            true => INVISR::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == INVISR::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == INVISR::ERROR
    }
}
#[doc = "Possible values of the field `INVER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVERR {
    #[doc = "Error has not occurred."]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl INVERR {
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
            INVERR::NO_ERROR => false,
            INVERR::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVERR {
        match value {
            false => INVERR::NO_ERROR,
            true => INVERR::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == INVERR::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == INVERR::ERROR
    }
}
#[doc = "Possible values of the field `AUVIOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUVIOLR {
    #[doc = "Error has not occurred."]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl AUVIOLR {
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
            AUVIOLR::NO_ERROR => false,
            AUVIOLR::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUVIOLR {
        match value {
            false => AUVIOLR::NO_ERROR,
            true => AUVIOLR::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == AUVIOLR::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == AUVIOLR::ERROR
    }
}
#[doc = "Possible values of the field `INVTRAN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVTRANR {
    #[doc = "Error has not occurred."]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl INVTRANR {
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
            INVTRANR::NO_ERROR => false,
            INVTRANR::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVTRANR {
        match value {
            false => INVTRANR::NO_ERROR,
            true => INVTRANR::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == INVTRANR::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == INVTRANR::ERROR
    }
}
#[doc = "Possible values of the field `LSPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPERRR {
    #[doc = "Error has not occurred."]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl LSPERRR {
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
            LSPERRR::NO_ERROR => false,
            LSPERRR::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSPERRR {
        match value {
            false => LSPERRR::NO_ERROR,
            true => LSPERRR::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == LSPERRR::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == LSPERRR::ERROR
    }
}
#[doc = "Possible values of the field `SFARVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFARVALIDR {
    #[doc = "SFAR content not valid."]
    NOT_VALID,
    #[doc = "SFAR content valid."]
    VALID,
}
impl SFARVALIDR {
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
            SFARVALIDR::NOT_VALID => false,
            SFARVALIDR::VALID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SFARVALIDR {
        match value {
            false => SFARVALIDR::NOT_VALID,
            true => SFARVALIDR::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline]
    pub fn is_not_valid(&self) -> bool {
        *self == SFARVALIDR::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == SFARVALIDR::VALID
    }
}
#[doc = "Possible values of the field `LSERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERRR {
    #[doc = "Error has not occurred"]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl LSERRR {
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
            LSERRR::NO_ERROR => false,
            LSERRR::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSERRR {
        match value {
            false => LSERRR::NO_ERROR,
            true => LSERRR::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == LSERRR::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == LSERRR::ERROR
    }
}
#[doc = "Values that can be written to the field `INVEP`"]
pub enum INVEPW {
    #[doc = "Error has not occurred."]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl INVEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVEPW::NO_ERROR => false,
            INVEPW::ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVEPW<'a> {
    w: &'a mut W,
}
impl<'a> _INVEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error has not occurred."]
    #[inline]
    pub fn no_error(self) -> &'a mut W {
        self.variant(INVEPW::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(INVEPW::ERROR)
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
#[doc = "Values that can be written to the field `INVIS`"]
pub enum INVISW {
    #[doc = "Error has not occurred."]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl INVISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVISW::NO_ERROR => false,
            INVISW::ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVISW<'a> {
    w: &'a mut W,
}
impl<'a> _INVISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error has not occurred."]
    #[inline]
    pub fn no_error(self) -> &'a mut W {
        self.variant(INVISW::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(INVISW::ERROR)
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
#[doc = "Values that can be written to the field `INVER`"]
pub enum INVERW {
    #[doc = "Error has not occurred."]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl INVERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVERW::NO_ERROR => false,
            INVERW::ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVERW<'a> {
    w: &'a mut W,
}
impl<'a> _INVERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error has not occurred."]
    #[inline]
    pub fn no_error(self) -> &'a mut W {
        self.variant(INVERW::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(INVERW::ERROR)
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
#[doc = "Values that can be written to the field `AUVIOL`"]
pub enum AUVIOLW {
    #[doc = "Error has not occurred."]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl AUVIOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUVIOLW::NO_ERROR => false,
            AUVIOLW::ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUVIOLW<'a> {
    w: &'a mut W,
}
impl<'a> _AUVIOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUVIOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error has not occurred."]
    #[inline]
    pub fn no_error(self) -> &'a mut W {
        self.variant(AUVIOLW::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(AUVIOLW::ERROR)
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
#[doc = "Values that can be written to the field `INVTRAN`"]
pub enum INVTRANW {
    #[doc = "Error has not occurred."]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl INVTRANW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVTRANW::NO_ERROR => false,
            INVTRANW::ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVTRANW<'a> {
    w: &'a mut W,
}
impl<'a> _INVTRANW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVTRANW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error has not occurred."]
    #[inline]
    pub fn no_error(self) -> &'a mut W {
        self.variant(INVTRANW::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(INVTRANW::ERROR)
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
#[doc = "Values that can be written to the field `LSPERR`"]
pub enum LSPERRW {
    #[doc = "Error has not occurred."]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl LSPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSPERRW::NO_ERROR => false,
            LSPERRW::ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSPERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error has not occurred."]
    #[inline]
    pub fn no_error(self) -> &'a mut W {
        self.variant(LSPERRW::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(LSPERRW::ERROR)
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
#[doc = "Values that can be written to the field `SFARVALID`"]
pub enum SFARVALIDW {
    #[doc = "SFAR content not valid."]
    NOT_VALID,
    #[doc = "SFAR content valid."]
    VALID,
}
impl SFARVALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SFARVALIDW::NOT_VALID => false,
            SFARVALIDW::VALID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SFARVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _SFARVALIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SFARVALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SFAR content not valid."]
    #[inline]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(SFARVALIDW::NOT_VALID)
    }
    #[doc = "SFAR content valid."]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(SFARVALIDW::VALID)
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
#[doc = "Values that can be written to the field `LSERR`"]
pub enum LSERRW {
    #[doc = "Error has not occurred"]
    NO_ERROR,
    #[doc = "Error has occurred."]
    ERROR,
}
impl LSERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSERRW::NO_ERROR => false,
            LSERRW::ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSERRW<'a> {
    w: &'a mut W,
}
impl<'a> _LSERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error has not occurred"]
    #[inline]
    pub fn no_error(self) -> &'a mut W {
        self.variant(LSERRW::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(LSERRW::ERROR)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Invalid entry point."]
    #[inline]
    pub fn invep(&self) -> INVEPR {
        INVEPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Invalid integrity signature flag."]
    #[inline]
    pub fn invis(&self) -> INVISR {
        INVISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Invalid exception return flag."]
    #[inline]
    pub fn inver(&self) -> INVERR {
        INVERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Attribution unit violation flag."]
    #[inline]
    pub fn auviol(&self) -> AUVIOLR {
        AUVIOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Invalid transition flag."]
    #[inline]
    pub fn invtran(&self) -> INVTRANR {
        INVTRANR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Lazy state preservation error flag."]
    #[inline]
    pub fn lsperr(&self) -> LSPERRR {
        LSPERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Secure fault address valid."]
    #[inline]
    pub fn sfarvalid(&self) -> SFARVALIDR {
        SFARVALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Lazy state error flag."]
    #[inline]
    pub fn lserr(&self) -> LSERRR {
        LSERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Invalid entry point."]
    #[inline]
    pub fn invep(&mut self) -> _INVEPW {
        _INVEPW { w: self }
    }
    #[doc = "Bit 1 - Invalid integrity signature flag."]
    #[inline]
    pub fn invis(&mut self) -> _INVISW {
        _INVISW { w: self }
    }
    #[doc = "Bit 2 - Invalid exception return flag."]
    #[inline]
    pub fn inver(&mut self) -> _INVERW {
        _INVERW { w: self }
    }
    #[doc = "Bit 3 - Attribution unit violation flag."]
    #[inline]
    pub fn auviol(&mut self) -> _AUVIOLW {
        _AUVIOLW { w: self }
    }
    #[doc = "Bit 4 - Invalid transition flag."]
    #[inline]
    pub fn invtran(&mut self) -> _INVTRANW {
        _INVTRANW { w: self }
    }
    #[doc = "Bit 5 - Lazy state preservation error flag."]
    #[inline]
    pub fn lsperr(&mut self) -> _LSPERRW {
        _LSPERRW { w: self }
    }
    #[doc = "Bit 6 - Secure fault address valid."]
    #[inline]
    pub fn sfarvalid(&mut self) -> _SFARVALIDW {
        _SFARVALIDW { w: self }
    }
    #[doc = "Bit 7 - Lazy state error flag."]
    #[inline]
    pub fn lserr(&mut self) -> _LSERRW {
        _LSERRW { w: self }
    }
}
