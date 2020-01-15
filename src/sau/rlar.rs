#[doc = "Reader of register RLAR"]
pub type R = crate::R<u32, super::RLAR>;
#[doc = "Writer for register RLAR"]
pub type W = crate::W<u32, super::RLAR>;
#[doc = "Register RLAR `reset()`'s with value 0"]
impl crate::ResetValue for super::RLAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable. SAU region enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: SAU region is enabled."]
    ENABLED = 0,
    #[doc = "1: SAU region is disabled."]
    DISABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::ENABLED,
            true => ENABLE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SAU region is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
    }
    #[doc = "SAU region is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSC_A {
    #[doc = "0: Region is not Non-secure callable."]
    NOT_NON_SECURE_CALLABLE = 0,
    #[doc = "1: Region is Non-secure callable."]
    NON_SECURE_CALLABLE = 1,
}
impl From<NSC_A> for bool {
    #[inline(always)]
    fn from(variant: NSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSC`"]
pub type NSC_R = crate::R<bool, NSC_A>;
impl NSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSC_A {
        match self.bits {
            false => NSC_A::NOT_NON_SECURE_CALLABLE,
            true => NSC_A::NON_SECURE_CALLABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_NON_SECURE_CALLABLE`"]
    #[inline(always)]
    pub fn is_not_non_secure_callable(&self) -> bool {
        *self == NSC_A::NOT_NON_SECURE_CALLABLE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE_CALLABLE`"]
    #[inline(always)]
    pub fn is_non_secure_callable(&self) -> bool {
        *self == NSC_A::NON_SECURE_CALLABLE
    }
}
#[doc = "Write proxy for field `NSC`"]
pub struct NSC_W<'a> {
    w: &'a mut W,
}
impl<'a> NSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Region is not Non-secure callable."]
    #[inline(always)]
    pub fn not_non_secure_callable(self) -> &'a mut W {
        self.variant(NSC_A::NOT_NON_SECURE_CALLABLE)
    }
    #[doc = "Region is Non-secure callable."]
    #[inline(always)]
    pub fn non_secure_callable(self) -> &'a mut W {
        self.variant(NSC_A::NON_SECURE_CALLABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `LADDR`"]
pub type LADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LADDR`"]
pub struct LADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> LADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | (((value as u32) & 0x07ff_ffff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable. SAU region enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[inline(always)]
    pub fn nsc(&self) -> NSC_R {
        NSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 5:31 - Limit address. Holds bits\\[31:5\\]
of the limit address for the selected SAU region. Bits\\[4:0\\]
of the limit address are defined as 0x1F."]
    #[inline(always)]
    pub fn laddr(&self) -> LADDR_R {
        LADDR_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Enable. SAU region enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[inline(always)]
    pub fn nsc(&mut self) -> NSC_W {
        NSC_W { w: self }
    }
    #[doc = "Bits 5:31 - Limit address. Holds bits\\[31:5\\]
of the limit address for the selected SAU region. Bits\\[4:0\\]
of the limit address are defined as 0x1F."]
    #[inline(always)]
    pub fn laddr(&mut self) -> LADDR_W {
        LADDR_W { w: self }
    }
}
