#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MASTER_SEC_LEVEL {
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
pub struct MCM33CR {
    bits: u8,
}
impl MCM33CR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MCM33SR {
    bits: u8,
}
impl MCM33SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USBFSDR {
    bits: u8,
}
impl USBFSDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SDMA0R {
    bits: u8,
}
impl SDMA0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EZH_DR {
    bits: u8,
}
impl EZH_DR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EZH_IR {
    bits: u8,
}
impl EZH_IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SDIOR {
    bits: u8,
}
impl SDIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PQR {
    bits: u8,
}
impl PQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HASHR {
    bits: u8,
}
impl HASHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USBFSHR {
    bits: u8,
}
impl USBFSHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SDMA1R {
    bits: u8,
}
impl SDMA1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MASTER_SEC_LEVEL_LOCKR {
    bits: u8,
}
impl MASTER_SEC_LEVEL_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _MCM33CW<'a> {
    w: &'a mut W,
}
impl<'a> _MCM33CW<'a> {
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
pub struct _MCM33SW<'a> {
    w: &'a mut W,
}
impl<'a> _MCM33SW<'a> {
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
pub struct _USBFSDW<'a> {
    w: &'a mut W,
}
impl<'a> _USBFSDW<'a> {
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
pub struct _SDMA0W<'a> {
    w: &'a mut W,
}
impl<'a> _SDMA0W<'a> {
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
pub struct _EZH_DW<'a> {
    w: &'a mut W,
}
impl<'a> _EZH_DW<'a> {
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
pub struct _EZH_IW<'a> {
    w: &'a mut W,
}
impl<'a> _EZH_IW<'a> {
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
pub struct _SDIOW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIOW<'a> {
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
pub struct _PQW<'a> {
    w: &'a mut W,
}
impl<'a> _PQW<'a> {
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
pub struct _HASHW<'a> {
    w: &'a mut W,
}
impl<'a> _HASHW<'a> {
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
pub struct _USBFSHW<'a> {
    w: &'a mut W,
}
impl<'a> _USBFSHW<'a> {
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
pub struct _SDMA1W<'a> {
    w: &'a mut W,
}
impl<'a> _SDMA1W<'a> {
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
#[doc = r" Proxy"]
pub struct _MASTER_SEC_LEVEL_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _MASTER_SEC_LEVEL_LOCKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 4:5 - Micro-CM33 (CPU1) Code bus."]
    #[inline]
    pub fn mcm33c(&self) -> MCM33CR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MCM33CR { bits }
    }
    #[doc = "Bits 6:7 - Micro-CM33 (CPU1) System bus."]
    #[inline]
    pub fn mcm33s(&self) -> MCM33SR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MCM33SR { bits }
    }
    #[doc = "Bits 8:9 - USB Full Speed Device."]
    #[inline]
    pub fn usbfsd(&self) -> USBFSDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USBFSDR { bits }
    }
    #[doc = "Bits 10:11 - System DMA 0."]
    #[inline]
    pub fn sdma0(&self) -> SDMA0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SDMA0R { bits }
    }
    #[doc = "Bits 12:13 - EZH Data."]
    #[inline]
    pub fn ezh_d(&self) -> EZH_DR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EZH_DR { bits }
    }
    #[doc = "Bits 14:15 - EZH instruction."]
    #[inline]
    pub fn ezh_i(&self) -> EZH_IR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EZH_IR { bits }
    }
    #[doc = "Bits 16:17 - SDIO."]
    #[inline]
    pub fn sdio(&self) -> SDIOR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SDIOR { bits }
    }
    #[doc = "Bits 18:19 - Power Quad."]
    #[inline]
    pub fn pq(&self) -> PQR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PQR { bits }
    }
    #[doc = "Bits 20:21 - Hash."]
    #[inline]
    pub fn hash(&self) -> HASHR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HASHR { bits }
    }
    #[doc = "Bits 22:23 - USB Full speed Host."]
    #[inline]
    pub fn usbfsh(&self) -> USBFSHR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USBFSHR { bits }
    }
    #[doc = "Bits 24:25 - System DMA 1 security level."]
    #[inline]
    pub fn sdma1(&self) -> SDMA1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SDMA1R { bits }
    }
    #[doc = "Bits 30:31 - master_sec_reg write-lock. When 2'b10, this register can be written. With any other value, this register can't be written."]
    #[inline]
    pub fn master_sec_level_lock(&self) -> MASTER_SEC_LEVEL_LOCKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MASTER_SEC_LEVEL_LOCKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147483648 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 4:5 - Micro-CM33 (CPU1) Code bus."]
    #[inline]
    pub fn mcm33c(&mut self) -> _MCM33CW {
        _MCM33CW { w: self }
    }
    #[doc = "Bits 6:7 - Micro-CM33 (CPU1) System bus."]
    #[inline]
    pub fn mcm33s(&mut self) -> _MCM33SW {
        _MCM33SW { w: self }
    }
    #[doc = "Bits 8:9 - USB Full Speed Device."]
    #[inline]
    pub fn usbfsd(&mut self) -> _USBFSDW {
        _USBFSDW { w: self }
    }
    #[doc = "Bits 10:11 - System DMA 0."]
    #[inline]
    pub fn sdma0(&mut self) -> _SDMA0W {
        _SDMA0W { w: self }
    }
    #[doc = "Bits 12:13 - EZH Data."]
    #[inline]
    pub fn ezh_d(&mut self) -> _EZH_DW {
        _EZH_DW { w: self }
    }
    #[doc = "Bits 14:15 - EZH instruction."]
    #[inline]
    pub fn ezh_i(&mut self) -> _EZH_IW {
        _EZH_IW { w: self }
    }
    #[doc = "Bits 16:17 - SDIO."]
    #[inline]
    pub fn sdio(&mut self) -> _SDIOW {
        _SDIOW { w: self }
    }
    #[doc = "Bits 18:19 - Power Quad."]
    #[inline]
    pub fn pq(&mut self) -> _PQW {
        _PQW { w: self }
    }
    #[doc = "Bits 20:21 - Hash."]
    #[inline]
    pub fn hash(&mut self) -> _HASHW {
        _HASHW { w: self }
    }
    #[doc = "Bits 22:23 - USB Full speed Host."]
    #[inline]
    pub fn usbfsh(&mut self) -> _USBFSHW {
        _USBFSHW { w: self }
    }
    #[doc = "Bits 24:25 - System DMA 1 security level."]
    #[inline]
    pub fn sdma1(&mut self) -> _SDMA1W {
        _SDMA1W { w: self }
    }
    #[doc = "Bits 30:31 - master_sec_reg write-lock. When 2'b10, this register can be written. With any other value, this register can't be written."]
    #[inline]
    pub fn master_sec_level_lock(&mut self) -> _MASTER_SEC_LEVEL_LOCKW {
        _MASTER_SEC_LEVEL_LOCKW { w: self }
    }
}
