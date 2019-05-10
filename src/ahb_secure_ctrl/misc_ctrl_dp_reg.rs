#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MISC_CTRL_DP_REG {
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
pub struct WRITE_LOCKR {
    bits: u8,
}
impl WRITE_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENABLE_SECURE_CHECKINGR {
    bits: u8,
}
impl ENABLE_SECURE_CHECKINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENABLE_S_PRIV_CHECKR {
    bits: u8,
}
impl ENABLE_S_PRIV_CHECKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENABLE_NS_PRIV_CHECKR {
    bits: u8,
}
impl ENABLE_NS_PRIV_CHECKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DISABLE_VIOLATION_ABORTR {
    bits: u8,
}
impl DISABLE_VIOLATION_ABORTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DISABLE_SIMPLE_MASTER_STRICT_MODER {
    bits: u8,
}
impl DISABLE_SIMPLE_MASTER_STRICT_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DISABLE_SMART_MASTER_STRICT_MODER {
    bits: u8,
}
impl DISABLE_SMART_MASTER_STRICT_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDAU_ALL_NSR {
    bits: u8,
}
impl IDAU_ALL_NSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _WRITE_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITE_LOCKW<'a> {
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
pub struct _ENABLE_SECURE_CHECKINGW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_SECURE_CHECKINGW<'a> {
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
pub struct _ENABLE_S_PRIV_CHECKW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_S_PRIV_CHECKW<'a> {
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
pub struct _ENABLE_NS_PRIV_CHECKW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_NS_PRIV_CHECKW<'a> {
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
pub struct _DISABLE_VIOLATION_ABORTW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_VIOLATION_ABORTW<'a> {
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
pub struct _DISABLE_SIMPLE_MASTER_STRICT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_SIMPLE_MASTER_STRICT_MODEW<'a> {
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
pub struct _DISABLE_SMART_MASTER_STRICT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_SMART_MASTER_STRICT_MODEW<'a> {
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
pub struct _IDAU_ALL_NSW<'a> {
    w: &'a mut W,
}
impl<'a> _IDAU_ALL_NSW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - write lock"]
    #[inline]
    pub fn write_lock(&self) -> WRITE_LOCKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WRITE_LOCKR { bits }
    }
    #[doc = "Bits 2:3 - AHB bus matrix enable secure checking. 10: disabled. All other values: enabled (restrictive mode)"]
    #[inline]
    pub fn enable_secure_checking(&self) -> ENABLE_SECURE_CHECKINGR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ENABLE_SECURE_CHECKINGR { bits }
    }
    #[doc = "Bits 4:5 - AHB bus matrix enable secure privilege check"]
    #[inline]
    pub fn enable_s_priv_check(&self) -> ENABLE_S_PRIV_CHECKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ENABLE_S_PRIV_CHECKR { bits }
    }
    #[doc = "Bits 6:7 - AHB bus matrix enable non-secure privilege check"]
    #[inline]
    pub fn enable_ns_priv_check(&self) -> ENABLE_NS_PRIV_CHECKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ENABLE_NS_PRIV_CHECKR { bits }
    }
    #[doc = "Bits 8:9 - Disable secure violation abort"]
    #[inline]
    pub fn disable_violation_abort(&self) -> DISABLE_VIOLATION_ABORTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DISABLE_VIOLATION_ABORTR { bits }
    }
    #[doc = "Bits 10:11 - 00, 11, 10 = Simple master in strict mode"]
    #[inline]
    pub fn disable_simple_master_strict_mode(&self) -> DISABLE_SIMPLE_MASTER_STRICT_MODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DISABLE_SIMPLE_MASTER_STRICT_MODER { bits }
    }
    #[doc = "Bits 12:13 - 00, 11, 10 = Smart masters in strict mode"]
    #[inline]
    pub fn disable_smart_master_strict_mode(&self) -> DISABLE_SMART_MASTER_STRICT_MODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DISABLE_SMART_MASTER_STRICT_MODER { bits }
    }
    #[doc = "Bits 14:15 - 00, 11, 10 : IDAU is enabled"]
    #[inline]
    pub fn idau_all_ns(&self) -> IDAU_ALL_NSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDAU_ALL_NSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 43690 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - write lock"]
    #[inline]
    pub fn write_lock(&mut self) -> _WRITE_LOCKW {
        _WRITE_LOCKW { w: self }
    }
    #[doc = "Bits 2:3 - AHB bus matrix enable secure checking. 10: disabled. All other values: enabled (restrictive mode)"]
    #[inline]
    pub fn enable_secure_checking(&mut self) -> _ENABLE_SECURE_CHECKINGW {
        _ENABLE_SECURE_CHECKINGW { w: self }
    }
    #[doc = "Bits 4:5 - AHB bus matrix enable secure privilege check"]
    #[inline]
    pub fn enable_s_priv_check(&mut self) -> _ENABLE_S_PRIV_CHECKW {
        _ENABLE_S_PRIV_CHECKW { w: self }
    }
    #[doc = "Bits 6:7 - AHB bus matrix enable non-secure privilege check"]
    #[inline]
    pub fn enable_ns_priv_check(&mut self) -> _ENABLE_NS_PRIV_CHECKW {
        _ENABLE_NS_PRIV_CHECKW { w: self }
    }
    #[doc = "Bits 8:9 - Disable secure violation abort"]
    #[inline]
    pub fn disable_violation_abort(&mut self) -> _DISABLE_VIOLATION_ABORTW {
        _DISABLE_VIOLATION_ABORTW { w: self }
    }
    #[doc = "Bits 10:11 - 00, 11, 10 = Simple master in strict mode"]
    #[inline]
    pub fn disable_simple_master_strict_mode(&mut self) -> _DISABLE_SIMPLE_MASTER_STRICT_MODEW {
        _DISABLE_SIMPLE_MASTER_STRICT_MODEW { w: self }
    }
    #[doc = "Bits 12:13 - 00, 11, 10 = Smart masters in strict mode"]
    #[inline]
    pub fn disable_smart_master_strict_mode(&mut self) -> _DISABLE_SMART_MASTER_STRICT_MODEW {
        _DISABLE_SMART_MASTER_STRICT_MODEW { w: self }
    }
    #[doc = "Bits 14:15 - 00, 11, 10 : IDAU is enabled"]
    #[inline]
    pub fn idau_all_ns(&mut self) -> _IDAU_ALL_NSW {
        _IDAU_ALL_NSW { w: self }
    }
}
