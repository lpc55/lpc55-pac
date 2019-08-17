#[doc = "Reader of register ALLOW"]
pub type R = crate::R<u32, super::ALLOW>;
#[doc = "Writer for register ALLOW"]
pub type W = crate::W<u32, super::ALLOW>;
#[doc = "Register ALLOW `reset()`'s with value 0"]
impl crate::ResetValue for super::ALLOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALLOWENROLL`"]
pub type ALLOWENROLL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALLOWSTART`"]
pub type ALLOWSTART_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALLOWSETKEY`"]
pub type ALLOWSETKEY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALLOWGETKEY`"]
pub type ALLOWGETKEY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Enroll operation is allowed"]
    #[inline(always)]
    pub fn allowenroll(&self) -> ALLOWENROLL_R {
        ALLOWENROLL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start operation is allowed"]
    #[inline(always)]
    pub fn allowstart(&self) -> ALLOWSTART_R {
        ALLOWSTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set Key operations are allowed"]
    #[inline(always)]
    pub fn allowsetkey(&self) -> ALLOWSETKEY_R {
        ALLOWSETKEY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Get Key operation is allowed"]
    #[inline(always)]
    pub fn allowgetkey(&self) -> ALLOWGETKEY_R {
        ALLOWGETKEY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {}
