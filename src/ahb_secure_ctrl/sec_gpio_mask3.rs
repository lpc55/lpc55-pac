#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEC_GPIO_MASK3 {
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
pub struct PIO3_PIN0_SEC_MASKR {
    bits: bool,
}
impl PIO3_PIN0_SEC_MASKR {
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
pub struct PIO3_PIN1_SEC_MASKR {
    bits: bool,
}
impl PIO3_PIN1_SEC_MASKR {
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
pub struct PIO3_PIN2_SEC_MASKR {
    bits: bool,
}
impl PIO3_PIN2_SEC_MASKR {
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
pub struct PIO3_PIN3_SEC_MASKR {
    bits: bool,
}
impl PIO3_PIN3_SEC_MASKR {
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
pub struct PIO3_PIN4_SEC_MASKR {
    bits: bool,
}
impl PIO3_PIN4_SEC_MASKR {
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
pub struct PIO3_PIN5_SEC_MASKR {
    bits: bool,
}
impl PIO3_PIN5_SEC_MASKR {
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
pub struct _PIO3_PIN0_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO3_PIN0_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO3_PIN1_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO3_PIN1_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO3_PIN2_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO3_PIN2_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO3_PIN3_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO3_PIN3_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO3_PIN4_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO3_PIN4_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO3_PIN5_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO3_PIN5_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - 0 : Pin PIO3_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio3_pin0_sec_mask(&self) -> PIO3_PIN0_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO3_PIN0_SEC_MASKR { bits }
    }
    #[doc = "Bit 1 - 0 : Pin PIO3_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio3_pin1_sec_mask(&self) -> PIO3_PIN1_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO3_PIN1_SEC_MASKR { bits }
    }
    #[doc = "Bit 2 - 0 : Pin PIO3_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio3_pin2_sec_mask(&self) -> PIO3_PIN2_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO3_PIN2_SEC_MASKR { bits }
    }
    #[doc = "Bit 3 - 0 : Pin PIO3_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio3_pin3_sec_mask(&self) -> PIO3_PIN3_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO3_PIN3_SEC_MASKR { bits }
    }
    #[doc = "Bit 4 - 0 : Pin PIO3_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio3_pin4_sec_mask(&self) -> PIO3_PIN4_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO3_PIN4_SEC_MASKR { bits }
    }
    #[doc = "Bit 5 - 0 : Pin PIO3_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio3_pin5_sec_mask(&self) -> PIO3_PIN5_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO3_PIN5_SEC_MASKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - 0 : Pin PIO3_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio3_pin0_sec_mask(&mut self) -> _PIO3_PIN0_SEC_MASKW {
        _PIO3_PIN0_SEC_MASKW { w: self }
    }
    #[doc = "Bit 1 - 0 : Pin PIO3_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio3_pin1_sec_mask(&mut self) -> _PIO3_PIN1_SEC_MASKW {
        _PIO3_PIN1_SEC_MASKW { w: self }
    }
    #[doc = "Bit 2 - 0 : Pin PIO3_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio3_pin2_sec_mask(&mut self) -> _PIO3_PIN2_SEC_MASKW {
        _PIO3_PIN2_SEC_MASKW { w: self }
    }
    #[doc = "Bit 3 - 0 : Pin PIO3_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio3_pin3_sec_mask(&mut self) -> _PIO3_PIN3_SEC_MASKW {
        _PIO3_PIN3_SEC_MASKW { w: self }
    }
    #[doc = "Bit 4 - 0 : Pin PIO3_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio3_pin4_sec_mask(&mut self) -> _PIO3_PIN4_SEC_MASKW {
        _PIO3_PIN4_SEC_MASKW { w: self }
    }
    #[doc = "Bit 5 - 0 : Pin PIO3_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio3_pin5_sec_mask(&mut self) -> _PIO3_PIN5_SEC_MASKW {
        _PIO3_PIN5_SEC_MASKW { w: self }
    }
}
