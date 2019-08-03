#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MISC_CTRL_DP_REG {
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
#[doc = "Possible values of the field `WRITE_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_LOCKR {
    #[doc = "Restricted mode."]
    RESTRICTED,
    #[doc = "Secure control registers can be written."]
    ACCESSIBLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WRITE_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WRITE_LOCKR::RESTRICTED => 1,
            WRITE_LOCKR::ACCESSIBLE => 2,
            WRITE_LOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WRITE_LOCKR {
        match value {
            1 => WRITE_LOCKR::RESTRICTED,
            2 => WRITE_LOCKR::ACCESSIBLE,
            i => WRITE_LOCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESTRICTED`"]
    #[inline]
    pub fn is_restricted(&self) -> bool {
        *self == WRITE_LOCKR::RESTRICTED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline]
    pub fn is_accessible(&self) -> bool {
        *self == WRITE_LOCKR::ACCESSIBLE
    }
}
#[doc = "Possible values of the field `ENABLE_SECURE_CHECKING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_SECURE_CHECKINGR {
    #[doc = "Restricted mode."]
    ENABLE,
    #[doc = "Disable check."]
    DISABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENABLE_SECURE_CHECKINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENABLE_SECURE_CHECKINGR::ENABLE => 1,
            ENABLE_SECURE_CHECKINGR::DISABLE => 2,
            ENABLE_SECURE_CHECKINGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENABLE_SECURE_CHECKINGR {
        match value {
            1 => ENABLE_SECURE_CHECKINGR::ENABLE,
            2 => ENABLE_SECURE_CHECKINGR::DISABLE,
            i => ENABLE_SECURE_CHECKINGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_SECURE_CHECKINGR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_SECURE_CHECKINGR::DISABLE
    }
}
#[doc = "Possible values of the field `ENABLE_S_PRIV_CHECK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_S_PRIV_CHECKR {
    #[doc = "Restricted mode."]
    ENABLE,
    #[doc = "Disable check."]
    DISABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENABLE_S_PRIV_CHECKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENABLE_S_PRIV_CHECKR::ENABLE => 1,
            ENABLE_S_PRIV_CHECKR::DISABLE => 2,
            ENABLE_S_PRIV_CHECKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENABLE_S_PRIV_CHECKR {
        match value {
            1 => ENABLE_S_PRIV_CHECKR::ENABLE,
            2 => ENABLE_S_PRIV_CHECKR::DISABLE,
            i => ENABLE_S_PRIV_CHECKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_S_PRIV_CHECKR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_S_PRIV_CHECKR::DISABLE
    }
}
#[doc = "Possible values of the field `ENABLE_NS_PRIV_CHECK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_NS_PRIV_CHECKR {
    #[doc = "Restricted mode."]
    ENABLE,
    #[doc = "Disable check."]
    DISABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENABLE_NS_PRIV_CHECKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENABLE_NS_PRIV_CHECKR::ENABLE => 1,
            ENABLE_NS_PRIV_CHECKR::DISABLE => 2,
            ENABLE_NS_PRIV_CHECKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENABLE_NS_PRIV_CHECKR {
        match value {
            1 => ENABLE_NS_PRIV_CHECKR::ENABLE,
            2 => ENABLE_NS_PRIV_CHECKR::DISABLE,
            i => ENABLE_NS_PRIV_CHECKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_NS_PRIV_CHECKR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_NS_PRIV_CHECKR::DISABLE
    }
}
#[doc = "Possible values of the field `DISABLE_VIOLATION_ABORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_VIOLATION_ABORTR {
    #[doc = "Disable abort fort secure checker."]
    DISABLE,
    #[doc = "Enable abort fort secure checker."]
    ENABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DISABLE_VIOLATION_ABORTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DISABLE_VIOLATION_ABORTR::DISABLE => 1,
            DISABLE_VIOLATION_ABORTR::ENABLE => 2,
            DISABLE_VIOLATION_ABORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DISABLE_VIOLATION_ABORTR {
        match value {
            1 => DISABLE_VIOLATION_ABORTR::DISABLE,
            2 => DISABLE_VIOLATION_ABORTR::ENABLE,
            i => DISABLE_VIOLATION_ABORTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DISABLE_VIOLATION_ABORTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DISABLE_VIOLATION_ABORTR::ENABLE
    }
}
#[doc = "Possible values of the field `DISABLE_SIMPLE_MASTER_STRICT_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_SIMPLE_MASTER_STRICT_MODER {
    #[doc = "Simple master in tier mode."]
    TIER_MODE,
    #[doc = "Simple master in strict mode."]
    STRICT_MODE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DISABLE_SIMPLE_MASTER_STRICT_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DISABLE_SIMPLE_MASTER_STRICT_MODER::TIER_MODE => 1,
            DISABLE_SIMPLE_MASTER_STRICT_MODER::STRICT_MODE => 2,
            DISABLE_SIMPLE_MASTER_STRICT_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DISABLE_SIMPLE_MASTER_STRICT_MODER {
        match value {
            1 => DISABLE_SIMPLE_MASTER_STRICT_MODER::TIER_MODE,
            2 => DISABLE_SIMPLE_MASTER_STRICT_MODER::STRICT_MODE,
            i => DISABLE_SIMPLE_MASTER_STRICT_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIER_MODE`"]
    #[inline]
    pub fn is_tier_mode(&self) -> bool {
        *self == DISABLE_SIMPLE_MASTER_STRICT_MODER::TIER_MODE
    }
    #[doc = "Checks if the value of the field is `STRICT_MODE`"]
    #[inline]
    pub fn is_strict_mode(&self) -> bool {
        *self == DISABLE_SIMPLE_MASTER_STRICT_MODER::STRICT_MODE
    }
}
#[doc = "Possible values of the field `DISABLE_SMART_MASTER_STRICT_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_SMART_MASTER_STRICT_MODER {
    #[doc = "Smart master in tier mode."]
    TIER_MODE,
    #[doc = "Smart master in strict mode."]
    STRICT_MODE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DISABLE_SMART_MASTER_STRICT_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DISABLE_SMART_MASTER_STRICT_MODER::TIER_MODE => 1,
            DISABLE_SMART_MASTER_STRICT_MODER::STRICT_MODE => 2,
            DISABLE_SMART_MASTER_STRICT_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DISABLE_SMART_MASTER_STRICT_MODER {
        match value {
            1 => DISABLE_SMART_MASTER_STRICT_MODER::TIER_MODE,
            2 => DISABLE_SMART_MASTER_STRICT_MODER::STRICT_MODE,
            i => DISABLE_SMART_MASTER_STRICT_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIER_MODE`"]
    #[inline]
    pub fn is_tier_mode(&self) -> bool {
        *self == DISABLE_SMART_MASTER_STRICT_MODER::TIER_MODE
    }
    #[doc = "Checks if the value of the field is `STRICT_MODE`"]
    #[inline]
    pub fn is_strict_mode(&self) -> bool {
        *self == DISABLE_SMART_MASTER_STRICT_MODER::STRICT_MODE
    }
}
#[doc = "Possible values of the field `IDAU_ALL_NS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDAU_ALL_NSR {
    #[doc = "IDAU is disable."]
    DISABLE,
    #[doc = "IDAU is enabled."]
    ENABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IDAU_ALL_NSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDAU_ALL_NSR::DISABLE => 1,
            IDAU_ALL_NSR::ENABLE => 2,
            IDAU_ALL_NSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDAU_ALL_NSR {
        match value {
            1 => IDAU_ALL_NSR::DISABLE,
            2 => IDAU_ALL_NSR::ENABLE,
            i => IDAU_ALL_NSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == IDAU_ALL_NSR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == IDAU_ALL_NSR::ENABLE
    }
}
#[doc = "Values that can be written to the field `WRITE_LOCK`"]
pub enum WRITE_LOCKW {
    #[doc = "Restricted mode."]
    RESTRICTED,
    #[doc = "Secure control registers can be written."]
    ACCESSIBLE,
}
impl WRITE_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WRITE_LOCKW::RESTRICTED => 1,
            WRITE_LOCKW::ACCESSIBLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRITE_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITE_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRITE_LOCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn restricted(self) -> &'a mut W {
        self.variant(WRITE_LOCKW::RESTRICTED)
    }
    #[doc = "Secure control registers can be written."]
    #[inline]
    pub fn accessible(self) -> &'a mut W {
        self.variant(WRITE_LOCKW::ACCESSIBLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENABLE_SECURE_CHECKING`"]
pub enum ENABLE_SECURE_CHECKINGW {
    #[doc = "Restricted mode."]
    ENABLE,
    #[doc = "Disable check."]
    DISABLE,
}
impl ENABLE_SECURE_CHECKINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENABLE_SECURE_CHECKINGW::ENABLE => 1,
            ENABLE_SECURE_CHECKINGW::DISABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_SECURE_CHECKINGW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_SECURE_CHECKINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLE_SECURE_CHECKINGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_SECURE_CHECKINGW::ENABLE)
    }
    #[doc = "Disable check."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_SECURE_CHECKINGW::DISABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENABLE_S_PRIV_CHECK`"]
pub enum ENABLE_S_PRIV_CHECKW {
    #[doc = "Restricted mode."]
    ENABLE,
    #[doc = "Disable check."]
    DISABLE,
}
impl ENABLE_S_PRIV_CHECKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENABLE_S_PRIV_CHECKW::ENABLE => 1,
            ENABLE_S_PRIV_CHECKW::DISABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_S_PRIV_CHECKW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_S_PRIV_CHECKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLE_S_PRIV_CHECKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_S_PRIV_CHECKW::ENABLE)
    }
    #[doc = "Disable check."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_S_PRIV_CHECKW::DISABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ENABLE_NS_PRIV_CHECK`"]
pub enum ENABLE_NS_PRIV_CHECKW {
    #[doc = "Restricted mode."]
    ENABLE,
    #[doc = "Disable check."]
    DISABLE,
}
impl ENABLE_NS_PRIV_CHECKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENABLE_NS_PRIV_CHECKW::ENABLE => 1,
            ENABLE_NS_PRIV_CHECKW::DISABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLE_NS_PRIV_CHECKW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_NS_PRIV_CHECKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLE_NS_PRIV_CHECKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Restricted mode."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_NS_PRIV_CHECKW::ENABLE)
    }
    #[doc = "Disable check."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_NS_PRIV_CHECKW::DISABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DISABLE_VIOLATION_ABORT`"]
pub enum DISABLE_VIOLATION_ABORTW {
    #[doc = "Disable abort fort secure checker."]
    DISABLE,
    #[doc = "Enable abort fort secure checker."]
    ENABLE,
}
impl DISABLE_VIOLATION_ABORTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DISABLE_VIOLATION_ABORTW::DISABLE => 1,
            DISABLE_VIOLATION_ABORTW::ENABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_VIOLATION_ABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_VIOLATION_ABORTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISABLE_VIOLATION_ABORTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable abort fort secure checker."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DISABLE_VIOLATION_ABORTW::DISABLE)
    }
    #[doc = "Enable abort fort secure checker."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DISABLE_VIOLATION_ABORTW::ENABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DISABLE_SIMPLE_MASTER_STRICT_MODE`"]
pub enum DISABLE_SIMPLE_MASTER_STRICT_MODEW {
    #[doc = "Simple master in tier mode."]
    TIER_MODE,
    #[doc = "Simple master in strict mode."]
    STRICT_MODE,
}
impl DISABLE_SIMPLE_MASTER_STRICT_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DISABLE_SIMPLE_MASTER_STRICT_MODEW::TIER_MODE => 1,
            DISABLE_SIMPLE_MASTER_STRICT_MODEW::STRICT_MODE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_SIMPLE_MASTER_STRICT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_SIMPLE_MASTER_STRICT_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISABLE_SIMPLE_MASTER_STRICT_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Simple master in tier mode."]
    #[inline]
    pub fn tier_mode(self) -> &'a mut W {
        self.variant(DISABLE_SIMPLE_MASTER_STRICT_MODEW::TIER_MODE)
    }
    #[doc = "Simple master in strict mode."]
    #[inline]
    pub fn strict_mode(self) -> &'a mut W {
        self.variant(DISABLE_SIMPLE_MASTER_STRICT_MODEW::STRICT_MODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DISABLE_SMART_MASTER_STRICT_MODE`"]
pub enum DISABLE_SMART_MASTER_STRICT_MODEW {
    #[doc = "Smart master in tier mode."]
    TIER_MODE,
    #[doc = "Smart master in strict mode."]
    STRICT_MODE,
}
impl DISABLE_SMART_MASTER_STRICT_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DISABLE_SMART_MASTER_STRICT_MODEW::TIER_MODE => 1,
            DISABLE_SMART_MASTER_STRICT_MODEW::STRICT_MODE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_SMART_MASTER_STRICT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_SMART_MASTER_STRICT_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISABLE_SMART_MASTER_STRICT_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Smart master in tier mode."]
    #[inline]
    pub fn tier_mode(self) -> &'a mut W {
        self.variant(DISABLE_SMART_MASTER_STRICT_MODEW::TIER_MODE)
    }
    #[doc = "Smart master in strict mode."]
    #[inline]
    pub fn strict_mode(self) -> &'a mut W {
        self.variant(DISABLE_SMART_MASTER_STRICT_MODEW::STRICT_MODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDAU_ALL_NS`"]
pub enum IDAU_ALL_NSW {
    #[doc = "IDAU is disable."]
    DISABLE,
    #[doc = "IDAU is enabled."]
    ENABLE,
}
impl IDAU_ALL_NSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDAU_ALL_NSW::DISABLE => 1,
            IDAU_ALL_NSW::ENABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDAU_ALL_NSW<'a> {
    w: &'a mut W,
}
impl<'a> _IDAU_ALL_NSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDAU_ALL_NSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IDAU is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDAU_ALL_NSW::DISABLE)
    }
    #[doc = "IDAU is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(IDAU_ALL_NSW::ENABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - write lock."]
    #[inline]
    pub fn write_lock(&self) -> WRITE_LOCKR {
        WRITE_LOCKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - AHB bus matrix enable secure check."]
    #[inline]
    pub fn enable_secure_checking(&self) -> ENABLE_SECURE_CHECKINGR {
        ENABLE_SECURE_CHECKINGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - AHB bus matrix enable secure privilege check."]
    #[inline]
    pub fn enable_s_priv_check(&self) -> ENABLE_S_PRIV_CHECKR {
        ENABLE_S_PRIV_CHECKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - AHB bus matrix enable non-secure privilege check."]
    #[inline]
    pub fn enable_ns_priv_check(&self) -> ENABLE_NS_PRIV_CHECKR {
        ENABLE_NS_PRIV_CHECKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Disable secure violation abort."]
    #[inline]
    pub fn disable_violation_abort(&self) -> DISABLE_VIOLATION_ABORTR {
        DISABLE_VIOLATION_ABORTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Disable simple master strict mode."]
    #[inline]
    pub fn disable_simple_master_strict_mode(&self) -> DISABLE_SIMPLE_MASTER_STRICT_MODER {
        DISABLE_SIMPLE_MASTER_STRICT_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Disable smart master strict mode."]
    #[inline]
    pub fn disable_smart_master_strict_mode(&self) -> DISABLE_SMART_MASTER_STRICT_MODER {
        DISABLE_SMART_MASTER_STRICT_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Disable IDAU."]
    #[inline]
    pub fn idau_all_ns(&self) -> IDAU_ALL_NSR {
        IDAU_ALL_NSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 43690 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - write lock."]
    #[inline]
    pub fn write_lock(&mut self) -> _WRITE_LOCKW {
        _WRITE_LOCKW { w: self }
    }
    #[doc = "Bits 2:3 - AHB bus matrix enable secure check."]
    #[inline]
    pub fn enable_secure_checking(&mut self) -> _ENABLE_SECURE_CHECKINGW {
        _ENABLE_SECURE_CHECKINGW { w: self }
    }
    #[doc = "Bits 4:5 - AHB bus matrix enable secure privilege check."]
    #[inline]
    pub fn enable_s_priv_check(&mut self) -> _ENABLE_S_PRIV_CHECKW {
        _ENABLE_S_PRIV_CHECKW { w: self }
    }
    #[doc = "Bits 6:7 - AHB bus matrix enable non-secure privilege check."]
    #[inline]
    pub fn enable_ns_priv_check(&mut self) -> _ENABLE_NS_PRIV_CHECKW {
        _ENABLE_NS_PRIV_CHECKW { w: self }
    }
    #[doc = "Bits 8:9 - Disable secure violation abort."]
    #[inline]
    pub fn disable_violation_abort(&mut self) -> _DISABLE_VIOLATION_ABORTW {
        _DISABLE_VIOLATION_ABORTW { w: self }
    }
    #[doc = "Bits 10:11 - Disable simple master strict mode."]
    #[inline]
    pub fn disable_simple_master_strict_mode(&mut self) -> _DISABLE_SIMPLE_MASTER_STRICT_MODEW {
        _DISABLE_SIMPLE_MASTER_STRICT_MODEW { w: self }
    }
    #[doc = "Bits 12:13 - Disable smart master strict mode."]
    #[inline]
    pub fn disable_smart_master_strict_mode(&mut self) -> _DISABLE_SMART_MASTER_STRICT_MODEW {
        _DISABLE_SMART_MASTER_STRICT_MODEW { w: self }
    }
    #[doc = "Bits 14:15 - Disable IDAU."]
    #[inline]
    pub fn idau_all_ns(&mut self) -> _IDAU_ALL_NSW {
        _IDAU_ALL_NSW { w: self }
    }
}
