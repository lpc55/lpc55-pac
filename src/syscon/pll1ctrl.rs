#[doc = "Reader of register PLL1CTRL"]
pub type R = crate::R<u32, super::PLL1CTRL>;
#[doc = "Writer for register PLL1CTRL"]
pub type W = crate::W<u32, super::PLL1CTRL>;
#[doc = "Register PLL1CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL1CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SELR`"]
pub type SELR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SELR`"]
pub struct SELR_W<'a> {
    w: &'a mut W,
}
impl<'a> SELR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `SELI`"]
pub type SELI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SELI`"]
pub struct SELI_W<'a> {
    w: &'a mut W,
}
impl<'a> SELI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SELP`"]
pub type SELP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SELP`"]
pub struct SELP_W<'a> {
    w: &'a mut W,
}
impl<'a> SELP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `BYPASSPLL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSPLL_A {
    #[doc = "use PLL."]
    USED,
    #[doc = "PLL input clock is sent directly to the PLL output."]
    BYPASSED,
}
impl From<BYPASSPLL_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSPLL_A) -> Self {
        match variant {
            BYPASSPLL_A::USED => false,
            BYPASSPLL_A::BYPASSED => true,
        }
    }
}
#[doc = "Reader of field `BYPASSPLL`"]
pub type BYPASSPLL_R = crate::R<bool, BYPASSPLL_A>;
impl BYPASSPLL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASSPLL_A {
        match self.bits {
            false => BYPASSPLL_A::USED,
            true => BYPASSPLL_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == BYPASSPLL_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASSPLL_A::BYPASSED
    }
}
#[doc = "Write proxy for field `BYPASSPLL`"]
pub struct BYPASSPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASSPLL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASSPLL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "use PLL."]
    #[inline(always)]
    pub fn used(self) -> &'a mut W {
        self.variant(BYPASSPLL_A::USED)
    }
    #[doc = "PLL input clock is sent directly to the PLL output."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(BYPASSPLL_A::BYPASSED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Possible values of the field `BYPASSPOSTDIV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSPOSTDIV2_A {
    #[doc = "use the divide-by-2 divider in the post-divider."]
    USED,
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    BYPASSED,
}
impl From<BYPASSPOSTDIV2_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSPOSTDIV2_A) -> Self {
        match variant {
            BYPASSPOSTDIV2_A::USED => false,
            BYPASSPOSTDIV2_A::BYPASSED => true,
        }
    }
}
#[doc = "Reader of field `BYPASSPOSTDIV2`"]
pub type BYPASSPOSTDIV2_R = crate::R<bool, BYPASSPOSTDIV2_A>;
impl BYPASSPOSTDIV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASSPOSTDIV2_A {
        match self.bits {
            false => BYPASSPOSTDIV2_A::USED,
            true => BYPASSPOSTDIV2_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == BYPASSPOSTDIV2_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASSPOSTDIV2_A::BYPASSED
    }
}
#[doc = "Write proxy for field `BYPASSPOSTDIV2`"]
pub struct BYPASSPOSTDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASSPOSTDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASSPOSTDIV2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "use the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub fn used(self) -> &'a mut W {
        self.variant(BYPASSPOSTDIV2_A::USED)
    }
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(BYPASSPOSTDIV2_A::BYPASSED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `LIMUPOFF`"]
pub type LIMUPOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LIMUPOFF`"]
pub struct LIMUPOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMUPOFF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Possible values of the field `BWDIRECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWDIRECT_A {
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    SYNC,
    #[doc = "modify the bandwidth of the PLL directly."]
    DIRECT,
}
impl From<BWDIRECT_A> for bool {
    #[inline(always)]
    fn from(variant: BWDIRECT_A) -> Self {
        match variant {
            BWDIRECT_A::SYNC => false,
            BWDIRECT_A::DIRECT => true,
        }
    }
}
#[doc = "Reader of field `BWDIRECT`"]
pub type BWDIRECT_R = crate::R<bool, BWDIRECT_A>;
impl BWDIRECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWDIRECT_A {
        match self.bits {
            false => BWDIRECT_A::SYNC,
            true => BWDIRECT_A::DIRECT,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == BWDIRECT_A::SYNC
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == BWDIRECT_A::DIRECT
    }
}
#[doc = "Write proxy for field `BWDIRECT`"]
pub struct BWDIRECT_W<'a> {
    w: &'a mut W,
}
impl<'a> BWDIRECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWDIRECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(BWDIRECT_A::SYNC)
    }
    #[doc = "modify the bandwidth of the PLL directly."]
    #[inline(always)]
    pub fn direct(self) -> &'a mut W {
        self.variant(BWDIRECT_A::DIRECT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `BYPASSPREDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSPREDIV_A {
    #[doc = "use the pre-divider."]
    USED,
    #[doc = "bypass of the pre-divider."]
    BYPASSED,
}
impl From<BYPASSPREDIV_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSPREDIV_A) -> Self {
        match variant {
            BYPASSPREDIV_A::USED => false,
            BYPASSPREDIV_A::BYPASSED => true,
        }
    }
}
#[doc = "Reader of field `BYPASSPREDIV`"]
pub type BYPASSPREDIV_R = crate::R<bool, BYPASSPREDIV_A>;
impl BYPASSPREDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASSPREDIV_A {
        match self.bits {
            false => BYPASSPREDIV_A::USED,
            true => BYPASSPREDIV_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == BYPASSPREDIV_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASSPREDIV_A::BYPASSED
    }
}
#[doc = "Write proxy for field `BYPASSPREDIV`"]
pub struct BYPASSPREDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASSPREDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASSPREDIV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "use the pre-divider."]
    #[inline(always)]
    pub fn used(self) -> &'a mut W {
        self.variant(BYPASSPREDIV_A::USED)
    }
    #[doc = "bypass of the pre-divider."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(BYPASSPREDIV_A::BYPASSED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Possible values of the field `BYPASSPOSTDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSPOSTDIV_A {
    #[doc = "use the post-divider."]
    USED,
    #[doc = "bypass of the post-divider."]
    BYPASSED,
}
impl From<BYPASSPOSTDIV_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSPOSTDIV_A) -> Self {
        match variant {
            BYPASSPOSTDIV_A::USED => false,
            BYPASSPOSTDIV_A::BYPASSED => true,
        }
    }
}
#[doc = "Reader of field `BYPASSPOSTDIV`"]
pub type BYPASSPOSTDIV_R = crate::R<bool, BYPASSPOSTDIV_A>;
impl BYPASSPOSTDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASSPOSTDIV_A {
        match self.bits {
            false => BYPASSPOSTDIV_A::USED,
            true => BYPASSPOSTDIV_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == BYPASSPOSTDIV_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASSPOSTDIV_A::BYPASSED
    }
}
#[doc = "Write proxy for field `BYPASSPOSTDIV`"]
pub struct BYPASSPOSTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASSPOSTDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASSPOSTDIV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "use the post-divider."]
    #[inline(always)]
    pub fn used(self) -> &'a mut W {
        self.variant(BYPASSPOSTDIV_A::USED)
    }
    #[doc = "bypass of the post-divider."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(BYPASSPOSTDIV_A::BYPASSED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `CLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKEN_A {
    #[doc = "Disable the output clock."]
    DISABLE,
    #[doc = "Enable the output clock."]
    ENABLE,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        match variant {
            CLKEN_A::DISABLE => false,
            CLKEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CLKEN`"]
pub type CLKEN_R = crate::R<bool, CLKEN_A>;
impl CLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::DISABLE,
            true => CLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLKEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CLKEN`"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the output clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLKEN_A::DISABLE)
    }
    #[doc = "Enable the output clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLKEN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `FRMEN`"]
pub type FRMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRMEN`"]
pub struct FRMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `FRMCLKSTABLE`"]
pub type FRMCLKSTABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRMCLKSTABLE`"]
pub struct FRMCLKSTABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMCLKSTABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `SKEWEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEWEN_A {
    #[doc = "skewmode is disable."]
    DISABLE,
    #[doc = "skewmode is enable."]
    ENABLE,
}
impl From<SKEWEN_A> for bool {
    #[inline(always)]
    fn from(variant: SKEWEN_A) -> Self {
        match variant {
            SKEWEN_A::DISABLE => false,
            SKEWEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SKEWEN`"]
pub type SKEWEN_R = crate::R<bool, SKEWEN_A>;
impl SKEWEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKEWEN_A {
        match self.bits {
            false => SKEWEN_A::DISABLE,
            true => SKEWEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SKEWEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SKEWEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `SKEWEN`"]
pub struct SKEWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SKEWEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SKEWEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "skewmode is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SKEWEN_A::DISABLE)
    }
    #[doc = "skewmode is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SKEWEN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline(always)]
    pub fn selr(&self) -> SELR_R {
        SELR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline(always)]
    pub fn seli(&self) -> SELI_R {
        SELI_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:14 - Bandwidth select P value."]
    #[inline(always)]
    pub fn selp(&self) -> SELP_R {
        SELP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[inline(always)]
    pub fn bypasspll(&self) -> BYPASSPLL_R {
        BYPASSPLL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub fn bypasspostdiv2(&self) -> BYPASSPOSTDIV2_R {
        BYPASSPOSTDIV2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[inline(always)]
    pub fn limupoff(&self) -> LIMUPOFF_R {
        LIMUPOFF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - control of the bandwidth of the PLL."]
    #[inline(always)]
    pub fn bwdirect(&self) -> BWDIRECT_R {
        BWDIRECT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - bypass of the pre-divider."]
    #[inline(always)]
    pub fn bypassprediv(&self) -> BYPASSPREDIV_R {
        BYPASSPREDIV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - bypass of the post-divider."]
    #[inline(always)]
    pub fn bypasspostdiv(&self) -> BYPASSPOSTDIV_R {
        BYPASSPOSTDIV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - enable the output clock."]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 1: free running mode."]
    #[inline(always)]
    pub fn frmen(&self) -> FRMEN_R {
        FRMEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
    #[inline(always)]
    pub fn frmclkstable(&self) -> FRMCLKSTABLE_R {
        FRMCLKSTABLE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Skew mode."]
    #[inline(always)]
    pub fn skewen(&self) -> SKEWEN_R {
        SKEWEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline(always)]
    pub fn selr(&mut self) -> SELR_W {
        SELR_W { w: self }
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline(always)]
    pub fn seli(&mut self) -> SELI_W {
        SELI_W { w: self }
    }
    #[doc = "Bits 10:14 - Bandwidth select P value."]
    #[inline(always)]
    pub fn selp(&mut self) -> SELP_W {
        SELP_W { w: self }
    }
    #[doc = "Bit 15 - Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[inline(always)]
    pub fn bypasspll(&mut self) -> BYPASSPLL_W {
        BYPASSPLL_W { w: self }
    }
    #[doc = "Bit 16 - bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub fn bypasspostdiv2(&mut self) -> BYPASSPOSTDIV2_W {
        BYPASSPOSTDIV2_W { w: self }
    }
    #[doc = "Bit 17 - limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[inline(always)]
    pub fn limupoff(&mut self) -> LIMUPOFF_W {
        LIMUPOFF_W { w: self }
    }
    #[doc = "Bit 18 - control of the bandwidth of the PLL."]
    #[inline(always)]
    pub fn bwdirect(&mut self) -> BWDIRECT_W {
        BWDIRECT_W { w: self }
    }
    #[doc = "Bit 19 - bypass of the pre-divider."]
    #[inline(always)]
    pub fn bypassprediv(&mut self) -> BYPASSPREDIV_W {
        BYPASSPREDIV_W { w: self }
    }
    #[doc = "Bit 20 - bypass of the post-divider."]
    #[inline(always)]
    pub fn bypasspostdiv(&mut self) -> BYPASSPOSTDIV_W {
        BYPASSPOSTDIV_W { w: self }
    }
    #[doc = "Bit 21 - enable the output clock."]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Bit 22 - 1: free running mode."]
    #[inline(always)]
    pub fn frmen(&mut self) -> FRMEN_W {
        FRMEN_W { w: self }
    }
    #[doc = "Bit 23 - free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
    #[inline(always)]
    pub fn frmclkstable(&mut self) -> FRMCLKSTABLE_W {
        FRMCLKSTABLE_W { w: self }
    }
    #[doc = "Bit 24 - Skew mode."]
    #[inline(always)]
    pub fn skewen(&mut self) -> SKEWEN_W {
        SKEWEN_W { w: self }
    }
}
