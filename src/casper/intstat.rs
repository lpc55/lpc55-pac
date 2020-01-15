#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Writer for register INTSTAT"]
pub type W = crate::W<u32, super::INTSTAT>;
#[doc = "Register INTSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "If set, interrupt is caused by accelerator being done.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_A {
    #[doc = "0: Not caused by accelerator being done"]
    NOT_CAUSED = 0,
    #[doc = "1: Caused by accelerator being done"]
    CAUSED = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, DONE_A>;
impl DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::NOT_CAUSED,
            true => DONE_A::CAUSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_CAUSED`"]
    #[inline(always)]
    pub fn is_not_caused(&self) -> bool {
        *self == DONE_A::NOT_CAUSED
    }
    #[doc = "Checks if the value of the field is `CAUSED`"]
    #[inline(always)]
    pub fn is_caused(&self) -> bool {
        *self == DONE_A::CAUSED
    }
}
impl R {
    #[doc = "Bit 0 - If set, interrupt is caused by accelerator being done."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {}
