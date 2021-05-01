#[doc = "Register `IDXBLK_H_DP` reader"]
pub struct R(crate::R<IDXBLK_H_DP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDXBLK_H_DP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IDXBLK_H_DP_SPEC>> for R {
    fn from(reader: crate::R<IDXBLK_H_DP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDXBLK_H_DP` writer"]
pub struct W(crate::W<IDXBLK_H_DP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDXBLK_H_DP_SPEC>;
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
impl core::convert::From<crate::W<IDXBLK_H_DP_SPEC>> for W {
    fn from(writer: crate::W<IDXBLK_H_DP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDX8` reader - Use to block PUF index 8"]
pub struct IDX8_R(crate::FieldReader<u8, u8>);
impl IDX8_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX8` writer - Use to block PUF index 8"]
pub struct IDX8_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `IDX9` reader - Use to block PUF index 9"]
pub struct IDX9_R(crate::FieldReader<u8, u8>);
impl IDX9_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX9` writer - Use to block PUF index 9"]
pub struct IDX9_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `IDX10` reader - Use to block PUF index 10"]
pub struct IDX10_R(crate::FieldReader<u8, u8>);
impl IDX10_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX10` writer - Use to block PUF index 10"]
pub struct IDX10_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `IDX11` reader - Use to block PUF index 11"]
pub struct IDX11_R(crate::FieldReader<u8, u8>);
impl IDX11_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX11` writer - Use to block PUF index 11"]
pub struct IDX11_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `IDX12` reader - Use to block PUF index 12"]
pub struct IDX12_R(crate::FieldReader<u8, u8>);
impl IDX12_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX12` writer - Use to block PUF index 12"]
pub struct IDX12_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `IDX13` reader - Use to block PUF index 13"]
pub struct IDX13_R(crate::FieldReader<u8, u8>);
impl IDX13_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX13` writer - Use to block PUF index 13"]
pub struct IDX13_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `IDX14` reader - Use to block PUF index 14"]
pub struct IDX14_R(crate::FieldReader<u8, u8>);
impl IDX14_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX14` writer - Use to block PUF index 14"]
pub struct IDX14_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `IDX15` reader - Use to block PUF index 15"]
pub struct IDX15_R(crate::FieldReader<u8, u8>);
impl IDX15_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX15` writer - Use to block PUF index 15"]
pub struct IDX15_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Use to block PUF index 8"]
    #[inline(always)]
    pub fn idx8(&self) -> IDX8_R {
        IDX8_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Use to block PUF index 9"]
    #[inline(always)]
    pub fn idx9(&self) -> IDX9_R {
        IDX9_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Use to block PUF index 10"]
    #[inline(always)]
    pub fn idx10(&self) -> IDX10_R {
        IDX10_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Use to block PUF index 11"]
    #[inline(always)]
    pub fn idx11(&self) -> IDX11_R {
        IDX11_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Use to block PUF index 12"]
    #[inline(always)]
    pub fn idx12(&self) -> IDX12_R {
        IDX12_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Use to block PUF index 13"]
    #[inline(always)]
    pub fn idx13(&self) -> IDX13_R {
        IDX13_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Use to block PUF index 14"]
    #[inline(always)]
    pub fn idx14(&self) -> IDX14_R {
        IDX14_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Use to block PUF index 15"]
    #[inline(always)]
    pub fn idx15(&self) -> IDX15_R {
        IDX15_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Use to block PUF index 8"]
    #[inline(always)]
    pub fn idx8(&mut self) -> IDX8_W {
        IDX8_W { w: self }
    }
    #[doc = "Bits 2:3 - Use to block PUF index 9"]
    #[inline(always)]
    pub fn idx9(&mut self) -> IDX9_W {
        IDX9_W { w: self }
    }
    #[doc = "Bits 4:5 - Use to block PUF index 10"]
    #[inline(always)]
    pub fn idx10(&mut self) -> IDX10_W {
        IDX10_W { w: self }
    }
    #[doc = "Bits 6:7 - Use to block PUF index 11"]
    #[inline(always)]
    pub fn idx11(&mut self) -> IDX11_W {
        IDX11_W { w: self }
    }
    #[doc = "Bits 8:9 - Use to block PUF index 12"]
    #[inline(always)]
    pub fn idx12(&mut self) -> IDX12_W {
        IDX12_W { w: self }
    }
    #[doc = "Bits 10:11 - Use to block PUF index 13"]
    #[inline(always)]
    pub fn idx13(&mut self) -> IDX13_W {
        IDX13_W { w: self }
    }
    #[doc = "Bits 12:13 - Use to block PUF index 14"]
    #[inline(always)]
    pub fn idx14(&mut self) -> IDX14_W {
        IDX14_W { w: self }
    }
    #[doc = "Bits 14:15 - Use to block PUF index 15"]
    #[inline(always)]
    pub fn idx15(&mut self) -> IDX15_W {
        IDX15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idxblk_h_dp](index.html) module"]
pub struct IDXBLK_H_DP_SPEC;
impl crate::RegisterSpec for IDXBLK_H_DP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idxblk_h_dp::R](R) reader structure"]
impl crate::Readable for IDXBLK_H_DP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idxblk_h_dp::W](W) writer structure"]
impl crate::Writable for IDXBLK_H_DP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDXBLK_H_DP to value 0xaaaa"]
impl crate::Resettable for IDXBLK_H_DP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xaaaa
    }
}
