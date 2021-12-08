#[doc = "Register `SEC_GPIO_MASK0` reader"]
pub struct R(crate::R<SEC_GPIO_MASK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_GPIO_MASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_GPIO_MASK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_GPIO_MASK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_GPIO_MASK0` writer"]
pub struct W(crate::W<SEC_GPIO_MASK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_GPIO_MASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SEC_GPIO_MASK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_GPIO_MASK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Secure mask for pin P0_0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN0_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN0_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN0_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN0_SEC_MASK` reader - Secure mask for pin P0_0"]
pub struct PIO0_PIN0_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN0_SEC_MASK_A>);
impl PIO0_PIN0_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN0_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN0_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN0_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN0_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN0_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN0_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN0_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN0_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN0_SEC_MASK` writer - Secure mask for pin P0_0"]
pub struct PIO0_PIN0_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN0_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN0_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN0_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN0_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Secure mask for pin P0_1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN1_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN1_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN1_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN1_SEC_MASK` reader - Secure mask for pin P0_1"]
pub struct PIO0_PIN1_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN1_SEC_MASK_A>);
impl PIO0_PIN1_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN1_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN1_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN1_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN1_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN1_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN1_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN1_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN1_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN1_SEC_MASK` writer - Secure mask for pin P0_1"]
pub struct PIO0_PIN1_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN1_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN1_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN1_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN1_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Secure mask for pin P0_2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN2_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN2_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN2_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN2_SEC_MASK` reader - Secure mask for pin P0_2"]
pub struct PIO0_PIN2_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN2_SEC_MASK_A>);
impl PIO0_PIN2_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN2_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN2_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN2_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN2_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN2_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN2_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN2_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN2_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN2_SEC_MASK` writer - Secure mask for pin P0_2"]
pub struct PIO0_PIN2_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN2_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN2_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN2_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN2_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Secure mask for pin P0_3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN3_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN3_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN3_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN3_SEC_MASK` reader - Secure mask for pin P0_3"]
pub struct PIO0_PIN3_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN3_SEC_MASK_A>);
impl PIO0_PIN3_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN3_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN3_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN3_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN3_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN3_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN3_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN3_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN3_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN3_SEC_MASK` writer - Secure mask for pin P0_3"]
pub struct PIO0_PIN3_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN3_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN3_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN3_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN3_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Secure mask for pin P0_4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN4_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN4_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN4_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN4_SEC_MASK` reader - Secure mask for pin P0_4"]
pub struct PIO0_PIN4_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN4_SEC_MASK_A>);
impl PIO0_PIN4_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN4_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN4_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN4_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN4_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN4_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN4_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN4_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN4_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN4_SEC_MASK` writer - Secure mask for pin P0_4"]
pub struct PIO0_PIN4_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN4_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN4_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN4_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN4_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Secure mask for pin P0_5\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN5_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN5_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN5_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN5_SEC_MASK` reader - Secure mask for pin P0_5"]
pub struct PIO0_PIN5_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN5_SEC_MASK_A>);
impl PIO0_PIN5_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN5_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN5_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN5_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN5_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN5_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN5_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN5_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN5_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN5_SEC_MASK` writer - Secure mask for pin P0_5"]
pub struct PIO0_PIN5_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN5_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN5_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN5_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN5_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Secure mask for pin P0_6\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN6_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN6_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN6_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN6_SEC_MASK` reader - Secure mask for pin P0_6"]
pub struct PIO0_PIN6_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN6_SEC_MASK_A>);
impl PIO0_PIN6_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN6_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN6_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN6_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN6_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN6_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN6_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN6_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN6_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN6_SEC_MASK` writer - Secure mask for pin P0_6"]
pub struct PIO0_PIN6_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN6_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN6_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN6_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN6_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Secure mask for pin P0_7\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN7_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN7_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN7_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN7_SEC_MASK` reader - Secure mask for pin P0_7"]
pub struct PIO0_PIN7_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN7_SEC_MASK_A>);
impl PIO0_PIN7_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN7_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN7_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN7_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN7_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN7_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN7_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN7_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN7_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN7_SEC_MASK` writer - Secure mask for pin P0_7"]
pub struct PIO0_PIN7_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN7_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN7_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN7_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN7_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Secure mask for pin P0_8\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN8_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN8_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN8_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN8_SEC_MASK` reader - Secure mask for pin P0_8"]
pub struct PIO0_PIN8_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN8_SEC_MASK_A>);
impl PIO0_PIN8_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN8_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN8_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN8_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN8_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN8_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN8_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN8_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN8_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN8_SEC_MASK` writer - Secure mask for pin P0_8"]
pub struct PIO0_PIN8_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN8_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN8_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN8_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN8_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Secure mask for pin P0_9\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN9_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN9_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN9_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN9_SEC_MASK` reader - Secure mask for pin P0_9"]
pub struct PIO0_PIN9_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN9_SEC_MASK_A>);
impl PIO0_PIN9_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN9_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN9_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN9_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN9_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN9_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN9_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN9_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN9_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN9_SEC_MASK` writer - Secure mask for pin P0_9"]
pub struct PIO0_PIN9_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN9_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN9_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN9_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN9_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Secure mask for pin P0_10\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN10_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN10_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN10_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN10_SEC_MASK` reader - Secure mask for pin P0_10"]
pub struct PIO0_PIN10_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN10_SEC_MASK_A>);
impl PIO0_PIN10_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN10_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN10_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN10_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN10_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN10_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN10_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN10_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN10_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN10_SEC_MASK` writer - Secure mask for pin P0_10"]
pub struct PIO0_PIN10_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN10_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN10_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN10_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN10_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Secure mask for pin P0_11\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN11_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN11_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN11_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN11_SEC_MASK` reader - Secure mask for pin P0_11"]
pub struct PIO0_PIN11_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN11_SEC_MASK_A>);
impl PIO0_PIN11_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN11_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN11_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN11_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN11_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN11_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN11_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN11_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN11_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN11_SEC_MASK` writer - Secure mask for pin P0_11"]
pub struct PIO0_PIN11_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN11_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN11_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN11_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN11_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Secure mask for pin P0_12\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN12_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN12_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN12_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN12_SEC_MASK` reader - Secure mask for pin P0_12"]
pub struct PIO0_PIN12_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN12_SEC_MASK_A>);
impl PIO0_PIN12_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN12_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN12_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN12_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN12_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN12_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN12_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN12_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN12_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN12_SEC_MASK` writer - Secure mask for pin P0_12"]
pub struct PIO0_PIN12_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN12_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN12_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN12_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN12_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Secure mask for pin P0_13\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN13_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN13_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN13_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN13_SEC_MASK` reader - Secure mask for pin P0_13"]
pub struct PIO0_PIN13_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN13_SEC_MASK_A>);
impl PIO0_PIN13_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN13_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN13_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN13_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN13_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN13_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN13_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN13_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN13_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN13_SEC_MASK` writer - Secure mask for pin P0_13"]
pub struct PIO0_PIN13_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN13_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN13_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN13_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN13_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Secure mask for pin P0_14\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN14_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN14_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN14_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN14_SEC_MASK` reader - Secure mask for pin P0_14"]
pub struct PIO0_PIN14_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN14_SEC_MASK_A>);
impl PIO0_PIN14_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN14_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN14_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN14_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN14_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN14_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN14_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN14_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN14_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN14_SEC_MASK` writer - Secure mask for pin P0_14"]
pub struct PIO0_PIN14_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN14_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN14_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN14_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN14_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Secure mask for pin P0_15\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN15_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN15_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN15_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN15_SEC_MASK` reader - Secure mask for pin P0_15"]
pub struct PIO0_PIN15_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN15_SEC_MASK_A>);
impl PIO0_PIN15_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN15_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN15_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN15_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN15_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN15_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN15_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN15_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN15_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN15_SEC_MASK` writer - Secure mask for pin P0_15"]
pub struct PIO0_PIN15_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN15_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN15_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN15_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN15_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Secure mask for pin P0_16\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN16_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN16_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN16_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN16_SEC_MASK` reader - Secure mask for pin P0_16"]
pub struct PIO0_PIN16_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN16_SEC_MASK_A>);
impl PIO0_PIN16_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN16_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN16_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN16_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN16_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN16_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN16_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN16_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN16_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN16_SEC_MASK` writer - Secure mask for pin P0_16"]
pub struct PIO0_PIN16_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN16_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN16_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN16_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN16_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Secure mask for pin P0_17\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN17_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN17_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN17_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN17_SEC_MASK` reader - Secure mask for pin P0_17"]
pub struct PIO0_PIN17_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN17_SEC_MASK_A>);
impl PIO0_PIN17_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN17_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN17_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN17_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN17_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN17_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN17_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN17_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN17_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN17_SEC_MASK` writer - Secure mask for pin P0_17"]
pub struct PIO0_PIN17_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN17_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN17_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN17_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN17_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Secure mask for pin P0_18\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN18_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN18_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN18_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN18_SEC_MASK` reader - Secure mask for pin P0_18"]
pub struct PIO0_PIN18_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN18_SEC_MASK_A>);
impl PIO0_PIN18_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN18_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN18_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN18_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN18_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN18_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN18_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN18_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN18_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN18_SEC_MASK` writer - Secure mask for pin P0_18"]
pub struct PIO0_PIN18_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN18_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN18_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN18_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN18_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Secure mask for pin P0_19\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN19_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN19_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN19_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN19_SEC_MASK` reader - Secure mask for pin P0_19"]
pub struct PIO0_PIN19_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN19_SEC_MASK_A>);
impl PIO0_PIN19_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN19_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN19_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN19_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN19_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN19_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN19_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN19_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN19_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN19_SEC_MASK` writer - Secure mask for pin P0_19"]
pub struct PIO0_PIN19_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN19_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN19_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN19_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN19_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Secure mask for pin P0_20\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN20_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN20_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN20_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN20_SEC_MASK` reader - Secure mask for pin P0_20"]
pub struct PIO0_PIN20_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN20_SEC_MASK_A>);
impl PIO0_PIN20_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN20_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN20_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN20_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN20_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN20_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN20_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN20_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN20_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN20_SEC_MASK` writer - Secure mask for pin P0_20"]
pub struct PIO0_PIN20_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN20_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN20_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN20_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN20_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Secure mask for pin P0_21\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN21_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN21_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN21_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN21_SEC_MASK` reader - Secure mask for pin P0_21"]
pub struct PIO0_PIN21_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN21_SEC_MASK_A>);
impl PIO0_PIN21_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN21_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN21_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN21_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN21_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN21_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN21_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN21_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN21_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN21_SEC_MASK` writer - Secure mask for pin P0_21"]
pub struct PIO0_PIN21_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN21_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN21_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN21_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN21_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Secure mask for pin P0_22\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN22_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN22_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN22_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN22_SEC_MASK` reader - Secure mask for pin P0_22"]
pub struct PIO0_PIN22_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN22_SEC_MASK_A>);
impl PIO0_PIN22_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN22_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN22_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN22_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN22_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN22_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN22_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN22_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN22_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN22_SEC_MASK` writer - Secure mask for pin P0_22"]
pub struct PIO0_PIN22_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN22_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN22_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN22_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN22_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Secure mask for pin P0_23\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN23_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN23_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN23_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN23_SEC_MASK` reader - Secure mask for pin P0_23"]
pub struct PIO0_PIN23_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN23_SEC_MASK_A>);
impl PIO0_PIN23_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN23_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN23_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN23_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN23_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN23_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN23_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN23_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN23_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN23_SEC_MASK` writer - Secure mask for pin P0_23"]
pub struct PIO0_PIN23_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN23_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN23_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN23_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN23_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Secure mask for pin P0_24\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN24_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN24_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN24_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN24_SEC_MASK` reader - Secure mask for pin P0_24"]
pub struct PIO0_PIN24_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN24_SEC_MASK_A>);
impl PIO0_PIN24_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN24_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN24_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN24_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN24_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN24_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN24_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN24_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN24_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN24_SEC_MASK` writer - Secure mask for pin P0_24"]
pub struct PIO0_PIN24_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN24_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN24_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN24_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN24_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Secure mask for pin P0_25\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN25_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN25_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN25_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN25_SEC_MASK` reader - Secure mask for pin P0_25"]
pub struct PIO0_PIN25_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN25_SEC_MASK_A>);
impl PIO0_PIN25_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN25_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN25_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN25_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN25_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN25_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN25_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN25_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN25_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN25_SEC_MASK` writer - Secure mask for pin P0_25"]
pub struct PIO0_PIN25_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN25_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN25_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN25_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN25_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Secure mask for pin P0_26\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN26_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN26_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN26_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN26_SEC_MASK` reader - Secure mask for pin P0_26"]
pub struct PIO0_PIN26_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN26_SEC_MASK_A>);
impl PIO0_PIN26_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN26_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN26_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN26_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN26_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN26_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN26_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN26_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN26_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN26_SEC_MASK` writer - Secure mask for pin P0_26"]
pub struct PIO0_PIN26_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN26_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN26_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN26_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN26_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Secure mask for pin P0_27\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN27_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN27_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN27_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN27_SEC_MASK` reader - Secure mask for pin P0_27"]
pub struct PIO0_PIN27_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN27_SEC_MASK_A>);
impl PIO0_PIN27_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN27_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN27_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN27_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN27_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN27_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN27_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN27_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN27_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN27_SEC_MASK` writer - Secure mask for pin P0_27"]
pub struct PIO0_PIN27_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN27_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN27_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN27_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN27_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Secure mask for pin P0_28\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN28_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN28_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN28_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN28_SEC_MASK` reader - Secure mask for pin P0_28"]
pub struct PIO0_PIN28_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN28_SEC_MASK_A>);
impl PIO0_PIN28_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN28_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN28_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN28_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN28_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN28_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN28_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN28_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN28_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN28_SEC_MASK` writer - Secure mask for pin P0_28"]
pub struct PIO0_PIN28_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN28_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN28_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN28_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN28_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Secure mask for pin P0_29\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN29_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN29_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN29_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN29_SEC_MASK` reader - Secure mask for pin P0_29"]
pub struct PIO0_PIN29_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN29_SEC_MASK_A>);
impl PIO0_PIN29_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN29_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN29_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN29_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN29_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN29_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN29_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN29_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN29_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN29_SEC_MASK` writer - Secure mask for pin P0_29"]
pub struct PIO0_PIN29_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN29_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN29_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN29_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN29_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Secure mask for pin P0_30\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN30_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN30_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN30_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN30_SEC_MASK` reader - Secure mask for pin P0_30"]
pub struct PIO0_PIN30_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN30_SEC_MASK_A>);
impl PIO0_PIN30_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN30_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN30_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN30_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN30_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN30_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN30_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN30_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN30_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN30_SEC_MASK` writer - Secure mask for pin P0_30"]
pub struct PIO0_PIN30_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN30_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN30_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN30_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN30_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Secure mask for pin P0_31\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_PIN31_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO0_PIN31_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_PIN31_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0_PIN31_SEC_MASK` reader - Secure mask for pin P0_31"]
pub struct PIO0_PIN31_SEC_MASK_R(crate::FieldReader<bool, PIO0_PIN31_SEC_MASK_A>);
impl PIO0_PIN31_SEC_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIO0_PIN31_SEC_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_PIN31_SEC_MASK_A {
        match self.bits {
            false => PIO0_PIN31_SEC_MASK_A::BLOCKED,
            true => PIO0_PIN31_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        **self == PIO0_PIN31_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        **self == PIO0_PIN31_SEC_MASK_A::READABLE
    }
}
impl core::ops::Deref for PIO0_PIN31_SEC_MASK_R {
    type Target = crate::FieldReader<bool, PIO0_PIN31_SEC_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIO0_PIN31_SEC_MASK` writer - Secure mask for pin P0_31"]
pub struct PIO0_PIN31_SEC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PIO0_PIN31_SEC_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIO0_PIN31_SEC_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO0_PIN31_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO0_PIN31_SEC_MASK_A::READABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Secure mask for pin P0_0"]
    #[inline(always)]
    pub fn pio0_pin0_sec_mask(&self) -> PIO0_PIN0_SEC_MASK_R {
        PIO0_PIN0_SEC_MASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Secure mask for pin P0_1"]
    #[inline(always)]
    pub fn pio0_pin1_sec_mask(&self) -> PIO0_PIN1_SEC_MASK_R {
        PIO0_PIN1_SEC_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Secure mask for pin P0_2"]
    #[inline(always)]
    pub fn pio0_pin2_sec_mask(&self) -> PIO0_PIN2_SEC_MASK_R {
        PIO0_PIN2_SEC_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Secure mask for pin P0_3"]
    #[inline(always)]
    pub fn pio0_pin3_sec_mask(&self) -> PIO0_PIN3_SEC_MASK_R {
        PIO0_PIN3_SEC_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Secure mask for pin P0_4"]
    #[inline(always)]
    pub fn pio0_pin4_sec_mask(&self) -> PIO0_PIN4_SEC_MASK_R {
        PIO0_PIN4_SEC_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Secure mask for pin P0_5"]
    #[inline(always)]
    pub fn pio0_pin5_sec_mask(&self) -> PIO0_PIN5_SEC_MASK_R {
        PIO0_PIN5_SEC_MASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Secure mask for pin P0_6"]
    #[inline(always)]
    pub fn pio0_pin6_sec_mask(&self) -> PIO0_PIN6_SEC_MASK_R {
        PIO0_PIN6_SEC_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Secure mask for pin P0_7"]
    #[inline(always)]
    pub fn pio0_pin7_sec_mask(&self) -> PIO0_PIN7_SEC_MASK_R {
        PIO0_PIN7_SEC_MASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Secure mask for pin P0_8"]
    #[inline(always)]
    pub fn pio0_pin8_sec_mask(&self) -> PIO0_PIN8_SEC_MASK_R {
        PIO0_PIN8_SEC_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Secure mask for pin P0_9"]
    #[inline(always)]
    pub fn pio0_pin9_sec_mask(&self) -> PIO0_PIN9_SEC_MASK_R {
        PIO0_PIN9_SEC_MASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Secure mask for pin P0_10"]
    #[inline(always)]
    pub fn pio0_pin10_sec_mask(&self) -> PIO0_PIN10_SEC_MASK_R {
        PIO0_PIN10_SEC_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Secure mask for pin P0_11"]
    #[inline(always)]
    pub fn pio0_pin11_sec_mask(&self) -> PIO0_PIN11_SEC_MASK_R {
        PIO0_PIN11_SEC_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Secure mask for pin P0_12"]
    #[inline(always)]
    pub fn pio0_pin12_sec_mask(&self) -> PIO0_PIN12_SEC_MASK_R {
        PIO0_PIN12_SEC_MASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Secure mask for pin P0_13"]
    #[inline(always)]
    pub fn pio0_pin13_sec_mask(&self) -> PIO0_PIN13_SEC_MASK_R {
        PIO0_PIN13_SEC_MASK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Secure mask for pin P0_14"]
    #[inline(always)]
    pub fn pio0_pin14_sec_mask(&self) -> PIO0_PIN14_SEC_MASK_R {
        PIO0_PIN14_SEC_MASK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Secure mask for pin P0_15"]
    #[inline(always)]
    pub fn pio0_pin15_sec_mask(&self) -> PIO0_PIN15_SEC_MASK_R {
        PIO0_PIN15_SEC_MASK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Secure mask for pin P0_16"]
    #[inline(always)]
    pub fn pio0_pin16_sec_mask(&self) -> PIO0_PIN16_SEC_MASK_R {
        PIO0_PIN16_SEC_MASK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Secure mask for pin P0_17"]
    #[inline(always)]
    pub fn pio0_pin17_sec_mask(&self) -> PIO0_PIN17_SEC_MASK_R {
        PIO0_PIN17_SEC_MASK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Secure mask for pin P0_18"]
    #[inline(always)]
    pub fn pio0_pin18_sec_mask(&self) -> PIO0_PIN18_SEC_MASK_R {
        PIO0_PIN18_SEC_MASK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Secure mask for pin P0_19"]
    #[inline(always)]
    pub fn pio0_pin19_sec_mask(&self) -> PIO0_PIN19_SEC_MASK_R {
        PIO0_PIN19_SEC_MASK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Secure mask for pin P0_20"]
    #[inline(always)]
    pub fn pio0_pin20_sec_mask(&self) -> PIO0_PIN20_SEC_MASK_R {
        PIO0_PIN20_SEC_MASK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Secure mask for pin P0_21"]
    #[inline(always)]
    pub fn pio0_pin21_sec_mask(&self) -> PIO0_PIN21_SEC_MASK_R {
        PIO0_PIN21_SEC_MASK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Secure mask for pin P0_22"]
    #[inline(always)]
    pub fn pio0_pin22_sec_mask(&self) -> PIO0_PIN22_SEC_MASK_R {
        PIO0_PIN22_SEC_MASK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Secure mask for pin P0_23"]
    #[inline(always)]
    pub fn pio0_pin23_sec_mask(&self) -> PIO0_PIN23_SEC_MASK_R {
        PIO0_PIN23_SEC_MASK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Secure mask for pin P0_24"]
    #[inline(always)]
    pub fn pio0_pin24_sec_mask(&self) -> PIO0_PIN24_SEC_MASK_R {
        PIO0_PIN24_SEC_MASK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Secure mask for pin P0_25"]
    #[inline(always)]
    pub fn pio0_pin25_sec_mask(&self) -> PIO0_PIN25_SEC_MASK_R {
        PIO0_PIN25_SEC_MASK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Secure mask for pin P0_26"]
    #[inline(always)]
    pub fn pio0_pin26_sec_mask(&self) -> PIO0_PIN26_SEC_MASK_R {
        PIO0_PIN26_SEC_MASK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Secure mask for pin P0_27"]
    #[inline(always)]
    pub fn pio0_pin27_sec_mask(&self) -> PIO0_PIN27_SEC_MASK_R {
        PIO0_PIN27_SEC_MASK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Secure mask for pin P0_28"]
    #[inline(always)]
    pub fn pio0_pin28_sec_mask(&self) -> PIO0_PIN28_SEC_MASK_R {
        PIO0_PIN28_SEC_MASK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Secure mask for pin P0_29"]
    #[inline(always)]
    pub fn pio0_pin29_sec_mask(&self) -> PIO0_PIN29_SEC_MASK_R {
        PIO0_PIN29_SEC_MASK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Secure mask for pin P0_30"]
    #[inline(always)]
    pub fn pio0_pin30_sec_mask(&self) -> PIO0_PIN30_SEC_MASK_R {
        PIO0_PIN30_SEC_MASK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Secure mask for pin P0_31"]
    #[inline(always)]
    pub fn pio0_pin31_sec_mask(&self) -> PIO0_PIN31_SEC_MASK_R {
        PIO0_PIN31_SEC_MASK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure mask for pin P0_0"]
    #[inline(always)]
    pub fn pio0_pin0_sec_mask(&mut self) -> PIO0_PIN0_SEC_MASK_W {
        PIO0_PIN0_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 1 - Secure mask for pin P0_1"]
    #[inline(always)]
    pub fn pio0_pin1_sec_mask(&mut self) -> PIO0_PIN1_SEC_MASK_W {
        PIO0_PIN1_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 2 - Secure mask for pin P0_2"]
    #[inline(always)]
    pub fn pio0_pin2_sec_mask(&mut self) -> PIO0_PIN2_SEC_MASK_W {
        PIO0_PIN2_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 3 - Secure mask for pin P0_3"]
    #[inline(always)]
    pub fn pio0_pin3_sec_mask(&mut self) -> PIO0_PIN3_SEC_MASK_W {
        PIO0_PIN3_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 4 - Secure mask for pin P0_4"]
    #[inline(always)]
    pub fn pio0_pin4_sec_mask(&mut self) -> PIO0_PIN4_SEC_MASK_W {
        PIO0_PIN4_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 5 - Secure mask for pin P0_5"]
    #[inline(always)]
    pub fn pio0_pin5_sec_mask(&mut self) -> PIO0_PIN5_SEC_MASK_W {
        PIO0_PIN5_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 6 - Secure mask for pin P0_6"]
    #[inline(always)]
    pub fn pio0_pin6_sec_mask(&mut self) -> PIO0_PIN6_SEC_MASK_W {
        PIO0_PIN6_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 7 - Secure mask for pin P0_7"]
    #[inline(always)]
    pub fn pio0_pin7_sec_mask(&mut self) -> PIO0_PIN7_SEC_MASK_W {
        PIO0_PIN7_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 8 - Secure mask for pin P0_8"]
    #[inline(always)]
    pub fn pio0_pin8_sec_mask(&mut self) -> PIO0_PIN8_SEC_MASK_W {
        PIO0_PIN8_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 9 - Secure mask for pin P0_9"]
    #[inline(always)]
    pub fn pio0_pin9_sec_mask(&mut self) -> PIO0_PIN9_SEC_MASK_W {
        PIO0_PIN9_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 10 - Secure mask for pin P0_10"]
    #[inline(always)]
    pub fn pio0_pin10_sec_mask(&mut self) -> PIO0_PIN10_SEC_MASK_W {
        PIO0_PIN10_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 11 - Secure mask for pin P0_11"]
    #[inline(always)]
    pub fn pio0_pin11_sec_mask(&mut self) -> PIO0_PIN11_SEC_MASK_W {
        PIO0_PIN11_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 12 - Secure mask for pin P0_12"]
    #[inline(always)]
    pub fn pio0_pin12_sec_mask(&mut self) -> PIO0_PIN12_SEC_MASK_W {
        PIO0_PIN12_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 13 - Secure mask for pin P0_13"]
    #[inline(always)]
    pub fn pio0_pin13_sec_mask(&mut self) -> PIO0_PIN13_SEC_MASK_W {
        PIO0_PIN13_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 14 - Secure mask for pin P0_14"]
    #[inline(always)]
    pub fn pio0_pin14_sec_mask(&mut self) -> PIO0_PIN14_SEC_MASK_W {
        PIO0_PIN14_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 15 - Secure mask for pin P0_15"]
    #[inline(always)]
    pub fn pio0_pin15_sec_mask(&mut self) -> PIO0_PIN15_SEC_MASK_W {
        PIO0_PIN15_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 16 - Secure mask for pin P0_16"]
    #[inline(always)]
    pub fn pio0_pin16_sec_mask(&mut self) -> PIO0_PIN16_SEC_MASK_W {
        PIO0_PIN16_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 17 - Secure mask for pin P0_17"]
    #[inline(always)]
    pub fn pio0_pin17_sec_mask(&mut self) -> PIO0_PIN17_SEC_MASK_W {
        PIO0_PIN17_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 18 - Secure mask for pin P0_18"]
    #[inline(always)]
    pub fn pio0_pin18_sec_mask(&mut self) -> PIO0_PIN18_SEC_MASK_W {
        PIO0_PIN18_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 19 - Secure mask for pin P0_19"]
    #[inline(always)]
    pub fn pio0_pin19_sec_mask(&mut self) -> PIO0_PIN19_SEC_MASK_W {
        PIO0_PIN19_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 20 - Secure mask for pin P0_20"]
    #[inline(always)]
    pub fn pio0_pin20_sec_mask(&mut self) -> PIO0_PIN20_SEC_MASK_W {
        PIO0_PIN20_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 21 - Secure mask for pin P0_21"]
    #[inline(always)]
    pub fn pio0_pin21_sec_mask(&mut self) -> PIO0_PIN21_SEC_MASK_W {
        PIO0_PIN21_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 22 - Secure mask for pin P0_22"]
    #[inline(always)]
    pub fn pio0_pin22_sec_mask(&mut self) -> PIO0_PIN22_SEC_MASK_W {
        PIO0_PIN22_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 23 - Secure mask for pin P0_23"]
    #[inline(always)]
    pub fn pio0_pin23_sec_mask(&mut self) -> PIO0_PIN23_SEC_MASK_W {
        PIO0_PIN23_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 24 - Secure mask for pin P0_24"]
    #[inline(always)]
    pub fn pio0_pin24_sec_mask(&mut self) -> PIO0_PIN24_SEC_MASK_W {
        PIO0_PIN24_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 25 - Secure mask for pin P0_25"]
    #[inline(always)]
    pub fn pio0_pin25_sec_mask(&mut self) -> PIO0_PIN25_SEC_MASK_W {
        PIO0_PIN25_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 26 - Secure mask for pin P0_26"]
    #[inline(always)]
    pub fn pio0_pin26_sec_mask(&mut self) -> PIO0_PIN26_SEC_MASK_W {
        PIO0_PIN26_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 27 - Secure mask for pin P0_27"]
    #[inline(always)]
    pub fn pio0_pin27_sec_mask(&mut self) -> PIO0_PIN27_SEC_MASK_W {
        PIO0_PIN27_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 28 - Secure mask for pin P0_28"]
    #[inline(always)]
    pub fn pio0_pin28_sec_mask(&mut self) -> PIO0_PIN28_SEC_MASK_W {
        PIO0_PIN28_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 29 - Secure mask for pin P0_29"]
    #[inline(always)]
    pub fn pio0_pin29_sec_mask(&mut self) -> PIO0_PIN29_SEC_MASK_W {
        PIO0_PIN29_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 30 - Secure mask for pin P0_30"]
    #[inline(always)]
    pub fn pio0_pin30_sec_mask(&mut self) -> PIO0_PIN30_SEC_MASK_W {
        PIO0_PIN30_SEC_MASK_W { w: self }
    }
    #[doc = "Bit 31 - Secure mask for pin P0_31"]
    #[inline(always)]
    pub fn pio0_pin31_sec_mask(&mut self) -> PIO0_PIN31_SEC_MASK_W {
        PIO0_PIN31_SEC_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure GPIO mask for port 0 pins.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_gpio_mask0](index.html) module"]
pub struct SEC_GPIO_MASK0_SPEC;
impl crate::RegisterSpec for SEC_GPIO_MASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_gpio_mask0::R](R) reader structure"]
impl crate::Readable for SEC_GPIO_MASK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_gpio_mask0::W](W) writer structure"]
impl crate::Writable for SEC_GPIO_MASK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_GPIO_MASK0 to value 0xffff_ffff"]
impl crate::Resettable for SEC_GPIO_MASK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
