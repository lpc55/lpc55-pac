#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOCK {
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
#[doc = "Possible values of the field `SECLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECLOCKR {
    #[doc = "Unlocks, so block is open to all. But, AHB Master will only issue non-secure requests."]
    UNLOCK,
    #[doc = "Locks to the current security level. AHB Master will issue requests at this level."]
    LOCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SECLOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SECLOCKR::UNLOCK => 0,
            SECLOCKR::LOCK => 1,
            SECLOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SECLOCKR {
        match value {
            0 => SECLOCKR::UNLOCK,
            1 => SECLOCKR::LOCK,
            i => SECLOCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCK`"]
    #[inline]
    pub fn is_unlock(&self) -> bool {
        *self == SECLOCKR::UNLOCK
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline]
    pub fn is_lock(&self) -> bool {
        *self == SECLOCKR::LOCK
    }
}
#[doc = r" Value of the field"]
pub struct PATTERNR {
    bits: u16,
}
impl PATTERNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SECLOCK`"]
pub enum SECLOCKW {
    #[doc = "Unlocks, so block is open to all. But, AHB Master will only issue non-secure requests."]
    UNLOCK,
    #[doc = "Locks to the current security level. AHB Master will issue requests at this level."]
    LOCK,
}
impl SECLOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SECLOCKW::UNLOCK => 0,
            SECLOCKW::LOCK => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SECLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _SECLOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SECLOCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Unlocks, so block is open to all. But, AHB Master will only issue non-secure requests."]
    #[inline]
    pub fn unlock(self) -> &'a mut W {
        self.variant(SECLOCKW::UNLOCK)
    }
    #[doc = "Locks to the current security level. AHB Master will issue requests at this level."]
    #[inline]
    pub fn lock(self) -> &'a mut W {
        self.variant(SECLOCKW::LOCK)
    }
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
pub struct _PATTERNW<'a> {
    w: &'a mut W,
}
impl<'a> _PATTERNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
    #[inline]
    pub fn seclock(&self) -> SECLOCKR {
        SECLOCKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:15 - Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
    #[inline]
    pub fn pattern(&self) -> PATTERNR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PATTERNR { bits }
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
    #[doc = "Bits 0:1 - Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
    #[inline]
    pub fn seclock(&mut self) -> _SECLOCKW {
        _SECLOCKW { w: self }
    }
    #[doc = "Bits 4:15 - Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
    #[inline]
    pub fn pattern(&mut self) -> _PATTERNW {
        _PATTERNW { w: self }
    }
}
