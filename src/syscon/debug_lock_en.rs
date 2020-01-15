#[doc = "Reader of register DEBUG_LOCK_EN"]
pub type R = crate::R<u32, super::DEBUG_LOCK_EN>;
#[doc = "Writer for register DEBUG_LOCK_EN"]
pub type W = crate::W<u32, super::DEBUG_LOCK_EN>;
#[doc = "Register DEBUG_LOCK_EN `reset()`'s with value 0x05"]
impl crate::ResetValue for super::DEBUG_LOCK_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x05
    }
}
#[doc = "Control write access to CODESECURITYPROTTEST, CODESECURITYPROTCPU0, CODESECURITYPROTCPU1, CPU0_DEBUG_FEATURES, CPU1_DEBUG_FEATURES and DBG_AUTH_SCRATCH registers.\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_ALL_A {
    #[doc = "0: Any other value than b1010: disable write access to all 6 registers."]
    DISABLE = 0,
    #[doc = "10: 1010: Enable write access to all 6 registers."]
    ENABLE = 10,
}
impl From<LOCK_ALL_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_ALL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOCK_ALL`"]
pub type LOCK_ALL_R = crate::R<u8, LOCK_ALL_A>;
impl LOCK_ALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOCK_ALL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOCK_ALL_A::DISABLE),
            10 => Val(LOCK_ALL_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOCK_ALL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LOCK_ALL_A::ENABLE
    }
}
#[doc = "Write proxy for field `LOCK_ALL`"]
pub struct LOCK_ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_ALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_ALL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Any other value than b1010: disable write access to all 6 registers."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOCK_ALL_A::DISABLE)
    }
    #[doc = "1010: Enable write access to all 6 registers."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOCK_ALL_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Control write access to CODESECURITYPROTTEST, CODESECURITYPROTCPU0, CODESECURITYPROTCPU1, CPU0_DEBUG_FEATURES, CPU1_DEBUG_FEATURES and DBG_AUTH_SCRATCH registers."]
    #[inline(always)]
    pub fn lock_all(&self) -> LOCK_ALL_R {
        LOCK_ALL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Control write access to CODESECURITYPROTTEST, CODESECURITYPROTCPU0, CODESECURITYPROTCPU1, CPU0_DEBUG_FEATURES, CPU1_DEBUG_FEATURES and DBG_AUTH_SCRATCH registers."]
    #[inline(always)]
    pub fn lock_all(&mut self) -> LOCK_ALL_W {
        LOCK_ALL_W { w: self }
    }
}
