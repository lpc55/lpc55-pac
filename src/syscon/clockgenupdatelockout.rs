#[doc = "Reader of register CLOCKGENUPDATELOCKOUT"]
pub type R = crate::R<u32, super::CLOCKGENUPDATELOCKOUT>;
#[doc = "Writer for register CLOCKGENUPDATELOCKOUT"]
pub type W = crate::W<u32, super::CLOCKGENUPDATELOCKOUT>;
#[doc = "Register CLOCKGENUPDATELOCKOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCKGENUPDATELOCKOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CLOCKGENUPDATELOCKOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCKGENUPDATELOCKOUT_A {
    #[doc = "all hardware clock configruration are freeze."]
    FREEZE,
    #[doc = "update all clock configuration."]
    ENABLE,
}
impl From<CLOCKGENUPDATELOCKOUT_A> for u32 {
    #[inline(always)]
    fn from(variant: CLOCKGENUPDATELOCKOUT_A) -> Self {
        match variant {
            CLOCKGENUPDATELOCKOUT_A::FREEZE => 0,
            CLOCKGENUPDATELOCKOUT_A::ENABLE => 1,
        }
    }
}
#[doc = "Reader of field `CLOCKGENUPDATELOCKOUT`"]
pub type CLOCKGENUPDATELOCKOUT_R = crate::R<u32, CLOCKGENUPDATELOCKOUT_A>;
impl CLOCKGENUPDATELOCKOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CLOCKGENUPDATELOCKOUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLOCKGENUPDATELOCKOUT_A::FREEZE),
            1 => Val(CLOCKGENUPDATELOCKOUT_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FREEZE`"]
    #[inline(always)]
    pub fn is_freeze(&self) -> bool {
        *self == CLOCKGENUPDATELOCKOUT_A::FREEZE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLOCKGENUPDATELOCKOUT_A::ENABLE
    }
}
#[doc = "Write proxy for field `CLOCKGENUPDATELOCKOUT`"]
pub struct CLOCKGENUPDATELOCKOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKGENUPDATELOCKOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCKGENUPDATELOCKOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "all hardware clock configruration are freeze."]
    #[inline(always)]
    pub fn freeze(self) -> &'a mut W {
        self.variant(CLOCKGENUPDATELOCKOUT_A::FREEZE)
    }
    #[doc = "update all clock configuration."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLOCKGENUPDATELOCKOUT_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Control clock configuration registers access (like xxxDIV, xxxSEL)."]
    #[inline(always)]
    pub fn clockgenupdatelockout(&self) -> CLOCKGENUPDATELOCKOUT_R {
        CLOCKGENUPDATELOCKOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Control clock configuration registers access (like xxxDIV, xxxSEL)."]
    #[inline(always)]
    pub fn clockgenupdatelockout(&mut self) -> CLOCKGENUPDATELOCKOUT_W {
        CLOCKGENUPDATELOCKOUT_W { w: self }
    }
}
