#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `WAITING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITING_A {
    #[doc = "Not waiting for data - may be disabled or may be busy. Note that for cryptographic uses, this is not set if IsLast is set nor will it set until at least 1 word is read of the output."]
    NOT_WAITING,
    #[doc = "Waiting for data to be written in (16 words)"]
    WAITING,
}
impl From<WAITING_A> for bool {
    #[inline(always)]
    fn from(variant: WAITING_A) -> Self {
        match variant {
            WAITING_A::NOT_WAITING => false,
            WAITING_A::WAITING => true,
        }
    }
}
#[doc = "Reader of field `WAITING`"]
pub type WAITING_R = crate::R<bool, WAITING_A>;
impl WAITING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITING_A {
        match self.bits {
            false => WAITING_A::NOT_WAITING,
            true => WAITING_A::WAITING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_WAITING`"]
    #[inline(always)]
    pub fn is_not_waiting(&self) -> bool {
        *self == WAITING_A::NOT_WAITING
    }
    #[doc = "Checks if the value of the field is `WAITING`"]
    #[inline(always)]
    pub fn is_waiting(&self) -> bool {
        *self == WAITING_A::WAITING
    }
}
#[doc = "Possible values of the field `DIGEST_aka_OUTDATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGEST_AKA_OUTDATA_A {
    #[doc = "No Digest is ready"]
    NOT_READY,
    #[doc = "Digest is ready. Application may read it or may write more data"]
    READY,
}
impl From<DIGEST_AKA_OUTDATA_A> for bool {
    #[inline(always)]
    fn from(variant: DIGEST_AKA_OUTDATA_A) -> Self {
        match variant {
            DIGEST_AKA_OUTDATA_A::NOT_READY => false,
            DIGEST_AKA_OUTDATA_A::READY => true,
        }
    }
}
#[doc = "Reader of field `DIGEST_aka_OUTDATA`"]
pub type DIGEST_AKA_OUTDATA_R = crate::R<bool, DIGEST_AKA_OUTDATA_A>;
impl DIGEST_AKA_OUTDATA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIGEST_AKA_OUTDATA_A {
        match self.bits {
            false => DIGEST_AKA_OUTDATA_A::NOT_READY,
            true => DIGEST_AKA_OUTDATA_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == DIGEST_AKA_OUTDATA_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == DIGEST_AKA_OUTDATA_A::READY
    }
}
#[doc = "Possible values of the field `ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "No error."]
    NO_ERROR,
    #[doc = "An error occurred since last cleared (written 1 to clear)."]
    ERROR,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        match variant {
            ERROR_A::NO_ERROR => false,
            ERROR_A::ERROR => true,
        }
    }
}
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, ERROR_A>;
impl ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::NO_ERROR,
            true => ERROR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ERROR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERROR_A::ERROR
    }
}
#[doc = "Write proxy for field `ERROR`"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(ERROR_A::NO_ERROR)
    }
    #[doc = "An error occurred since last cleared (written 1 to clear)."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(ERROR_A::ERROR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `NEEDKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEEDKEY_A {
    #[doc = "No Key is needed and writes will not be treated as Key"]
    NOT_NEED,
    #[doc = "Key is needed and INDATA/ALIAS will be accepted as Key. Will also set WAITING."]
    NEED,
}
impl From<NEEDKEY_A> for bool {
    #[inline(always)]
    fn from(variant: NEEDKEY_A) -> Self {
        match variant {
            NEEDKEY_A::NOT_NEED => false,
            NEEDKEY_A::NEED => true,
        }
    }
}
#[doc = "Reader of field `NEEDKEY`"]
pub type NEEDKEY_R = crate::R<bool, NEEDKEY_A>;
impl NEEDKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEEDKEY_A {
        match self.bits {
            false => NEEDKEY_A::NOT_NEED,
            true => NEEDKEY_A::NEED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_NEED`"]
    #[inline(always)]
    pub fn is_not_need(&self) -> bool {
        *self == NEEDKEY_A::NOT_NEED
    }
    #[doc = "Checks if the value of the field is `NEED`"]
    #[inline(always)]
    pub fn is_need(&self) -> bool {
        *self == NEEDKEY_A::NEED
    }
}
#[doc = "Possible values of the field `NEEDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEEDIV_A {
    #[doc = "No IV/Nonce is needed, either because written already or because not needed."]
    NOT_NEED,
    #[doc = "IV/Nonce is needed and INDATA/ALIAS will be accepted as IV/Nonce. Will also set WAITING."]
    NEED,
}
impl From<NEEDIV_A> for bool {
    #[inline(always)]
    fn from(variant: NEEDIV_A) -> Self {
        match variant {
            NEEDIV_A::NOT_NEED => false,
            NEEDIV_A::NEED => true,
        }
    }
}
#[doc = "Reader of field `NEEDIV`"]
pub type NEEDIV_R = crate::R<bool, NEEDIV_A>;
impl NEEDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEEDIV_A {
        match self.bits {
            false => NEEDIV_A::NOT_NEED,
            true => NEEDIV_A::NEED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_NEED`"]
    #[inline(always)]
    pub fn is_not_need(&self) -> bool {
        *self == NEEDIV_A::NOT_NEED
    }
    #[doc = "Checks if the value of the field is `NEED`"]
    #[inline(always)]
    pub fn is_need(&self) -> bool {
        *self == NEEDIV_A::NEED
    }
}
#[doc = "Reader of field `ICBIDX`"]
pub type ICBIDX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - If 1, the block is waiting for more data to process."]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - For Hash, if 1 then a DIGEST is ready and waiting and there is no active next block already started. For Cryptographic uses, this will be set for each block processed, indicating OUTDATA (and OUTDATA2 if larger output) contains the next value to read out. This is cleared when any data is written, when New is written, for Cryptographic uses when the last word is read out, or when the block is disabled."]
    #[inline(always)]
    pub fn digest_aka_outdata(&self) -> DIGEST_AKA_OUTDATA_R {
        DIGEST_AKA_OUTDATA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates the block wants the key to be written in (set along with WAITING)"]
    #[inline(always)]
    pub fn needkey(&self) -> NEEDKEY_R {
        NEEDKEY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates the block wants an IV/NONE to be written in (set along with WAITING)"]
    #[inline(always)]
    pub fn neediv(&self) -> NEEDIV_R {
        NEEDIV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - If ICB-AES is selected, then reads as the ICB index count based on ICBSTRM (from CRYPTCFG). That is, if 3 bits of ICBSTRM, then this will count from 0 to 7 and then back to 0. On 0, it has to compute the full ICB, quicker when not 0."]
    #[inline(always)]
    pub fn icbidx(&self) -> ICBIDX_R {
        ICBIDX_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
}
