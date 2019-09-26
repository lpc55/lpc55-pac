#[doc = "Reader of register CSW"]
pub type R = crate::R<u32, super::CSW>;
#[doc = "Writer for register CSW"]
pub type W = crate::W<u32, super::CSW>;
#[doc = "Register CSW `reset()`'s with value 0"]
impl crate::ResetValue for super::CSW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESYNCH_REQ`"]
pub type RESYNCH_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESYNCH_REQ`"]
pub struct RESYNCH_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESYNCH_REQ_W<'a> {
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
#[doc = "Reader of field `REQ_PENDING`"]
pub type REQ_PENDING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REQ_PENDING`"]
pub struct REQ_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_PENDING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DBG_OR_ERR`"]
pub type DBG_OR_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_OR_ERR`"]
pub struct DBG_OR_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_OR_ERR_W<'a> {
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
#[doc = "Reader of field `AHB_OR_ERR`"]
pub type AHB_OR_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_OR_ERR`"]
pub struct AHB_OR_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_OR_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SOFT_RESET`"]
pub type SOFT_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFT_RESET`"]
pub struct SOFT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `CHIP_RESET_REQ`"]
pub struct CHIP_RESET_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_RESET_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Debugger will set this bit to 1 to request a resynchronrisation"]
    #[inline(always)]
    pub fn resynch_req(&self) -> RESYNCH_REQ_R {
        RESYNCH_REQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Request is pending from debugger (i.e unread value in REQUEST)"]
    #[inline(always)]
    pub fn req_pending(&self) -> REQ_PENDING_R {
        REQ_PENDING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Debugger overrun error (previous REQUEST overwritten before being picked up by ROM)"]
    #[inline(always)]
    pub fn dbg_or_err(&self) -> DBG_OR_ERR_R {
        DBG_OR_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AHB overrun Error (Return value overwritten by ROM)"]
    #[inline(always)]
    pub fn ahb_or_err(&self) -> AHB_OR_ERR_R {
        AHB_OR_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Soft Reset for DM (write-only from AHB, not readable and selfclearing). A write to this bit will cause a soft reset for DM."]
    #[inline(always)]
    pub fn soft_reset(&self) -> SOFT_RESET_R {
        SOFT_RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debugger will set this bit to 1 to request a resynchronrisation"]
    #[inline(always)]
    pub fn resynch_req(&mut self) -> RESYNCH_REQ_W {
        RESYNCH_REQ_W { w: self }
    }
    #[doc = "Bit 1 - Request is pending from debugger (i.e unread value in REQUEST)"]
    #[inline(always)]
    pub fn req_pending(&mut self) -> REQ_PENDING_W {
        REQ_PENDING_W { w: self }
    }
    #[doc = "Bit 2 - Debugger overrun error (previous REQUEST overwritten before being picked up by ROM)"]
    #[inline(always)]
    pub fn dbg_or_err(&mut self) -> DBG_OR_ERR_W {
        DBG_OR_ERR_W { w: self }
    }
    #[doc = "Bit 3 - AHB overrun Error (Return value overwritten by ROM)"]
    #[inline(always)]
    pub fn ahb_or_err(&mut self) -> AHB_OR_ERR_W {
        AHB_OR_ERR_W { w: self }
    }
    #[doc = "Bit 4 - Soft Reset for DM (write-only from AHB, not readable and selfclearing). A write to this bit will cause a soft reset for DM."]
    #[inline(always)]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W {
        SOFT_RESET_W { w: self }
    }
    #[doc = "Bit 5 - Write only bit. Once written will cause the chip to reset (note that the DM is not reset by this reset as it is only resettable by a SOFT reset or a POR/BOD event)"]
    #[inline(always)]
    pub fn chip_reset_req(&mut self) -> CHIP_RESET_REQ_W {
        CHIP_RESET_REQ_W { w: self }
    }
}
