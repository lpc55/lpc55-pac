#[doc = "Reader of register INT_STATUS"]
pub type R = crate::R<u32, super::INT_STATUS>;
#[doc = "Writer for register INT_STATUS"]
pub type W = crate::W<u32, super::INT_STATUS>;
#[doc = "Register INT_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_STATUS {
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
    #[doc = "Bit 0 - This status bit is set if execution of a (legal) command failed."]
    #[inline(always)]
    pub fn fail(&self) -> FAIL_R {
        FAIL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This status bit is set if execution of an illegal command is detected."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This status bit is set at the end of command execution."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This status bit is set if, during a memory read operation (either a user-requested read, or a speculative read, or reads performed by a controller command), a correctable or uncorrectable error is detected by ECC decoding logic."]
    #[inline(always)]
    pub fn ecc_err(&self) -> ECC_ERR_R {
        ECC_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {}
