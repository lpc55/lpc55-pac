#[doc = "Register `AHBMATPRIO` reader"]
pub struct R(crate::R<AHBMATPRIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBMATPRIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AHBMATPRIO_SPEC>> for R {
    fn from(reader: crate::R<AHBMATPRIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBMATPRIO` writer"]
pub struct W(crate::W<AHBMATPRIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBMATPRIO_SPEC>;
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
impl core::convert::From<crate::W<AHBMATPRIO_SPEC>> for W {
    fn from(writer: crate::W<AHBMATPRIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_CPU0_CBUS` reader - CPU0 C-AHB bus."]
pub struct PRI_CPU0_CBUS_R(crate::FieldReader<u8, u8>);
impl PRI_CPU0_CBUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_CPU0_CBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_CPU0_CBUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_CPU0_CBUS` writer - CPU0 C-AHB bus."]
pub struct PRI_CPU0_CBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_CPU0_CBUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PRI_CPU0_SBUS` reader - CPU0 S-AHB bus."]
pub struct PRI_CPU0_SBUS_R(crate::FieldReader<u8, u8>);
impl PRI_CPU0_SBUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_CPU0_SBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_CPU0_SBUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_CPU0_SBUS` writer - CPU0 S-AHB bus."]
pub struct PRI_CPU0_SBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_CPU0_SBUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `PRI_CPU1_CBUS` reader - CPU1 C-AHB bus."]
pub struct PRI_CPU1_CBUS_R(crate::FieldReader<u8, u8>);
impl PRI_CPU1_CBUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_CPU1_CBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_CPU1_CBUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_CPU1_CBUS` writer - CPU1 C-AHB bus."]
pub struct PRI_CPU1_CBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_CPU1_CBUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `PRI_CPU1_SBUS` reader - CPU1 S-AHB bus."]
pub struct PRI_CPU1_SBUS_R(crate::FieldReader<u8, u8>);
impl PRI_CPU1_SBUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_CPU1_SBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_CPU1_SBUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_CPU1_SBUS` writer - CPU1 S-AHB bus."]
pub struct PRI_CPU1_SBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_CPU1_SBUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `PRI_USB_FS` reader - USB-FS.(USB0)"]
pub struct PRI_USB_FS_R(crate::FieldReader<u8, u8>);
impl PRI_USB_FS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_USB_FS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_USB_FS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_USB_FS` writer - USB-FS.(USB0)"]
pub struct PRI_USB_FS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_USB_FS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `PRI_SDMA0` reader - DMA0 controller priority."]
pub struct PRI_SDMA0_R(crate::FieldReader<u8, u8>);
impl PRI_SDMA0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_SDMA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_SDMA0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_SDMA0` writer - DMA0 controller priority."]
pub struct PRI_SDMA0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SDMA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `PRI_SDIO` reader - SDIO."]
pub struct PRI_SDIO_R(crate::FieldReader<u8, u8>);
impl PRI_SDIO_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_SDIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_SDIO` writer - SDIO."]
pub struct PRI_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `PRI_PQ` reader - PQ (HW Accelerator)."]
pub struct PRI_PQ_R(crate::FieldReader<u8, u8>);
impl PRI_PQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_PQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_PQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_PQ` writer - PQ (HW Accelerator)."]
pub struct PRI_PQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_PQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `PRI_HASH_AES` reader - HASH_AES."]
pub struct PRI_HASH_AES_R(crate::FieldReader<u8, u8>);
impl PRI_HASH_AES_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_HASH_AES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_HASH_AES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_HASH_AES` writer - HASH_AES."]
pub struct PRI_HASH_AES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_HASH_AES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `PRI_USB_HS` reader - USB-HS.(USB1)"]
pub struct PRI_USB_HS_R(crate::FieldReader<u8, u8>);
impl PRI_USB_HS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_USB_HS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_USB_HS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_USB_HS` writer - USB-HS.(USB1)"]
pub struct PRI_USB_HS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_USB_HS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `PRI_SDMA1` reader - DMA1 controller priority."]
pub struct PRI_SDMA1_R(crate::FieldReader<u8, u8>);
impl PRI_SDMA1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_SDMA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_SDMA1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_SDMA1` writer - DMA1 controller priority."]
pub struct PRI_SDMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SDMA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CPU0 C-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu0_cbus(&self) -> PRI_CPU0_CBUS_R {
        PRI_CPU0_CBUS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - CPU0 S-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu0_sbus(&self) -> PRI_CPU0_SBUS_R {
        PRI_CPU0_SBUS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - CPU1 C-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu1_cbus(&self) -> PRI_CPU1_CBUS_R {
        PRI_CPU1_CBUS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - CPU1 S-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu1_sbus(&self) -> PRI_CPU1_SBUS_R {
        PRI_CPU1_SBUS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - USB-FS.(USB0)"]
    #[inline(always)]
    pub fn pri_usb_fs(&self) -> PRI_USB_FS_R {
        PRI_USB_FS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - DMA0 controller priority."]
    #[inline(always)]
    pub fn pri_sdma0(&self) -> PRI_SDMA0_R {
        PRI_SDMA0_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - SDIO."]
    #[inline(always)]
    pub fn pri_sdio(&self) -> PRI_SDIO_R {
        PRI_SDIO_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - PQ (HW Accelerator)."]
    #[inline(always)]
    pub fn pri_pq(&self) -> PRI_PQ_R {
        PRI_PQ_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - HASH_AES."]
    #[inline(always)]
    pub fn pri_hash_aes(&self) -> PRI_HASH_AES_R {
        PRI_HASH_AES_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - USB-HS.(USB1)"]
    #[inline(always)]
    pub fn pri_usb_hs(&self) -> PRI_USB_HS_R {
        PRI_USB_HS_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - DMA1 controller priority."]
    #[inline(always)]
    pub fn pri_sdma1(&self) -> PRI_SDMA1_R {
        PRI_SDMA1_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CPU0 C-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu0_cbus(&mut self) -> PRI_CPU0_CBUS_W {
        PRI_CPU0_CBUS_W { w: self }
    }
    #[doc = "Bits 2:3 - CPU0 S-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu0_sbus(&mut self) -> PRI_CPU0_SBUS_W {
        PRI_CPU0_SBUS_W { w: self }
    }
    #[doc = "Bits 4:5 - CPU1 C-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu1_cbus(&mut self) -> PRI_CPU1_CBUS_W {
        PRI_CPU1_CBUS_W { w: self }
    }
    #[doc = "Bits 6:7 - CPU1 S-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu1_sbus(&mut self) -> PRI_CPU1_SBUS_W {
        PRI_CPU1_SBUS_W { w: self }
    }
    #[doc = "Bits 8:9 - USB-FS.(USB0)"]
    #[inline(always)]
    pub fn pri_usb_fs(&mut self) -> PRI_USB_FS_W {
        PRI_USB_FS_W { w: self }
    }
    #[doc = "Bits 10:11 - DMA0 controller priority."]
    #[inline(always)]
    pub fn pri_sdma0(&mut self) -> PRI_SDMA0_W {
        PRI_SDMA0_W { w: self }
    }
    #[doc = "Bits 16:17 - SDIO."]
    #[inline(always)]
    pub fn pri_sdio(&mut self) -> PRI_SDIO_W {
        PRI_SDIO_W { w: self }
    }
    #[doc = "Bits 18:19 - PQ (HW Accelerator)."]
    #[inline(always)]
    pub fn pri_pq(&mut self) -> PRI_PQ_W {
        PRI_PQ_W { w: self }
    }
    #[doc = "Bits 20:21 - HASH_AES."]
    #[inline(always)]
    pub fn pri_hash_aes(&mut self) -> PRI_HASH_AES_W {
        PRI_HASH_AES_W { w: self }
    }
    #[doc = "Bits 22:23 - USB-HS.(USB1)"]
    #[inline(always)]
    pub fn pri_usb_hs(&mut self) -> PRI_USB_HS_W {
        PRI_USB_HS_W { w: self }
    }
    #[doc = "Bits 24:25 - DMA1 controller priority."]
    #[inline(always)]
    pub fn pri_sdma1(&mut self) -> PRI_SDMA1_W {
        PRI_SDMA1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbmatprio](index.html) module"]
pub struct AHBMATPRIO_SPEC;
impl crate::RegisterSpec for AHBMATPRIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbmatprio::R](R) reader structure"]
impl crate::Readable for AHBMATPRIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbmatprio::W](W) writer structure"]
impl crate::Writable for AHBMATPRIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBMATPRIO to value 0"]
impl crate::Resettable for AHBMATPRIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
