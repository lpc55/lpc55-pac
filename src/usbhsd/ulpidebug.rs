#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ULPIDEBUG {
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
pub struct PHY_ADDRR {
    bits: u8,
}
impl PHY_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PHY_WDATAR {
    bits: u8,
}
impl PHY_WDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PHY_RDATAR {
    bits: u8,
}
impl PHY_RDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PHY_RWR {
    bits: bool,
}
impl PHY_RWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PHY_ACCESSR {
    bits: bool,
}
impl PHY_ACCESSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PHY_MODER {
    bits: bool,
}
impl PHY_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _PHY_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_ADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHY_WDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_WDATAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHY_RDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_RDATAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHY_RWW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_RWW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHY_ACCESSW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_ACCESSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHY_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_MODEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:7 - ULPI mode: Bits 7:0 are used as the address when doing a register access over the ULPI interface."]
    #[inline]
    pub fn phy_addr(&self) -> PHY_ADDRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PHY_ADDRR { bits }
    }
    #[doc = "Bits 8:15 - UTMI+ mode: Reserved."]
    #[inline]
    pub fn phy_wdata(&self) -> PHY_WDATAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PHY_WDATAR { bits }
    }
    #[doc = "Bits 16:23 - UTMI+ mode: Bits 7:0 contains the value returned by the VStatus signal on Vendor Interface of UTMI+."]
    #[inline]
    pub fn phy_rdata(&self) -> PHY_RDATAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PHY_RDATAR { bits }
    }
    #[doc = "Bit 24 - UTMI+ mode: Reserved."]
    #[inline]
    pub fn phy_rw(&self) -> PHY_RWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PHY_RWR { bits }
    }
    #[doc = "Bit 25 - Software writes this bit to one to start a read or write operation."]
    #[inline]
    pub fn phy_access(&self) -> PHY_ACCESSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PHY_ACCESSR { bits }
    }
    #[doc = "Bit 31 - This bit indicates if the interface between the controller is UTMI+ or ULPI."]
    #[inline]
    pub fn phy_mode(&self) -> PHY_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PHY_MODER { bits }
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
    #[doc = "Bits 0:7 - ULPI mode: Bits 7:0 are used as the address when doing a register access over the ULPI interface."]
    #[inline]
    pub fn phy_addr(&mut self) -> _PHY_ADDRW {
        _PHY_ADDRW { w: self }
    }
    #[doc = "Bits 8:15 - UTMI+ mode: Reserved."]
    #[inline]
    pub fn phy_wdata(&mut self) -> _PHY_WDATAW {
        _PHY_WDATAW { w: self }
    }
    #[doc = "Bits 16:23 - UTMI+ mode: Bits 7:0 contains the value returned by the VStatus signal on Vendor Interface of UTMI+."]
    #[inline]
    pub fn phy_rdata(&mut self) -> _PHY_RDATAW {
        _PHY_RDATAW { w: self }
    }
    #[doc = "Bit 24 - UTMI+ mode: Reserved."]
    #[inline]
    pub fn phy_rw(&mut self) -> _PHY_RWW {
        _PHY_RWW { w: self }
    }
    #[doc = "Bit 25 - Software writes this bit to one to start a read or write operation."]
    #[inline]
    pub fn phy_access(&mut self) -> _PHY_ACCESSW {
        _PHY_ACCESSW { w: self }
    }
    #[doc = "Bit 31 - This bit indicates if the interface between the controller is UTMI+ or ULPI."]
    #[inline]
    pub fn phy_mode(&mut self) -> _PHY_MODEW {
        _PHY_MODEW { w: self }
    }
}
