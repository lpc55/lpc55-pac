#[doc = "Reader of register INT_ENABLE"]
pub type R = crate::R<u32, super::INT_ENABLE>;
#[doc = "Writer for register INT_ENABLE"]
pub type W = crate::W<u32, super::INT_ENABLE>;
#[doc = "Register INT_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FAIL`"]
pub type FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECC_ERR`"]
pub type ECC_ERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If an INT_ENABLE bit is set, an interrupt request will be generated if the corresponding INT_STATUS bit is high."]
    #[inline(always)]
    pub fn ecc_err(&self) -> ECC_ERR_R {
        ECC_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {}
