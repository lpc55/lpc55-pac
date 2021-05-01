#[doc = "Register `IDXBLK_L_DP` reader"]
pub struct R(crate::R<IDXBLK_L_DP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDXBLK_L_DP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IDXBLK_L_DP_SPEC>> for R {
    fn from(reader: crate::R<IDXBLK_L_DP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDXBLK_L_DP` writer"]
pub struct W(crate::W<IDXBLK_L_DP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDXBLK_L_DP_SPEC>;
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
impl core::convert::From<crate::W<IDXBLK_L_DP_SPEC>> for W {
    fn from(writer: crate::W<IDXBLK_L_DP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDX1` reader - Use to block PUF index 1"]
pub struct IDX1_R(crate::FieldReader<u8, u8>);
impl IDX1_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX1` writer - Use to block PUF index 1"]
pub struct IDX1_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `IDX2` reader - Use to block PUF index 2"]
pub struct IDX2_R(crate::FieldReader<u8, u8>);
impl IDX2_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX2` writer - Use to block PUF index 2"]
pub struct IDX2_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `IDX3` reader - Use to block PUF index 3"]
pub struct IDX3_R(crate::FieldReader<u8, u8>);
impl IDX3_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX3` writer - Use to block PUF index 3"]
pub struct IDX3_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `IDX4` reader - Use to block PUF index 4"]
pub struct IDX4_R(crate::FieldReader<u8, u8>);
impl IDX4_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX4` writer - Use to block PUF index 4"]
pub struct IDX4_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `IDX5` reader - Use to block PUF index 5"]
pub struct IDX5_R(crate::FieldReader<u8, u8>);
impl IDX5_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX5` writer - Use to block PUF index 5"]
pub struct IDX5_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `IDX6` reader - Use to block PUF index 6"]
pub struct IDX6_R(crate::FieldReader<u8, u8>);
impl IDX6_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX6` writer - Use to block PUF index 6"]
pub struct IDX6_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `IDX7` reader - Use to block PUF index 7"]
pub struct IDX7_R(crate::FieldReader<u8, u8>);
impl IDX7_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDX7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDX7` writer - Use to block PUF index 7"]
pub struct IDX7_W<'a> {
    w: &'a mut W,
}
impl<'a> IDX7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3 - Use to block PUF index 1"]
    #[inline(always)]
    pub fn idx1(&self) -> IDX1_R {
        IDX1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Use to block PUF index 2"]
    #[inline(always)]
    pub fn idx2(&self) -> IDX2_R {
        IDX2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Use to block PUF index 3"]
    #[inline(always)]
    pub fn idx3(&self) -> IDX3_R {
        IDX3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Use to block PUF index 4"]
    #[inline(always)]
    pub fn idx4(&self) -> IDX4_R {
        IDX4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Use to block PUF index 5"]
    #[inline(always)]
    pub fn idx5(&self) -> IDX5_R {
        IDX5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Use to block PUF index 6"]
    #[inline(always)]
    pub fn idx6(&self) -> IDX6_R {
        IDX6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Use to block PUF index 7"]
    #[inline(always)]
    pub fn idx7(&self) -> IDX7_R {
        IDX7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - Use to block PUF index 1"]
    #[inline(always)]
    pub fn idx1(&mut self) -> IDX1_W {
        IDX1_W { w: self }
    }
    #[doc = "Bits 4:5 - Use to block PUF index 2"]
    #[inline(always)]
    pub fn idx2(&mut self) -> IDX2_W {
        IDX2_W { w: self }
    }
    #[doc = "Bits 6:7 - Use to block PUF index 3"]
    #[inline(always)]
    pub fn idx3(&mut self) -> IDX3_W {
        IDX3_W { w: self }
    }
    #[doc = "Bits 8:9 - Use to block PUF index 4"]
    #[inline(always)]
    pub fn idx4(&mut self) -> IDX4_W {
        IDX4_W { w: self }
    }
    #[doc = "Bits 10:11 - Use to block PUF index 5"]
    #[inline(always)]
    pub fn idx5(&mut self) -> IDX5_W {
        IDX5_W { w: self }
    }
    #[doc = "Bits 12:13 - Use to block PUF index 6"]
    #[inline(always)]
    pub fn idx6(&mut self) -> IDX6_W {
        IDX6_W { w: self }
    }
    #[doc = "Bits 14:15 - Use to block PUF index 7"]
    #[inline(always)]
    pub fn idx7(&mut self) -> IDX7_W {
        IDX7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idxblk_l_dp](index.html) module"]
pub struct IDXBLK_L_DP_SPEC;
impl crate::RegisterSpec for IDXBLK_L_DP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idxblk_l_dp::R](R) reader structure"]
impl crate::Readable for IDXBLK_L_DP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idxblk_l_dp::W](W) writer structure"]
impl crate::Writable for IDXBLK_L_DP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDXBLK_L_DP to value 0xaaaa"]
impl crate::Resettable for IDXBLK_L_DP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xaaaa
    }
}
