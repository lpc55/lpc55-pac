#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROTKH_REVOKE {
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
pub struct ROTK0_ENR {
    bits: u8,
}
impl ROTK0_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ROTK1_ENR {
    bits: u8,
}
impl ROTK1_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ROTK2_ENR {
    bits: u8,
}
impl ROTK2_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ROTK0_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ROTK0_ENW<'a> {
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
pub struct _ROTK1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ROTK1_ENW<'a> {
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
pub struct _ROTK2_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ROTK2_ENW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline]
    pub fn ro_tk0_en(&self) -> ROTK0_ENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ROTK0_ENR { bits }
    }
    #[doc = "Bits 2:3 - RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline]
    pub fn ro_tk1_en(&self) -> ROTK1_ENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ROTK1_ENR { bits }
    }
    #[doc = "Bits 4:5 - RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline]
    pub fn ro_tk2_en(&self) -> ROTK2_ENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ROTK2_ENR { bits }
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
    #[doc = "Bits 0:1 - RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline]
    pub fn ro_tk0_en(&mut self) -> _ROTK0_ENW {
        _ROTK0_ENW { w: self }
    }
    #[doc = "Bits 2:3 - RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline]
    pub fn ro_tk1_en(&mut self) -> _ROTK1_ENW {
        _ROTK1_ENW { w: self }
    }
    #[doc = "Bits 4:5 - RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline]
    pub fn ro_tk2_en(&mut self) -> _ROTK2_ENW {
        _ROTK2_ENW { w: self }
    }
}
