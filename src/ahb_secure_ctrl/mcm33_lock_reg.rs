#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCM33_LOCK_REG {
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
pub struct LOCK_NS_VTORR {
    bits: u8,
}
impl LOCK_NS_VTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOCK_NS_MPUR {
    bits: u8,
}
impl LOCK_NS_MPUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MCM33_LOCK_REG_LOCKR {
    bits: u8,
}
impl MCM33_LOCK_REG_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_NS_VTORW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_NS_VTORW<'a> {
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
pub struct _LOCK_NS_MPUW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_NS_MPUW<'a> {
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
pub struct _MCM33_LOCK_REG_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _MCM33_LOCK_REG_LOCKW<'a> {
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
    #[doc = "Bits 0:1 - 2'b10: micro-CM33 (CPU1) LOCKNSVTOR is 0. All other values: micro-CM33 (CPU1) LOCKNSVTOR is 1"]
    #[inline]
    pub fn lock_ns_vtor(&self) -> LOCK_NS_VTORR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOCK_NS_VTORR { bits }
    }
    #[doc = "Bits 2:3 - 2'b10: micro-CM33 (CPU1) LOCKNSMPU is 0. All other values: micro-CM33 (CPU1) LOCKNSMPU is 1"]
    #[inline]
    pub fn lock_ns_mpu(&self) -> LOCK_NS_MPUR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOCK_NS_MPUR { bits }
    }
    #[doc = "Bits 30:31 - 2'b10: this register can be written. All other values: this register can't be written"]
    #[inline]
    pub fn mcm33_lock_reg_lock(&self) -> MCM33_LOCK_REG_LOCKR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MCM33_LOCK_REG_LOCKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147483658 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - 2'b10: micro-CM33 (CPU1) LOCKNSVTOR is 0. All other values: micro-CM33 (CPU1) LOCKNSVTOR is 1"]
    #[inline]
    pub fn lock_ns_vtor(&mut self) -> _LOCK_NS_VTORW {
        _LOCK_NS_VTORW { w: self }
    }
    #[doc = "Bits 2:3 - 2'b10: micro-CM33 (CPU1) LOCKNSMPU is 0. All other values: micro-CM33 (CPU1) LOCKNSMPU is 1"]
    #[inline]
    pub fn lock_ns_mpu(&mut self) -> _LOCK_NS_MPUW {
        _LOCK_NS_MPUW { w: self }
    }
    #[doc = "Bits 30:31 - 2'b10: this register can be written. All other values: this register can't be written"]
    #[inline]
    pub fn mcm33_lock_reg_lock(&mut self) -> _MCM33_LOCK_REG_LOCKW {
        _MCM33_LOCK_REG_LOCKW { w: self }
    }
}
