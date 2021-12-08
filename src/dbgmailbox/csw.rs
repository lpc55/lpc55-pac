#[doc = "Register `CSW` reader"]
pub struct R(crate::R<CSW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSW` writer"]
pub struct W(crate::W<CSW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSW_SPEC>;
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
impl From<crate::W<CSW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESYNCH_REQ` reader - Debugger will set this bit to 1 to request a resynchronrisation"]
pub struct RESYNCH_REQ_R(crate::FieldReader<bool, bool>);
impl RESYNCH_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESYNCH_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESYNCH_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESYNCH_REQ` writer - Debugger will set this bit to 1 to request a resynchronrisation"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `REQ_PENDING` reader - Request is pending from debugger (i.e unread value in REQUEST)"]
pub struct REQ_PENDING_R(crate::FieldReader<bool, bool>);
impl REQ_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REQ_PENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQ_PENDING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQ_PENDING` writer - Request is pending from debugger (i.e unread value in REQUEST)"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DBG_OR_ERR` reader - Debugger overrun error (previous REQUEST overwritten before being picked up by ROM)"]
pub struct DBG_OR_ERR_R(crate::FieldReader<bool, bool>);
impl DBG_OR_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBG_OR_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_OR_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_OR_ERR` writer - Debugger overrun error (previous REQUEST overwritten before being picked up by ROM)"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `AHB_OR_ERR` reader - AHB overrun Error (Return value overwritten by ROM)"]
pub struct AHB_OR_ERR_R(crate::FieldReader<bool, bool>);
impl AHB_OR_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB_OR_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_OR_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_OR_ERR` writer - AHB overrun Error (Return value overwritten by ROM)"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SOFT_RESET` reader - Soft Reset for DM (write-only from AHB, not readable and selfclearing). A write to this bit will cause a soft reset for DM."]
pub struct SOFT_RESET_R(crate::FieldReader<bool, bool>);
impl SOFT_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFT_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFT_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFT_RESET` writer - Soft Reset for DM (write-only from AHB, not readable and selfclearing). A write to this bit will cause a soft reset for DM."]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CHIP_RESET_REQ` writer - Write only bit. Once written will cause the chip to reset (note that the DM is not reset by this reset as it is only resettable by a SOFT reset or a POR/BOD event)"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csw](index.html) module"]
pub struct CSW_SPEC;
impl crate::RegisterSpec for CSW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csw::R](R) reader structure"]
impl crate::Readable for CSW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csw::W](W) writer structure"]
impl crate::Writable for CSW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSW to value 0"]
impl crate::Resettable for CSW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
