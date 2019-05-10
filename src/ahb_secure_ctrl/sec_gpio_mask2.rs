#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEC_GPIO_MASK2 {
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
pub struct PIO2_PIN0_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN0_SEC_MASKR {
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
pub struct PIO2_PIN1_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN1_SEC_MASKR {
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
pub struct PIO2_PIN2_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN2_SEC_MASKR {
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
pub struct PIO2_PIN3_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN3_SEC_MASKR {
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
pub struct PIO2_PIN4_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN4_SEC_MASKR {
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
pub struct PIO2_PIN5_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN5_SEC_MASKR {
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
pub struct PIO2_PIN6_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN6_SEC_MASKR {
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
pub struct PIO2_PIN7_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN7_SEC_MASKR {
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
pub struct PIO2_PIN8_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN8_SEC_MASKR {
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
pub struct PIO2_PIN9_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN9_SEC_MASKR {
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
pub struct PIO2_PIN10_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN10_SEC_MASKR {
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
pub struct PIO2_PIN11_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN11_SEC_MASKR {
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
pub struct PIO2_PIN12_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN12_SEC_MASKR {
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
pub struct PIO2_PIN13_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN13_SEC_MASKR {
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
pub struct PIO2_PIN14_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN14_SEC_MASKR {
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
pub struct PIO2_PIN15_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN15_SEC_MASKR {
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
pub struct PIO2_PIN16_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN16_SEC_MASKR {
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
pub struct PIO2_PIN17_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN17_SEC_MASKR {
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
pub struct PIO2_PIN18_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN18_SEC_MASKR {
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
pub struct PIO2_PIN19_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN19_SEC_MASKR {
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
pub struct PIO2_PIN20_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN20_SEC_MASKR {
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
pub struct PIO2_PIN21_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN21_SEC_MASKR {
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
pub struct PIO2_PIN22_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN22_SEC_MASKR {
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
pub struct PIO2_PIN23_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN23_SEC_MASKR {
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
pub struct PIO2_PIN24_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN24_SEC_MASKR {
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
pub struct PIO2_PIN25_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN25_SEC_MASKR {
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
pub struct PIO2_PIN26_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN26_SEC_MASKR {
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
pub struct PIO2_PIN27_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN27_SEC_MASKR {
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
pub struct PIO2_PIN28_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN28_SEC_MASKR {
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
pub struct PIO2_PIN29_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN29_SEC_MASKR {
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
pub struct PIO2_PIN30_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN30_SEC_MASKR {
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
pub struct PIO2_PIN31_SEC_MASKR {
    bits: bool,
}
impl PIO2_PIN31_SEC_MASKR {
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
pub struct _PIO2_PIN0_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN0_SEC_MASKW<'a> {
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
pub struct _PIO2_PIN1_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN1_SEC_MASKW<'a> {
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
pub struct _PIO2_PIN2_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN2_SEC_MASKW<'a> {
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
pub struct _PIO2_PIN3_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN3_SEC_MASKW<'a> {
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
pub struct _PIO2_PIN4_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN4_SEC_MASKW<'a> {
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
pub struct _PIO2_PIN5_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN5_SEC_MASKW<'a> {
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
#[doc = r" Proxy"]
pub struct _PIO2_PIN6_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN6_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN7_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN7_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN8_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN8_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN9_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN9_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN10_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN10_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN11_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN11_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN12_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN12_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN13_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN13_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN14_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN14_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN15_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN15_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN16_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN16_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN17_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN17_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN18_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN18_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN19_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN19_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN20_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN20_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN21_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN21_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN22_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN22_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN23_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN23_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN24_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN24_SEC_MASKW<'a> {
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
pub struct _PIO2_PIN25_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN25_SEC_MASKW<'a> {
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
pub struct _PIO2_PIN26_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN26_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN27_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN27_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN28_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN28_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN29_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN29_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN30_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN30_SEC_MASKW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PIO2_PIN31_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO2_PIN31_SEC_MASKW<'a> {
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
    #[doc = "Bit 0 - 0 : Pin PIO2_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin0_sec_mask(&self) -> PIO2_PIN0_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN0_SEC_MASKR { bits }
    }
    #[doc = "Bit 1 - 0 : Pin PIO2_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin1_sec_mask(&self) -> PIO2_PIN1_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN1_SEC_MASKR { bits }
    }
    #[doc = "Bit 2 - 0 : Pin PIO2_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin2_sec_mask(&self) -> PIO2_PIN2_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN2_SEC_MASKR { bits }
    }
    #[doc = "Bit 3 - 0 : Pin PIO2_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin3_sec_mask(&self) -> PIO2_PIN3_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN3_SEC_MASKR { bits }
    }
    #[doc = "Bit 4 - 0 : Pin PIO2_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin4_sec_mask(&self) -> PIO2_PIN4_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN4_SEC_MASKR { bits }
    }
    #[doc = "Bit 5 - 0 : Pin PIO2_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin5_sec_mask(&self) -> PIO2_PIN5_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN5_SEC_MASKR { bits }
    }
    #[doc = "Bit 6 - 0 : Pin PIO2_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin6_sec_mask(&self) -> PIO2_PIN6_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN6_SEC_MASKR { bits }
    }
    #[doc = "Bit 7 - 0 : Pin PIO2_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin7_sec_mask(&self) -> PIO2_PIN7_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN7_SEC_MASKR { bits }
    }
    #[doc = "Bit 8 - 0 : Pin PIO2_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin8_sec_mask(&self) -> PIO2_PIN8_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN8_SEC_MASKR { bits }
    }
    #[doc = "Bit 9 - 0 : Pin PIO2_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin9_sec_mask(&self) -> PIO2_PIN9_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN9_SEC_MASKR { bits }
    }
    #[doc = "Bit 10 - 0 : Pin PIO2_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin10_sec_mask(&self) -> PIO2_PIN10_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN10_SEC_MASKR { bits }
    }
    #[doc = "Bit 11 - 0 : Pin PIO2_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin11_sec_mask(&self) -> PIO2_PIN11_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN11_SEC_MASKR { bits }
    }
    #[doc = "Bit 12 - 0 : Pin PIO2_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin12_sec_mask(&self) -> PIO2_PIN12_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN12_SEC_MASKR { bits }
    }
    #[doc = "Bit 13 - 0 : Pin PIO2_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin13_sec_mask(&self) -> PIO2_PIN13_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN13_SEC_MASKR { bits }
    }
    #[doc = "Bit 14 - 0 : Pin PIO2_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin14_sec_mask(&self) -> PIO2_PIN14_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN14_SEC_MASKR { bits }
    }
    #[doc = "Bit 15 - 0 : Pin PIO2_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin15_sec_mask(&self) -> PIO2_PIN15_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN15_SEC_MASKR { bits }
    }
    #[doc = "Bit 16 - 0 : Pin PIO2_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin16_sec_mask(&self) -> PIO2_PIN16_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN16_SEC_MASKR { bits }
    }
    #[doc = "Bit 17 - 0 : Pin PIO2_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin17_sec_mask(&self) -> PIO2_PIN17_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN17_SEC_MASKR { bits }
    }
    #[doc = "Bit 18 - 0 : Pin PIO2_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin18_sec_mask(&self) -> PIO2_PIN18_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN18_SEC_MASKR { bits }
    }
    #[doc = "Bit 19 - 0 : Pin PIO2_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin19_sec_mask(&self) -> PIO2_PIN19_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN19_SEC_MASKR { bits }
    }
    #[doc = "Bit 20 - 0 : Pin PIO2_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin20_sec_mask(&self) -> PIO2_PIN20_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN20_SEC_MASKR { bits }
    }
    #[doc = "Bit 21 - 0 : Pin PIO2_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin21_sec_mask(&self) -> PIO2_PIN21_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN21_SEC_MASKR { bits }
    }
    #[doc = "Bit 22 - 0 : Pin PIO2_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin22_sec_mask(&self) -> PIO2_PIN22_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN22_SEC_MASKR { bits }
    }
    #[doc = "Bit 23 - 0 : Pin PIO2_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin23_sec_mask(&self) -> PIO2_PIN23_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN23_SEC_MASKR { bits }
    }
    #[doc = "Bit 24 - 0 : Pin PIO2_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin24_sec_mask(&self) -> PIO2_PIN24_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN24_SEC_MASKR { bits }
    }
    #[doc = "Bit 25 - 0 : Pin PIO2_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin25_sec_mask(&self) -> PIO2_PIN25_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN25_SEC_MASKR { bits }
    }
    #[doc = "Bit 26 - 0 : Pin PIO2_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin26_sec_mask(&self) -> PIO2_PIN26_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN26_SEC_MASKR { bits }
    }
    #[doc = "Bit 27 - 0 : Pin PIO2_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin27_sec_mask(&self) -> PIO2_PIN27_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN27_SEC_MASKR { bits }
    }
    #[doc = "Bit 28 - 0 : Pin PIO2_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin28_sec_mask(&self) -> PIO2_PIN28_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN28_SEC_MASKR { bits }
    }
    #[doc = "Bit 29 - 0 : Pin PIO2_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin29_sec_mask(&self) -> PIO2_PIN29_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN29_SEC_MASKR { bits }
    }
    #[doc = "Bit 30 - 0 : Pin PIO2_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin30_sec_mask(&self) -> PIO2_PIN30_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN30_SEC_MASKR { bits }
    }
    #[doc = "Bit 31 - 0 : Pin PIO2_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin31_sec_mask(&self) -> PIO2_PIN31_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO2_PIN31_SEC_MASKR { bits }
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
    #[doc = "Bit 0 - 0 : Pin PIO2_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin0_sec_mask(&mut self) -> _PIO2_PIN0_SEC_MASKW {
        _PIO2_PIN0_SEC_MASKW { w: self }
    }
    #[doc = "Bit 1 - 0 : Pin PIO2_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin1_sec_mask(&mut self) -> _PIO2_PIN1_SEC_MASKW {
        _PIO2_PIN1_SEC_MASKW { w: self }
    }
    #[doc = "Bit 2 - 0 : Pin PIO2_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin2_sec_mask(&mut self) -> _PIO2_PIN2_SEC_MASKW {
        _PIO2_PIN2_SEC_MASKW { w: self }
    }
    #[doc = "Bit 3 - 0 : Pin PIO2_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin3_sec_mask(&mut self) -> _PIO2_PIN3_SEC_MASKW {
        _PIO2_PIN3_SEC_MASKW { w: self }
    }
    #[doc = "Bit 4 - 0 : Pin PIO2_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin4_sec_mask(&mut self) -> _PIO2_PIN4_SEC_MASKW {
        _PIO2_PIN4_SEC_MASKW { w: self }
    }
    #[doc = "Bit 5 - 0 : Pin PIO2_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin5_sec_mask(&mut self) -> _PIO2_PIN5_SEC_MASKW {
        _PIO2_PIN5_SEC_MASKW { w: self }
    }
    #[doc = "Bit 6 - 0 : Pin PIO2_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin6_sec_mask(&mut self) -> _PIO2_PIN6_SEC_MASKW {
        _PIO2_PIN6_SEC_MASKW { w: self }
    }
    #[doc = "Bit 7 - 0 : Pin PIO2_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin7_sec_mask(&mut self) -> _PIO2_PIN7_SEC_MASKW {
        _PIO2_PIN7_SEC_MASKW { w: self }
    }
    #[doc = "Bit 8 - 0 : Pin PIO2_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin8_sec_mask(&mut self) -> _PIO2_PIN8_SEC_MASKW {
        _PIO2_PIN8_SEC_MASKW { w: self }
    }
    #[doc = "Bit 9 - 0 : Pin PIO2_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin9_sec_mask(&mut self) -> _PIO2_PIN9_SEC_MASKW {
        _PIO2_PIN9_SEC_MASKW { w: self }
    }
    #[doc = "Bit 10 - 0 : Pin PIO2_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin10_sec_mask(&mut self) -> _PIO2_PIN10_SEC_MASKW {
        _PIO2_PIN10_SEC_MASKW { w: self }
    }
    #[doc = "Bit 11 - 0 : Pin PIO2_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin11_sec_mask(&mut self) -> _PIO2_PIN11_SEC_MASKW {
        _PIO2_PIN11_SEC_MASKW { w: self }
    }
    #[doc = "Bit 12 - 0 : Pin PIO2_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin12_sec_mask(&mut self) -> _PIO2_PIN12_SEC_MASKW {
        _PIO2_PIN12_SEC_MASKW { w: self }
    }
    #[doc = "Bit 13 - 0 : Pin PIO2_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin13_sec_mask(&mut self) -> _PIO2_PIN13_SEC_MASKW {
        _PIO2_PIN13_SEC_MASKW { w: self }
    }
    #[doc = "Bit 14 - 0 : Pin PIO2_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin14_sec_mask(&mut self) -> _PIO2_PIN14_SEC_MASKW {
        _PIO2_PIN14_SEC_MASKW { w: self }
    }
    #[doc = "Bit 15 - 0 : Pin PIO2_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin15_sec_mask(&mut self) -> _PIO2_PIN15_SEC_MASKW {
        _PIO2_PIN15_SEC_MASKW { w: self }
    }
    #[doc = "Bit 16 - 0 : Pin PIO2_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin16_sec_mask(&mut self) -> _PIO2_PIN16_SEC_MASKW {
        _PIO2_PIN16_SEC_MASKW { w: self }
    }
    #[doc = "Bit 17 - 0 : Pin PIO2_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin17_sec_mask(&mut self) -> _PIO2_PIN17_SEC_MASKW {
        _PIO2_PIN17_SEC_MASKW { w: self }
    }
    #[doc = "Bit 18 - 0 : Pin PIO2_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin18_sec_mask(&mut self) -> _PIO2_PIN18_SEC_MASKW {
        _PIO2_PIN18_SEC_MASKW { w: self }
    }
    #[doc = "Bit 19 - 0 : Pin PIO2_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin19_sec_mask(&mut self) -> _PIO2_PIN19_SEC_MASKW {
        _PIO2_PIN19_SEC_MASKW { w: self }
    }
    #[doc = "Bit 20 - 0 : Pin PIO2_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin20_sec_mask(&mut self) -> _PIO2_PIN20_SEC_MASKW {
        _PIO2_PIN20_SEC_MASKW { w: self }
    }
    #[doc = "Bit 21 - 0 : Pin PIO2_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin21_sec_mask(&mut self) -> _PIO2_PIN21_SEC_MASKW {
        _PIO2_PIN21_SEC_MASKW { w: self }
    }
    #[doc = "Bit 22 - 0 : Pin PIO2_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin22_sec_mask(&mut self) -> _PIO2_PIN22_SEC_MASKW {
        _PIO2_PIN22_SEC_MASKW { w: self }
    }
    #[doc = "Bit 23 - 0 : Pin PIO2_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin23_sec_mask(&mut self) -> _PIO2_PIN23_SEC_MASKW {
        _PIO2_PIN23_SEC_MASKW { w: self }
    }
    #[doc = "Bit 24 - 0 : Pin PIO2_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin24_sec_mask(&mut self) -> _PIO2_PIN24_SEC_MASKW {
        _PIO2_PIN24_SEC_MASKW { w: self }
    }
    #[doc = "Bit 25 - 0 : Pin PIO2_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin25_sec_mask(&mut self) -> _PIO2_PIN25_SEC_MASKW {
        _PIO2_PIN25_SEC_MASKW { w: self }
    }
    #[doc = "Bit 26 - 0 : Pin PIO2_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin26_sec_mask(&mut self) -> _PIO2_PIN26_SEC_MASKW {
        _PIO2_PIN26_SEC_MASKW { w: self }
    }
    #[doc = "Bit 27 - 0 : Pin PIO2_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin27_sec_mask(&mut self) -> _PIO2_PIN27_SEC_MASKW {
        _PIO2_PIN27_SEC_MASKW { w: self }
    }
    #[doc = "Bit 28 - 0 : Pin PIO2_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin28_sec_mask(&mut self) -> _PIO2_PIN28_SEC_MASKW {
        _PIO2_PIN28_SEC_MASKW { w: self }
    }
    #[doc = "Bit 29 - 0 : Pin PIO2_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin29_sec_mask(&mut self) -> _PIO2_PIN29_SEC_MASKW {
        _PIO2_PIN29_SEC_MASKW { w: self }
    }
    #[doc = "Bit 30 - 0 : Pin PIO2_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin30_sec_mask(&mut self) -> _PIO2_PIN30_SEC_MASKW {
        _PIO2_PIN30_SEC_MASKW { w: self }
    }
    #[doc = "Bit 31 - 0 : Pin PIO2_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio2_pin31_sec_mask(&mut self) -> _PIO2_PIN31_SEC_MASKW {
        _PIO2_PIN31_SEC_MASKW { w: self }
    }
}
