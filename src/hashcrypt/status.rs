#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `WAITING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITINGR {
    #[doc = "Not waiting for data - may be disabled or may be busy. Note that for cryptographic uses, this is not set if IsLast is set nor will it set until at least 1 word is read of the output."]
    NOT_WAITING,
    #[doc = "Waiting for data to be written in (16 words)"]
    WAITING,
}
impl WAITINGR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WAITINGR::NOT_WAITING => false,
            WAITINGR::WAITING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAITINGR {
        match value {
            false => WAITINGR::NOT_WAITING,
            true => WAITINGR::WAITING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_WAITING`"]
    #[inline]
    pub fn is_not_waiting(&self) -> bool {
        *self == WAITINGR::NOT_WAITING
    }
    #[doc = "Checks if the value of the field is `WAITING`"]
    #[inline]
    pub fn is_waiting(&self) -> bool {
        *self == WAITINGR::WAITING
    }
}
#[doc = "Possible values of the field `DIGEST_aka_OUTDATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGEST_AKA_OUTDATAR {
    #[doc = "No Digest is ready"]
    NOT_READY,
    #[doc = "Digest is ready. Application may read it or may write more data"]
    READY,
}
impl DIGEST_AKA_OUTDATAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DIGEST_AKA_OUTDATAR::NOT_READY => false,
            DIGEST_AKA_OUTDATAR::READY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIGEST_AKA_OUTDATAR {
        match value {
            false => DIGEST_AKA_OUTDATAR::NOT_READY,
            true => DIGEST_AKA_OUTDATAR::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline]
    pub fn is_not_ready(&self) -> bool {
        *self == DIGEST_AKA_OUTDATAR::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == DIGEST_AKA_OUTDATAR::READY
    }
}
#[doc = "Possible values of the field `ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORR {
    #[doc = "No error."]
    NO_ERROR,
    #[doc = "An error occurred since last cleared (written 1 to clear)."]
    ERROR,
}
impl ERRORR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ERRORR::NO_ERROR => false,
            ERRORR::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRORR {
        match value {
            false => ERRORR::NO_ERROR,
            true => ERRORR::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == ERRORR::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == ERRORR::ERROR
    }
}
#[doc = "Possible values of the field `NEEDKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEEDKEYR {
    #[doc = "No Key is needed and writes will not be treated as Key"]
    NOT_NEED,
    #[doc = "Key is needed and INDATA/ALIAS will be accepted as Key. Will also set WAITING."]
    NEED,
}
impl NEEDKEYR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            NEEDKEYR::NOT_NEED => false,
            NEEDKEYR::NEED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NEEDKEYR {
        match value {
            false => NEEDKEYR::NOT_NEED,
            true => NEEDKEYR::NEED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_NEED`"]
    #[inline]
    pub fn is_not_need(&self) -> bool {
        *self == NEEDKEYR::NOT_NEED
    }
    #[doc = "Checks if the value of the field is `NEED`"]
    #[inline]
    pub fn is_need(&self) -> bool {
        *self == NEEDKEYR::NEED
    }
}
#[doc = "Possible values of the field `NEEDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEEDIVR {
    #[doc = "No IV/Nonce is needed, either because written already or because not needed."]
    NOT_NEED,
    #[doc = "IV/Nonce is needed and INDATA/ALIAS will be accepted as IV/Nonce. Will also set WAITING."]
    NEED,
}
impl NEEDIVR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            NEEDIVR::NOT_NEED => false,
            NEEDIVR::NEED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NEEDIVR {
        match value {
            false => NEEDIVR::NOT_NEED,
            true => NEEDIVR::NEED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_NEED`"]
    #[inline]
    pub fn is_not_need(&self) -> bool {
        *self == NEEDIVR::NOT_NEED
    }
    #[doc = "Checks if the value of the field is `NEED`"]
    #[inline]
    pub fn is_need(&self) -> bool {
        *self == NEEDIVR::NEED
    }
}
#[doc = r" Value of the field"]
pub struct ICBIDXR {
    bits: u8,
}
impl ICBIDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ERROR`"]
pub enum ERRORW {
    #[doc = "No error."]
    NO_ERROR,
    #[doc = "An error occurred since last cleared (written 1 to clear)."]
    ERROR,
}
impl ERRORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRORW::NO_ERROR => false,
            ERRORW::ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error."]
    #[inline]
    pub fn no_error(self) -> &'a mut W {
        self.variant(ERRORW::NO_ERROR)
    }
    #[doc = "An error occurred since last cleared (written 1 to clear)."]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(ERRORW::ERROR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - If 1, the block is waiting for more data to process."]
    #[inline]
    pub fn waiting(&self) -> WAITINGR {
        WAITINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - For Hash, if 1 then a DIGEST is ready and waiting and there is no active next block already started. For Cryptographic uses, this will be set for each block processed, indicating OUTDATA (and OUTDATA2 if larger output) contains the next value to read out. This is cleared when any data is written, when New is written, for Cryptographic uses when the last word is read out, or when the block is disabled."]
    #[inline]
    pub fn digest_aka_outdata(&self) -> DIGEST_AKA_OUTDATAR {
        DIGEST_AKA_OUTDATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
    #[inline]
    pub fn error(&self) -> ERRORR {
        ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Indicates the block wants the key to be written in (set along with WAITING)"]
    #[inline]
    pub fn needkey(&self) -> NEEDKEYR {
        NEEDKEYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Indicates the block wants an IV/NONE to be written in (set along with WAITING)"]
    #[inline]
    pub fn neediv(&self) -> NEEDIVR {
        NEEDIVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:21 - If ICB-AES is selected, then reads as the ICB index count based on ICBSTRM (from CRYPTCFG). That is, if 3 bits of ICBSTRM, then this will count from 0 to 7 and then back to 0. On 0, it has to compute the full ICB, quicker when not 0."]
    #[inline]
    pub fn icbidx(&self) -> ICBIDXR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ICBIDXR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - If 1, an error occurred. For normal uses, this is due to an attempted overrun: INDATA was written when it was not appropriate. For Master cases, this is an AHB bus error; the COUNT field will indicate which block it was on."]
    #[inline]
    pub fn error(&mut self) -> _ERRORW {
        _ERRORW { w: self }
    }
}
