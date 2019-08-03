#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEC_CPU_INT_MASK0 {
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
#[doc = "Possible values of the field `SYS_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYS_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SYS_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SYS_IRQR::INVISIBLE => false,
            SYS_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYS_IRQR {
        match value {
            false => SYS_IRQR::INVISIBLE,
            true => SYS_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == SYS_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == SYS_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `SDMA0_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA0_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SDMA0_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SDMA0_IRQR::INVISIBLE => false,
            SDMA0_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDMA0_IRQR {
        match value {
            false => SDMA0_IRQR::INVISIBLE,
            true => SDMA0_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == SDMA0_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == SDMA0_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `GPIO_GLOBALINT0_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_GLOBALINT0_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_GLOBALINT0_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GPIO_GLOBALINT0_IRQR::INVISIBLE => false,
            GPIO_GLOBALINT0_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_GLOBALINT0_IRQR {
        match value {
            false => GPIO_GLOBALINT0_IRQR::INVISIBLE,
            true => GPIO_GLOBALINT0_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_GLOBALINT0_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_GLOBALINT0_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `GPIO_GLOBALINT1_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_GLOBALINT1_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_GLOBALINT1_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GPIO_GLOBALINT1_IRQR::INVISIBLE => false,
            GPIO_GLOBALINT1_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_GLOBALINT1_IRQR {
        match value {
            false => GPIO_GLOBALINT1_IRQR::INVISIBLE,
            true => GPIO_GLOBALINT1_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_GLOBALINT1_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_GLOBALINT1_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `GPIO_INT0_IRQ0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ0R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ0R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GPIO_INT0_IRQ0R::INVISIBLE => false,
            GPIO_INT0_IRQ0R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_INT0_IRQ0R {
        match value {
            false => GPIO_INT0_IRQ0R::INVISIBLE,
            true => GPIO_INT0_IRQ0R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ0R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ0R::VISIBLE
    }
}
#[doc = "Possible values of the field `GPIO_INT0_IRQ1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ1R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ1R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GPIO_INT0_IRQ1R::INVISIBLE => false,
            GPIO_INT0_IRQ1R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_INT0_IRQ1R {
        match value {
            false => GPIO_INT0_IRQ1R::INVISIBLE,
            true => GPIO_INT0_IRQ1R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ1R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ1R::VISIBLE
    }
}
#[doc = "Possible values of the field `GPIO_INT0_IRQ2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ2R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ2R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GPIO_INT0_IRQ2R::INVISIBLE => false,
            GPIO_INT0_IRQ2R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_INT0_IRQ2R {
        match value {
            false => GPIO_INT0_IRQ2R::INVISIBLE,
            true => GPIO_INT0_IRQ2R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ2R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ2R::VISIBLE
    }
}
#[doc = "Possible values of the field `GPIO_INT0_IRQ3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ3R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ3R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GPIO_INT0_IRQ3R::INVISIBLE => false,
            GPIO_INT0_IRQ3R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_INT0_IRQ3R {
        match value {
            false => GPIO_INT0_IRQ3R::INVISIBLE,
            true => GPIO_INT0_IRQ3R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ3R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ3R::VISIBLE
    }
}
#[doc = "Possible values of the field `UTICK_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UTICK_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl UTICK_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            UTICK_IRQR::INVISIBLE => false,
            UTICK_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UTICK_IRQR {
        match value {
            false => UTICK_IRQR::INVISIBLE,
            true => UTICK_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == UTICK_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == UTICK_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `MRT_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl MRT_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MRT_IRQR::INVISIBLE => false,
            MRT_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MRT_IRQR {
        match value {
            false => MRT_IRQR::INVISIBLE,
            true => MRT_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == MRT_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == MRT_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `CTIMER0_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER0_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl CTIMER0_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CTIMER0_IRQR::INVISIBLE => false,
            CTIMER0_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTIMER0_IRQR {
        match value {
            false => CTIMER0_IRQR::INVISIBLE,
            true => CTIMER0_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == CTIMER0_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == CTIMER0_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `CTIMER1_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER1_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl CTIMER1_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CTIMER1_IRQR::INVISIBLE => false,
            CTIMER1_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTIMER1_IRQR {
        match value {
            false => CTIMER1_IRQR::INVISIBLE,
            true => CTIMER1_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == CTIMER1_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == CTIMER1_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `SCT_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SCT_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SCT_IRQR::INVISIBLE => false,
            SCT_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCT_IRQR {
        match value {
            false => SCT_IRQR::INVISIBLE,
            true => SCT_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == SCT_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == SCT_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `CTIMER3_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER3_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl CTIMER3_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CTIMER3_IRQR::INVISIBLE => false,
            CTIMER3_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTIMER3_IRQR {
        match value {
            false => CTIMER3_IRQR::INVISIBLE,
            true => CTIMER3_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == CTIMER3_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == CTIMER3_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `FLEXCOMM0_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM0_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM0_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FLEXCOMM0_IRQR::INVISIBLE => false,
            FLEXCOMM0_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXCOMM0_IRQR {
        match value {
            false => FLEXCOMM0_IRQR::INVISIBLE,
            true => FLEXCOMM0_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM0_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM0_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `FLEXCOMM1_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM1_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM1_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FLEXCOMM1_IRQR::INVISIBLE => false,
            FLEXCOMM1_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXCOMM1_IRQR {
        match value {
            false => FLEXCOMM1_IRQR::INVISIBLE,
            true => FLEXCOMM1_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM1_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM1_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `FLEXCOMM2_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM2_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM2_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FLEXCOMM2_IRQR::INVISIBLE => false,
            FLEXCOMM2_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXCOMM2_IRQR {
        match value {
            false => FLEXCOMM2_IRQR::INVISIBLE,
            true => FLEXCOMM2_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM2_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM2_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `FLEXCOMM3_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM3_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM3_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FLEXCOMM3_IRQR::INVISIBLE => false,
            FLEXCOMM3_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXCOMM3_IRQR {
        match value {
            false => FLEXCOMM3_IRQR::INVISIBLE,
            true => FLEXCOMM3_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM3_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM3_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `FLEXCOMM4_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM4_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM4_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FLEXCOMM4_IRQR::INVISIBLE => false,
            FLEXCOMM4_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXCOMM4_IRQR {
        match value {
            false => FLEXCOMM4_IRQR::INVISIBLE,
            true => FLEXCOMM4_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM4_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM4_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `FLEXCOMM5_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM5_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM5_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FLEXCOMM5_IRQR::INVISIBLE => false,
            FLEXCOMM5_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXCOMM5_IRQR {
        match value {
            false => FLEXCOMM5_IRQR::INVISIBLE,
            true => FLEXCOMM5_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM5_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM5_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `FLEXCOMM6_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM6_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM6_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FLEXCOMM6_IRQR::INVISIBLE => false,
            FLEXCOMM6_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXCOMM6_IRQR {
        match value {
            false => FLEXCOMM6_IRQR::INVISIBLE,
            true => FLEXCOMM6_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM6_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM6_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `FLEXCOMM7_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM7_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM7_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FLEXCOMM7_IRQR::INVISIBLE => false,
            FLEXCOMM7_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXCOMM7_IRQR {
        match value {
            false => FLEXCOMM7_IRQR::INVISIBLE,
            true => FLEXCOMM7_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == FLEXCOMM7_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == FLEXCOMM7_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `ADC_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl ADC_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ADC_IRQR::INVISIBLE => false,
            ADC_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_IRQR {
        match value {
            false => ADC_IRQR::INVISIBLE,
            true => ADC_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == ADC_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == ADC_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `RESERVED0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED0R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RESERVED0R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RESERVED0R::INVISIBLE => false,
            RESERVED0R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESERVED0R {
        match value {
            false => RESERVED0R::INVISIBLE,
            true => RESERVED0R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED0R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED0R::VISIBLE
    }
}
#[doc = "Possible values of the field `ACMP_CAPT0_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_CAPT0_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl ACMP_CAPT0_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ACMP_CAPT0_IRQR::INVISIBLE => false,
            ACMP_CAPT0_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMP_CAPT0_IRQR {
        match value {
            false => ACMP_CAPT0_IRQR::INVISIBLE,
            true => ACMP_CAPT0_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == ACMP_CAPT0_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == ACMP_CAPT0_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `RESERVED1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED1R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RESERVED1R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RESERVED1R::INVISIBLE => false,
            RESERVED1R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESERVED1R {
        match value {
            false => RESERVED1R::INVISIBLE,
            true => RESERVED1R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED1R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED1R::VISIBLE
    }
}
#[doc = "Possible values of the field `RESERVED2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED2R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RESERVED2R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RESERVED2R::INVISIBLE => false,
            RESERVED2R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESERVED2R {
        match value {
            false => RESERVED2R::INVISIBLE,
            true => RESERVED2R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED2R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED2R::VISIBLE
    }
}
#[doc = "Possible values of the field `USB0_NEEDCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_NEEDCLKR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl USB0_NEEDCLKR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USB0_NEEDCLKR::INVISIBLE => false,
            USB0_NEEDCLKR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0_NEEDCLKR {
        match value {
            false => USB0_NEEDCLKR::INVISIBLE,
            true => USB0_NEEDCLKR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == USB0_NEEDCLKR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == USB0_NEEDCLKR::VISIBLE
    }
}
#[doc = "Possible values of the field `USB0_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl USB0_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USB0_IRQR::INVISIBLE => false,
            USB0_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB0_IRQR {
        match value {
            false => USB0_IRQR::INVISIBLE,
            true => USB0_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == USB0_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == USB0_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `RTC_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RTC_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RTC_IRQR::INVISIBLE => false,
            RTC_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_IRQR {
        match value {
            false => RTC_IRQR::INVISIBLE,
            true => RTC_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == RTC_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == RTC_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `RESERVED3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED3R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RESERVED3R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RESERVED3R::INVISIBLE => false,
            RESERVED3R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESERVED3R {
        match value {
            false => RESERVED3R::INVISIBLE,
            true => RESERVED3R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED3R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED3R::VISIBLE
    }
}
#[doc = "Possible values of the field `MAILBOX_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAILBOX_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl MAILBOX_IRQR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MAILBOX_IRQR::INVISIBLE => false,
            MAILBOX_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAILBOX_IRQR {
        match value {
            false => MAILBOX_IRQR::INVISIBLE,
            true => MAILBOX_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == MAILBOX_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == MAILBOX_IRQR::VISIBLE
    }
}
#[doc = "Values that can be written to the field `SYS_IRQ`"]
pub enum SYS_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SYS_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYS_IRQW::INVISIBLE => false,
            SYS_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYS_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYS_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SYS_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(SYS_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `SDMA0_IRQ`"]
pub enum SDMA0_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SDMA0_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDMA0_IRQW::INVISIBLE => false,
            SDMA0_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDMA0_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMA0_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDMA0_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SDMA0_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(SDMA0_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `GPIO_GLOBALINT0_IRQ`"]
pub enum GPIO_GLOBALINT0_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_GLOBALINT0_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_GLOBALINT0_IRQW::INVISIBLE => false,
            GPIO_GLOBALINT0_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GLOBALINT0_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GLOBALINT0_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GLOBALINT0_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_GLOBALINT0_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_GLOBALINT0_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `GPIO_GLOBALINT1_IRQ`"]
pub enum GPIO_GLOBALINT1_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_GLOBALINT1_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_GLOBALINT1_IRQW::INVISIBLE => false,
            GPIO_GLOBALINT1_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_GLOBALINT1_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_GLOBALINT1_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_GLOBALINT1_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_GLOBALINT1_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_GLOBALINT1_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `GPIO_INT0_IRQ0`"]
pub enum GPIO_INT0_IRQ0W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_INT0_IRQ0W::INVISIBLE => false,
            GPIO_INT0_IRQ0W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT0_IRQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT0_IRQ0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_INT0_IRQ0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ0W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ0W::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `GPIO_INT0_IRQ1`"]
pub enum GPIO_INT0_IRQ1W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_INT0_IRQ1W::INVISIBLE => false,
            GPIO_INT0_IRQ1W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT0_IRQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT0_IRQ1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_INT0_IRQ1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ1W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ1W::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `GPIO_INT0_IRQ2`"]
pub enum GPIO_INT0_IRQ2W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_INT0_IRQ2W::INVISIBLE => false,
            GPIO_INT0_IRQ2W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT0_IRQ2W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT0_IRQ2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_INT0_IRQ2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ2W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ2W::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `GPIO_INT0_IRQ3`"]
pub enum GPIO_INT0_IRQ3W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_INT0_IRQ3W::INVISIBLE => false,
            GPIO_INT0_IRQ3W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT0_IRQ3W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT0_IRQ3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_INT0_IRQ3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ3W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ3W::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `UTICK_IRQ`"]
pub enum UTICK_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl UTICK_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UTICK_IRQW::INVISIBLE => false,
            UTICK_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UTICK_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _UTICK_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UTICK_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(UTICK_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(UTICK_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `MRT_IRQ`"]
pub enum MRT_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl MRT_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MRT_IRQW::INVISIBLE => false,
            MRT_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MRT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _MRT_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MRT_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(MRT_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(MRT_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `CTIMER0_IRQ`"]
pub enum CTIMER0_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl CTIMER0_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTIMER0_IRQW::INVISIBLE => false,
            CTIMER0_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER0_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER0_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER0_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER0_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER0_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `CTIMER1_IRQ`"]
pub enum CTIMER1_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl CTIMER1_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTIMER1_IRQW::INVISIBLE => false,
            CTIMER1_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER1_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER1_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER1_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER1_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER1_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `SCT_IRQ`"]
pub enum SCT_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SCT_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCT_IRQW::INVISIBLE => false,
            SCT_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SCT_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCT_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SCT_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(SCT_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `CTIMER3_IRQ`"]
pub enum CTIMER3_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl CTIMER3_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTIMER3_IRQW::INVISIBLE => false,
            CTIMER3_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER3_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER3_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER3_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER3_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER3_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `FLEXCOMM0_IRQ`"]
pub enum FLEXCOMM0_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM0_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXCOMM0_IRQW::INVISIBLE => false,
            FLEXCOMM0_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXCOMM0_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM0_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXCOMM0_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM0_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM0_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `FLEXCOMM1_IRQ`"]
pub enum FLEXCOMM1_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM1_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXCOMM1_IRQW::INVISIBLE => false,
            FLEXCOMM1_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXCOMM1_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM1_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXCOMM1_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM1_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM1_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `FLEXCOMM2_IRQ`"]
pub enum FLEXCOMM2_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM2_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXCOMM2_IRQW::INVISIBLE => false,
            FLEXCOMM2_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXCOMM2_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM2_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXCOMM2_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM2_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM2_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `FLEXCOMM3_IRQ`"]
pub enum FLEXCOMM3_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM3_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXCOMM3_IRQW::INVISIBLE => false,
            FLEXCOMM3_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXCOMM3_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM3_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXCOMM3_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM3_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM3_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `FLEXCOMM4_IRQ`"]
pub enum FLEXCOMM4_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM4_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXCOMM4_IRQW::INVISIBLE => false,
            FLEXCOMM4_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXCOMM4_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM4_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXCOMM4_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM4_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM4_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `FLEXCOMM5_IRQ`"]
pub enum FLEXCOMM5_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM5_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXCOMM5_IRQW::INVISIBLE => false,
            FLEXCOMM5_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXCOMM5_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM5_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXCOMM5_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM5_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM5_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `FLEXCOMM6_IRQ`"]
pub enum FLEXCOMM6_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM6_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXCOMM6_IRQW::INVISIBLE => false,
            FLEXCOMM6_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXCOMM6_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM6_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXCOMM6_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM6_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM6_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `FLEXCOMM7_IRQ`"]
pub enum FLEXCOMM7_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl FLEXCOMM7_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXCOMM7_IRQW::INVISIBLE => false,
            FLEXCOMM7_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXCOMM7_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM7_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXCOMM7_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM7_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM7_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `ADC_IRQ`"]
pub enum ADC_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl ADC_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_IRQW::INVISIBLE => false,
            ADC_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(ADC_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(ADC_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `RESERVED0`"]
pub enum RESERVED0W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RESERVED0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESERVED0W::INVISIBLE => false,
            RESERVED0W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED0W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESERVED0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED0W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED0W::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `ACMP_CAPT0_IRQ`"]
pub enum ACMP_CAPT0_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl ACMP_CAPT0_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMP_CAPT0_IRQW::INVISIBLE => false,
            ACMP_CAPT0_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP_CAPT0_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP_CAPT0_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP_CAPT0_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(ACMP_CAPT0_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(ACMP_CAPT0_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `RESERVED1`"]
pub enum RESERVED1W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RESERVED1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESERVED1W::INVISIBLE => false,
            RESERVED1W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED1W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESERVED1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED1W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED1W::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `RESERVED2`"]
pub enum RESERVED2W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RESERVED2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESERVED2W::INVISIBLE => false,
            RESERVED2W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESERVED2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED2W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED2W::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `USB0_NEEDCLK`"]
pub enum USB0_NEEDCLKW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl USB0_NEEDCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0_NEEDCLKW::INVISIBLE => false,
            USB0_NEEDCLKW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0_NEEDCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_NEEDCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0_NEEDCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB0_NEEDCLKW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB0_NEEDCLKW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `USB0_IRQ`"]
pub enum USB0_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl USB0_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB0_IRQW::INVISIBLE => false,
            USB0_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB0_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB0_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB0_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB0_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `RTC_IRQ`"]
pub enum RTC_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RTC_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_IRQW::INVISIBLE => false,
            RTC_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RTC_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(RTC_IRQW::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `RESERVED3`"]
pub enum RESERVED3W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RESERVED3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESERVED3W::INVISIBLE => false,
            RESERVED3W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESERVED3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED3W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED3W::VISIBLE)
    }
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
#[doc = "Values that can be written to the field `MAILBOX_IRQ`"]
pub enum MAILBOX_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl MAILBOX_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAILBOX_IRQW::INVISIBLE => false,
            MAILBOX_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAILBOX_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _MAILBOX_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAILBOX_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(MAILBOX_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(MAILBOX_IRQW::VISIBLE)
    }
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
    #[doc = "Bit 0 - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
    #[inline]
    pub fn sys_irq(&self) -> SYS_IRQR {
        SYS_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - System DMA 0 (non-secure) interrupt."]
    #[inline]
    pub fn sdma0_irq(&self) -> SDMA0_IRQR {
        SDMA0_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - GPIO Group 0 interrupt."]
    #[inline]
    pub fn gpio_globalint0_irq(&self) -> GPIO_GLOBALINT0_IRQR {
        GPIO_GLOBALINT0_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - GPIO Group 1 interrupt."]
    #[inline]
    pub fn gpio_globalint1_irq(&self) -> GPIO_GLOBALINT1_IRQR {
        GPIO_GLOBALINT1_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline]
    pub fn gpio_int0_irq0(&self) -> GPIO_INT0_IRQ0R {
        GPIO_INT0_IRQ0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline]
    pub fn gpio_int0_irq1(&self) -> GPIO_INT0_IRQ1R {
        GPIO_INT0_IRQ1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
    #[inline]
    pub fn gpio_int0_irq2(&self) -> GPIO_INT0_IRQ2R {
        GPIO_INT0_IRQ2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
    #[inline]
    pub fn gpio_int0_irq3(&self) -> GPIO_INT0_IRQ3R {
        GPIO_INT0_IRQ3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Micro Tick Timer interrupt."]
    #[inline]
    pub fn utick_irq(&self) -> UTICK_IRQR {
        UTICK_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Multi-Rate Timer interrupt."]
    #[inline]
    pub fn mrt_irq(&self) -> MRT_IRQR {
        MRT_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Standard counter/timer 0 interrupt."]
    #[inline]
    pub fn ctimer0_irq(&self) -> CTIMER0_IRQR {
        CTIMER0_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Standard counter/timer 1 interrupt."]
    #[inline]
    pub fn ctimer1_irq(&self) -> CTIMER1_IRQR {
        CTIMER1_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SCTimer/PWM interrupt."]
    #[inline]
    pub fn sct_irq(&self) -> SCT_IRQR {
        SCT_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Standard counter/timer 3 interrupt."]
    #[inline]
    pub fn ctimer3_irq(&self) -> CTIMER3_IRQR {
        CTIMER3_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm0_irq(&self) -> FLEXCOMM0_IRQR {
        FLEXCOMM0_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm1_irq(&self) -> FLEXCOMM1_IRQR {
        FLEXCOMM1_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm2_irq(&self) -> FLEXCOMM2_IRQR {
        FLEXCOMM2_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm3_irq(&self) -> FLEXCOMM3_IRQR {
        FLEXCOMM3_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm4_irq(&self) -> FLEXCOMM4_IRQR {
        FLEXCOMM4_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm5_irq(&self) -> FLEXCOMM5_IRQR {
        FLEXCOMM5_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm6_irq(&self) -> FLEXCOMM6_IRQR {
        FLEXCOMM6_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm7_irq(&self) -> FLEXCOMM7_IRQR {
        FLEXCOMM7_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - General Purpose ADC interrupt."]
    #[inline]
    pub fn adc_irq(&self) -> ADC_IRQR {
        ADC_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved0(&self) -> RESERVED0R {
        RESERVED0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Analog Comparator interrupt."]
    #[inline]
    pub fn acmp_capt0_irq(&self) -> ACMP_CAPT0_IRQR {
        ACMP_CAPT0_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        RESERVED1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        RESERVED2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - USB Full Speed Controller Clock request interrupt."]
    #[inline]
    pub fn usb0_needclk(&self) -> USB0_NEEDCLKR {
        USB0_NEEDCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - USB High Speed Controller interrupt."]
    #[inline]
    pub fn usb0_irq(&self) -> USB0_IRQR {
        USB0_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
    #[inline]
    pub fn rtc_irq(&self) -> RTC_IRQR {
        RTC_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        RESERVED3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Mailbox interrupt."]
    #[inline]
    pub fn mailbox_irq(&self) -> MAILBOX_IRQR {
        MAILBOX_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 0 - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
    #[inline]
    pub fn sys_irq(&mut self) -> _SYS_IRQW {
        _SYS_IRQW { w: self }
    }
    #[doc = "Bit 1 - System DMA 0 (non-secure) interrupt."]
    #[inline]
    pub fn sdma0_irq(&mut self) -> _SDMA0_IRQW {
        _SDMA0_IRQW { w: self }
    }
    #[doc = "Bit 2 - GPIO Group 0 interrupt."]
    #[inline]
    pub fn gpio_globalint0_irq(&mut self) -> _GPIO_GLOBALINT0_IRQW {
        _GPIO_GLOBALINT0_IRQW { w: self }
    }
    #[doc = "Bit 3 - GPIO Group 1 interrupt."]
    #[inline]
    pub fn gpio_globalint1_irq(&mut self) -> _GPIO_GLOBALINT1_IRQW {
        _GPIO_GLOBALINT1_IRQW { w: self }
    }
    #[doc = "Bit 4 - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline]
    pub fn gpio_int0_irq0(&mut self) -> _GPIO_INT0_IRQ0W {
        _GPIO_INT0_IRQ0W { w: self }
    }
    #[doc = "Bit 5 - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline]
    pub fn gpio_int0_irq1(&mut self) -> _GPIO_INT0_IRQ1W {
        _GPIO_INT0_IRQ1W { w: self }
    }
    #[doc = "Bit 6 - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
    #[inline]
    pub fn gpio_int0_irq2(&mut self) -> _GPIO_INT0_IRQ2W {
        _GPIO_INT0_IRQ2W { w: self }
    }
    #[doc = "Bit 7 - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
    #[inline]
    pub fn gpio_int0_irq3(&mut self) -> _GPIO_INT0_IRQ3W {
        _GPIO_INT0_IRQ3W { w: self }
    }
    #[doc = "Bit 8 - Micro Tick Timer interrupt."]
    #[inline]
    pub fn utick_irq(&mut self) -> _UTICK_IRQW {
        _UTICK_IRQW { w: self }
    }
    #[doc = "Bit 9 - Multi-Rate Timer interrupt."]
    #[inline]
    pub fn mrt_irq(&mut self) -> _MRT_IRQW {
        _MRT_IRQW { w: self }
    }
    #[doc = "Bit 10 - Standard counter/timer 0 interrupt."]
    #[inline]
    pub fn ctimer0_irq(&mut self) -> _CTIMER0_IRQW {
        _CTIMER0_IRQW { w: self }
    }
    #[doc = "Bit 11 - Standard counter/timer 1 interrupt."]
    #[inline]
    pub fn ctimer1_irq(&mut self) -> _CTIMER1_IRQW {
        _CTIMER1_IRQW { w: self }
    }
    #[doc = "Bit 12 - SCTimer/PWM interrupt."]
    #[inline]
    pub fn sct_irq(&mut self) -> _SCT_IRQW {
        _SCT_IRQW { w: self }
    }
    #[doc = "Bit 13 - Standard counter/timer 3 interrupt."]
    #[inline]
    pub fn ctimer3_irq(&mut self) -> _CTIMER3_IRQW {
        _CTIMER3_IRQW { w: self }
    }
    #[doc = "Bit 14 - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm0_irq(&mut self) -> _FLEXCOMM0_IRQW {
        _FLEXCOMM0_IRQW { w: self }
    }
    #[doc = "Bit 15 - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm1_irq(&mut self) -> _FLEXCOMM1_IRQW {
        _FLEXCOMM1_IRQW { w: self }
    }
    #[doc = "Bit 16 - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm2_irq(&mut self) -> _FLEXCOMM2_IRQW {
        _FLEXCOMM2_IRQW { w: self }
    }
    #[doc = "Bit 17 - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm3_irq(&mut self) -> _FLEXCOMM3_IRQW {
        _FLEXCOMM3_IRQW { w: self }
    }
    #[doc = "Bit 18 - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm4_irq(&mut self) -> _FLEXCOMM4_IRQW {
        _FLEXCOMM4_IRQW { w: self }
    }
    #[doc = "Bit 19 - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm5_irq(&mut self) -> _FLEXCOMM5_IRQW {
        _FLEXCOMM5_IRQW { w: self }
    }
    #[doc = "Bit 20 - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm6_irq(&mut self) -> _FLEXCOMM6_IRQW {
        _FLEXCOMM6_IRQW { w: self }
    }
    #[doc = "Bit 21 - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
    #[inline]
    pub fn flexcomm7_irq(&mut self) -> _FLEXCOMM7_IRQW {
        _FLEXCOMM7_IRQW { w: self }
    }
    #[doc = "Bit 22 - General Purpose ADC interrupt."]
    #[inline]
    pub fn adc_irq(&mut self) -> _ADC_IRQW {
        _ADC_IRQW { w: self }
    }
    #[doc = "Bit 23 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved0(&mut self) -> _RESERVED0W {
        _RESERVED0W { w: self }
    }
    #[doc = "Bit 24 - Analog Comparator interrupt."]
    #[inline]
    pub fn acmp_capt0_irq(&mut self) -> _ACMP_CAPT0_IRQW {
        _ACMP_CAPT0_IRQW { w: self }
    }
    #[doc = "Bit 25 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved1(&mut self) -> _RESERVED1W {
        _RESERVED1W { w: self }
    }
    #[doc = "Bit 26 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bit 27 - USB Full Speed Controller Clock request interrupt."]
    #[inline]
    pub fn usb0_needclk(&mut self) -> _USB0_NEEDCLKW {
        _USB0_NEEDCLKW { w: self }
    }
    #[doc = "Bit 28 - USB High Speed Controller interrupt."]
    #[inline]
    pub fn usb0_irq(&mut self) -> _USB0_IRQW {
        _USB0_IRQW { w: self }
    }
    #[doc = "Bit 29 - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
    #[inline]
    pub fn rtc_irq(&mut self) -> _RTC_IRQW {
        _RTC_IRQW { w: self }
    }
    #[doc = "Bit 30 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 31 - Mailbox interrupt."]
    #[inline]
    pub fn mailbox_irq(&mut self) -> _MAILBOX_IRQW {
        _MAILBOX_IRQW { w: self }
    }
}
