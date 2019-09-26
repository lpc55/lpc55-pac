#[doc = "Reader of register SEC_CPU_INT_MASK1"]
pub type R = crate::R<u32, super::SEC_CPU_INT_MASK1>;
#[doc = "Writer for register SEC_CPU_INT_MASK1"]
pub type W = crate::W<u32, super::SEC_CPU_INT_MASK1>;
#[doc = "Register SEC_CPU_INT_MASK1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::SEC_CPU_INT_MASK1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Possible values of the field `GPIO_INT0_IRQ4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ4_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<GPIO_INT0_IRQ4_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ4_A) -> Self {
        match variant {
            GPIO_INT0_IRQ4_A::INVISIBLE => false,
            GPIO_INT0_IRQ4_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO_INT0_IRQ4`"]
pub type GPIO_INT0_IRQ4_R = crate::R<bool, GPIO_INT0_IRQ4_A>;
impl GPIO_INT0_IRQ4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ4_A {
        match self.bits {
            false => GPIO_INT0_IRQ4_A::INVISIBLE,
            true => GPIO_INT0_IRQ4_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ4_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ4_A::VISIBLE
    }
}
#[doc = "Write proxy for field `GPIO_INT0_IRQ4`"]
pub struct GPIO_INT0_IRQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT0_IRQ4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT0_IRQ4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ4_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ4_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `GPIO_INT0_IRQ5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ5_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<GPIO_INT0_IRQ5_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ5_A) -> Self {
        match variant {
            GPIO_INT0_IRQ5_A::INVISIBLE => false,
            GPIO_INT0_IRQ5_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO_INT0_IRQ5`"]
pub type GPIO_INT0_IRQ5_R = crate::R<bool, GPIO_INT0_IRQ5_A>;
impl GPIO_INT0_IRQ5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ5_A {
        match self.bits {
            false => GPIO_INT0_IRQ5_A::INVISIBLE,
            true => GPIO_INT0_IRQ5_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ5_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ5_A::VISIBLE
    }
}
#[doc = "Write proxy for field `GPIO_INT0_IRQ5`"]
pub struct GPIO_INT0_IRQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT0_IRQ5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT0_IRQ5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ5_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ5_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `GPIO_INT0_IRQ6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ6_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<GPIO_INT0_IRQ6_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ6_A) -> Self {
        match variant {
            GPIO_INT0_IRQ6_A::INVISIBLE => false,
            GPIO_INT0_IRQ6_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO_INT0_IRQ6`"]
pub type GPIO_INT0_IRQ6_R = crate::R<bool, GPIO_INT0_IRQ6_A>;
impl GPIO_INT0_IRQ6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ6_A {
        match self.bits {
            false => GPIO_INT0_IRQ6_A::INVISIBLE,
            true => GPIO_INT0_IRQ6_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ6_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ6_A::VISIBLE
    }
}
#[doc = "Write proxy for field `GPIO_INT0_IRQ6`"]
pub struct GPIO_INT0_IRQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT0_IRQ6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT0_IRQ6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ6_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ6_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `GPIO_INT0_IRQ7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ7_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<GPIO_INT0_IRQ7_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ7_A) -> Self {
        match variant {
            GPIO_INT0_IRQ7_A::INVISIBLE => false,
            GPIO_INT0_IRQ7_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `GPIO_INT0_IRQ7`"]
pub type GPIO_INT0_IRQ7_R = crate::R<bool, GPIO_INT0_IRQ7_A>;
impl GPIO_INT0_IRQ7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ7_A {
        match self.bits {
            false => GPIO_INT0_IRQ7_A::INVISIBLE,
            true => GPIO_INT0_IRQ7_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GPIO_INT0_IRQ7_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GPIO_INT0_IRQ7_A::VISIBLE
    }
}
#[doc = "Write proxy for field `GPIO_INT0_IRQ7`"]
pub struct GPIO_INT0_IRQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT0_IRQ7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT0_IRQ7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ7_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ7_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `CTIMER2_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER2_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<CTIMER2_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER2_IRQ_A) -> Self {
        match variant {
            CTIMER2_IRQ_A::INVISIBLE => false,
            CTIMER2_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `CTIMER2_IRQ`"]
pub type CTIMER2_IRQ_R = crate::R<bool, CTIMER2_IRQ_A>;
impl CTIMER2_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER2_IRQ_A {
        match self.bits {
            false => CTIMER2_IRQ_A::INVISIBLE,
            true => CTIMER2_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == CTIMER2_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == CTIMER2_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `CTIMER2_IRQ`"]
pub struct CTIMER2_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER2_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER2_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER2_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER2_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `CTIMER4_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER4_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<CTIMER4_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER4_IRQ_A) -> Self {
        match variant {
            CTIMER4_IRQ_A::INVISIBLE => false,
            CTIMER4_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `CTIMER4_IRQ`"]
pub type CTIMER4_IRQ_R = crate::R<bool, CTIMER4_IRQ_A>;
impl CTIMER4_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER4_IRQ_A {
        match self.bits {
            false => CTIMER4_IRQ_A::INVISIBLE,
            true => CTIMER4_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == CTIMER4_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == CTIMER4_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `CTIMER4_IRQ`"]
pub struct CTIMER4_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER4_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER4_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER4_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER4_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Possible values of the field `OS_EVENT_TIMER_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OS_EVENT_TIMER_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<OS_EVENT_TIMER_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: OS_EVENT_TIMER_IRQ_A) -> Self {
        match variant {
            OS_EVENT_TIMER_IRQ_A::INVISIBLE => false,
            OS_EVENT_TIMER_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `OS_EVENT_TIMER_IRQ`"]
pub type OS_EVENT_TIMER_IRQ_R = crate::R<bool, OS_EVENT_TIMER_IRQ_A>;
impl OS_EVENT_TIMER_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OS_EVENT_TIMER_IRQ_A {
        match self.bits {
            false => OS_EVENT_TIMER_IRQ_A::INVISIBLE,
            true => OS_EVENT_TIMER_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == OS_EVENT_TIMER_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == OS_EVENT_TIMER_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `OS_EVENT_TIMER_IRQ`"]
pub struct OS_EVENT_TIMER_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> OS_EVENT_TIMER_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OS_EVENT_TIMER_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(OS_EVENT_TIMER_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `RESERVED0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED0_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<RESERVED0_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED0_A) -> Self {
        match variant {
            RESERVED0_A::INVISIBLE => false,
            RESERVED0_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<bool, RESERVED0_A>;
impl RESERVED0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESERVED0_A {
        match self.bits {
            false => RESERVED0_A::INVISIBLE,
            true => RESERVED0_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED0_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED0_A::VISIBLE
    }
}
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESERVED0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED0_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED0_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `RESERVED1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED1_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<RESERVED1_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED1_A) -> Self {
        match variant {
            RESERVED1_A::INVISIBLE => false,
            RESERVED1_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<bool, RESERVED1_A>;
impl RESERVED1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESERVED1_A {
        match self.bits {
            false => RESERVED1_A::INVISIBLE,
            true => RESERVED1_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED1_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED1_A::VISIBLE
    }
}
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESERVED1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED1_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED1_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `RESERVED2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED2_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<RESERVED2_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED2_A) -> Self {
        match variant {
            RESERVED2_A::INVISIBLE => false,
            RESERVED2_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<bool, RESERVED2_A>;
impl RESERVED2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESERVED2_A {
        match self.bits {
            false => RESERVED2_A::INVISIBLE,
            true => RESERVED2_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED2_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED2_A::VISIBLE
    }
}
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESERVED2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED2_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED2_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `SDIO_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIO_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<SDIO_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SDIO_IRQ_A) -> Self {
        match variant {
            SDIO_IRQ_A::INVISIBLE => false,
            SDIO_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `SDIO_IRQ`"]
pub type SDIO_IRQ_R = crate::R<bool, SDIO_IRQ_A>;
impl SDIO_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIO_IRQ_A {
        match self.bits {
            false => SDIO_IRQ_A::INVISIBLE,
            true => SDIO_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SDIO_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SDIO_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `SDIO_IRQ`"]
pub struct SDIO_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIO_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SDIO_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SDIO_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `RESERVED3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED3_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<RESERVED3_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED3_A) -> Self {
        match variant {
            RESERVED3_A::INVISIBLE => false,
            RESERVED3_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<bool, RESERVED3_A>;
impl RESERVED3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESERVED3_A {
        match self.bits {
            false => RESERVED3_A::INVISIBLE,
            true => RESERVED3_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED3_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED3_A::VISIBLE
    }
}
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESERVED3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED3_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED3_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `RESERVED4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED4_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<RESERVED4_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED4_A) -> Self {
        match variant {
            RESERVED4_A::INVISIBLE => false,
            RESERVED4_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `RESERVED4`"]
pub type RESERVED4_R = crate::R<bool, RESERVED4_A>;
impl RESERVED4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESERVED4_A {
        match self.bits {
            false => RESERVED4_A::INVISIBLE,
            true => RESERVED4_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED4_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED4_A::VISIBLE
    }
}
#[doc = "Write proxy for field `RESERVED4`"]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESERVED4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED4_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED4_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `RESERVED5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED5_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<RESERVED5_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED5_A) -> Self {
        match variant {
            RESERVED5_A::INVISIBLE => false,
            RESERVED5_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `RESERVED5`"]
pub type RESERVED5_R = crate::R<bool, RESERVED5_A>;
impl RESERVED5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESERVED5_A {
        match self.bits {
            false => RESERVED5_A::INVISIBLE,
            true => RESERVED5_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RESERVED5_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RESERVED5_A::VISIBLE
    }
}
#[doc = "Write proxy for field `RESERVED5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESERVED5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RESERVED5_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RESERVED5_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Possible values of the field `USB1_PHY_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_PHY_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<USB1_PHY_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_PHY_IRQ_A) -> Self {
        match variant {
            USB1_PHY_IRQ_A::INVISIBLE => false,
            USB1_PHY_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `USB1_PHY_IRQ`"]
pub type USB1_PHY_IRQ_R = crate::R<bool, USB1_PHY_IRQ_A>;
impl USB1_PHY_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_PHY_IRQ_A {
        match self.bits {
            false => USB1_PHY_IRQ_A::INVISIBLE,
            true => USB1_PHY_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == USB1_PHY_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == USB1_PHY_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `USB1_PHY_IRQ`"]
pub struct USB1_PHY_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_PHY_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_PHY_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB1_PHY_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB1_PHY_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `USB1_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<USB1_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_IRQ_A) -> Self {
        match variant {
            USB1_IRQ_A::INVISIBLE => false,
            USB1_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `USB1_IRQ`"]
pub type USB1_IRQ_R = crate::R<bool, USB1_IRQ_A>;
impl USB1_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_IRQ_A {
        match self.bits {
            false => USB1_IRQ_A::INVISIBLE,
            true => USB1_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == USB1_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == USB1_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `USB1_IRQ`"]
pub struct USB1_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB1_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB1_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Possible values of the field `USB1_NEEDCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB1_NEEDCLK_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<USB1_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: USB1_NEEDCLK_A) -> Self {
        match variant {
            USB1_NEEDCLK_A::INVISIBLE => false,
            USB1_NEEDCLK_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `USB1_NEEDCLK`"]
pub type USB1_NEEDCLK_R = crate::R<bool, USB1_NEEDCLK_A>;
impl USB1_NEEDCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB1_NEEDCLK_A {
        match self.bits {
            false => USB1_NEEDCLK_A::INVISIBLE,
            true => USB1_NEEDCLK_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == USB1_NEEDCLK_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == USB1_NEEDCLK_A::VISIBLE
    }
}
#[doc = "Write proxy for field `USB1_NEEDCLK`"]
pub struct USB1_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1_NEEDCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB1_NEEDCLK_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB1_NEEDCLK_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `SEC_HYPERVISOR_CALL_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_HYPERVISOR_CALL_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<SEC_HYPERVISOR_CALL_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_HYPERVISOR_CALL_IRQ_A) -> Self {
        match variant {
            SEC_HYPERVISOR_CALL_IRQ_A::INVISIBLE => false,
            SEC_HYPERVISOR_CALL_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `SEC_HYPERVISOR_CALL_IRQ`"]
pub type SEC_HYPERVISOR_CALL_IRQ_R = crate::R<bool, SEC_HYPERVISOR_CALL_IRQ_A>;
impl SEC_HYPERVISOR_CALL_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_HYPERVISOR_CALL_IRQ_A {
        match self.bits {
            false => SEC_HYPERVISOR_CALL_IRQ_A::INVISIBLE,
            true => SEC_HYPERVISOR_CALL_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SEC_HYPERVISOR_CALL_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SEC_HYPERVISOR_CALL_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `SEC_HYPERVISOR_CALL_IRQ`"]
pub struct SEC_HYPERVISOR_CALL_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_HYPERVISOR_CALL_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_HYPERVISOR_CALL_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SEC_HYPERVISOR_CALL_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SEC_HYPERVISOR_CALL_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Possible values of the field `SEC_GPIO_INT0_IRQ0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_GPIO_INT0_IRQ0_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<SEC_GPIO_INT0_IRQ0_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_GPIO_INT0_IRQ0_A) -> Self {
        match variant {
            SEC_GPIO_INT0_IRQ0_A::INVISIBLE => false,
            SEC_GPIO_INT0_IRQ0_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `SEC_GPIO_INT0_IRQ0`"]
pub type SEC_GPIO_INT0_IRQ0_R = crate::R<bool, SEC_GPIO_INT0_IRQ0_A>;
impl SEC_GPIO_INT0_IRQ0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_GPIO_INT0_IRQ0_A {
        match self.bits {
            false => SEC_GPIO_INT0_IRQ0_A::INVISIBLE,
            true => SEC_GPIO_INT0_IRQ0_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SEC_GPIO_INT0_IRQ0_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SEC_GPIO_INT0_IRQ0_A::VISIBLE
    }
}
#[doc = "Write proxy for field `SEC_GPIO_INT0_IRQ0`"]
pub struct SEC_GPIO_INT0_IRQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_GPIO_INT0_IRQ0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_GPIO_INT0_IRQ0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT0_IRQ0_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT0_IRQ0_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `SEC_GPIO_INT0_IRQ1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_GPIO_INT0_IRQ1_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<SEC_GPIO_INT0_IRQ1_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_GPIO_INT0_IRQ1_A) -> Self {
        match variant {
            SEC_GPIO_INT0_IRQ1_A::INVISIBLE => false,
            SEC_GPIO_INT0_IRQ1_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `SEC_GPIO_INT0_IRQ1`"]
pub type SEC_GPIO_INT0_IRQ1_R = crate::R<bool, SEC_GPIO_INT0_IRQ1_A>;
impl SEC_GPIO_INT0_IRQ1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_GPIO_INT0_IRQ1_A {
        match self.bits {
            false => SEC_GPIO_INT0_IRQ1_A::INVISIBLE,
            true => SEC_GPIO_INT0_IRQ1_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SEC_GPIO_INT0_IRQ1_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SEC_GPIO_INT0_IRQ1_A::VISIBLE
    }
}
#[doc = "Write proxy for field `SEC_GPIO_INT0_IRQ1`"]
pub struct SEC_GPIO_INT0_IRQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_GPIO_INT0_IRQ1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_GPIO_INT0_IRQ1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT0_IRQ1_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SEC_GPIO_INT0_IRQ1_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Possible values of the field `PLU_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLU_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<PLU_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: PLU_IRQ_A) -> Self {
        match variant {
            PLU_IRQ_A::INVISIBLE => false,
            PLU_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `PLU_IRQ`"]
pub type PLU_IRQ_R = crate::R<bool, PLU_IRQ_A>;
impl PLU_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLU_IRQ_A {
        match self.bits {
            false => PLU_IRQ_A::INVISIBLE,
            true => PLU_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == PLU_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == PLU_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `PLU_IRQ`"]
pub struct PLU_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLU_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLU_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(PLU_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(PLU_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `SEC_VIO_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_VIO_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<SEC_VIO_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_VIO_IRQ_A) -> Self {
        match variant {
            SEC_VIO_IRQ_A::INVISIBLE => false,
            SEC_VIO_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `SEC_VIO_IRQ`"]
pub type SEC_VIO_IRQ_R = crate::R<bool, SEC_VIO_IRQ_A>;
impl SEC_VIO_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_VIO_IRQ_A {
        match self.bits {
            false => SEC_VIO_IRQ_A::INVISIBLE,
            true => SEC_VIO_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SEC_VIO_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SEC_VIO_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `SEC_VIO_IRQ`"]
pub struct SEC_VIO_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_VIO_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_VIO_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SEC_VIO_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SEC_VIO_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Possible values of the field `SHA_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHA_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<SHA_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SHA_IRQ_A) -> Self {
        match variant {
            SHA_IRQ_A::INVISIBLE => false,
            SHA_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `SHA_IRQ`"]
pub type SHA_IRQ_R = crate::R<bool, SHA_IRQ_A>;
impl SHA_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHA_IRQ_A {
        match self.bits {
            false => SHA_IRQ_A::INVISIBLE,
            true => SHA_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SHA_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SHA_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `SHA_IRQ`"]
pub struct SHA_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHA_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SHA_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SHA_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `CASPER_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASPER_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<CASPER_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CASPER_IRQ_A) -> Self {
        match variant {
            CASPER_IRQ_A::INVISIBLE => false,
            CASPER_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `CASPER_IRQ`"]
pub type CASPER_IRQ_R = crate::R<bool, CASPER_IRQ_A>;
impl CASPER_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_IRQ_A {
        match self.bits {
            false => CASPER_IRQ_A::INVISIBLE,
            true => CASPER_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == CASPER_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == CASPER_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `CASPER_IRQ`"]
pub struct CASPER_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CASPER_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CASPER_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CASPER_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(CASPER_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `PUFKEY_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUFKEY_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<PUFKEY_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: PUFKEY_IRQ_A) -> Self {
        match variant {
            PUFKEY_IRQ_A::INVISIBLE => false,
            PUFKEY_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `PUFKEY_IRQ`"]
pub type PUFKEY_IRQ_R = crate::R<bool, PUFKEY_IRQ_A>;
impl PUFKEY_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUFKEY_IRQ_A {
        match self.bits {
            false => PUFKEY_IRQ_A::INVISIBLE,
            true => PUFKEY_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == PUFKEY_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == PUFKEY_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `PUFKEY_IRQ`"]
pub struct PUFKEY_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PUFKEY_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUFKEY_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(PUFKEY_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(PUFKEY_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `PQ_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PQ_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<PQ_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: PQ_IRQ_A) -> Self {
        match variant {
            PQ_IRQ_A::INVISIBLE => false,
            PQ_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `PQ_IRQ`"]
pub type PQ_IRQ_R = crate::R<bool, PQ_IRQ_A>;
impl PQ_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PQ_IRQ_A {
        match self.bits {
            false => PQ_IRQ_A::INVISIBLE,
            true => PQ_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == PQ_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == PQ_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `PQ_IRQ`"]
pub struct PQ_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PQ_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PQ_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(PQ_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(PQ_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Possible values of the field `SDMA1_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA1_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<SDMA1_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA1_IRQ_A) -> Self {
        match variant {
            SDMA1_IRQ_A::INVISIBLE => false,
            SDMA1_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `SDMA1_IRQ`"]
pub type SDMA1_IRQ_R = crate::R<bool, SDMA1_IRQ_A>;
impl SDMA1_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA1_IRQ_A {
        match self.bits {
            false => SDMA1_IRQ_A::INVISIBLE,
            true => SDMA1_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SDMA1_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SDMA1_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `SDMA1_IRQ`"]
pub struct SDMA1_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA1_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMA1_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SDMA1_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SDMA1_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `LSPI_HS_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPI_HS_IRQ_A {
    #[doc = "no description available"]
    INVISIBLE,
    #[doc = "no description available"]
    VISIBLE,
}
impl From<LSPI_HS_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: LSPI_HS_IRQ_A) -> Self {
        match variant {
            LSPI_HS_IRQ_A::INVISIBLE => false,
            LSPI_HS_IRQ_A::VISIBLE => true,
        }
    }
}
#[doc = "Reader of field `LSPI_HS_IRQ`"]
pub type LSPI_HS_IRQ_R = crate::R<bool, LSPI_HS_IRQ_A>;
impl LSPI_HS_IRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSPI_HS_IRQ_A {
        match self.bits {
            false => LSPI_HS_IRQ_A::INVISIBLE,
            true => LSPI_HS_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == LSPI_HS_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == LSPI_HS_IRQ_A::VISIBLE
    }
}
#[doc = "Write proxy for field `LSPI_HS_IRQ`"]
pub struct LSPI_HS_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPI_HS_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSPI_HS_IRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(LSPI_HS_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(LSPI_HS_IRQ_A::VISIBLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Pin interrupt 4 or pattern match engine slice 4 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq4(&self) -> GPIO_INT0_IRQ4_R {
        GPIO_INT0_IRQ4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pin interrupt 5 or pattern match engine slice 5 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq5(&self) -> GPIO_INT0_IRQ5_R {
        GPIO_INT0_IRQ5_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pin interrupt 6 or pattern match engine slice 6 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq6(&self) -> GPIO_INT0_IRQ6_R {
        GPIO_INT0_IRQ6_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pin interrupt 7 or pattern match engine slice 7 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq7(&self) -> GPIO_INT0_IRQ7_R {
        GPIO_INT0_IRQ7_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Standard counter/timer 2 interrupt."]
    #[inline(always)]
    pub fn ctimer2_irq(&self) -> CTIMER2_IRQ_R {
        CTIMER2_IRQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Standard counter/timer 4 interrupt."]
    #[inline(always)]
    pub fn ctimer4_irq(&self) -> CTIMER4_IRQ_R {
        CTIMER4_IRQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OS Event Timer and OS Event Timer Wakeup interrupts"]
    #[inline(always)]
    pub fn os_event_timer_irq(&self) -> OS_EVENT_TIMER_IRQ_R {
        OS_EVENT_TIMER_IRQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SDIO Controller interrupt."]
    #[inline(always)]
    pub fn sdio_irq(&self) -> SDIO_IRQ_R {
        SDIO_IRQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - USB High Speed PHY Controller interrupt."]
    #[inline(always)]
    pub fn usb1_phy_irq(&self) -> USB1_PHY_IRQ_R {
        USB1_PHY_IRQ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - USB High Speed Controller interrupt."]
    #[inline(always)]
    pub fn usb1_irq(&self) -> USB1_IRQ_R {
        USB1_IRQ_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB High Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub fn usb1_needclk(&self) -> USB1_NEEDCLK_R {
        USB1_NEEDCLK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Secure fault Hyper Visor call interrupt."]
    #[inline(always)]
    pub fn sec_hypervisor_call_irq(&self) -> SEC_HYPERVISOR_CALL_IRQ_R {
        SEC_HYPERVISOR_CALL_IRQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub fn sec_gpio_int0_irq0(&self) -> SEC_GPIO_INT0_IRQ0_R {
        SEC_GPIO_INT0_IRQ0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub fn sec_gpio_int0_irq1(&self) -> SEC_GPIO_INT0_IRQ1_R {
        SEC_GPIO_INT0_IRQ1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Programmable Look-Up Controller interrupt."]
    #[inline(always)]
    pub fn plu_irq(&self) -> PLU_IRQ_R {
        PLU_IRQ_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Security Violation interrupt."]
    #[inline(always)]
    pub fn sec_vio_irq(&self) -> SEC_VIO_IRQ_R {
        SEC_VIO_IRQ_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - HASH-AES interrupt."]
    #[inline(always)]
    pub fn sha_irq(&self) -> SHA_IRQ_R {
        SHA_IRQ_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CASPER interrupt."]
    #[inline(always)]
    pub fn casper_irq(&self) -> CASPER_IRQ_R {
        CASPER_IRQ_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PUF interrupt."]
    #[inline(always)]
    pub fn pufkey_irq(&self) -> PUFKEY_IRQ_R {
        PUFKEY_IRQ_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Power Quad interrupt."]
    #[inline(always)]
    pub fn pq_irq(&self) -> PQ_IRQ_R {
        PQ_IRQ_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - System DMA 1 (Secure) interrupt"]
    #[inline(always)]
    pub fn sdma1_irq(&self) -> SDMA1_IRQ_R {
        SDMA1_IRQ_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - High Speed SPI interrupt"]
    #[inline(always)]
    pub fn lspi_hs_irq(&self) -> LSPI_HS_IRQ_R {
        LSPI_HS_IRQ_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin interrupt 4 or pattern match engine slice 4 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq4(&mut self) -> GPIO_INT0_IRQ4_W {
        GPIO_INT0_IRQ4_W { w: self }
    }
    #[doc = "Bit 1 - Pin interrupt 5 or pattern match engine slice 5 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq5(&mut self) -> GPIO_INT0_IRQ5_W {
        GPIO_INT0_IRQ5_W { w: self }
    }
    #[doc = "Bit 2 - Pin interrupt 6 or pattern match engine slice 6 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq6(&mut self) -> GPIO_INT0_IRQ6_W {
        GPIO_INT0_IRQ6_W { w: self }
    }
    #[doc = "Bit 3 - Pin interrupt 7 or pattern match engine slice 7 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq7(&mut self) -> GPIO_INT0_IRQ7_W {
        GPIO_INT0_IRQ7_W { w: self }
    }
    #[doc = "Bit 4 - Standard counter/timer 2 interrupt."]
    #[inline(always)]
    pub fn ctimer2_irq(&mut self) -> CTIMER2_IRQ_W {
        CTIMER2_IRQ_W { w: self }
    }
    #[doc = "Bit 5 - Standard counter/timer 4 interrupt."]
    #[inline(always)]
    pub fn ctimer4_irq(&mut self) -> CTIMER4_IRQ_W {
        CTIMER4_IRQ_W { w: self }
    }
    #[doc = "Bit 6 - OS Event Timer and OS Event Timer Wakeup interrupts"]
    #[inline(always)]
    pub fn os_event_timer_irq(&mut self) -> OS_EVENT_TIMER_IRQ_W {
        OS_EVENT_TIMER_IRQ_W { w: self }
    }
    #[doc = "Bit 7 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Bit 8 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 9 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 10 - SDIO Controller interrupt."]
    #[inline(always)]
    pub fn sdio_irq(&mut self) -> SDIO_IRQ_W {
        SDIO_IRQ_W { w: self }
    }
    #[doc = "Bit 11 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 12 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 13 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bit 14 - USB High Speed PHY Controller interrupt."]
    #[inline(always)]
    pub fn usb1_phy_irq(&mut self) -> USB1_PHY_IRQ_W {
        USB1_PHY_IRQ_W { w: self }
    }
    #[doc = "Bit 15 - USB High Speed Controller interrupt."]
    #[inline(always)]
    pub fn usb1_irq(&mut self) -> USB1_IRQ_W {
        USB1_IRQ_W { w: self }
    }
    #[doc = "Bit 16 - USB High Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub fn usb1_needclk(&mut self) -> USB1_NEEDCLK_W {
        USB1_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 17 - Secure fault Hyper Visor call interrupt."]
    #[inline(always)]
    pub fn sec_hypervisor_call_irq(&mut self) -> SEC_HYPERVISOR_CALL_IRQ_W {
        SEC_HYPERVISOR_CALL_IRQ_W { w: self }
    }
    #[doc = "Bit 18 - Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub fn sec_gpio_int0_irq0(&mut self) -> SEC_GPIO_INT0_IRQ0_W {
        SEC_GPIO_INT0_IRQ0_W { w: self }
    }
    #[doc = "Bit 19 - Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub fn sec_gpio_int0_irq1(&mut self) -> SEC_GPIO_INT0_IRQ1_W {
        SEC_GPIO_INT0_IRQ1_W { w: self }
    }
    #[doc = "Bit 20 - Programmable Look-Up Controller interrupt."]
    #[inline(always)]
    pub fn plu_irq(&mut self) -> PLU_IRQ_W {
        PLU_IRQ_W { w: self }
    }
    #[doc = "Bit 21 - Security Violation interrupt."]
    #[inline(always)]
    pub fn sec_vio_irq(&mut self) -> SEC_VIO_IRQ_W {
        SEC_VIO_IRQ_W { w: self }
    }
    #[doc = "Bit 22 - HASH-AES interrupt."]
    #[inline(always)]
    pub fn sha_irq(&mut self) -> SHA_IRQ_W {
        SHA_IRQ_W { w: self }
    }
    #[doc = "Bit 23 - CASPER interrupt."]
    #[inline(always)]
    pub fn casper_irq(&mut self) -> CASPER_IRQ_W {
        CASPER_IRQ_W { w: self }
    }
    #[doc = "Bit 24 - PUF interrupt."]
    #[inline(always)]
    pub fn pufkey_irq(&mut self) -> PUFKEY_IRQ_W {
        PUFKEY_IRQ_W { w: self }
    }
    #[doc = "Bit 25 - Power Quad interrupt."]
    #[inline(always)]
    pub fn pq_irq(&mut self) -> PQ_IRQ_W {
        PQ_IRQ_W { w: self }
    }
    #[doc = "Bit 26 - System DMA 1 (Secure) interrupt"]
    #[inline(always)]
    pub fn sdma1_irq(&mut self) -> SDMA1_IRQ_W {
        SDMA1_IRQ_W { w: self }
    }
    #[doc = "Bit 27 - High Speed SPI interrupt"]
    #[inline(always)]
    pub fn lspi_hs_irq(&mut self) -> LSPI_HS_IRQ_W {
        LSPI_HS_IRQ_W { w: self }
    }
}
