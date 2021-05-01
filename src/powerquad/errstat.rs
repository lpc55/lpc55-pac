#[doc = "Register `ERRSTAT` reader"]
pub struct R(crate::R<ERRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ERRSTAT_SPEC>> for R {
    fn from(reader: crate::R<ERRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRSTAT` writer"]
pub struct W(crate::W<ERRSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRSTAT_SPEC>;
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
impl core::convert::From<crate::W<ERRSTAT_SPEC>> for W {
    fn from(writer: crate::W<ERRSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVERFLOW` reader - overflow"]
pub struct OVERFLOW_R(crate::FieldReader<bool, bool>);
impl OVERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERFLOW` writer - overflow"]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
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
#[doc = "Field `NAN` reader - nan"]
pub struct NAN_R(crate::FieldReader<bool, bool>);
impl NAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAN` writer - nan"]
pub struct NAN_W<'a> {
    w: &'a mut W,
}
impl<'a> NAN_W<'a> {
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
#[doc = "Field `FIXEDOVERFLOW` reader - fixed_pt_overflow"]
pub struct FIXEDOVERFLOW_R(crate::FieldReader<bool, bool>);
impl FIXEDOVERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIXEDOVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIXEDOVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIXEDOVERFLOW` writer - fixed_pt_overflow"]
pub struct FIXEDOVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXEDOVERFLOW_W<'a> {
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
#[doc = "Field `UNDERFLOW` reader - underflow"]
pub struct UNDERFLOW_R(crate::FieldReader<bool, bool>);
impl UNDERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNDERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDERFLOW` writer - underflow"]
pub struct UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFLOW_W<'a> {
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
#[doc = "Field `BUSERROR` reader - bus_error"]
pub struct BUSERROR_R(crate::FieldReader<bool, bool>);
impl BUSERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSERROR` writer - bus_error"]
pub struct BUSERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSERROR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - overflow"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - nan"]
    #[inline(always)]
    pub fn nan(&self) -> NAN_R {
        NAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - fixed_pt_overflow"]
    #[inline(always)]
    pub fn fixedoverflow(&self) -> FIXEDOVERFLOW_R {
        FIXEDOVERFLOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - underflow"]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - bus_error"]
    #[inline(always)]
    pub fn buserror(&self) -> BUSERROR_R {
        BUSERROR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - overflow"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 1 - nan"]
    #[inline(always)]
    pub fn nan(&mut self) -> NAN_W {
        NAN_W { w: self }
    }
    #[doc = "Bit 2 - fixed_pt_overflow"]
    #[inline(always)]
    pub fn fixedoverflow(&mut self) -> FIXEDOVERFLOW_W {
        FIXEDOVERFLOW_W { w: self }
    }
    #[doc = "Bit 3 - underflow"]
    #[inline(always)]
    pub fn underflow(&mut self) -> UNDERFLOW_W {
        UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 4 - bus_error"]
    #[inline(always)]
    pub fn buserror(&mut self) -> BUSERROR_W {
        BUSERROR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read/Write register where error statuses are captured (sticky)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errstat](index.html) module"]
pub struct ERRSTAT_SPEC;
impl crate::RegisterSpec for ERRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errstat::R](R) reader structure"]
impl crate::Readable for ERRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errstat::W](W) writer structure"]
impl crate::Writable for ERRSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERRSTAT to value 0"]
impl crate::Resettable for ERRSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
