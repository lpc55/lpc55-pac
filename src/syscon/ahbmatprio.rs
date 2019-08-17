#[doc = "Reader of register AHBMATPRIO"]
pub type R = crate::R<u32, super::AHBMATPRIO>;
#[doc = "Writer for register AHBMATPRIO"]
pub type W = crate::W<u32, super::AHBMATPRIO>;
#[doc = "Register AHBMATPRIO `reset()`'s with value 0"]
impl crate::ResetValue for super::AHBMATPRIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_TEAL_CBUS`"]
pub type PRI_TEAL_CBUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_TEAL_CBUS`"]
pub struct PRI_TEAL_CBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_TEAL_CBUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PRI_TEAL_SBUS`"]
pub type PRI_TEAL_SBUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_TEAL_SBUS`"]
pub struct PRI_TEAL_SBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_TEAL_SBUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `PRI_UTEAL_CBUS`"]
pub type PRI_UTEAL_CBUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_UTEAL_CBUS`"]
pub struct PRI_UTEAL_CBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_UTEAL_CBUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `PRI_UTEAL_SBUS`"]
pub type PRI_UTEAL_SBUS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_UTEAL_SBUS`"]
pub struct PRI_UTEAL_SBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_UTEAL_SBUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PRI_USB_FS`"]
pub type PRI_USB_FS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_USB_FS`"]
pub struct PRI_USB_FS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_USB_FS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRI_SDMA0`"]
pub type PRI_SDMA0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_SDMA0`"]
pub struct PRI_SDMA0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SDMA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `PRI_EZH_B_D`"]
pub type PRI_EZH_B_D_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_EZH_B_D`"]
pub struct PRI_EZH_B_D_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_EZH_B_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `PRI_EZH_B_I`"]
pub type PRI_EZH_B_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_EZH_B_I`"]
pub struct PRI_EZH_B_I_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_EZH_B_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `PRI_SDIO`"]
pub type PRI_SDIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_SDIO`"]
pub struct PRI_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `PRI_PQ`"]
pub type PRI_PQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_PQ`"]
pub struct PRI_PQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_PQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `PRI_SHA2`"]
pub type PRI_SHA2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_SHA2`"]
pub struct PRI_SHA2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SHA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `PRI_USB_HS`"]
pub type PRI_USB_HS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_USB_HS`"]
pub struct PRI_USB_HS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_USB_HS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `PRI_SDMA1`"]
pub type PRI_SDMA1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_SDMA1`"]
pub struct PRI_SDMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SDMA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Teal C-AHB bus."]
    #[inline(always)]
    pub fn pri_teal_cbus(&self) -> PRI_TEAL_CBUS_R {
        PRI_TEAL_CBUS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Teal S-AHB bus."]
    #[inline(always)]
    pub fn pri_teal_sbus(&self) -> PRI_TEAL_SBUS_R {
        PRI_TEAL_SBUS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Micro Teal C-AHB bus."]
    #[inline(always)]
    pub fn pri_uteal_cbus(&self) -> PRI_UTEAL_CBUS_R {
        PRI_UTEAL_CBUS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Micro Teal S-AHB bus."]
    #[inline(always)]
    pub fn pri_uteal_sbus(&self) -> PRI_UTEAL_SBUS_R {
        PRI_UTEAL_SBUS_R::new(((self.bits >> 6) & 0x03) as u8)
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
    #[doc = "Bits 12:13 - EZH B data bus."]
    #[inline(always)]
    pub fn pri_ezh_b_d(&self) -> PRI_EZH_B_D_R {
        PRI_EZH_B_D_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - EZH B instruction bus."]
    #[inline(always)]
    pub fn pri_ezh_b_i(&self) -> PRI_EZH_B_I_R {
        PRI_EZH_B_I_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - SDIO."]
    #[inline(always)]
    pub fn pri_sdio(&self) -> PRI_SDIO_R {
        PRI_SDIO_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - PQ (Teal HW Accelerator)."]
    #[inline(always)]
    pub fn pri_pq(&self) -> PRI_PQ_R {
        PRI_PQ_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - SHA-2."]
    #[inline(always)]
    pub fn pri_sha2(&self) -> PRI_SHA2_R {
        PRI_SHA2_R::new(((self.bits >> 20) & 0x03) as u8)
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
    #[doc = "Bits 0:1 - Teal C-AHB bus."]
    #[inline(always)]
    pub fn pri_teal_cbus(&mut self) -> PRI_TEAL_CBUS_W {
        PRI_TEAL_CBUS_W { w: self }
    }
    #[doc = "Bits 2:3 - Teal S-AHB bus."]
    #[inline(always)]
    pub fn pri_teal_sbus(&mut self) -> PRI_TEAL_SBUS_W {
        PRI_TEAL_SBUS_W { w: self }
    }
    #[doc = "Bits 4:5 - Micro Teal C-AHB bus."]
    #[inline(always)]
    pub fn pri_uteal_cbus(&mut self) -> PRI_UTEAL_CBUS_W {
        PRI_UTEAL_CBUS_W { w: self }
    }
    #[doc = "Bits 6:7 - Micro Teal S-AHB bus."]
    #[inline(always)]
    pub fn pri_uteal_sbus(&mut self) -> PRI_UTEAL_SBUS_W {
        PRI_UTEAL_SBUS_W { w: self }
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
    #[doc = "Bits 12:13 - EZH B data bus."]
    #[inline(always)]
    pub fn pri_ezh_b_d(&mut self) -> PRI_EZH_B_D_W {
        PRI_EZH_B_D_W { w: self }
    }
    #[doc = "Bits 14:15 - EZH B instruction bus."]
    #[inline(always)]
    pub fn pri_ezh_b_i(&mut self) -> PRI_EZH_B_I_W {
        PRI_EZH_B_I_W { w: self }
    }
    #[doc = "Bits 16:17 - SDIO."]
    #[inline(always)]
    pub fn pri_sdio(&mut self) -> PRI_SDIO_W {
        PRI_SDIO_W { w: self }
    }
    #[doc = "Bits 18:19 - PQ (Teal HW Accelerator)."]
    #[inline(always)]
    pub fn pri_pq(&mut self) -> PRI_PQ_W {
        PRI_PQ_W { w: self }
    }
    #[doc = "Bits 20:21 - SHA-2."]
    #[inline(always)]
    pub fn pri_sha2(&mut self) -> PRI_SHA2_W {
        PRI_SHA2_W { w: self }
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
}
