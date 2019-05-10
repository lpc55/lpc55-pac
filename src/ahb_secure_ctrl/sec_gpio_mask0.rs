#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEC_GPIO_MASK0 {
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
pub struct PIO0_PIN0_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN0_SEC_MASKR {
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
pub struct PIO0_PIN1_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN1_SEC_MASKR {
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
pub struct PIO0_PIN2_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN2_SEC_MASKR {
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
pub struct PIO0_PIN3_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN3_SEC_MASKR {
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
pub struct PIO0_PIN4_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN4_SEC_MASKR {
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
pub struct PIO0_PIN5_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN5_SEC_MASKR {
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
pub struct PIO0_PIN6_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN6_SEC_MASKR {
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
pub struct PIO0_PIN7_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN7_SEC_MASKR {
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
pub struct PIO0_PIN8_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN8_SEC_MASKR {
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
pub struct PIO0_PIN9_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN9_SEC_MASKR {
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
pub struct PIO0_PIN10_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN10_SEC_MASKR {
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
pub struct PIO0_PIN11_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN11_SEC_MASKR {
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
pub struct PIO0_PIN12_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN12_SEC_MASKR {
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
pub struct PIO0_PIN13_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN13_SEC_MASKR {
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
pub struct PIO0_PIN14_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN14_SEC_MASKR {
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
pub struct PIO0_PIN15_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN15_SEC_MASKR {
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
pub struct PIO0_PIN16_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN16_SEC_MASKR {
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
pub struct PIO0_PIN17_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN17_SEC_MASKR {
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
pub struct PIO0_PIN18_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN18_SEC_MASKR {
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
pub struct PIO0_PIN19_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN19_SEC_MASKR {
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
pub struct PIO0_PIN20_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN20_SEC_MASKR {
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
pub struct PIO0_PIN21_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN21_SEC_MASKR {
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
pub struct PIO0_PIN22_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN22_SEC_MASKR {
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
pub struct PIO0_PIN23_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN23_SEC_MASKR {
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
pub struct PIO0_PIN24_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN24_SEC_MASKR {
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
pub struct PIO0_PIN25_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN25_SEC_MASKR {
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
pub struct PIO0_PIN26_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN26_SEC_MASKR {
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
pub struct PIO0_PIN27_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN27_SEC_MASKR {
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
pub struct PIO0_PIN28_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN28_SEC_MASKR {
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
pub struct PIO0_PIN29_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN29_SEC_MASKR {
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
pub struct PIO0_PIN30_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN30_SEC_MASKR {
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
pub struct PIO0_PIN31_SEC_MASKR {
    bits: bool,
}
impl PIO0_PIN31_SEC_MASKR {
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
pub struct _PIO0_PIN0_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN0_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN1_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN1_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN2_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN2_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN3_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN3_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN4_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN4_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN5_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN5_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN6_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN6_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN7_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN7_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN8_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN8_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN9_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN9_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN10_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN10_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN11_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN11_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN12_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN12_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN13_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN13_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN14_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN14_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN15_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN15_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN16_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN16_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN17_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN17_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN18_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN18_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN19_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN19_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN20_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN20_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN21_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN21_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN22_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN22_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN23_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN23_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN24_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN24_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN25_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN25_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN26_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN26_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN27_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN27_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN28_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN28_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN29_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN29_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN30_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN30_SEC_MASKW<'a> {
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
pub struct _PIO0_PIN31_SEC_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _PIO0_PIN31_SEC_MASKW<'a> {
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
    #[doc = "Bit 0 - 0 : Pin PIO0_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin0_sec_mask(&self) -> PIO0_PIN0_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN0_SEC_MASKR { bits }
    }
    #[doc = "Bit 1 - 0 : Pin PIO0_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin1_sec_mask(&self) -> PIO0_PIN1_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN1_SEC_MASKR { bits }
    }
    #[doc = "Bit 2 - 0 : Pin PIO0_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin2_sec_mask(&self) -> PIO0_PIN2_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN2_SEC_MASKR { bits }
    }
    #[doc = "Bit 3 - 0 : Pin PIO0_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin3_sec_mask(&self) -> PIO0_PIN3_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN3_SEC_MASKR { bits }
    }
    #[doc = "Bit 4 - 0 : Pin PIO0_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin4_sec_mask(&self) -> PIO0_PIN4_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN4_SEC_MASKR { bits }
    }
    #[doc = "Bit 5 - 0 : Pin PIO0_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin5_sec_mask(&self) -> PIO0_PIN5_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN5_SEC_MASKR { bits }
    }
    #[doc = "Bit 6 - 0 : Pin PIO0_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin6_sec_mask(&self) -> PIO0_PIN6_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN6_SEC_MASKR { bits }
    }
    #[doc = "Bit 7 - 0 : Pin PIO0_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin7_sec_mask(&self) -> PIO0_PIN7_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN7_SEC_MASKR { bits }
    }
    #[doc = "Bit 8 - 0 : Pin PIO0_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin8_sec_mask(&self) -> PIO0_PIN8_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN8_SEC_MASKR { bits }
    }
    #[doc = "Bit 9 - 0 : Pin PIO0_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin9_sec_mask(&self) -> PIO0_PIN9_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN9_SEC_MASKR { bits }
    }
    #[doc = "Bit 10 - 0 : Pin PIO0_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin10_sec_mask(&self) -> PIO0_PIN10_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN10_SEC_MASKR { bits }
    }
    #[doc = "Bit 11 - 0 : Pin PIO0_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin11_sec_mask(&self) -> PIO0_PIN11_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN11_SEC_MASKR { bits }
    }
    #[doc = "Bit 12 - 0 : Pin PIO0_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin12_sec_mask(&self) -> PIO0_PIN12_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN12_SEC_MASKR { bits }
    }
    #[doc = "Bit 13 - 0 : Pin PIO0_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin13_sec_mask(&self) -> PIO0_PIN13_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN13_SEC_MASKR { bits }
    }
    #[doc = "Bit 14 - 0 : Pin PIO0_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin14_sec_mask(&self) -> PIO0_PIN14_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN14_SEC_MASKR { bits }
    }
    #[doc = "Bit 15 - 0 : Pin PIO0_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin15_sec_mask(&self) -> PIO0_PIN15_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN15_SEC_MASKR { bits }
    }
    #[doc = "Bit 16 - 0 : Pin PIO0_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin16_sec_mask(&self) -> PIO0_PIN16_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN16_SEC_MASKR { bits }
    }
    #[doc = "Bit 17 - 0 : Pin PIO0_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin17_sec_mask(&self) -> PIO0_PIN17_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN17_SEC_MASKR { bits }
    }
    #[doc = "Bit 18 - 0 : Pin PIO0_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin18_sec_mask(&self) -> PIO0_PIN18_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN18_SEC_MASKR { bits }
    }
    #[doc = "Bit 19 - 0 : Pin PIO0_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin19_sec_mask(&self) -> PIO0_PIN19_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN19_SEC_MASKR { bits }
    }
    #[doc = "Bit 20 - 0 : Pin PIO0_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin20_sec_mask(&self) -> PIO0_PIN20_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN20_SEC_MASKR { bits }
    }
    #[doc = "Bit 21 - 0 : Pin PIO0_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin21_sec_mask(&self) -> PIO0_PIN21_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN21_SEC_MASKR { bits }
    }
    #[doc = "Bit 22 - 0 : Pin PIO0_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin22_sec_mask(&self) -> PIO0_PIN22_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN22_SEC_MASKR { bits }
    }
    #[doc = "Bit 23 - 0 : Pin PIO0_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin23_sec_mask(&self) -> PIO0_PIN23_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN23_SEC_MASKR { bits }
    }
    #[doc = "Bit 24 - 0 : Pin PIO0_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin24_sec_mask(&self) -> PIO0_PIN24_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN24_SEC_MASKR { bits }
    }
    #[doc = "Bit 25 - 0 : Pin PIO0_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin25_sec_mask(&self) -> PIO0_PIN25_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN25_SEC_MASKR { bits }
    }
    #[doc = "Bit 26 - 0 : Pin PIO0_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin26_sec_mask(&self) -> PIO0_PIN26_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN26_SEC_MASKR { bits }
    }
    #[doc = "Bit 27 - 0 : Pin PIO0_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin27_sec_mask(&self) -> PIO0_PIN27_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN27_SEC_MASKR { bits }
    }
    #[doc = "Bit 28 - 0 : Pin PIO0_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin28_sec_mask(&self) -> PIO0_PIN28_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN28_SEC_MASKR { bits }
    }
    #[doc = "Bit 29 - 0 : Pin PIO0_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin29_sec_mask(&self) -> PIO0_PIN29_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN29_SEC_MASKR { bits }
    }
    #[doc = "Bit 30 - 0 : Pin PIO0_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin30_sec_mask(&self) -> PIO0_PIN30_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN30_SEC_MASKR { bits }
    }
    #[doc = "Bit 31 - 0 : Pin PIO0_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin31_sec_mask(&self) -> PIO0_PIN31_SEC_MASKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIO0_PIN31_SEC_MASKR { bits }
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
    #[doc = "Bit 0 - 0 : Pin PIO0_0 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin0_sec_mask(&mut self) -> _PIO0_PIN0_SEC_MASKW {
        _PIO0_PIN0_SEC_MASKW { w: self }
    }
    #[doc = "Bit 1 - 0 : Pin PIO0_1 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin1_sec_mask(&mut self) -> _PIO0_PIN1_SEC_MASKW {
        _PIO0_PIN1_SEC_MASKW { w: self }
    }
    #[doc = "Bit 2 - 0 : Pin PIO0_2 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin2_sec_mask(&mut self) -> _PIO0_PIN2_SEC_MASKW {
        _PIO0_PIN2_SEC_MASKW { w: self }
    }
    #[doc = "Bit 3 - 0 : Pin PIO0_3 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin3_sec_mask(&mut self) -> _PIO0_PIN3_SEC_MASKW {
        _PIO0_PIN3_SEC_MASKW { w: self }
    }
    #[doc = "Bit 4 - 0 : Pin PIO0_4 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin4_sec_mask(&mut self) -> _PIO0_PIN4_SEC_MASKW {
        _PIO0_PIN4_SEC_MASKW { w: self }
    }
    #[doc = "Bit 5 - 0 : Pin PIO0_5 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin5_sec_mask(&mut self) -> _PIO0_PIN5_SEC_MASKW {
        _PIO0_PIN5_SEC_MASKW { w: self }
    }
    #[doc = "Bit 6 - 0 : Pin PIO0_6 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin6_sec_mask(&mut self) -> _PIO0_PIN6_SEC_MASKW {
        _PIO0_PIN6_SEC_MASKW { w: self }
    }
    #[doc = "Bit 7 - 0 : Pin PIO0_7 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin7_sec_mask(&mut self) -> _PIO0_PIN7_SEC_MASKW {
        _PIO0_PIN7_SEC_MASKW { w: self }
    }
    #[doc = "Bit 8 - 0 : Pin PIO0_8 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin8_sec_mask(&mut self) -> _PIO0_PIN8_SEC_MASKW {
        _PIO0_PIN8_SEC_MASKW { w: self }
    }
    #[doc = "Bit 9 - 0 : Pin PIO0_9 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin9_sec_mask(&mut self) -> _PIO0_PIN9_SEC_MASKW {
        _PIO0_PIN9_SEC_MASKW { w: self }
    }
    #[doc = "Bit 10 - 0 : Pin PIO0_10 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin10_sec_mask(&mut self) -> _PIO0_PIN10_SEC_MASKW {
        _PIO0_PIN10_SEC_MASKW { w: self }
    }
    #[doc = "Bit 11 - 0 : Pin PIO0_11 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin11_sec_mask(&mut self) -> _PIO0_PIN11_SEC_MASKW {
        _PIO0_PIN11_SEC_MASKW { w: self }
    }
    #[doc = "Bit 12 - 0 : Pin PIO0_12 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin12_sec_mask(&mut self) -> _PIO0_PIN12_SEC_MASKW {
        _PIO0_PIN12_SEC_MASKW { w: self }
    }
    #[doc = "Bit 13 - 0 : Pin PIO0_13 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin13_sec_mask(&mut self) -> _PIO0_PIN13_SEC_MASKW {
        _PIO0_PIN13_SEC_MASKW { w: self }
    }
    #[doc = "Bit 14 - 0 : Pin PIO0_14 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin14_sec_mask(&mut self) -> _PIO0_PIN14_SEC_MASKW {
        _PIO0_PIN14_SEC_MASKW { w: self }
    }
    #[doc = "Bit 15 - 0 : Pin PIO0_15 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin15_sec_mask(&mut self) -> _PIO0_PIN15_SEC_MASKW {
        _PIO0_PIN15_SEC_MASKW { w: self }
    }
    #[doc = "Bit 16 - 0 : Pin PIO0_16 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin16_sec_mask(&mut self) -> _PIO0_PIN16_SEC_MASKW {
        _PIO0_PIN16_SEC_MASKW { w: self }
    }
    #[doc = "Bit 17 - 0 : Pin PIO0_17 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin17_sec_mask(&mut self) -> _PIO0_PIN17_SEC_MASKW {
        _PIO0_PIN17_SEC_MASKW { w: self }
    }
    #[doc = "Bit 18 - 0 : Pin PIO0_18 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin18_sec_mask(&mut self) -> _PIO0_PIN18_SEC_MASKW {
        _PIO0_PIN18_SEC_MASKW { w: self }
    }
    #[doc = "Bit 19 - 0 : Pin PIO0_19 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin19_sec_mask(&mut self) -> _PIO0_PIN19_SEC_MASKW {
        _PIO0_PIN19_SEC_MASKW { w: self }
    }
    #[doc = "Bit 20 - 0 : Pin PIO0_20 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin20_sec_mask(&mut self) -> _PIO0_PIN20_SEC_MASKW {
        _PIO0_PIN20_SEC_MASKW { w: self }
    }
    #[doc = "Bit 21 - 0 : Pin PIO0_21 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin21_sec_mask(&mut self) -> _PIO0_PIN21_SEC_MASKW {
        _PIO0_PIN21_SEC_MASKW { w: self }
    }
    #[doc = "Bit 22 - 0 : Pin PIO0_22 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin22_sec_mask(&mut self) -> _PIO0_PIN22_SEC_MASKW {
        _PIO0_PIN22_SEC_MASKW { w: self }
    }
    #[doc = "Bit 23 - 0 : Pin PIO0_23 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin23_sec_mask(&mut self) -> _PIO0_PIN23_SEC_MASKW {
        _PIO0_PIN23_SEC_MASKW { w: self }
    }
    #[doc = "Bit 24 - 0 : Pin PIO0_24 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin24_sec_mask(&mut self) -> _PIO0_PIN24_SEC_MASKW {
        _PIO0_PIN24_SEC_MASKW { w: self }
    }
    #[doc = "Bit 25 - 0 : Pin PIO0_25 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin25_sec_mask(&mut self) -> _PIO0_PIN25_SEC_MASKW {
        _PIO0_PIN25_SEC_MASKW { w: self }
    }
    #[doc = "Bit 26 - 0 : Pin PIO0_26 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin26_sec_mask(&mut self) -> _PIO0_PIN26_SEC_MASKW {
        _PIO0_PIN26_SEC_MASKW { w: self }
    }
    #[doc = "Bit 27 - 0 : Pin PIO0_27 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin27_sec_mask(&mut self) -> _PIO0_PIN27_SEC_MASKW {
        _PIO0_PIN27_SEC_MASKW { w: self }
    }
    #[doc = "Bit 28 - 0 : Pin PIO0_28 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin28_sec_mask(&mut self) -> _PIO0_PIN28_SEC_MASKW {
        _PIO0_PIN28_SEC_MASKW { w: self }
    }
    #[doc = "Bit 29 - 0 : Pin PIO0_29 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin29_sec_mask(&mut self) -> _PIO0_PIN29_SEC_MASKW {
        _PIO0_PIN29_SEC_MASKW { w: self }
    }
    #[doc = "Bit 30 - 0 : Pin PIO0_30 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin30_sec_mask(&mut self) -> _PIO0_PIN30_SEC_MASKW {
        _PIO0_PIN30_SEC_MASKW { w: self }
    }
    #[doc = "Bit 31 - 0 : Pin PIO0_31 state is readable by non-secure world through non-secure GPIO port control registers"]
    #[inline]
    pub fn pio0_pin31_sec_mask(&mut self) -> _PIO0_PIN31_SEC_MASKW {
        _PIO0_PIN31_SEC_MASKW { w: self }
    }
}
