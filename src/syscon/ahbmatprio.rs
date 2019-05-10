#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHBMATPRIO {
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
#[doc = r" Value of the field"]
pub struct PRI_TEAL_CBUSR {
    bits: u8,
}
impl PRI_TEAL_CBUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_TEAL_SBUSR {
    bits: u8,
}
impl PRI_TEAL_SBUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_UTEAL_CBUSR {
    bits: u8,
}
impl PRI_UTEAL_CBUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_UTEAL_SBUSR {
    bits: u8,
}
impl PRI_UTEAL_SBUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_USB_FSR {
    bits: u8,
}
impl PRI_USB_FSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_SDMA0R {
    bits: u8,
}
impl PRI_SDMA0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_EZH_B_DR {
    bits: u8,
}
impl PRI_EZH_B_DR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_EZH_B_IR {
    bits: u8,
}
impl PRI_EZH_B_IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_SDIOR {
    bits: u8,
}
impl PRI_SDIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_PQR {
    bits: u8,
}
impl PRI_PQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_SHA2R {
    bits: u8,
}
impl PRI_SHA2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_USB_HSR {
    bits: u8,
}
impl PRI_USB_HSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_SDMA1R {
    bits: u8,
}
impl PRI_SDMA1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PRI_TEAL_CBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_TEAL_CBUSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_TEAL_SBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_TEAL_SBUSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_UTEAL_CBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_UTEAL_CBUSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_UTEAL_SBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_UTEAL_SBUSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_USB_FSW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_USB_FSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_SDMA0W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_SDMA0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_EZH_B_DW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_EZH_B_DW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_EZH_B_IW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_EZH_B_IW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_SDIOW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_SDIOW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_PQW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_PQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_SHA2W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_SHA2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_USB_HSW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_USB_HSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRI_SDMA1W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_SDMA1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:1 - Teal C-AHB bus."]
    #[inline]
    pub fn pri_teal_cbus(&self) -> PRI_TEAL_CBUSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_TEAL_CBUSR { bits }
    }
    #[doc = "Bits 2:3 - Teal S-AHB bus."]
    #[inline]
    pub fn pri_teal_sbus(&self) -> PRI_TEAL_SBUSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_TEAL_SBUSR { bits }
    }
    #[doc = "Bits 4:5 - Micro Teal C-AHB bus."]
    #[inline]
    pub fn pri_uteal_cbus(&self) -> PRI_UTEAL_CBUSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_UTEAL_CBUSR { bits }
    }
    #[doc = "Bits 6:7 - Micro Teal S-AHB bus."]
    #[inline]
    pub fn pri_uteal_sbus(&self) -> PRI_UTEAL_SBUSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_UTEAL_SBUSR { bits }
    }
    #[doc = "Bits 8:9 - USB-FS.(USB0)"]
    #[inline]
    pub fn pri_usb_fs(&self) -> PRI_USB_FSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_USB_FSR { bits }
    }
    #[doc = "Bits 10:11 - DMA0 controller priority."]
    #[inline]
    pub fn pri_sdma0(&self) -> PRI_SDMA0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_SDMA0R { bits }
    }
    #[doc = "Bits 12:13 - EZH B data bus."]
    #[inline]
    pub fn pri_ezh_b_d(&self) -> PRI_EZH_B_DR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_EZH_B_DR { bits }
    }
    #[doc = "Bits 14:15 - EZH B instruction bus."]
    #[inline]
    pub fn pri_ezh_b_i(&self) -> PRI_EZH_B_IR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_EZH_B_IR { bits }
    }
    #[doc = "Bits 16:17 - SDIO."]
    #[inline]
    pub fn pri_sdio(&self) -> PRI_SDIOR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_SDIOR { bits }
    }
    #[doc = "Bits 18:19 - PQ (Teal HW Accelerator)."]
    #[inline]
    pub fn pri_pq(&self) -> PRI_PQR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_PQR { bits }
    }
    #[doc = "Bits 20:21 - SHA-2."]
    #[inline]
    pub fn pri_sha2(&self) -> PRI_SHA2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_SHA2R { bits }
    }
    #[doc = "Bits 22:23 - USB-HS.(USB1)"]
    #[inline]
    pub fn pri_usb_hs(&self) -> PRI_USB_HSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_USB_HSR { bits }
    }
    #[doc = "Bits 24:25 - DMA1 controller priority."]
    #[inline]
    pub fn pri_sdma1(&self) -> PRI_SDMA1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_SDMA1R { bits }
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
    #[doc = "Bits 0:1 - Teal C-AHB bus."]
    #[inline]
    pub fn pri_teal_cbus(&mut self) -> _PRI_TEAL_CBUSW {
        _PRI_TEAL_CBUSW { w: self }
    }
    #[doc = "Bits 2:3 - Teal S-AHB bus."]
    #[inline]
    pub fn pri_teal_sbus(&mut self) -> _PRI_TEAL_SBUSW {
        _PRI_TEAL_SBUSW { w: self }
    }
    #[doc = "Bits 4:5 - Micro Teal C-AHB bus."]
    #[inline]
    pub fn pri_uteal_cbus(&mut self) -> _PRI_UTEAL_CBUSW {
        _PRI_UTEAL_CBUSW { w: self }
    }
    #[doc = "Bits 6:7 - Micro Teal S-AHB bus."]
    #[inline]
    pub fn pri_uteal_sbus(&mut self) -> _PRI_UTEAL_SBUSW {
        _PRI_UTEAL_SBUSW { w: self }
    }
    #[doc = "Bits 8:9 - USB-FS.(USB0)"]
    #[inline]
    pub fn pri_usb_fs(&mut self) -> _PRI_USB_FSW {
        _PRI_USB_FSW { w: self }
    }
    #[doc = "Bits 10:11 - DMA0 controller priority."]
    #[inline]
    pub fn pri_sdma0(&mut self) -> _PRI_SDMA0W {
        _PRI_SDMA0W { w: self }
    }
    #[doc = "Bits 12:13 - EZH B data bus."]
    #[inline]
    pub fn pri_ezh_b_d(&mut self) -> _PRI_EZH_B_DW {
        _PRI_EZH_B_DW { w: self }
    }
    #[doc = "Bits 14:15 - EZH B instruction bus."]
    #[inline]
    pub fn pri_ezh_b_i(&mut self) -> _PRI_EZH_B_IW {
        _PRI_EZH_B_IW { w: self }
    }
    #[doc = "Bits 16:17 - SDIO."]
    #[inline]
    pub fn pri_sdio(&mut self) -> _PRI_SDIOW {
        _PRI_SDIOW { w: self }
    }
    #[doc = "Bits 18:19 - PQ (Teal HW Accelerator)."]
    #[inline]
    pub fn pri_pq(&mut self) -> _PRI_PQW {
        _PRI_PQW { w: self }
    }
    #[doc = "Bits 20:21 - SHA-2."]
    #[inline]
    pub fn pri_sha2(&mut self) -> _PRI_SHA2W {
        _PRI_SHA2W { w: self }
    }
    #[doc = "Bits 22:23 - USB-HS.(USB1)"]
    #[inline]
    pub fn pri_usb_hs(&mut self) -> _PRI_USB_HSW {
        _PRI_USB_HSW { w: self }
    }
    #[doc = "Bits 24:25 - DMA1 controller priority."]
    #[inline]
    pub fn pri_sdma1(&mut self) -> _PRI_SDMA1W {
        _PRI_SDMA1W { w: self }
    }
}
