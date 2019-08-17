#[doc = "Reader of register MEMCTRL"]
pub type R = crate::R<u32, super::MEMCTRL>;
#[doc = "Writer for register MEMCTRL"]
pub type W = crate::W<u32, super::MEMCTRL>;
#[doc = "Register MEMCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MEMCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `MASTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTER_A {
    #[doc = "Mastering is not used and the normal DMA or Interrupt based model is used with INDATA."]
    NOT_USED,
    #[doc = "Mastering is enabled and DMA and INDATA should not be used."]
    ENABLED,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        match variant {
            MASTER_A::NOT_USED => false,
            MASTER_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `MASTER`"]
pub type MASTER_R = crate::R<bool, MASTER_A>;
impl MASTER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::NOT_USED,
            true => MASTER_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_USED`"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == MASTER_A::NOT_USED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MASTER_A::ENABLED
    }
}
#[doc = "Write proxy for field `MASTER`"]
pub struct MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASTER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mastering is not used and the normal DMA or Interrupt based model is used with INDATA."]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut W {
        self.variant(MASTER_A::NOT_USED)
    }
    #[doc = "Mastering is enabled and DMA and INDATA should not be used."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MASTER_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNT`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Number of 512-bit (128-bit if AES, except 1st block which may include key and IV) blocks to copy starting at MEMADDR. This register will decrement after each block is copied, ending in 0. For Hash, the DIGEST interrupt will occur when it reaches 0. Fro AES, the DIGEST/OUTDATA interrupt will occur on ever block. If a bus error occurs, it will stop with this field set to the block that failed. 0:Done - nothing to process. 1 to 2K: Number of 512-bit (or 128bit) blocks to hash."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W {
        MASTER_W { w: self }
    }
    #[doc = "Bits 16:26 - Number of 512-bit (128-bit if AES, except 1st block which may include key and IV) blocks to copy starting at MEMADDR. This register will decrement after each block is copied, ending in 0. For Hash, the DIGEST interrupt will occur when it reaches 0. Fro AES, the DIGEST/OUTDATA interrupt will occur on ever block. If a bus error occurs, it will stop with this field set to the block that failed. 0:Done - nothing to process. 1 to 2K: Number of 512-bit (or 128bit) blocks to hash."]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
}
