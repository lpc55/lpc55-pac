#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "If 1, the block is waiting for more data to process.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITING_A {
    #[doc = "0: Not waiting for data - may be disabled or may be busy. Note that for cryptographic uses, this is not set if IsLast is set nor will it set until at least 1 word is read of the output."]
    NOT_WAITING = 0,
    #[doc = "1: Waiting for data to be written in (16 words)"]
    WAITING = 1,
}
impl From<WAITING_A> for bool {
    #[inline(always)]
    fn from(variant: WAITING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITING` reader - If 1, the block is waiting for more data to process."]
pub struct WAITING_R(crate::FieldReader<bool, WAITING_A>);
impl WAITING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAITING_R(crate::FieldReader::new(bits))
    }
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
        **self == WAITING_A::NOT_WAITING
    }
    #[doc = "Checks if the value of the field is `WAITING`"]
    #[inline(always)]
    pub fn is_waiting(&self) -> bool {
        **self == WAITING_A::WAITING
    }
}
impl core::ops::Deref for WAITING_R {
    type Target = crate::FieldReader<bool, WAITING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "For Hash, if 1 then a DIGEST is ready and waiting and there is no active next block already started. For Cryptographic uses, this will be set for each block processed, indicating OUTDATA (and OUTDATA2 if larger output) contains the next value to read out. This is cleared when any data is written, when New is written, for Cryptographic uses when the last word is read out, or when the block is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGEST_A {
    #[doc = "0: No Digest is ready"]
    NOT_READY = 0,
    #[doc = "1: Digest is ready. Application may read it or may write more data"]
    READY = 1,
}
impl From<DIGEST_A> for bool {
    #[inline(always)]
    fn from(variant: DIGEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIGEST` reader - For Hash, if 1 then a DIGEST is ready and waiting and there is no active next block already started. For Cryptographic uses, this will be set for each block processed, indicating OUTDATA (and OUTDATA2 if larger output) contains the next value to read out. This is cleared when any data is written, when New is written, for Cryptographic uses when the last word is read out, or when the block is disabled."]
pub struct DIGEST_R(crate::FieldReader<bool, DIGEST_A>);
impl DIGEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIGEST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIGEST_A {
        match self.bits {
            false => DIGEST_A::NOT_READY,
            true => DIGEST_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == DIGEST_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == DIGEST_A::READY
    }
}
impl core::ops::Deref for DIGEST_R {
    type Target = crate::FieldReader<bool, DIGEST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: No error."]
    NO_ERROR = 0,
    #[doc = "1: An error occurred since last cleared (written 1 to clear)."]
    ERROR = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
pub struct ERROR_R(crate::FieldReader<bool, ERROR_A>);
impl ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
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
        **self == ERROR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == ERROR_A::ERROR
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, ERROR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR` writer - If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Indicates the block wants the key to be written in (set along with WAITING)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEEDKEY_A {
    #[doc = "0: No Key is needed and writes will not be treated as Key"]
    NOT_NEED = 0,
    #[doc = "1: Key is needed and INDATA/ALIAS will be accepted as Key. Will also set WAITING."]
    NEED = 1,
}
impl From<NEEDKEY_A> for bool {
    #[inline(always)]
    fn from(variant: NEEDKEY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEEDKEY` reader - Indicates the block wants the key to be written in (set along with WAITING)"]
pub struct NEEDKEY_R(crate::FieldReader<bool, NEEDKEY_A>);
impl NEEDKEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NEEDKEY_R(crate::FieldReader::new(bits))
    }
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
        **self == NEEDKEY_A::NOT_NEED
    }
    #[doc = "Checks if the value of the field is `NEED`"]
    #[inline(always)]
    pub fn is_need(&self) -> bool {
        **self == NEEDKEY_A::NEED
    }
}
impl core::ops::Deref for NEEDKEY_R {
    type Target = crate::FieldReader<bool, NEEDKEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Indicates the block wants an IV/NONE to be written in (set along with WAITING)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEEDIV_A {
    #[doc = "0: No IV/Nonce is needed, either because written already or because not needed."]
    NOT_NEED = 0,
    #[doc = "1: IV/Nonce is needed and INDATA/ALIAS will be accepted as IV/Nonce. Will also set WAITING."]
    NEED = 1,
}
impl From<NEEDIV_A> for bool {
    #[inline(always)]
    fn from(variant: NEEDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEEDIV` reader - Indicates the block wants an IV/NONE to be written in (set along with WAITING)"]
pub struct NEEDIV_R(crate::FieldReader<bool, NEEDIV_A>);
impl NEEDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NEEDIV_R(crate::FieldReader::new(bits))
    }
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
        **self == NEEDIV_A::NOT_NEED
    }
    #[doc = "Checks if the value of the field is `NEED`"]
    #[inline(always)]
    pub fn is_need(&self) -> bool {
        **self == NEEDIV_A::NEED
    }
}
impl core::ops::Deref for NEEDIV_R {
    type Target = crate::FieldReader<bool, NEEDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICBIDX` reader - If ICB-AES is selected, then reads as the ICB index count based on ICBSTRM (from CRYPTCFG). That is, if 3 bits of ICBSTRM, then this will count from 0 to 7 and then back to 0. On 0, it has to compute the full ICB, quicker when not 0."]
pub struct ICBIDX_R(crate::FieldReader<u8, u8>);
impl ICBIDX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ICBIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICBIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - If 1, the block is waiting for more data to process."]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - For Hash, if 1 then a DIGEST is ready and waiting and there is no active next block already started. For Cryptographic uses, this will be set for each block processed, indicating OUTDATA (and OUTDATA2 if larger output) contains the next value to read out. This is cleared when any data is written, when New is written, for Cryptographic uses when the last word is read out, or when the block is disabled."]
    #[inline(always)]
    pub fn digest(&self) -> DIGEST_R {
        DIGEST_R::new(((self.bits >> 1) & 0x01) != 0)
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicates status of Hash peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
