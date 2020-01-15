#[doc = "Reader of register SEC_GPIO_MASK1"]
pub type R = crate::R<u32, super::SEC_GPIO_MASK1>;
#[doc = "Writer for register SEC_GPIO_MASK1"]
pub type W = crate::W<u32, super::SEC_GPIO_MASK1>;
#[doc = "Register SEC_GPIO_MASK1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::SEC_GPIO_MASK1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Secure mask for pin P1_0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN0_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN0_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN0_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN0_SEC_MASK`"]
pub type PIO1_PIN0_SEC_MASK_R = crate::R<bool, PIO1_PIN0_SEC_MASK_A>;
impl PIO1_PIN0_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN0_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN0_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN0_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN0_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN0_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN0_SEC_MASK`"]
pub struct PIO1_PIN0_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN0_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN0_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN0_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN0_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN1_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN1_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN1_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN1_SEC_MASK`"]
pub type PIO1_PIN1_SEC_MASK_R = crate::R<bool, PIO1_PIN1_SEC_MASK_A>;
impl PIO1_PIN1_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN1_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN1_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN1_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN1_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN1_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN1_SEC_MASK`"]
pub struct PIO1_PIN1_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN1_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN1_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN1_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN1_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN2_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN2_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN2_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN2_SEC_MASK`"]
pub type PIO1_PIN2_SEC_MASK_R = crate::R<bool, PIO1_PIN2_SEC_MASK_A>;
impl PIO1_PIN2_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN2_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN2_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN2_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN2_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN2_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN2_SEC_MASK`"]
pub struct PIO1_PIN2_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN2_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN2_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN2_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN2_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN3_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN3_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN3_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN3_SEC_MASK`"]
pub type PIO1_PIN3_SEC_MASK_R = crate::R<bool, PIO1_PIN3_SEC_MASK_A>;
impl PIO1_PIN3_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN3_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN3_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN3_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN3_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN3_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN3_SEC_MASK`"]
pub struct PIO1_PIN3_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN3_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN3_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN3_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN3_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN4_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN4_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN4_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN4_SEC_MASK`"]
pub type PIO1_PIN4_SEC_MASK_R = crate::R<bool, PIO1_PIN4_SEC_MASK_A>;
impl PIO1_PIN4_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN4_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN4_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN4_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN4_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN4_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN4_SEC_MASK`"]
pub struct PIO1_PIN4_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN4_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN4_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN4_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN4_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_5\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN5_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN5_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN5_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN5_SEC_MASK`"]
pub type PIO1_PIN5_SEC_MASK_R = crate::R<bool, PIO1_PIN5_SEC_MASK_A>;
impl PIO1_PIN5_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN5_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN5_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN5_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN5_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN5_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN5_SEC_MASK`"]
pub struct PIO1_PIN5_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN5_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN5_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN5_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN5_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_6\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN6_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN6_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN6_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN6_SEC_MASK`"]
pub type PIO1_PIN6_SEC_MASK_R = crate::R<bool, PIO1_PIN6_SEC_MASK_A>;
impl PIO1_PIN6_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN6_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN6_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN6_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN6_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN6_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN6_SEC_MASK`"]
pub struct PIO1_PIN6_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN6_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN6_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN6_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN6_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_7\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN7_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN7_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN7_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN7_SEC_MASK`"]
pub type PIO1_PIN7_SEC_MASK_R = crate::R<bool, PIO1_PIN7_SEC_MASK_A>;
impl PIO1_PIN7_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN7_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN7_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN7_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN7_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN7_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN7_SEC_MASK`"]
pub struct PIO1_PIN7_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN7_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN7_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN7_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN7_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_8\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN8_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN8_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN8_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN8_SEC_MASK`"]
pub type PIO1_PIN8_SEC_MASK_R = crate::R<bool, PIO1_PIN8_SEC_MASK_A>;
impl PIO1_PIN8_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN8_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN8_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN8_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN8_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN8_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN8_SEC_MASK`"]
pub struct PIO1_PIN8_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN8_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN8_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN8_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN8_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_9\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN9_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN9_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN9_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN9_SEC_MASK`"]
pub type PIO1_PIN9_SEC_MASK_R = crate::R<bool, PIO1_PIN9_SEC_MASK_A>;
impl PIO1_PIN9_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN9_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN9_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN9_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN9_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN9_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN9_SEC_MASK`"]
pub struct PIO1_PIN9_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN9_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN9_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN9_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN9_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_10\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN10_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN10_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN10_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN10_SEC_MASK`"]
pub type PIO1_PIN10_SEC_MASK_R = crate::R<bool, PIO1_PIN10_SEC_MASK_A>;
impl PIO1_PIN10_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN10_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN10_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN10_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN10_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN10_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN10_SEC_MASK`"]
pub struct PIO1_PIN10_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN10_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN10_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN10_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN10_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_11\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN11_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN11_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN11_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN11_SEC_MASK`"]
pub type PIO1_PIN11_SEC_MASK_R = crate::R<bool, PIO1_PIN11_SEC_MASK_A>;
impl PIO1_PIN11_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN11_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN11_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN11_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN11_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN11_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN11_SEC_MASK`"]
pub struct PIO1_PIN11_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN11_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN11_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN11_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN11_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_12\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN12_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN12_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN12_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN12_SEC_MASK`"]
pub type PIO1_PIN12_SEC_MASK_R = crate::R<bool, PIO1_PIN12_SEC_MASK_A>;
impl PIO1_PIN12_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN12_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN12_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN12_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN12_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN12_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN12_SEC_MASK`"]
pub struct PIO1_PIN12_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN12_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN12_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN12_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN12_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_13\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN13_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN13_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN13_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN13_SEC_MASK`"]
pub type PIO1_PIN13_SEC_MASK_R = crate::R<bool, PIO1_PIN13_SEC_MASK_A>;
impl PIO1_PIN13_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN13_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN13_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN13_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN13_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN13_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN13_SEC_MASK`"]
pub struct PIO1_PIN13_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN13_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN13_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN13_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN13_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_14\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN14_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN14_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN14_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN14_SEC_MASK`"]
pub type PIO1_PIN14_SEC_MASK_R = crate::R<bool, PIO1_PIN14_SEC_MASK_A>;
impl PIO1_PIN14_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN14_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN14_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN14_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN14_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN14_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN14_SEC_MASK`"]
pub struct PIO1_PIN14_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN14_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN14_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN14_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN14_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_15\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN15_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN15_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN15_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN15_SEC_MASK`"]
pub type PIO1_PIN15_SEC_MASK_R = crate::R<bool, PIO1_PIN15_SEC_MASK_A>;
impl PIO1_PIN15_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN15_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN15_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN15_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN15_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN15_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN15_SEC_MASK`"]
pub struct PIO1_PIN15_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN15_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN15_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN15_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN15_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_16\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN16_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN16_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN16_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN16_SEC_MASK`"]
pub type PIO1_PIN16_SEC_MASK_R = crate::R<bool, PIO1_PIN16_SEC_MASK_A>;
impl PIO1_PIN16_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN16_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN16_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN16_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN16_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN16_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN16_SEC_MASK`"]
pub struct PIO1_PIN16_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN16_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN16_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN16_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN16_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_17\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN17_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN17_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN17_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN17_SEC_MASK`"]
pub type PIO1_PIN17_SEC_MASK_R = crate::R<bool, PIO1_PIN17_SEC_MASK_A>;
impl PIO1_PIN17_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN17_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN17_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN17_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN17_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN17_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN17_SEC_MASK`"]
pub struct PIO1_PIN17_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN17_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN17_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN17_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN17_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_18\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN18_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN18_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN18_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN18_SEC_MASK`"]
pub type PIO1_PIN18_SEC_MASK_R = crate::R<bool, PIO1_PIN18_SEC_MASK_A>;
impl PIO1_PIN18_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN18_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN18_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN18_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN18_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN18_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN18_SEC_MASK`"]
pub struct PIO1_PIN18_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN18_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN18_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN18_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN18_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_19\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN19_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN19_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN19_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN19_SEC_MASK`"]
pub type PIO1_PIN19_SEC_MASK_R = crate::R<bool, PIO1_PIN19_SEC_MASK_A>;
impl PIO1_PIN19_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN19_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN19_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN19_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN19_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN19_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN19_SEC_MASK`"]
pub struct PIO1_PIN19_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN19_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN19_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN19_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN19_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_20\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN20_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN20_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN20_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN20_SEC_MASK`"]
pub type PIO1_PIN20_SEC_MASK_R = crate::R<bool, PIO1_PIN20_SEC_MASK_A>;
impl PIO1_PIN20_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN20_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN20_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN20_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN20_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN20_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN20_SEC_MASK`"]
pub struct PIO1_PIN20_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN20_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN20_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN20_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN20_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_21\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN21_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN21_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN21_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN21_SEC_MASK`"]
pub type PIO1_PIN21_SEC_MASK_R = crate::R<bool, PIO1_PIN21_SEC_MASK_A>;
impl PIO1_PIN21_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN21_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN21_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN21_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN21_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN21_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN21_SEC_MASK`"]
pub struct PIO1_PIN21_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN21_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN21_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN21_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN21_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_22\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN22_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN22_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN22_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN22_SEC_MASK`"]
pub type PIO1_PIN22_SEC_MASK_R = crate::R<bool, PIO1_PIN22_SEC_MASK_A>;
impl PIO1_PIN22_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN22_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN22_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN22_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN22_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN22_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN22_SEC_MASK`"]
pub struct PIO1_PIN22_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN22_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN22_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN22_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN22_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_23\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN23_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN23_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN23_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN23_SEC_MASK`"]
pub type PIO1_PIN23_SEC_MASK_R = crate::R<bool, PIO1_PIN23_SEC_MASK_A>;
impl PIO1_PIN23_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN23_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN23_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN23_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN23_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN23_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN23_SEC_MASK`"]
pub struct PIO1_PIN23_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN23_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN23_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN23_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN23_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_24\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN24_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN24_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN24_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN24_SEC_MASK`"]
pub type PIO1_PIN24_SEC_MASK_R = crate::R<bool, PIO1_PIN24_SEC_MASK_A>;
impl PIO1_PIN24_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN24_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN24_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN24_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN24_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN24_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN24_SEC_MASK`"]
pub struct PIO1_PIN24_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN24_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN24_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN24_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN24_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_25\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN25_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN25_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN25_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN25_SEC_MASK`"]
pub type PIO1_PIN25_SEC_MASK_R = crate::R<bool, PIO1_PIN25_SEC_MASK_A>;
impl PIO1_PIN25_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN25_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN25_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN25_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN25_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN25_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN25_SEC_MASK`"]
pub struct PIO1_PIN25_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN25_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN25_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN25_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN25_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_26\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN26_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN26_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN26_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN26_SEC_MASK`"]
pub type PIO1_PIN26_SEC_MASK_R = crate::R<bool, PIO1_PIN26_SEC_MASK_A>;
impl PIO1_PIN26_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN26_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN26_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN26_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN26_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN26_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN26_SEC_MASK`"]
pub struct PIO1_PIN26_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN26_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN26_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN26_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN26_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_27\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN27_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN27_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN27_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN27_SEC_MASK`"]
pub type PIO1_PIN27_SEC_MASK_R = crate::R<bool, PIO1_PIN27_SEC_MASK_A>;
impl PIO1_PIN27_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN27_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN27_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN27_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN27_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN27_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN27_SEC_MASK`"]
pub struct PIO1_PIN27_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN27_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN27_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN27_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN27_SEC_MASK_A::READABLE)
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
#[doc = "Secure mask for pin P1_28\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN28_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN28_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN28_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN28_SEC_MASK`"]
pub type PIO1_PIN28_SEC_MASK_R = crate::R<bool, PIO1_PIN28_SEC_MASK_A>;
impl PIO1_PIN28_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN28_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN28_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN28_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN28_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN28_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN28_SEC_MASK`"]
pub struct PIO1_PIN28_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN28_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN28_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN28_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN28_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Secure mask for pin P1_29\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN29_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN29_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN29_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN29_SEC_MASK`"]
pub type PIO1_PIN29_SEC_MASK_R = crate::R<bool, PIO1_PIN29_SEC_MASK_A>;
impl PIO1_PIN29_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN29_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN29_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN29_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN29_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN29_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN29_SEC_MASK`"]
pub struct PIO1_PIN29_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN29_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN29_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN29_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN29_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Secure mask for pin P1_30\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN30_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN30_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN30_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN30_SEC_MASK`"]
pub type PIO1_PIN30_SEC_MASK_R = crate::R<bool, PIO1_PIN30_SEC_MASK_A>;
impl PIO1_PIN30_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN30_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN30_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN30_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN30_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN30_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN30_SEC_MASK`"]
pub struct PIO1_PIN30_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN30_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN30_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN30_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN30_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Secure mask for pin P1_31\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN31_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN31_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN31_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIO1_PIN31_SEC_MASK`"]
pub type PIO1_PIN31_SEC_MASK_R = crate::R<bool, PIO1_PIN31_SEC_MASK_A>;
impl PIO1_PIN31_SEC_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN31_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN31_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN31_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN31_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN31_SEC_MASK_A::READABLE
    }
}
#[doc = "Write proxy for field `PIO1_PIN31_SEC_MASK`"]
pub struct PIO1_PIN31_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO1_PIN31_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO1_PIN31_SEC_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN31_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN31_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Secure mask for pin P1_0"]
    #[inline(always)]
    pub fn pio1_pin0_sec_mask(&self) -> PIO1_PIN0_SEC_MASK_R {
        PIO1_PIN0_SEC_MASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Secure mask for pin P1_1"]
    #[inline(always)]
    pub fn pio1_pin1_sec_mask(&self) -> PIO1_PIN1_SEC_MASK_R {
        PIO1_PIN1_SEC_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Secure mask for pin P1_2"]
    #[inline(always)]
    pub fn pio1_pin2_sec_mask(&self) -> PIO1_PIN2_SEC_MASK_R {
        PIO1_PIN2_SEC_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Secure mask for pin P1_3"]
    #[inline(always)]
    pub fn pio1_pin3_sec_mask(&self) -> PIO1_PIN3_SEC_MASK_R {
        PIO1_PIN3_SEC_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Secure mask for pin P1_4"]
    #[inline(always)]
    pub fn pio1_pin4_sec_mask(&self) -> PIO1_PIN4_SEC_MASK_R {
        PIO1_PIN4_SEC_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Secure mask for pin P1_5"]
    #[inline(always)]
    pub fn pio1_pin5_sec_mask(&self) -> PIO1_PIN5_SEC_MASK_R {
        PIO1_PIN5_SEC_MASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Secure mask for pin P1_6"]
    #[inline(always)]
    pub fn pio1_pin6_sec_mask(&self) -> PIO1_PIN6_SEC_MASK_R {
        PIO1_PIN6_SEC_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Secure mask for pin P1_7"]
    #[inline(always)]
    pub fn pio1_pin7_sec_mask(&self) -> PIO1_PIN7_SEC_MASK_R {
        PIO1_PIN7_SEC_MASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Secure mask for pin P1_8"]
    #[inline(always)]
    pub fn pio1_pin8_sec_mask(&self) -> PIO1_PIN8_SEC_MASK_R {
        PIO1_PIN8_SEC_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Secure mask for pin P1_9"]
    #[inline(always)]
    pub fn pio1_pin9_sec_mask(&self) -> PIO1_PIN9_SEC_MASK_R {
        PIO1_PIN9_SEC_MASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Secure mask for pin P1_10"]
    #[inline(always)]
    pub fn pio1_pin10_sec_mask(&self) -> PIO1_PIN10_SEC_MASK_R {
        PIO1_PIN10_SEC_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Secure mask for pin P1_11"]
    #[inline(always)]
    pub fn pio1_pin11_sec_mask(&self) -> PIO1_PIN11_SEC_MASK_R {
        PIO1_PIN11_SEC_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Secure mask for pin P1_12"]
    #[inline(always)]
    pub fn pio1_pin12_sec_mask(&self) -> PIO1_PIN12_SEC_MASK_R {
        PIO1_PIN12_SEC_MASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Secure mask for pin P1_13"]
    #[inline(always)]
    pub fn pio1_pin13_sec_mask(&self) -> PIO1_PIN13_SEC_MASK_R {
        PIO1_PIN13_SEC_MASK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Secure mask for pin P1_14"]
    #[inline(always)]
    pub fn pio1_pin14_sec_mask(&self) -> PIO1_PIN14_SEC_MASK_R {
        PIO1_PIN14_SEC_MASK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Secure mask for pin P1_15"]
    #[inline(always)]
    pub fn pio1_pin15_sec_mask(&self) -> PIO1_PIN15_SEC_MASK_R {
        PIO1_PIN15_SEC_MASK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Secure mask for pin P1_16"]
    #[inline(always)]
    pub fn pio1_pin16_sec_mask(&self) -> PIO1_PIN16_SEC_MASK_R {
        PIO1_PIN16_SEC_MASK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Secure mask for pin P1_17"]
    #[inline(always)]
    pub fn pio1_pin17_sec_mask(&self) -> PIO1_PIN17_SEC_MASK_R {
        PIO1_PIN17_SEC_MASK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Secure mask for pin P1_18"]
    #[inline(always)]
    pub fn pio1_pin18_sec_mask(&self) -> PIO1_PIN18_SEC_MASK_R {
        PIO1_PIN18_SEC_MASK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Secure mask for pin P1_19"]
    #[inline(always)]
    pub fn pio1_pin19_sec_mask(&self) -> PIO1_PIN19_SEC_MASK_R {
        PIO1_PIN19_SEC_MASK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Secure mask for pin P1_20"]
    #[inline(always)]
    pub fn pio1_pin20_sec_mask(&self) -> PIO1_PIN20_SEC_MASK_R {
        PIO1_PIN20_SEC_MASK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Secure mask for pin P1_21"]
    #[inline(always)]
    pub fn pio1_pin21_sec_mask(&self) -> PIO1_PIN21_SEC_MASK_R {
        PIO1_PIN21_SEC_MASK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Secure mask for pin P1_22"]
    #[inline(always)]
    pub fn pio1_pin22_sec_mask(&self) -> PIO1_PIN22_SEC_MASK_R {
        PIO1_PIN22_SEC_MASK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Secure mask for pin P1_23"]
    #[inline(always)]
    pub fn pio1_pin23_sec_mask(&self) -> PIO1_PIN23_SEC_MASK_R {
        PIO1_PIN23_SEC_MASK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Secure mask for pin P1_24"]
    #[inline(always)]
    pub fn pio1_pin24_sec_mask(&self) -> PIO1_PIN24_SEC_MASK_R {
        PIO1_PIN24_SEC_MASK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Secure mask for pin P1_25"]
    #[inline(always)]
    pub fn pio1_pin25_sec_mask(&self) -> PIO1_PIN25_SEC_MASK_R {
        PIO1_PIN25_SEC_MASK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Secure mask for pin P1_26"]
    #[inline(always)]
    pub fn pio1_pin26_sec_mask(&self) -> PIO1_PIN26_SEC_MASK_R {
        PIO1_PIN26_SEC_MASK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Secure mask for pin P1_27"]
    #[inline(always)]
    pub fn pio1_pin27_sec_mask(&self) -> PIO1_PIN27_SEC_MASK_R {
        PIO1_PIN27_SEC_MASK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Secure mask for pin P1_28"]
    #[inline(always)]
    pub fn pio1_pin28_sec_mask(&self) -> PIO1_PIN28_SEC_MASK_R {
        PIO1_PIN28_SEC_MASK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Secure mask for pin P1_29"]
    #[inline(always)]
    pub fn pio1_pin29_sec_mask(&self) -> PIO1_PIN29_SEC_MASK_R {
        PIO1_PIN29_SEC_MASK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Secure mask for pin P1_30"]
    #[inline(always)]
    pub fn pio1_pin30_sec_mask(&self) -> PIO1_PIN30_SEC_MASK_R {
        PIO1_PIN30_SEC_MASK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Secure mask for pin P1_31"]
    #[inline(always)]
    pub fn pio1_pin31_sec_mask(&self) -> PIO1_PIN31_SEC_MASK_R {
        PIO1_PIN31_SEC_MASK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure mask for pin P1_0"]
    #[inline(always)]
    pub fn pio1_pin0_sec_mask(&mut self) -> PIO1_PIN0_SEC_MASK_W {
        PIO1_PIN0_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 1 - Secure mask for pin P1_1"]
    #[inline(always)]
    pub fn pio1_pin1_sec_mask(&mut self) -> PIO1_PIN1_SEC_MASK_W {
        PIO1_PIN1_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 2 - Secure mask for pin P1_2"]
    #[inline(always)]
    pub fn pio1_pin2_sec_mask(&mut self) -> PIO1_PIN2_SEC_MASK_W {
        PIO1_PIN2_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 3 - Secure mask for pin P1_3"]
    #[inline(always)]
    pub fn pio1_pin3_sec_mask(&mut self) -> PIO1_PIN3_SEC_MASK_W {
        PIO1_PIN3_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 4 - Secure mask for pin P1_4"]
    #[inline(always)]
    pub fn pio1_pin4_sec_mask(&mut self) -> PIO1_PIN4_SEC_MASK_W {
        PIO1_PIN4_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 5 - Secure mask for pin P1_5"]
    #[inline(always)]
    pub fn pio1_pin5_sec_mask(&mut self) -> PIO1_PIN5_SEC_MASK_W {
        PIO1_PIN5_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 6 - Secure mask for pin P1_6"]
    #[inline(always)]
    pub fn pio1_pin6_sec_mask(&mut self) -> PIO1_PIN6_SEC_MASK_W {
        PIO1_PIN6_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 7 - Secure mask for pin P1_7"]
    #[inline(always)]
    pub fn pio1_pin7_sec_mask(&mut self) -> PIO1_PIN7_SEC_MASK_W {
        PIO1_PIN7_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 8 - Secure mask for pin P1_8"]
    #[inline(always)]
    pub fn pio1_pin8_sec_mask(&mut self) -> PIO1_PIN8_SEC_MASK_W {
        PIO1_PIN8_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 9 - Secure mask for pin P1_9"]
    #[inline(always)]
    pub fn pio1_pin9_sec_mask(&mut self) -> PIO1_PIN9_SEC_MASK_W {
        PIO1_PIN9_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 10 - Secure mask for pin P1_10"]
    #[inline(always)]
    pub fn pio1_pin10_sec_mask(&mut self) -> PIO1_PIN10_SEC_MASK_W {
        PIO1_PIN10_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 11 - Secure mask for pin P1_11"]
    #[inline(always)]
    pub fn pio1_pin11_sec_mask(&mut self) -> PIO1_PIN11_SEC_MASK_W {
        PIO1_PIN11_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 12 - Secure mask for pin P1_12"]
    #[inline(always)]
    pub fn pio1_pin12_sec_mask(&mut self) -> PIO1_PIN12_SEC_MASK_W {
        PIO1_PIN12_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 13 - Secure mask for pin P1_13"]
    #[inline(always)]
    pub fn pio1_pin13_sec_mask(&mut self) -> PIO1_PIN13_SEC_MASK_W {
        PIO1_PIN13_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 14 - Secure mask for pin P1_14"]
    #[inline(always)]
    pub fn pio1_pin14_sec_mask(&mut self) -> PIO1_PIN14_SEC_MASK_W {
        PIO1_PIN14_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 15 - Secure mask for pin P1_15"]
    #[inline(always)]
    pub fn pio1_pin15_sec_mask(&mut self) -> PIO1_PIN15_SEC_MASK_W {
        PIO1_PIN15_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 16 - Secure mask for pin P1_16"]
    #[inline(always)]
    pub fn pio1_pin16_sec_mask(&mut self) -> PIO1_PIN16_SEC_MASK_W {
        PIO1_PIN16_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 17 - Secure mask for pin P1_17"]
    #[inline(always)]
    pub fn pio1_pin17_sec_mask(&mut self) -> PIO1_PIN17_SEC_MASK_W {
        PIO1_PIN17_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 18 - Secure mask for pin P1_18"]
    #[inline(always)]
    pub fn pio1_pin18_sec_mask(&mut self) -> PIO1_PIN18_SEC_MASK_W {
        PIO1_PIN18_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 19 - Secure mask for pin P1_19"]
    #[inline(always)]
    pub fn pio1_pin19_sec_mask(&mut self) -> PIO1_PIN19_SEC_MASK_W {
        PIO1_PIN19_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 20 - Secure mask for pin P1_20"]
    #[inline(always)]
    pub fn pio1_pin20_sec_mask(&mut self) -> PIO1_PIN20_SEC_MASK_W {
        PIO1_PIN20_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 21 - Secure mask for pin P1_21"]
    #[inline(always)]
    pub fn pio1_pin21_sec_mask(&mut self) -> PIO1_PIN21_SEC_MASK_W {
        PIO1_PIN21_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 22 - Secure mask for pin P1_22"]
    #[inline(always)]
    pub fn pio1_pin22_sec_mask(&mut self) -> PIO1_PIN22_SEC_MASK_W {
        PIO1_PIN22_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 23 - Secure mask for pin P1_23"]
    #[inline(always)]
    pub fn pio1_pin23_sec_mask(&mut self) -> PIO1_PIN23_SEC_MASK_W {
        PIO1_PIN23_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 24 - Secure mask for pin P1_24"]
    #[inline(always)]
    pub fn pio1_pin24_sec_mask(&mut self) -> PIO1_PIN24_SEC_MASK_W {
        PIO1_PIN24_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 25 - Secure mask for pin P1_25"]
    #[inline(always)]
    pub fn pio1_pin25_sec_mask(&mut self) -> PIO1_PIN25_SEC_MASK_W {
        PIO1_PIN25_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 26 - Secure mask for pin P1_26"]
    #[inline(always)]
    pub fn pio1_pin26_sec_mask(&mut self) -> PIO1_PIN26_SEC_MASK_W {
        PIO1_PIN26_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 27 - Secure mask for pin P1_27"]
    #[inline(always)]
    pub fn pio1_pin27_sec_mask(&mut self) -> PIO1_PIN27_SEC_MASK_W {
        PIO1_PIN27_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 28 - Secure mask for pin P1_28"]
    #[inline(always)]
    pub fn pio1_pin28_sec_mask(&mut self) -> PIO1_PIN28_SEC_MASK_W {
        PIO1_PIN28_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 29 - Secure mask for pin P1_29"]
    #[inline(always)]
    pub fn pio1_pin29_sec_mask(&mut self) -> PIO1_PIN29_SEC_MASK_W {
        PIO1_PIN29_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 30 - Secure mask for pin P1_30"]
    #[inline(always)]
    pub fn pio1_pin30_sec_mask(&mut self) -> PIO1_PIN30_SEC_MASK_W {
        PIO1_PIN30_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 31 - Secure mask for pin P1_31"]
    #[inline(always)]
    pub fn pio1_pin31_sec_mask(&mut self) -> PIO1_PIN31_SEC_MASK_W {
        PIO1_PIN31_SEC_MASK_W { w: self }
    }
}
