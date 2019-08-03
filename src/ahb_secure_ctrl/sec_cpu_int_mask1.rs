#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEC_CPU_INT_MASK1 {
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
#[doc = "Possible values of the field `GPIO_INT0_IRQ4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ4R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ4R {
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
            GPIO_INT0_IRQ4R::INVISIBLE => false,
            GPIO_INT0_IRQ4R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_INT0_IRQ4R {
        match value {
            false => GPIO_INT0_IRQ4R::INVISIBLE,
            true => GPIO_INT0_IRQ4R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ4R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ4R::VISIBLE
    }
}
#[doc = "Possible values of the field `GPIO_INT0_IRQ5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ5R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ5R {
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
            GPIO_INT0_IRQ5R::INVISIBLE => false,
            GPIO_INT0_IRQ5R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_INT0_IRQ5R {
        match value {
            false => GPIO_INT0_IRQ5R::INVISIBLE,
            true => GPIO_INT0_IRQ5R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ5R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ5R::VISIBLE
    }
}
#[doc = "Possible values of the field `GPIO_INT0_IRQ6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ6R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ6R {
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
            GPIO_INT0_IRQ6R::INVISIBLE => false,
            GPIO_INT0_IRQ6R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_INT0_IRQ6R {
        match value {
            false => GPIO_INT0_IRQ6R::INVISIBLE,
            true => GPIO_INT0_IRQ6R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ6R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ6R::VISIBLE
    }
}
#[doc = "Possible values of the field `GPIO_INT0_IRQ7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ7R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ7R {
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
            GPIO_INT0_IRQ7R::INVISIBLE => false,
            GPIO_INT0_IRQ7R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO_INT0_IRQ7R {
        match value {
            false => GPIO_INT0_IRQ7R::INVISIBLE,
            true => GPIO_INT0_IRQ7R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ7R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ7R::VISIBLE
    }
}
#[doc = "Possible values of the field `CTIMER2_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER2_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl CTIMER2_IRQR {
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
            CTIMER2_IRQR::INVISIBLE => false,
            CTIMER2_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTIMER2_IRQR {
        match value {
            false => CTIMER2_IRQR::INVISIBLE,
            true => CTIMER2_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == CTIMER2_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == CTIMER2_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `CTIMER4_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER4_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl CTIMER4_IRQR {
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
            CTIMER4_IRQR::INVISIBLE => false,
            CTIMER4_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTIMER4_IRQR {
        match value {
            false => CTIMER4_IRQR::INVISIBLE,
            true => CTIMER4_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == CTIMER4_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == CTIMER4_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `OS_EVENT_TIMER_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OS_EVENT_TIMER_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl OS_EVENT_TIMER_IRQR {
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
            OS_EVENT_TIMER_IRQR::INVISIBLE => false,
            OS_EVENT_TIMER_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OS_EVENT_TIMER_IRQR {
        match value {
            false => OS_EVENT_TIMER_IRQR::INVISIBLE,
            true => OS_EVENT_TIMER_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == OS_EVENT_TIMER_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == OS_EVENT_TIMER_IRQR::VISIBLE
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
#[doc = "Possible values of the field `SDIO_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIO_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SDIO_IRQR {
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
            SDIO_IRQR::INVISIBLE => false,
            SDIO_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDIO_IRQR {
        match value {
            false => SDIO_IRQR::INVISIBLE,
            true => SDIO_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == SDIO_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == SDIO_IRQR::VISIBLE
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
#[doc = "Possible values of the field `RESERVED4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED4R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RESERVED4R {
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
            RESERVED4R::INVISIBLE => false,
            RESERVED4R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESERVED4R {
        match value {
            false => RESERVED4R::INVISIBLE,
            true => RESERVED4R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED4R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED4R::VISIBLE
    }
}
#[doc = "Possible values of the field `RESERVED5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED5R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RESERVED5R {
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
            RESERVED5R::INVISIBLE => false,
            RESERVED5R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESERVED5R {
        match value {
            false => RESERVED5R::INVISIBLE,
            true => RESERVED5R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED5R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED5R::VISIBLE
    }
}
#[doc = "Possible values of the field `USB1_UTMI_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_UTMI_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl USB1_UTMI_IRQR {
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
            USB1_UTMI_IRQR::INVISIBLE => false,
            USB1_UTMI_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_UTMI_IRQR {
        match value {
            false => USB1_UTMI_IRQR::INVISIBLE,
            true => USB1_UTMI_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == USB1_UTMI_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == USB1_UTMI_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `USB1_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl USB1_IRQR {
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
            USB1_IRQR::INVISIBLE => false,
            USB1_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_IRQR {
        match value {
            false => USB1_IRQR::INVISIBLE,
            true => USB1_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == USB1_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == USB1_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `USB1_NEEDCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_NEEDCLKR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl USB1_NEEDCLKR {
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
            USB1_NEEDCLKR::INVISIBLE => false,
            USB1_NEEDCLKR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB1_NEEDCLKR {
        match value {
            false => USB1_NEEDCLKR::INVISIBLE,
            true => USB1_NEEDCLKR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == USB1_NEEDCLKR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == USB1_NEEDCLKR::VISIBLE
    }
}
#[doc = "Possible values of the field `SEC_HYPERVISOR_CALL_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_HYPERVISOR_CALL_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SEC_HYPERVISOR_CALL_IRQR {
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
            SEC_HYPERVISOR_CALL_IRQR::INVISIBLE => false,
            SEC_HYPERVISOR_CALL_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEC_HYPERVISOR_CALL_IRQR {
        match value {
            false => SEC_HYPERVISOR_CALL_IRQR::INVISIBLE,
            true => SEC_HYPERVISOR_CALL_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == SEC_HYPERVISOR_CALL_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == SEC_HYPERVISOR_CALL_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `SEC_GPIO_INT0_IRQ0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_GPIO_INT0_IRQ0R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SEC_GPIO_INT0_IRQ0R {
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
            SEC_GPIO_INT0_IRQ0R::INVISIBLE => false,
            SEC_GPIO_INT0_IRQ0R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEC_GPIO_INT0_IRQ0R {
        match value {
            false => SEC_GPIO_INT0_IRQ0R::INVISIBLE,
            true => SEC_GPIO_INT0_IRQ0R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == SEC_GPIO_INT0_IRQ0R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == SEC_GPIO_INT0_IRQ0R::VISIBLE
    }
}
#[doc = "Possible values of the field `SEC_GPIO_INT0_IRQ1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_GPIO_INT0_IRQ1R {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SEC_GPIO_INT0_IRQ1R {
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
            SEC_GPIO_INT0_IRQ1R::INVISIBLE => false,
            SEC_GPIO_INT0_IRQ1R::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEC_GPIO_INT0_IRQ1R {
        match value {
            false => SEC_GPIO_INT0_IRQ1R::INVISIBLE,
            true => SEC_GPIO_INT0_IRQ1R::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == SEC_GPIO_INT0_IRQ1R::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == SEC_GPIO_INT0_IRQ1R::VISIBLE
    }
}
#[doc = "Possible values of the field `PLU_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLU_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl PLU_IRQR {
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
            PLU_IRQR::INVISIBLE => false,
            PLU_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLU_IRQR {
        match value {
            false => PLU_IRQR::INVISIBLE,
            true => PLU_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == PLU_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == PLU_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `SEC_VIO_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_VIO_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SEC_VIO_IRQR {
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
            SEC_VIO_IRQR::INVISIBLE => false,
            SEC_VIO_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEC_VIO_IRQR {
        match value {
            false => SEC_VIO_IRQR::INVISIBLE,
            true => SEC_VIO_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == SEC_VIO_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == SEC_VIO_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `SHA_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHA_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SHA_IRQR {
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
            SHA_IRQR::INVISIBLE => false,
            SHA_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SHA_IRQR {
        match value {
            false => SHA_IRQR::INVISIBLE,
            true => SHA_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == SHA_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == SHA_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `CASPER_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASPER_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl CASPER_IRQR {
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
            CASPER_IRQR::INVISIBLE => false,
            CASPER_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CASPER_IRQR {
        match value {
            false => CASPER_IRQR::INVISIBLE,
            true => CASPER_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == CASPER_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == CASPER_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `QDDKEY_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QDDKEY_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl QDDKEY_IRQR {
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
            QDDKEY_IRQR::INVISIBLE => false,
            QDDKEY_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QDDKEY_IRQR {
        match value {
            false => QDDKEY_IRQR::INVISIBLE,
            true => QDDKEY_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == QDDKEY_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == QDDKEY_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `PQ_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PQ_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl PQ_IRQR {
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
            PQ_IRQR::INVISIBLE => false,
            PQ_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PQ_IRQR {
        match value {
            false => PQ_IRQR::INVISIBLE,
            true => PQ_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == PQ_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == PQ_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `SDMA1_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA1_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SDMA1_IRQR {
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
            SDMA1_IRQR::INVISIBLE => false,
            SDMA1_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDMA1_IRQR {
        match value {
            false => SDMA1_IRQR::INVISIBLE,
            true => SDMA1_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == SDMA1_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == SDMA1_IRQR::VISIBLE
    }
}
#[doc = "Possible values of the field `LSPI_HS_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPI_HS_IRQR {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl LSPI_HS_IRQR {
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
            LSPI_HS_IRQR::INVISIBLE => false,
            LSPI_HS_IRQR::VISIBLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSPI_HS_IRQR {
        match value {
            false => LSPI_HS_IRQR::INVISIBLE,
            true => LSPI_HS_IRQR::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline]
    pub fn is_invisible(&self) -> bool {
        *self == LSPI_HS_IRQR::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline]
    pub fn is_visible(&self) -> bool {
        *self == LSPI_HS_IRQR::VISIBLE
    }
}
#[doc = "Values that can be written to the field `GPIO_INT0_IRQ4`"]
pub enum GPIO_INT0_IRQ4W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_INT0_IRQ4W::INVISIBLE => false,
            GPIO_INT0_IRQ4W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT0_IRQ4W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT0_IRQ4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_INT0_IRQ4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ4W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ4W::VISIBLE)
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
#[doc = "Values that can be written to the field `GPIO_INT0_IRQ5`"]
pub enum GPIO_INT0_IRQ5W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_INT0_IRQ5W::INVISIBLE => false,
            GPIO_INT0_IRQ5W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT0_IRQ5W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT0_IRQ5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_INT0_IRQ5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ5W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ5W::VISIBLE)
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
#[doc = "Values that can be written to the field `GPIO_INT0_IRQ6`"]
pub enum GPIO_INT0_IRQ6W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_INT0_IRQ6W::INVISIBLE => false,
            GPIO_INT0_IRQ6W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT0_IRQ6W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT0_IRQ6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_INT0_IRQ6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ6W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ6W::VISIBLE)
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
#[doc = "Values that can be written to the field `GPIO_INT0_IRQ7`"]
pub enum GPIO_INT0_IRQ7W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl GPIO_INT0_IRQ7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIO_INT0_IRQ7W::INVISIBLE => false,
            GPIO_INT0_IRQ7W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_INT0_IRQ7W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_INT0_IRQ7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_INT0_IRQ7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ7W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ7W::VISIBLE)
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
#[doc = "Values that can be written to the field `CTIMER2_IRQ`"]
pub enum CTIMER2_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl CTIMER2_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTIMER2_IRQW::INVISIBLE => false,
            CTIMER2_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER2_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER2_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER2_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER2_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER2_IRQW::VISIBLE)
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
#[doc = "Values that can be written to the field `CTIMER4_IRQ`"]
pub enum CTIMER4_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl CTIMER4_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTIMER4_IRQW::INVISIBLE => false,
            CTIMER4_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTIMER4_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER4_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTIMER4_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER4_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER4_IRQW::VISIBLE)
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
#[doc = "Values that can be written to the field `OS_EVENT_TIMER_IRQ`"]
pub enum OS_EVENT_TIMER_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl OS_EVENT_TIMER_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OS_EVENT_TIMER_IRQW::INVISIBLE => false,
            OS_EVENT_TIMER_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OS_EVENT_TIMER_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _OS_EVENT_TIMER_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OS_EVENT_TIMER_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_IRQW::VISIBLE)
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
        const OFFSET: u8 = 7;
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
        const OFFSET: u8 = 8;
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDIO_IRQ`"]
pub enum SDIO_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SDIO_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDIO_IRQW::INVISIBLE => false,
            SDIO_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDIO_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIO_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDIO_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SDIO_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(SDIO_IRQW::VISIBLE)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESERVED4`"]
pub enum RESERVED4W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RESERVED4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESERVED4W::INVISIBLE => false,
            RESERVED4W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED4W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESERVED4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED4W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED4W::VISIBLE)
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
#[doc = "Values that can be written to the field `RESERVED5`"]
pub enum RESERVED5W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl RESERVED5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESERVED5W::INVISIBLE => false,
            RESERVED5W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED5W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESERVED5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED5W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED5W::VISIBLE)
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
#[doc = "Values that can be written to the field `USB1_UTMI_IRQ`"]
pub enum USB1_UTMI_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl USB1_UTMI_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_UTMI_IRQW::INVISIBLE => false,
            USB1_UTMI_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_UTMI_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_UTMI_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_UTMI_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB1_UTMI_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB1_UTMI_IRQW::VISIBLE)
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
#[doc = "Values that can be written to the field `USB1_IRQ`"]
pub enum USB1_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl USB1_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_IRQW::INVISIBLE => false,
            USB1_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB1_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB1_IRQW::VISIBLE)
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
#[doc = "Values that can be written to the field `USB1_NEEDCLK`"]
pub enum USB1_NEEDCLKW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl USB1_NEEDCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB1_NEEDCLKW::INVISIBLE => false,
            USB1_NEEDCLKW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB1_NEEDCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_NEEDCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB1_NEEDCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB1_NEEDCLKW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB1_NEEDCLKW::VISIBLE)
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
#[doc = "Values that can be written to the field `SEC_HYPERVISOR_CALL_IRQ`"]
pub enum SEC_HYPERVISOR_CALL_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SEC_HYPERVISOR_CALL_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEC_HYPERVISOR_CALL_IRQW::INVISIBLE => false,
            SEC_HYPERVISOR_CALL_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_HYPERVISOR_CALL_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_HYPERVISOR_CALL_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_HYPERVISOR_CALL_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SEC_HYPERVISOR_CALL_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(SEC_HYPERVISOR_CALL_IRQW::VISIBLE)
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
#[doc = "Values that can be written to the field `SEC_GPIO_INT0_IRQ0`"]
pub enum SEC_GPIO_INT0_IRQ0W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SEC_GPIO_INT0_IRQ0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEC_GPIO_INT0_IRQ0W::INVISIBLE => false,
            SEC_GPIO_INT0_IRQ0W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_GPIO_INT0_IRQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_GPIO_INT0_IRQ0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_GPIO_INT0_IRQ0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT0_IRQ0W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT0_IRQ0W::VISIBLE)
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
#[doc = "Values that can be written to the field `SEC_GPIO_INT0_IRQ1`"]
pub enum SEC_GPIO_INT0_IRQ1W {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SEC_GPIO_INT0_IRQ1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEC_GPIO_INT0_IRQ1W::INVISIBLE => false,
            SEC_GPIO_INT0_IRQ1W::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_GPIO_INT0_IRQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_GPIO_INT0_IRQ1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_GPIO_INT0_IRQ1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT0_IRQ1W::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT0_IRQ1W::VISIBLE)
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
#[doc = "Values that can be written to the field `PLU_IRQ`"]
pub enum PLU_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl PLU_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLU_IRQW::INVISIBLE => false,
            PLU_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLU_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _PLU_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLU_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(PLU_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(PLU_IRQW::VISIBLE)
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
#[doc = "Values that can be written to the field `SEC_VIO_IRQ`"]
pub enum SEC_VIO_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SEC_VIO_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEC_VIO_IRQW::INVISIBLE => false,
            SEC_VIO_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEC_VIO_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SEC_VIO_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEC_VIO_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SEC_VIO_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(SEC_VIO_IRQW::VISIBLE)
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
#[doc = "Values that can be written to the field `SHA_IRQ`"]
pub enum SHA_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SHA_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SHA_IRQW::INVISIBLE => false,
            SHA_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SHA_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SHA_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SHA_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SHA_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(SHA_IRQW::VISIBLE)
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
#[doc = "Values that can be written to the field `CASPER_IRQ`"]
pub enum CASPER_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl CASPER_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CASPER_IRQW::INVISIBLE => false,
            CASPER_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CASPER_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _CASPER_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CASPER_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CASPER_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(CASPER_IRQW::VISIBLE)
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
#[doc = "Values that can be written to the field `QDDKEY_IRQ`"]
pub enum QDDKEY_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl QDDKEY_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QDDKEY_IRQW::INVISIBLE => false,
            QDDKEY_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QDDKEY_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _QDDKEY_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QDDKEY_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(QDDKEY_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(QDDKEY_IRQW::VISIBLE)
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
#[doc = "Values that can be written to the field `PQ_IRQ`"]
pub enum PQ_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl PQ_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PQ_IRQW::INVISIBLE => false,
            PQ_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PQ_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _PQ_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PQ_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(PQ_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(PQ_IRQW::VISIBLE)
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
#[doc = "Values that can be written to the field `SDMA1_IRQ`"]
pub enum SDMA1_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl SDMA1_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDMA1_IRQW::INVISIBLE => false,
            SDMA1_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDMA1_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMA1_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDMA1_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SDMA1_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(SDMA1_IRQW::VISIBLE)
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
#[doc = "Values that can be written to the field `LSPI_HS_IRQ`"]
pub enum LSPI_HS_IRQW {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl LSPI_HS_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSPI_HS_IRQW::INVISIBLE => false,
            LSPI_HS_IRQW::VISIBLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSPI_HS_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPI_HS_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSPI_HS_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn invisible(self) -> &'a mut W {
        self.variant(LSPI_HS_IRQW::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline]
    pub fn visible(self) -> &'a mut W {
        self.variant(LSPI_HS_IRQW::VISIBLE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Pin interrupt 4 or pattern match engine slice 4 interrupt."]
    #[inline]
    pub fn gpio_int0_irq4(&self) -> GPIO_INT0_IRQ4R {
        GPIO_INT0_IRQ4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pin interrupt 5 or pattern match engine slice 5 interrupt."]
    #[inline]
    pub fn gpio_int0_irq5(&self) -> GPIO_INT0_IRQ5R {
        GPIO_INT0_IRQ5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Pin interrupt 6 or pattern match engine slice 6 interrupt."]
    #[inline]
    pub fn gpio_int0_irq6(&self) -> GPIO_INT0_IRQ6R {
        GPIO_INT0_IRQ6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Pin interrupt 7 or pattern match engine slice 7 interrupt."]
    #[inline]
    pub fn gpio_int0_irq7(&self) -> GPIO_INT0_IRQ7R {
        GPIO_INT0_IRQ7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Standard counter/timer 2 interrupt."]
    #[inline]
    pub fn ctimer2_irq(&self) -> CTIMER2_IRQR {
        CTIMER2_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Standard counter/timer 4 interrupt."]
    #[inline]
    pub fn ctimer4_irq(&self) -> CTIMER4_IRQR {
        CTIMER4_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - OS Event Timer and OS Event Timer Wakeup interrupts"]
    #[inline]
    pub fn os_event_timer_irq(&self) -> OS_EVENT_TIMER_IRQR {
        OS_EVENT_TIMER_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved0(&self) -> RESERVED0R {
        RESERVED0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        RESERVED1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        RESERVED2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - SDIO Controller interrupt."]
    #[inline]
    pub fn sdio_irq(&self) -> SDIO_IRQR {
        SDIO_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        RESERVED3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        RESERVED4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved5(&self) -> RESERVED5R {
        RESERVED5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - USB High Speed Controller UTMI interrupt."]
    #[inline]
    pub fn usb1_utmi_irq(&self) -> USB1_UTMI_IRQR {
        USB1_UTMI_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - USB High Speed Controller interrupt."]
    #[inline]
    pub fn usb1_irq(&self) -> USB1_IRQR {
        USB1_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - USB High Speed Controller Clock request interrupt."]
    #[inline]
    pub fn usb1_needclk(&self) -> USB1_NEEDCLKR {
        USB1_NEEDCLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Secure fault Hyper Visor call interrupt."]
    #[inline]
    pub fn sec_hypervisor_call_irq(&self) -> SEC_HYPERVISOR_CALL_IRQR {
        SEC_HYPERVISOR_CALL_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline]
    pub fn sec_gpio_int0_irq0(&self) -> SEC_GPIO_INT0_IRQ0R {
        SEC_GPIO_INT0_IRQ0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline]
    pub fn sec_gpio_int0_irq1(&self) -> SEC_GPIO_INT0_IRQ1R {
        SEC_GPIO_INT0_IRQ1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Programmable Look-Up Controller interrupt."]
    #[inline]
    pub fn plu_irq(&self) -> PLU_IRQR {
        PLU_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Security Violation interrupt."]
    #[inline]
    pub fn sec_vio_irq(&self) -> SEC_VIO_IRQR {
        SEC_VIO_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - HASH-AES interrupt."]
    #[inline]
    pub fn sha_irq(&self) -> SHA_IRQR {
        SHA_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - CASPER interrupt."]
    #[inline]
    pub fn casper_irq(&self) -> CASPER_IRQR {
        CASPER_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - PUF interrupt."]
    #[inline]
    pub fn qddkey_irq(&self) -> QDDKEY_IRQR {
        QDDKEY_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Power Quad interrupt."]
    #[inline]
    pub fn pq_irq(&self) -> PQ_IRQR {
        PQ_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - System DMA 1 (Secure) interrupt"]
    #[inline]
    pub fn sdma1_irq(&self) -> SDMA1_IRQR {
        SDMA1_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - High Speed SPI interrupt"]
    #[inline]
    pub fn lspi_hs_irq(&self) -> LSPI_HS_IRQR {
        LSPI_HS_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
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
    #[doc = "Bit 0 - Pin interrupt 4 or pattern match engine slice 4 interrupt."]
    #[inline]
    pub fn gpio_int0_irq4(&mut self) -> _GPIO_INT0_IRQ4W {
        _GPIO_INT0_IRQ4W { w: self }
    }
    #[doc = "Bit 1 - Pin interrupt 5 or pattern match engine slice 5 interrupt."]
    #[inline]
    pub fn gpio_int0_irq5(&mut self) -> _GPIO_INT0_IRQ5W {
        _GPIO_INT0_IRQ5W { w: self }
    }
    #[doc = "Bit 2 - Pin interrupt 6 or pattern match engine slice 6 interrupt."]
    #[inline]
    pub fn gpio_int0_irq6(&mut self) -> _GPIO_INT0_IRQ6W {
        _GPIO_INT0_IRQ6W { w: self }
    }
    #[doc = "Bit 3 - Pin interrupt 7 or pattern match engine slice 7 interrupt."]
    #[inline]
    pub fn gpio_int0_irq7(&mut self) -> _GPIO_INT0_IRQ7W {
        _GPIO_INT0_IRQ7W { w: self }
    }
    #[doc = "Bit 4 - Standard counter/timer 2 interrupt."]
    #[inline]
    pub fn ctimer2_irq(&mut self) -> _CTIMER2_IRQW {
        _CTIMER2_IRQW { w: self }
    }
    #[doc = "Bit 5 - Standard counter/timer 4 interrupt."]
    #[inline]
    pub fn ctimer4_irq(&mut self) -> _CTIMER4_IRQW {
        _CTIMER4_IRQW { w: self }
    }
    #[doc = "Bit 6 - OS Event Timer and OS Event Timer Wakeup interrupts"]
    #[inline]
    pub fn os_event_timer_irq(&mut self) -> _OS_EVENT_TIMER_IRQW {
        _OS_EVENT_TIMER_IRQW { w: self }
    }
    #[doc = "Bit 7 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved0(&mut self) -> _RESERVED0W {
        _RESERVED0W { w: self }
    }
    #[doc = "Bit 8 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved1(&mut self) -> _RESERVED1W {
        _RESERVED1W { w: self }
    }
    #[doc = "Bit 9 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bit 10 - SDIO Controller interrupt."]
    #[inline]
    pub fn sdio_irq(&mut self) -> _SDIO_IRQW {
        _SDIO_IRQW { w: self }
    }
    #[doc = "Bit 11 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 12 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bit 13 - Reserved. Read value is undefined, only zero should be written."]
    #[inline]
    pub fn reserved5(&mut self) -> _RESERVED5W {
        _RESERVED5W { w: self }
    }
    #[doc = "Bit 14 - USB High Speed Controller UTMI interrupt."]
    #[inline]
    pub fn usb1_utmi_irq(&mut self) -> _USB1_UTMI_IRQW {
        _USB1_UTMI_IRQW { w: self }
    }
    #[doc = "Bit 15 - USB High Speed Controller interrupt."]
    #[inline]
    pub fn usb1_irq(&mut self) -> _USB1_IRQW {
        _USB1_IRQW { w: self }
    }
    #[doc = "Bit 16 - USB High Speed Controller Clock request interrupt."]
    #[inline]
    pub fn usb1_needclk(&mut self) -> _USB1_NEEDCLKW {
        _USB1_NEEDCLKW { w: self }
    }
    #[doc = "Bit 17 - Secure fault Hyper Visor call interrupt."]
    #[inline]
    pub fn sec_hypervisor_call_irq(&mut self) -> _SEC_HYPERVISOR_CALL_IRQW {
        _SEC_HYPERVISOR_CALL_IRQW { w: self }
    }
    #[doc = "Bit 18 - Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline]
    pub fn sec_gpio_int0_irq0(&mut self) -> _SEC_GPIO_INT0_IRQ0W {
        _SEC_GPIO_INT0_IRQ0W { w: self }
    }
    #[doc = "Bit 19 - Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline]
    pub fn sec_gpio_int0_irq1(&mut self) -> _SEC_GPIO_INT0_IRQ1W {
        _SEC_GPIO_INT0_IRQ1W { w: self }
    }
    #[doc = "Bit 20 - Programmable Look-Up Controller interrupt."]
    #[inline]
    pub fn plu_irq(&mut self) -> _PLU_IRQW {
        _PLU_IRQW { w: self }
    }
    #[doc = "Bit 21 - Security Violation interrupt."]
    #[inline]
    pub fn sec_vio_irq(&mut self) -> _SEC_VIO_IRQW {
        _SEC_VIO_IRQW { w: self }
    }
    #[doc = "Bit 22 - HASH-AES interrupt."]
    #[inline]
    pub fn sha_irq(&mut self) -> _SHA_IRQW {
        _SHA_IRQW { w: self }
    }
    #[doc = "Bit 23 - CASPER interrupt."]
    #[inline]
    pub fn casper_irq(&mut self) -> _CASPER_IRQW {
        _CASPER_IRQW { w: self }
    }
    #[doc = "Bit 24 - PUF interrupt."]
    #[inline]
    pub fn qddkey_irq(&mut self) -> _QDDKEY_IRQW {
        _QDDKEY_IRQW { w: self }
    }
    #[doc = "Bit 25 - Power Quad interrupt."]
    #[inline]
    pub fn pq_irq(&mut self) -> _PQ_IRQW {
        _PQ_IRQW { w: self }
    }
    #[doc = "Bit 26 - System DMA 1 (Secure) interrupt"]
    #[inline]
    pub fn sdma1_irq(&mut self) -> _SDMA1_IRQW {
        _SDMA1_IRQW { w: self }
    }
    #[doc = "Bit 27 - High Speed SPI interrupt"]
    #[inline]
    pub fn lspi_hs_irq(&mut self) -> _LSPI_HS_IRQW {
        _LSPI_HS_IRQW { w: self }
    }
}
