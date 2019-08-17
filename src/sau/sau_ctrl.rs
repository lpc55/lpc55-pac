#[doc = "Reader of register SAU_CTRL"]
pub type R = crate::R<u32, super::SAU_CTRL>;
#[doc = "Writer for register SAU_CTRL"]
pub type W = crate::W<u32, super::SAU_CTRL>;
#[doc = "Register SAU_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SAU_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "The SAU is disabled."]
    DISABLED,
    #[doc = "The SAU is enabled."]
    ENABLED,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        match variant {
            ENABLE_A::DISABLED => false,
            ENABLE_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
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
    #[doc = "The SAU is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "The SAU is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
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
#[doc = "Possible values of the field `ALLNS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALLNS_A {
    #[doc = "Memory is marked as Secure and is not Non-secure callable."]
    SECURED_MEMORY,
    #[doc = "Memory is marked as Non-secure."]
    NON_SECURED_MEMORY,
}
impl From<ALLNS_A> for bool {
    #[inline(always)]
    fn from(variant: ALLNS_A) -> Self {
        match variant {
            ALLNS_A::SECURED_MEMORY => false,
            ALLNS_A::NON_SECURED_MEMORY => true,
        }
    }
}
#[doc = "Reader of field `ALLNS`"]
pub type ALLNS_R = crate::R<bool, ALLNS_A>;
impl ALLNS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALLNS_A {
        match self.bits {
            false => ALLNS_A::SECURED_MEMORY,
            true => ALLNS_A::NON_SECURED_MEMORY,
        }
    }
    #[doc = "Checks if the value of the field is `SECURED_MEMORY`"]
    #[inline(always)]
    pub fn is_secured_memory(&self) -> bool {
        *self == ALLNS_A::SECURED_MEMORY
    }
    #[doc = "Checks if the value of the field is `NON_SECURED_MEMORY`"]
    #[inline(always)]
    pub fn is_non_secured_memory(&self) -> bool {
        *self == ALLNS_A::NON_SECURED_MEMORY
    }
}
#[doc = "Write proxy for field `ALLNS`"]
pub struct ALLNS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALLNS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory is marked as Secure and is not Non-secure callable."]
    #[inline(always)]
    pub fn secured_memory(self) -> &'a mut W {
        self.variant(ALLNS_A::SECURED_MEMORY)
    }
    #[doc = "Memory is marked as Non-secure."]
    #[inline(always)]
    pub fn non_secured_memory(self) -> &'a mut W {
        self.variant(ALLNS_A::NON_SECURED_MEMORY)
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
impl R {
    #[doc = "Bit 0 - Enable. Enables the SAU. This bit is RAZ/WI when the Security Extension is implemented without an SAU region."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - All Non-secure."]
    #[inline(always)]
    pub fn allns(&self) -> ALLNS_R {
        ALLNS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable. Enables the SAU. This bit is RAZ/WI when the Security Extension is implemented without an SAU region."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - All Non-secure."]
    #[inline(always)]
    pub fn allns(&mut self) -> ALLNS_W {
        ALLNS_W { w: self }
    }
}
