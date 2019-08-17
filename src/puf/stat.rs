#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0x01"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `busy`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUCCESS`"]
pub type SUCCESS_R = crate::R<bool, bool>;
#[doc = "Reader of field `error`"]
pub type ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEYINREQ`"]
pub type KEYINREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEYOUTAVAIL`"]
pub type KEYOUTAVAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CODEINREQ`"]
pub type CODEINREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `CODEOUTAVAIL`"]
pub type CODEOUTAVAIL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Indicates that operation is in progress"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Last operation was successful"]
    #[inline(always)]
    pub fn success(&self) -> SUCCESS_R {
        SUCCESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Quiddikey is in the Error state and no operations can be performed"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Request for next part of key"]
    #[inline(always)]
    pub fn keyinreq(&self) -> KEYINREQ_R {
        KEYINREQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Next part of key is available"]
    #[inline(always)]
    pub fn keyoutavail(&self) -> KEYOUTAVAIL_R {
        KEYOUTAVAIL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Request for next part of AC/KC"]
    #[inline(always)]
    pub fn codeinreq(&self) -> CODEINREQ_R {
        CODEINREQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Next part of AC/KC is available"]
    #[inline(always)]
    pub fn codeoutavail(&self) -> CODEOUTAVAIL_R {
        CODEOUTAVAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {}
