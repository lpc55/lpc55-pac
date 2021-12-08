#[doc = "Register `PLL1CTRL` reader"]
pub struct R(crate::R<PLL1CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL1CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL1CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL1CTRL` writer"]
pub struct W(crate::W<PLL1CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PLL1CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL1CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELR` reader - Bandwidth select R value."]
pub struct SELR_R(crate::FieldReader<u8, u8>);
impl SELR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SELR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELR` writer - Bandwidth select R value."]
pub struct SELR_W<'a> {
    w: &'a mut W,
}
impl<'a> SELR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `SELI` reader - Bandwidth select I value."]
pub struct SELI_R(crate::FieldReader<u8, u8>);
impl SELI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SELI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELI` writer - Bandwidth select I value."]
pub struct SELI_W<'a> {
    w: &'a mut W,
}
impl<'a> SELI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | ((value as u32 & 0x3f) << 4);
        self.w
    }
}
#[doc = "Field `SELP` reader - Bandwidth select P value."]
pub struct SELP_R(crate::FieldReader<u8, u8>);
impl SELP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SELP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELP` writer - Bandwidth select P value."]
pub struct SELP_W<'a> {
    w: &'a mut W,
}
impl<'a> SELP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Bypass PLL input clock is sent directly to the PLL output (default).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSPLL_A {
    #[doc = "0: use PLL."]
    USED = 0,
    #[doc = "1: PLL input clock is sent directly to the PLL output."]
    BYPASSED = 1,
}
impl From<BYPASSPLL_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSPLL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASSPLL` reader - Bypass PLL input clock is sent directly to the PLL output (default)."]
pub struct BYPASSPLL_R(crate::FieldReader<bool, BYPASSPLL_A>);
impl BYPASSPLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASSPLL_R(crate::FieldReader::new(bits))
    }
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
        **self == BYPASSPLL_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        **self == BYPASSPLL_A::BYPASSED
    }
}
impl core::ops::Deref for BYPASSPLL_R {
    type Target = crate::FieldReader<bool, BYPASSPLL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASSPLL` writer - Bypass PLL input clock is sent directly to the PLL output (default)."]
pub struct BYPASSPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASSPLL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASSPLL_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "bypass of the divide-by-2 divider in the post-divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSPOSTDIV2_A {
    #[doc = "0: use the divide-by-2 divider in the post-divider."]
    USED = 0,
    #[doc = "1: bypass of the divide-by-2 divider in the post-divider."]
    BYPASSED = 1,
}
impl From<BYPASSPOSTDIV2_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSPOSTDIV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASSPOSTDIV2` reader - bypass of the divide-by-2 divider in the post-divider."]
pub struct BYPASSPOSTDIV2_R(crate::FieldReader<bool, BYPASSPOSTDIV2_A>);
impl BYPASSPOSTDIV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASSPOSTDIV2_R(crate::FieldReader::new(bits))
    }
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
        **self == BYPASSPOSTDIV2_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        **self == BYPASSPOSTDIV2_A::BYPASSED
    }
}
impl core::ops::Deref for BYPASSPOSTDIV2_R {
    type Target = crate::FieldReader<bool, BYPASSPOSTDIV2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASSPOSTDIV2` writer - bypass of the divide-by-2 divider in the post-divider."]
pub struct BYPASSPOSTDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASSPOSTDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASSPOSTDIV2_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `LIMUPOFF` reader - limup_off = 1 in spread spectrum and fractional PLL applications."]
pub struct LIMUPOFF_R(crate::FieldReader<bool, bool>);
impl LIMUPOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LIMUPOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIMUPOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIMUPOFF` writer - limup_off = 1 in spread spectrum and fractional PLL applications."]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "control of the bandwidth of the PLL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWDIRECT_A {
    #[doc = "0: the bandwidth is changed synchronously with the feedback-divider."]
    SYNC = 0,
    #[doc = "1: modify the bandwidth of the PLL directly."]
    DIRECT = 1,
}
impl From<BWDIRECT_A> for bool {
    #[inline(always)]
    fn from(variant: BWDIRECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWDIRECT` reader - control of the bandwidth of the PLL."]
pub struct BWDIRECT_R(crate::FieldReader<bool, BWDIRECT_A>);
impl BWDIRECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BWDIRECT_R(crate::FieldReader::new(bits))
    }
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
        **self == BWDIRECT_A::SYNC
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        **self == BWDIRECT_A::DIRECT
    }
}
impl core::ops::Deref for BWDIRECT_R {
    type Target = crate::FieldReader<bool, BWDIRECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BWDIRECT` writer - control of the bandwidth of the PLL."]
pub struct BWDIRECT_W<'a> {
    w: &'a mut W,
}
impl<'a> BWDIRECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWDIRECT_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "bypass of the pre-divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSPREDIV_A {
    #[doc = "0: use the pre-divider."]
    USED = 0,
    #[doc = "1: bypass of the pre-divider."]
    BYPASSED = 1,
}
impl From<BYPASSPREDIV_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSPREDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASSPREDIV` reader - bypass of the pre-divider."]
pub struct BYPASSPREDIV_R(crate::FieldReader<bool, BYPASSPREDIV_A>);
impl BYPASSPREDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASSPREDIV_R(crate::FieldReader::new(bits))
    }
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
        **self == BYPASSPREDIV_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        **self == BYPASSPREDIV_A::BYPASSED
    }
}
impl core::ops::Deref for BYPASSPREDIV_R {
    type Target = crate::FieldReader<bool, BYPASSPREDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASSPREDIV` writer - bypass of the pre-divider."]
pub struct BYPASSPREDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASSPREDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASSPREDIV_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "bypass of the post-divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSPOSTDIV_A {
    #[doc = "0: use the post-divider."]
    USED = 0,
    #[doc = "1: bypass of the post-divider."]
    BYPASSED = 1,
}
impl From<BYPASSPOSTDIV_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSPOSTDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASSPOSTDIV` reader - bypass of the post-divider."]
pub struct BYPASSPOSTDIV_R(crate::FieldReader<bool, BYPASSPOSTDIV_A>);
impl BYPASSPOSTDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASSPOSTDIV_R(crate::FieldReader::new(bits))
    }
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
        **self == BYPASSPOSTDIV_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        **self == BYPASSPOSTDIV_A::BYPASSED
    }
}
impl core::ops::Deref for BYPASSPOSTDIV_R {
    type Target = crate::FieldReader<bool, BYPASSPOSTDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASSPOSTDIV` writer - bypass of the post-divider."]
pub struct BYPASSPOSTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASSPOSTDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASSPOSTDIV_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "enable the output clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKEN_A {
    #[doc = "0: Disable the output clock."]
    DISABLE = 0,
    #[doc = "1: Enable the output clock."]
    ENABLE = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - enable the output clock."]
pub struct CLKEN_R(crate::FieldReader<bool, CLKEN_A>);
impl CLKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKEN_R(crate::FieldReader::new(bits))
    }
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
        **self == CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CLKEN_A::ENABLE
    }
}
impl core::ops::Deref for CLKEN_R {
    type Target = crate::FieldReader<bool, CLKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKEN` writer - enable the output clock."]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `FRMEN` reader - 1: free running mode."]
pub struct FRMEN_R(crate::FieldReader<bool, bool>);
impl FRMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRMEN` writer - 1: free running mode."]
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `FRMCLKSTABLE` reader - free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
pub struct FRMCLKSTABLE_R(crate::FieldReader<bool, bool>);
impl FRMCLKSTABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRMCLKSTABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMCLKSTABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRMCLKSTABLE` writer - free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Skew mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEWEN_A {
    #[doc = "0: skewmode is disable."]
    DISABLE = 0,
    #[doc = "1: skewmode is enable."]
    ENABLE = 1,
}
impl From<SKEWEN_A> for bool {
    #[inline(always)]
    fn from(variant: SKEWEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SKEWEN` reader - Skew mode."]
pub struct SKEWEN_R(crate::FieldReader<bool, SKEWEN_A>);
impl SKEWEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SKEWEN_R(crate::FieldReader::new(bits))
    }
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
        **self == SKEWEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SKEWEN_A::ENABLE
    }
}
impl core::ops::Deref for SKEWEN_R {
    type Target = crate::FieldReader<bool, SKEWEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKEWEN` writer - Skew mode."]
pub struct SKEWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SKEWEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SKEWEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL1 550m control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1ctrl](index.html) module"]
pub struct PLL1CTRL_SPEC;
impl crate::RegisterSpec for PLL1CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll1ctrl::R](R) reader structure"]
impl crate::Readable for PLL1CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll1ctrl::W](W) writer structure"]
impl crate::Writable for PLL1CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL1CTRL to value 0"]
impl crate::Resettable for PLL1CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
