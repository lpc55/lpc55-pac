#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRINCE_BASE_ADDR {
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
pub struct ADDR0_PRGR {
    bits: u8,
}
impl ADDR0_PRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADDR1_PRGR {
    bits: u8,
}
impl ADDR1_PRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADDR2_PRGR {
    bits: u8,
}
impl ADDR2_PRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOCK_REG0R {
    bits: u8,
}
impl LOCK_REG0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOCK_REG1R {
    bits: u8,
}
impl LOCK_REG1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOCK_REG2R {
    bits: u8,
}
impl LOCK_REG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REG0_ERASE_CHECK_ENR {
    bits: u8,
}
impl REG0_ERASE_CHECK_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REG1_ERASE_CHECK_ENR {
    bits: u8,
}
impl REG1_ERASE_CHECK_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct REG2_ERASE_CHECK_ENR {
    bits: u8,
}
impl REG2_ERASE_CHECK_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ADDR0_PRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDR0_PRGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADDR1_PRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDR1_PRGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADDR2_PRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDR2_PRGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_REG0W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REG0W<'a> {
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
pub struct _LOCK_REG1W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REG1W<'a> {
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
pub struct _LOCK_REG2W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REG2W<'a> {
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
pub struct _REG0_ERASE_CHECK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _REG0_ERASE_CHECK_ENW<'a> {
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
pub struct _REG1_ERASE_CHECK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _REG1_ERASE_CHECK_ENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REG2_ERASE_CHECK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _REG2_ERASE_CHECK_ENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - Programmable portion of the base address of region 0."]
    #[inline]
    pub fn addr0_prg(&self) -> ADDR0_PRGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADDR0_PRGR { bits }
    }
    #[doc = "Bits 4:7 - Programmable portion of the base address of region 1."]
    #[inline]
    pub fn addr1_prg(&self) -> ADDR1_PRGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADDR1_PRGR { bits }
    }
    #[doc = "Bits 8:11 - Programmable portion of the base address of region 2."]
    #[inline]
    pub fn addr2_prg(&self) -> ADDR2_PRGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADDR2_PRGR { bits }
    }
    #[doc = "Bits 16:17 - Lock PRINCE region0 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline]
    pub fn lock_reg0(&self) -> LOCK_REG0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOCK_REG0R { bits }
    }
    #[doc = "Bits 18:19 - Lock PRINCE region1 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline]
    pub fn lock_reg1(&self) -> LOCK_REG1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOCK_REG1R { bits }
    }
    #[doc = "Bits 20:21 - Lock PRINCE region2 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline]
    pub fn lock_reg2(&self) -> LOCK_REG2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOCK_REG2R { bits }
    }
    #[doc = "Bits 24:25 - For PRINCE region0 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline]
    pub fn reg0_erase_check_en(&self) -> REG0_ERASE_CHECK_ENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REG0_ERASE_CHECK_ENR { bits }
    }
    #[doc = "Bits 26:27 - For PRINCE region1 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline]
    pub fn reg1_erase_check_en(&self) -> REG1_ERASE_CHECK_ENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REG1_ERASE_CHECK_ENR { bits }
    }
    #[doc = "Bits 28:29 - For PRINCE region2 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline]
    pub fn reg2_erase_check_en(&self) -> REG2_ERASE_CHECK_ENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REG2_ERASE_CHECK_ENR { bits }
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
    #[doc = "Bits 0:3 - Programmable portion of the base address of region 0."]
    #[inline]
    pub fn addr0_prg(&mut self) -> _ADDR0_PRGW {
        _ADDR0_PRGW { w: self }
    }
    #[doc = "Bits 4:7 - Programmable portion of the base address of region 1."]
    #[inline]
    pub fn addr1_prg(&mut self) -> _ADDR1_PRGW {
        _ADDR1_PRGW { w: self }
    }
    #[doc = "Bits 8:11 - Programmable portion of the base address of region 2."]
    #[inline]
    pub fn addr2_prg(&mut self) -> _ADDR2_PRGW {
        _ADDR2_PRGW { w: self }
    }
    #[doc = "Bits 16:17 - Lock PRINCE region0 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline]
    pub fn lock_reg0(&mut self) -> _LOCK_REG0W {
        _LOCK_REG0W { w: self }
    }
    #[doc = "Bits 18:19 - Lock PRINCE region1 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline]
    pub fn lock_reg1(&mut self) -> _LOCK_REG1W {
        _LOCK_REG1W { w: self }
    }
    #[doc = "Bits 20:21 - Lock PRINCE region2 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline]
    pub fn lock_reg2(&mut self) -> _LOCK_REG2W {
        _LOCK_REG2W { w: self }
    }
    #[doc = "Bits 24:25 - For PRINCE region0 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline]
    pub fn reg0_erase_check_en(&mut self) -> _REG0_ERASE_CHECK_ENW {
        _REG0_ERASE_CHECK_ENW { w: self }
    }
    #[doc = "Bits 26:27 - For PRINCE region1 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline]
    pub fn reg1_erase_check_en(&mut self) -> _REG1_ERASE_CHECK_ENW {
        _REG1_ERASE_CHECK_ENW { w: self }
    }
    #[doc = "Bits 28:29 - For PRINCE region2 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline]
    pub fn reg2_erase_check_en(&mut self) -> _REG2_ERASE_CHECK_ENW {
        _REG2_ERASE_CHECK_ENW { w: self }
    }
}
