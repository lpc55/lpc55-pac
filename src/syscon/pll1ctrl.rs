#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLL1CTRL {
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
#[doc = r" Value of the field"]
pub struct SELRR {
    bits: u8,
}
impl SELRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SELIR {
    bits: u8,
}
impl SELIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SELPR {
    bits: u8,
}
impl SELPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BYPASSPLL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSPLLR {
    #[doc = "use PLL."]
    USED,
    #[doc = "PLL input clock is sent directly to the PLL output."]
    BYPASSED,
}
impl BYPASSPLLR {
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
            BYPASSPLLR::USED => false,
            BYPASSPLLR::BYPASSED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASSPLLR {
        match value {
            false => BYPASSPLLR::USED,
            true => BYPASSPLLR::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline]
    pub fn is_used(&self) -> bool {
        *self == BYPASSPLLR::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASSPLLR::BYPASSED
    }
}
#[doc = "Possible values of the field `BYPASSPOSTDIV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSPOSTDIV2R {
    #[doc = "use the divide-by-2 divider in the post-divider."]
    USED,
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    BYPASSED,
}
impl BYPASSPOSTDIV2R {
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
            BYPASSPOSTDIV2R::USED => false,
            BYPASSPOSTDIV2R::BYPASSED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASSPOSTDIV2R {
        match value {
            false => BYPASSPOSTDIV2R::USED,
            true => BYPASSPOSTDIV2R::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline]
    pub fn is_used(&self) -> bool {
        *self == BYPASSPOSTDIV2R::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASSPOSTDIV2R::BYPASSED
    }
}
#[doc = r" Value of the field"]
pub struct LIMUPOFFR {
    bits: bool,
}
impl LIMUPOFFR {
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
#[doc = "Possible values of the field `BWDIRECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWDIRECTR {
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    SYNC,
    #[doc = "modify the bandwidth of the PLL directly."]
    DIRECT,
}
impl BWDIRECTR {
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
            BWDIRECTR::SYNC => false,
            BWDIRECTR::DIRECT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWDIRECTR {
        match value {
            false => BWDIRECTR::SYNC,
            true => BWDIRECTR::DIRECT,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline]
    pub fn is_sync(&self) -> bool {
        *self == BWDIRECTR::SYNC
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline]
    pub fn is_direct(&self) -> bool {
        *self == BWDIRECTR::DIRECT
    }
}
#[doc = "Possible values of the field `BYPASSPREDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSPREDIVR {
    #[doc = "use the pre-divider."]
    USED,
    #[doc = "bypass of the pre-divider."]
    BYPASSED,
}
impl BYPASSPREDIVR {
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
            BYPASSPREDIVR::USED => false,
            BYPASSPREDIVR::BYPASSED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASSPREDIVR {
        match value {
            false => BYPASSPREDIVR::USED,
            true => BYPASSPREDIVR::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline]
    pub fn is_used(&self) -> bool {
        *self == BYPASSPREDIVR::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASSPREDIVR::BYPASSED
    }
}
#[doc = "Possible values of the field `BYPASSPOSTDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSPOSTDIVR {
    #[doc = "use the post-divider."]
    USED,
    #[doc = "bypass of the post-divider."]
    BYPASSED,
}
impl BYPASSPOSTDIVR {
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
            BYPASSPOSTDIVR::USED => false,
            BYPASSPOSTDIVR::BYPASSED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASSPOSTDIVR {
        match value {
            false => BYPASSPOSTDIVR::USED,
            true => BYPASSPOSTDIVR::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline]
    pub fn is_used(&self) -> bool {
        *self == BYPASSPOSTDIVR::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASSPOSTDIVR::BYPASSED
    }
}
#[doc = "Possible values of the field `CLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKENR {
    #[doc = "Disable the output clock."]
    DISABLE,
    #[doc = "Enable the output clock."]
    ENABLE,
}
impl CLKENR {
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
            CLKENR::DISABLE => false,
            CLKENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKENR {
        match value {
            false => CLKENR::DISABLE,
            true => CLKENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CLKENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CLKENR::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct FRMENR {
    bits: bool,
}
impl FRMENR {
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
pub struct FRMCLKSTABLER {
    bits: bool,
}
impl FRMCLKSTABLER {
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
#[doc = "Possible values of the field `SKEWEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEWENR {
    #[doc = "skewmode is disable."]
    DISABLE,
    #[doc = "skewmode is enable."]
    ENABLE,
}
impl SKEWENR {
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
            SKEWENR::DISABLE => false,
            SKEWENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SKEWENR {
        match value {
            false => SKEWENR::DISABLE,
            true => SKEWENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SKEWENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SKEWENR::ENABLE
    }
}
#[doc = r" Proxy"]
pub struct _SELRW<'a> {
    w: &'a mut W,
}
impl<'a> _SELRW<'a> {
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
#[doc = r" Proxy"]
pub struct _SELIW<'a> {
    w: &'a mut W,
}
impl<'a> _SELIW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SELPW<'a> {
    w: &'a mut W,
}
impl<'a> _SELPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BYPASSPLL`"]
pub enum BYPASSPLLW {
    #[doc = "use PLL."]
    USED,
    #[doc = "PLL input clock is sent directly to the PLL output."]
    BYPASSED,
}
impl BYPASSPLLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSPLLW::USED => false,
            BYPASSPLLW::BYPASSED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSPLLW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSPLLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASSPLLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "use PLL."]
    #[inline]
    pub fn used(self) -> &'a mut W {
        self.variant(BYPASSPLLW::USED)
    }
    #[doc = "PLL input clock is sent directly to the PLL output."]
    #[inline]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(BYPASSPLLW::BYPASSED)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BYPASSPOSTDIV2`"]
pub enum BYPASSPOSTDIV2W {
    #[doc = "use the divide-by-2 divider in the post-divider."]
    USED,
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    BYPASSED,
}
impl BYPASSPOSTDIV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSPOSTDIV2W::USED => false,
            BYPASSPOSTDIV2W::BYPASSED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSPOSTDIV2W<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSPOSTDIV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASSPOSTDIV2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "use the divide-by-2 divider in the post-divider."]
    #[inline]
    pub fn used(self) -> &'a mut W {
        self.variant(BYPASSPOSTDIV2W::USED)
    }
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    #[inline]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(BYPASSPOSTDIV2W::BYPASSED)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LIMUPOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _LIMUPOFFW<'a> {
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
#[doc = "Values that can be written to the field `BWDIRECT`"]
pub enum BWDIRECTW {
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    SYNC,
    #[doc = "modify the bandwidth of the PLL directly."]
    DIRECT,
}
impl BWDIRECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWDIRECTW::SYNC => false,
            BWDIRECTW::DIRECT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWDIRECTW<'a> {
    w: &'a mut W,
}
impl<'a> _BWDIRECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWDIRECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    #[inline]
    pub fn sync(self) -> &'a mut W {
        self.variant(BWDIRECTW::SYNC)
    }
    #[doc = "modify the bandwidth of the PLL directly."]
    #[inline]
    pub fn direct(self) -> &'a mut W {
        self.variant(BWDIRECTW::DIRECT)
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
#[doc = "Values that can be written to the field `BYPASSPREDIV`"]
pub enum BYPASSPREDIVW {
    #[doc = "use the pre-divider."]
    USED,
    #[doc = "bypass of the pre-divider."]
    BYPASSED,
}
impl BYPASSPREDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSPREDIVW::USED => false,
            BYPASSPREDIVW::BYPASSED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSPREDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSPREDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASSPREDIVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "use the pre-divider."]
    #[inline]
    pub fn used(self) -> &'a mut W {
        self.variant(BYPASSPREDIVW::USED)
    }
    #[doc = "bypass of the pre-divider."]
    #[inline]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(BYPASSPREDIVW::BYPASSED)
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
#[doc = "Values that can be written to the field `BYPASSPOSTDIV`"]
pub enum BYPASSPOSTDIVW {
    #[doc = "use the post-divider."]
    USED,
    #[doc = "bypass of the post-divider."]
    BYPASSED,
}
impl BYPASSPOSTDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSPOSTDIVW::USED => false,
            BYPASSPOSTDIVW::BYPASSED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSPOSTDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSPOSTDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASSPOSTDIVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "use the post-divider."]
    #[inline]
    pub fn used(self) -> &'a mut W {
        self.variant(BYPASSPOSTDIVW::USED)
    }
    #[doc = "bypass of the post-divider."]
    #[inline]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(BYPASSPOSTDIVW::BYPASSED)
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
#[doc = "Values that can be written to the field `CLKEN`"]
pub enum CLKENW {
    #[doc = "Disable the output clock."]
    DISABLE,
    #[doc = "Enable the output clock."]
    ENABLE,
}
impl CLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKENW::DISABLE => false,
            CLKENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the output clock."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLKENW::DISABLE)
    }
    #[doc = "Enable the output clock."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLKENW::ENABLE)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRMENW<'a> {
    w: &'a mut W,
}
impl<'a> _FRMENW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRMCLKSTABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _FRMCLKSTABLEW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SKEWEN`"]
pub enum SKEWENW {
    #[doc = "skewmode is disable."]
    DISABLE,
    #[doc = "skewmode is enable."]
    ENABLE,
}
impl SKEWENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SKEWENW::DISABLE => false,
            SKEWENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SKEWENW<'a> {
    w: &'a mut W,
}
impl<'a> _SKEWENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SKEWENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "skewmode is disable."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SKEWENW::DISABLE)
    }
    #[doc = "skewmode is enable."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SKEWENW::ENABLE)
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline]
    pub fn selr(&self) -> SELRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SELRR { bits }
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline]
    pub fn seli(&self) -> SELIR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SELIR { bits }
    }
    #[doc = "Bits 10:14 - Bandwidth select P value."]
    #[inline]
    pub fn selp(&self) -> SELPR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SELPR { bits }
    }
    #[doc = "Bit 15 - Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[inline]
    pub fn bypasspll(&self) -> BYPASSPLLR {
        BYPASSPLLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - bypass of the divide-by-2 divider in the post-divider."]
    #[inline]
    pub fn bypasspostdiv2(&self) -> BYPASSPOSTDIV2R {
        BYPASSPOSTDIV2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[inline]
    pub fn limupoff(&self) -> LIMUPOFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LIMUPOFFR { bits }
    }
    #[doc = "Bit 18 - control of the bandwidth of the PLL."]
    #[inline]
    pub fn bwdirect(&self) -> BWDIRECTR {
        BWDIRECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - bypass of the pre-divider."]
    #[inline]
    pub fn bypassprediv(&self) -> BYPASSPREDIVR {
        BYPASSPREDIVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - bypass of the post-divider."]
    #[inline]
    pub fn bypasspostdiv(&self) -> BYPASSPOSTDIVR {
        BYPASSPOSTDIVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - enable the output clock."]
    #[inline]
    pub fn clken(&self) -> CLKENR {
        CLKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - 1: free running mode."]
    #[inline]
    pub fn frmen(&self) -> FRMENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRMENR { bits }
    }
    #[doc = "Bit 23 - free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
    #[inline]
    pub fn frmclkstable(&self) -> FRMCLKSTABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRMCLKSTABLER { bits }
    }
    #[doc = "Bit 24 - Skew mode."]
    #[inline]
    pub fn skewen(&self) -> SKEWENR {
        SKEWENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline]
    pub fn selr(&mut self) -> _SELRW {
        _SELRW { w: self }
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline]
    pub fn seli(&mut self) -> _SELIW {
        _SELIW { w: self }
    }
    #[doc = "Bits 10:14 - Bandwidth select P value."]
    #[inline]
    pub fn selp(&mut self) -> _SELPW {
        _SELPW { w: self }
    }
    #[doc = "Bit 15 - Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[inline]
    pub fn bypasspll(&mut self) -> _BYPASSPLLW {
        _BYPASSPLLW { w: self }
    }
    #[doc = "Bit 16 - bypass of the divide-by-2 divider in the post-divider."]
    #[inline]
    pub fn bypasspostdiv2(&mut self) -> _BYPASSPOSTDIV2W {
        _BYPASSPOSTDIV2W { w: self }
    }
    #[doc = "Bit 17 - limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[inline]
    pub fn limupoff(&mut self) -> _LIMUPOFFW {
        _LIMUPOFFW { w: self }
    }
    #[doc = "Bit 18 - control of the bandwidth of the PLL."]
    #[inline]
    pub fn bwdirect(&mut self) -> _BWDIRECTW {
        _BWDIRECTW { w: self }
    }
    #[doc = "Bit 19 - bypass of the pre-divider."]
    #[inline]
    pub fn bypassprediv(&mut self) -> _BYPASSPREDIVW {
        _BYPASSPREDIVW { w: self }
    }
    #[doc = "Bit 20 - bypass of the post-divider."]
    #[inline]
    pub fn bypasspostdiv(&mut self) -> _BYPASSPOSTDIVW {
        _BYPASSPOSTDIVW { w: self }
    }
    #[doc = "Bit 21 - enable the output clock."]
    #[inline]
    pub fn clken(&mut self) -> _CLKENW {
        _CLKENW { w: self }
    }
    #[doc = "Bit 22 - 1: free running mode."]
    #[inline]
    pub fn frmen(&mut self) -> _FRMENW {
        _FRMENW { w: self }
    }
    #[doc = "Bit 23 - free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
    #[inline]
    pub fn frmclkstable(&mut self) -> _FRMCLKSTABLEW {
        _FRMCLKSTABLEW { w: self }
    }
    #[doc = "Bit 24 - Skew mode."]
    #[inline]
    pub fn skewen(&mut self) -> _SKEWENW {
        _SKEWENW { w: self }
    }
}
