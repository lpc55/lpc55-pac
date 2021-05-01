#[doc = "Register `SEC_CPU_INT_MASK0` reader"]
pub struct R(crate::R<SEC_CPU_INT_MASK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CPU_INT_MASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SEC_CPU_INT_MASK0_SPEC>> for R {
    fn from(reader: crate::R<SEC_CPU_INT_MASK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CPU_INT_MASK0` writer"]
pub struct W(crate::W<SEC_CPU_INT_MASK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CPU_INT_MASK0_SPEC>;
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
impl core::convert::From<crate::W<SEC_CPU_INT_MASK0_SPEC>> for W {
    fn from(writer: crate::W<SEC_CPU_INT_MASK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Watchdog Timer, Brown Out Detectors and Flash Controller interrupts\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYS_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<SYS_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SYS_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYS_IRQ` reader - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
pub struct SYS_IRQ_R(crate::FieldReader<bool, SYS_IRQ_A>);
impl SYS_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYS_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYS_IRQ_A {
        match self.bits {
            false => SYS_IRQ_A::INVISIBLE,
            true => SYS_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == SYS_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == SYS_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for SYS_IRQ_R {
    type Target = crate::FieldReader<bool, SYS_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_IRQ` writer - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
pub struct SYS_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYS_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SYS_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SYS_IRQ_A::VISIBLE)
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
#[doc = "System DMA 0 (non-secure) interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA0_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<SDMA0_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA0_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA0_IRQ` reader - System DMA 0 (non-secure) interrupt."]
pub struct SDMA0_IRQ_R(crate::FieldReader<bool, SDMA0_IRQ_A>);
impl SDMA0_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMA0_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA0_IRQ_A {
        match self.bits {
            false => SDMA0_IRQ_A::INVISIBLE,
            true => SDMA0_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == SDMA0_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == SDMA0_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for SDMA0_IRQ_R {
    type Target = crate::FieldReader<bool, SDMA0_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMA0_IRQ` writer - System DMA 0 (non-secure) interrupt."]
pub struct SDMA0_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA0_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMA0_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SDMA0_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SDMA0_IRQ_A::VISIBLE)
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
#[doc = "GPIO Group 0 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_GLOBALINT0_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_GLOBALINT0_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_GLOBALINT0_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_GLOBALINT0_IRQ` reader - GPIO Group 0 interrupt."]
pub struct GPIO_GLOBALINT0_IRQ_R(crate::FieldReader<bool, GPIO_GLOBALINT0_IRQ_A>);
impl GPIO_GLOBALINT0_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_GLOBALINT0_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_GLOBALINT0_IRQ_A {
        match self.bits {
            false => GPIO_GLOBALINT0_IRQ_A::INVISIBLE,
            true => GPIO_GLOBALINT0_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == GPIO_GLOBALINT0_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == GPIO_GLOBALINT0_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for GPIO_GLOBALINT0_IRQ_R {
    type Target = crate::FieldReader<bool, GPIO_GLOBALINT0_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_GLOBALINT0_IRQ` writer - GPIO Group 0 interrupt."]
pub struct GPIO_GLOBALINT0_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_GLOBALINT0_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_GLOBALINT0_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_GLOBALINT0_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_GLOBALINT0_IRQ_A::VISIBLE)
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
#[doc = "GPIO Group 1 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_GLOBALINT1_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_GLOBALINT1_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_GLOBALINT1_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_GLOBALINT1_IRQ` reader - GPIO Group 1 interrupt."]
pub struct GPIO_GLOBALINT1_IRQ_R(crate::FieldReader<bool, GPIO_GLOBALINT1_IRQ_A>);
impl GPIO_GLOBALINT1_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_GLOBALINT1_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_GLOBALINT1_IRQ_A {
        match self.bits {
            false => GPIO_GLOBALINT1_IRQ_A::INVISIBLE,
            true => GPIO_GLOBALINT1_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == GPIO_GLOBALINT1_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == GPIO_GLOBALINT1_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for GPIO_GLOBALINT1_IRQ_R {
    type Target = crate::FieldReader<bool, GPIO_GLOBALINT1_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_GLOBALINT1_IRQ` writer - GPIO Group 1 interrupt."]
pub struct GPIO_GLOBALINT1_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_GLOBALINT1_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_GLOBALINT1_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_GLOBALINT1_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_GLOBALINT1_IRQ_A::VISIBLE)
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
#[doc = "Pin interrupt 0 or pattern match engine slice 0 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ0_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_INT0_IRQ0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ0` reader - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
pub struct GPIO_INT0_IRQ0_R(crate::FieldReader<bool, GPIO_INT0_IRQ0_A>);
impl GPIO_INT0_IRQ0_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_INT0_IRQ0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ0_A {
        match self.bits {
            false => GPIO_INT0_IRQ0_A::INVISIBLE,
            true => GPIO_INT0_IRQ0_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == GPIO_INT0_IRQ0_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == GPIO_INT0_IRQ0_A::VISIBLE
    }
}
impl core::ops::Deref for GPIO_INT0_IRQ0_R {
    type Target = crate::FieldReader<bool, GPIO_INT0_IRQ0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_INT0_IRQ0` writer - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
pub struct GPIO_INT0_IRQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT0_IRQ0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT0_IRQ0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ0_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ0_A::VISIBLE)
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
#[doc = "Pin interrupt 1 or pattern match engine slice 1 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ1_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_INT0_IRQ1_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ1` reader - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
pub struct GPIO_INT0_IRQ1_R(crate::FieldReader<bool, GPIO_INT0_IRQ1_A>);
impl GPIO_INT0_IRQ1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_INT0_IRQ1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ1_A {
        match self.bits {
            false => GPIO_INT0_IRQ1_A::INVISIBLE,
            true => GPIO_INT0_IRQ1_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == GPIO_INT0_IRQ1_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == GPIO_INT0_IRQ1_A::VISIBLE
    }
}
impl core::ops::Deref for GPIO_INT0_IRQ1_R {
    type Target = crate::FieldReader<bool, GPIO_INT0_IRQ1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_INT0_IRQ1` writer - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
pub struct GPIO_INT0_IRQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT0_IRQ1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT0_IRQ1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ1_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ1_A::VISIBLE)
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
#[doc = "Pin interrupt 2 or pattern match engine slice 2 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ2_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_INT0_IRQ2_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ2` reader - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
pub struct GPIO_INT0_IRQ2_R(crate::FieldReader<bool, GPIO_INT0_IRQ2_A>);
impl GPIO_INT0_IRQ2_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_INT0_IRQ2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ2_A {
        match self.bits {
            false => GPIO_INT0_IRQ2_A::INVISIBLE,
            true => GPIO_INT0_IRQ2_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == GPIO_INT0_IRQ2_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == GPIO_INT0_IRQ2_A::VISIBLE
    }
}
impl core::ops::Deref for GPIO_INT0_IRQ2_R {
    type Target = crate::FieldReader<bool, GPIO_INT0_IRQ2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_INT0_IRQ2` writer - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
pub struct GPIO_INT0_IRQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT0_IRQ2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT0_IRQ2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ2_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ2_A::VISIBLE)
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
#[doc = "Pin interrupt 3 or pattern match engine slice 3 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT0_IRQ3_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<GPIO_INT0_IRQ3_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT0_IRQ3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ3` reader - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
pub struct GPIO_INT0_IRQ3_R(crate::FieldReader<bool, GPIO_INT0_IRQ3_A>);
impl GPIO_INT0_IRQ3_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_INT0_IRQ3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT0_IRQ3_A {
        match self.bits {
            false => GPIO_INT0_IRQ3_A::INVISIBLE,
            true => GPIO_INT0_IRQ3_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == GPIO_INT0_IRQ3_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == GPIO_INT0_IRQ3_A::VISIBLE
    }
}
impl core::ops::Deref for GPIO_INT0_IRQ3_R {
    type Target = crate::FieldReader<bool, GPIO_INT0_IRQ3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_INT0_IRQ3` writer - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
pub struct GPIO_INT0_IRQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT0_IRQ3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT0_IRQ3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ3_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(GPIO_INT0_IRQ3_A::VISIBLE)
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
#[doc = "Micro Tick Timer interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UTICK_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<UTICK_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: UTICK_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK_IRQ` reader - Micro Tick Timer interrupt."]
pub struct UTICK_IRQ_R(crate::FieldReader<bool, UTICK_IRQ_A>);
impl UTICK_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTICK_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTICK_IRQ_A {
        match self.bits {
            false => UTICK_IRQ_A::INVISIBLE,
            true => UTICK_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == UTICK_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == UTICK_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for UTICK_IRQ_R {
    type Target = crate::FieldReader<bool, UTICK_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTICK_IRQ` writer - Micro Tick Timer interrupt."]
pub struct UTICK_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> UTICK_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UTICK_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(UTICK_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(UTICK_IRQ_A::VISIBLE)
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
#[doc = "Multi-Rate Timer interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<MRT_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: MRT_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT_IRQ` reader - Multi-Rate Timer interrupt."]
pub struct MRT_IRQ_R(crate::FieldReader<bool, MRT_IRQ_A>);
impl MRT_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRT_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_IRQ_A {
        match self.bits {
            false => MRT_IRQ_A::INVISIBLE,
            true => MRT_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == MRT_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == MRT_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for MRT_IRQ_R {
    type Target = crate::FieldReader<bool, MRT_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRT_IRQ` writer - Multi-Rate Timer interrupt."]
pub struct MRT_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRT_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(MRT_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(MRT_IRQ_A::VISIBLE)
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
#[doc = "Standard counter/timer 0 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER0_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<CTIMER0_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER0_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIMER0_IRQ` reader - Standard counter/timer 0 interrupt."]
pub struct CTIMER0_IRQ_R(crate::FieldReader<bool, CTIMER0_IRQ_A>);
impl CTIMER0_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER0_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER0_IRQ_A {
        match self.bits {
            false => CTIMER0_IRQ_A::INVISIBLE,
            true => CTIMER0_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == CTIMER0_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == CTIMER0_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for CTIMER0_IRQ_R {
    type Target = crate::FieldReader<bool, CTIMER0_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER0_IRQ` writer - Standard counter/timer 0 interrupt."]
pub struct CTIMER0_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER0_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER0_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER0_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER0_IRQ_A::VISIBLE)
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
#[doc = "Standard counter/timer 1 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER1_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<CTIMER1_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER1_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIMER1_IRQ` reader - Standard counter/timer 1 interrupt."]
pub struct CTIMER1_IRQ_R(crate::FieldReader<bool, CTIMER1_IRQ_A>);
impl CTIMER1_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER1_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER1_IRQ_A {
        match self.bits {
            false => CTIMER1_IRQ_A::INVISIBLE,
            true => CTIMER1_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == CTIMER1_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == CTIMER1_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for CTIMER1_IRQ_R {
    type Target = crate::FieldReader<bool, CTIMER1_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER1_IRQ` writer - Standard counter/timer 1 interrupt."]
pub struct CTIMER1_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER1_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER1_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER1_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER1_IRQ_A::VISIBLE)
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
#[doc = "SCTimer/PWM interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<SCT_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT_IRQ` reader - SCTimer/PWM interrupt."]
pub struct SCT_IRQ_R(crate::FieldReader<bool, SCT_IRQ_A>);
impl SCT_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCT_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_IRQ_A {
        match self.bits {
            false => SCT_IRQ_A::INVISIBLE,
            true => SCT_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == SCT_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == SCT_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for SCT_IRQ_R {
    type Target = crate::FieldReader<bool, SCT_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT_IRQ` writer - SCTimer/PWM interrupt."]
pub struct SCT_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(SCT_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(SCT_IRQ_A::VISIBLE)
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
#[doc = "Standard counter/timer 3 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER3_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<CTIMER3_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER3_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIMER3_IRQ` reader - Standard counter/timer 3 interrupt."]
pub struct CTIMER3_IRQ_R(crate::FieldReader<bool, CTIMER3_IRQ_A>);
impl CTIMER3_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER3_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER3_IRQ_A {
        match self.bits {
            false => CTIMER3_IRQ_A::INVISIBLE,
            true => CTIMER3_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == CTIMER3_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == CTIMER3_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for CTIMER3_IRQ_R {
    type Target = crate::FieldReader<bool, CTIMER3_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER3_IRQ` writer - Standard counter/timer 3 interrupt."]
pub struct CTIMER3_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER3_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER3_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(CTIMER3_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(CTIMER3_IRQ_A::VISIBLE)
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
#[doc = "Flexcomm 0 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM0_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM0_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM0_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM0_IRQ` reader - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM0_IRQ_R(crate::FieldReader<bool, FLEXCOMM0_IRQ_A>);
impl FLEXCOMM0_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM0_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM0_IRQ_A {
        match self.bits {
            false => FLEXCOMM0_IRQ_A::INVISIBLE,
            true => FLEXCOMM0_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == FLEXCOMM0_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == FLEXCOMM0_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for FLEXCOMM0_IRQ_R {
    type Target = crate::FieldReader<bool, FLEXCOMM0_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM0_IRQ` writer - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM0_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM0_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM0_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM0_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM0_IRQ_A::VISIBLE)
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
#[doc = "Flexcomm 1 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM1_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM1_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM1_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM1_IRQ` reader - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM1_IRQ_R(crate::FieldReader<bool, FLEXCOMM1_IRQ_A>);
impl FLEXCOMM1_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM1_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM1_IRQ_A {
        match self.bits {
            false => FLEXCOMM1_IRQ_A::INVISIBLE,
            true => FLEXCOMM1_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == FLEXCOMM1_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == FLEXCOMM1_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for FLEXCOMM1_IRQ_R {
    type Target = crate::FieldReader<bool, FLEXCOMM1_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM1_IRQ` writer - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM1_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM1_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM1_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM1_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM1_IRQ_A::VISIBLE)
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
#[doc = "Flexcomm 2 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM2_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM2_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM2_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM2_IRQ` reader - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM2_IRQ_R(crate::FieldReader<bool, FLEXCOMM2_IRQ_A>);
impl FLEXCOMM2_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM2_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM2_IRQ_A {
        match self.bits {
            false => FLEXCOMM2_IRQ_A::INVISIBLE,
            true => FLEXCOMM2_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == FLEXCOMM2_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == FLEXCOMM2_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for FLEXCOMM2_IRQ_R {
    type Target = crate::FieldReader<bool, FLEXCOMM2_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM2_IRQ` writer - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM2_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM2_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM2_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM2_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM2_IRQ_A::VISIBLE)
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
#[doc = "Flexcomm 3 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM3_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM3_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM3_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM3_IRQ` reader - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM3_IRQ_R(crate::FieldReader<bool, FLEXCOMM3_IRQ_A>);
impl FLEXCOMM3_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM3_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM3_IRQ_A {
        match self.bits {
            false => FLEXCOMM3_IRQ_A::INVISIBLE,
            true => FLEXCOMM3_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == FLEXCOMM3_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == FLEXCOMM3_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for FLEXCOMM3_IRQ_R {
    type Target = crate::FieldReader<bool, FLEXCOMM3_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM3_IRQ` writer - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM3_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM3_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM3_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM3_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM3_IRQ_A::VISIBLE)
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
#[doc = "Flexcomm 4 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM4_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM4_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM4_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM4_IRQ` reader - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM4_IRQ_R(crate::FieldReader<bool, FLEXCOMM4_IRQ_A>);
impl FLEXCOMM4_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM4_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM4_IRQ_A {
        match self.bits {
            false => FLEXCOMM4_IRQ_A::INVISIBLE,
            true => FLEXCOMM4_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == FLEXCOMM4_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == FLEXCOMM4_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for FLEXCOMM4_IRQ_R {
    type Target = crate::FieldReader<bool, FLEXCOMM4_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM4_IRQ` writer - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM4_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM4_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM4_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM4_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM4_IRQ_A::VISIBLE)
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
#[doc = "Flexcomm 5 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM5_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM5_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM5_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM5_IRQ` reader - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM5_IRQ_R(crate::FieldReader<bool, FLEXCOMM5_IRQ_A>);
impl FLEXCOMM5_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM5_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM5_IRQ_A {
        match self.bits {
            false => FLEXCOMM5_IRQ_A::INVISIBLE,
            true => FLEXCOMM5_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == FLEXCOMM5_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == FLEXCOMM5_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for FLEXCOMM5_IRQ_R {
    type Target = crate::FieldReader<bool, FLEXCOMM5_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM5_IRQ` writer - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM5_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM5_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM5_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM5_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM5_IRQ_A::VISIBLE)
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
#[doc = "Flexcomm 6 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM6_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM6_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM6_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM6_IRQ` reader - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM6_IRQ_R(crate::FieldReader<bool, FLEXCOMM6_IRQ_A>);
impl FLEXCOMM6_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM6_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM6_IRQ_A {
        match self.bits {
            false => FLEXCOMM6_IRQ_A::INVISIBLE,
            true => FLEXCOMM6_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == FLEXCOMM6_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == FLEXCOMM6_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for FLEXCOMM6_IRQ_R {
    type Target = crate::FieldReader<bool, FLEXCOMM6_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM6_IRQ` writer - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM6_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM6_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM6_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM6_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM6_IRQ_A::VISIBLE)
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
#[doc = "Flexcomm 7 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXCOMM7_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<FLEXCOMM7_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCOMM7_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM7_IRQ` reader - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM7_IRQ_R(crate::FieldReader<bool, FLEXCOMM7_IRQ_A>);
impl FLEXCOMM7_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM7_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM7_IRQ_A {
        match self.bits {
            false => FLEXCOMM7_IRQ_A::INVISIBLE,
            true => FLEXCOMM7_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == FLEXCOMM7_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == FLEXCOMM7_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for FLEXCOMM7_IRQ_R {
    type Target = crate::FieldReader<bool, FLEXCOMM7_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM7_IRQ` writer - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
pub struct FLEXCOMM7_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM7_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXCOMM7_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(FLEXCOMM7_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(FLEXCOMM7_IRQ_A::VISIBLE)
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
#[doc = "General Purpose ADC interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<ADC_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_IRQ` reader - General Purpose ADC interrupt."]
pub struct ADC_IRQ_R(crate::FieldReader<bool, ADC_IRQ_A>);
impl ADC_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_IRQ_A {
        match self.bits {
            false => ADC_IRQ_A::INVISIBLE,
            true => ADC_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == ADC_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == ADC_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for ADC_IRQ_R {
    type Target = crate::FieldReader<bool, ADC_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_IRQ` writer - General Purpose ADC interrupt."]
pub struct ADC_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(ADC_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(ADC_IRQ_A::VISIBLE)
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
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED0_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<RESERVED0_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESERVED0` reader - Reserved. Read value is undefined, only zero should be written."]
pub struct RESERVED0_R(crate::FieldReader<bool, RESERVED0_A>);
impl RESERVED0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
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
        **self == RESERVED0_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == RESERVED0_A::VISIBLE
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<bool, RESERVED0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - Reserved. Read value is undefined, only zero should be written."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESERVED0_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Analog Comparator interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<ACMP_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP_IRQ` reader - Analog Comparator interrupt."]
pub struct ACMP_IRQ_R(crate::FieldReader<bool, ACMP_IRQ_A>);
impl ACMP_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMP_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_IRQ_A {
        match self.bits {
            false => ACMP_IRQ_A::INVISIBLE,
            true => ACMP_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == ACMP_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == ACMP_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for ACMP_IRQ_R {
    type Target = crate::FieldReader<bool, ACMP_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP_IRQ` writer - Analog Comparator interrupt."]
pub struct ACMP_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(ACMP_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(ACMP_IRQ_A::VISIBLE)
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
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED1_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<RESERVED1_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESERVED1` reader - Reserved. Read value is undefined, only zero should be written."]
pub struct RESERVED1_R(crate::FieldReader<bool, RESERVED1_A>);
impl RESERVED1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED1_R(crate::FieldReader::new(bits))
    }
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
        **self == RESERVED1_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == RESERVED1_A::VISIBLE
    }
}
impl core::ops::Deref for RESERVED1_R {
    type Target = crate::FieldReader<bool, RESERVED1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED1` writer - Reserved. Read value is undefined, only zero should be written."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESERVED1_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED2_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<RESERVED2_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESERVED2` reader - Reserved. Read value is undefined, only zero should be written."]
pub struct RESERVED2_R(crate::FieldReader<bool, RESERVED2_A>);
impl RESERVED2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED2_R(crate::FieldReader::new(bits))
    }
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
        **self == RESERVED2_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == RESERVED2_A::VISIBLE
    }
}
impl core::ops::Deref for RESERVED2_R {
    type Target = crate::FieldReader<bool, RESERVED2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED2` writer - Reserved. Read value is undefined, only zero should be written."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESERVED2_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "USB Full Speed Controller Clock request interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_NEEDCLK_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<USB0_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_NEEDCLK` reader - USB Full Speed Controller Clock request interrupt."]
pub struct USB0_NEEDCLK_R(crate::FieldReader<bool, USB0_NEEDCLK_A>);
impl USB0_NEEDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0_NEEDCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_NEEDCLK_A {
        match self.bits {
            false => USB0_NEEDCLK_A::INVISIBLE,
            true => USB0_NEEDCLK_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == USB0_NEEDCLK_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == USB0_NEEDCLK_A::VISIBLE
    }
}
impl core::ops::Deref for USB0_NEEDCLK_R {
    type Target = crate::FieldReader<bool, USB0_NEEDCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0_NEEDCLK` writer - USB Full Speed Controller Clock request interrupt."]
pub struct USB0_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_NEEDCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB0_NEEDCLK_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB0_NEEDCLK_A::VISIBLE)
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
#[doc = "USB Full Speed Controller interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<USB0_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_IRQ` reader - USB Full Speed Controller interrupt."]
pub struct USB0_IRQ_R(crate::FieldReader<bool, USB0_IRQ_A>);
impl USB0_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_IRQ_A {
        match self.bits {
            false => USB0_IRQ_A::INVISIBLE,
            true => USB0_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == USB0_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == USB0_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for USB0_IRQ_R {
    type Target = crate::FieldReader<bool, USB0_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0_IRQ` writer - USB Full Speed Controller interrupt."]
pub struct USB0_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(USB0_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(USB0_IRQ_A::VISIBLE)
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
#[doc = "RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<RTC_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_IRQ` reader - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
pub struct RTC_IRQ_R(crate::FieldReader<bool, RTC_IRQ_A>);
impl RTC_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_IRQ_A {
        match self.bits {
            false => RTC_IRQ_A::INVISIBLE,
            true => RTC_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == RTC_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == RTC_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for RTC_IRQ_R {
    type Target = crate::FieldReader<bool, RTC_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_IRQ` writer - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
pub struct RTC_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(RTC_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(RTC_IRQ_A::VISIBLE)
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
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED3_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<RESERVED3_A> for bool {
    #[inline(always)]
    fn from(variant: RESERVED3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESERVED3` reader - Reserved. Read value is undefined, only zero should be written."]
pub struct RESERVED3_R(crate::FieldReader<bool, RESERVED3_A>);
impl RESERVED3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
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
        **self == RESERVED3_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == RESERVED3_A::VISIBLE
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<bool, RESERVED3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - Reserved. Read value is undefined, only zero should be written."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESERVED3_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Mailbox interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAILBOX_IRQ_A {
    #[doc = "0: no description available"]
    INVISIBLE = 0,
    #[doc = "1: no description available"]
    VISIBLE = 1,
}
impl From<MAILBOX_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: MAILBOX_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAILBOX_IRQ` reader - Mailbox interrupt."]
pub struct MAILBOX_IRQ_R(crate::FieldReader<bool, MAILBOX_IRQ_A>);
impl MAILBOX_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAILBOX_IRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAILBOX_IRQ_A {
        match self.bits {
            false => MAILBOX_IRQ_A::INVISIBLE,
            true => MAILBOX_IRQ_A::VISIBLE,
        }
    }
    #[doc = "Checks if the value of the field is `INVISIBLE`"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        **self == MAILBOX_IRQ_A::INVISIBLE
    }
    #[doc = "Checks if the value of the field is `VISIBLE`"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        **self == MAILBOX_IRQ_A::VISIBLE
    }
}
impl core::ops::Deref for MAILBOX_IRQ_R {
    type Target = crate::FieldReader<bool, MAILBOX_IRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAILBOX_IRQ` writer - Mailbox interrupt."]
pub struct MAILBOX_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MAILBOX_IRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAILBOX_IRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut W {
        self.variant(MAILBOX_IRQ_A::INVISIBLE)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut W {
        self.variant(MAILBOX_IRQ_A::VISIBLE)
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
    #[doc = "Bit 0 - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
    #[inline(always)]
    pub fn sys_irq(&self) -> SYS_IRQ_R {
        SYS_IRQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - System DMA 0 (non-secure) interrupt."]
    #[inline(always)]
    pub fn sdma0_irq(&self) -> SDMA0_IRQ_R {
        SDMA0_IRQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO Group 0 interrupt."]
    #[inline(always)]
    pub fn gpio_globalint0_irq(&self) -> GPIO_GLOBALINT0_IRQ_R {
        GPIO_GLOBALINT0_IRQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO Group 1 interrupt."]
    #[inline(always)]
    pub fn gpio_globalint1_irq(&self) -> GPIO_GLOBALINT1_IRQ_R {
        GPIO_GLOBALINT1_IRQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq0(&self) -> GPIO_INT0_IRQ0_R {
        GPIO_INT0_IRQ0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq1(&self) -> GPIO_INT0_IRQ1_R {
        GPIO_INT0_IRQ1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq2(&self) -> GPIO_INT0_IRQ2_R {
        GPIO_INT0_IRQ2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq3(&self) -> GPIO_INT0_IRQ3_R {
        GPIO_INT0_IRQ3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Micro Tick Timer interrupt."]
    #[inline(always)]
    pub fn utick_irq(&self) -> UTICK_IRQ_R {
        UTICK_IRQ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Multi-Rate Timer interrupt."]
    #[inline(always)]
    pub fn mrt_irq(&self) -> MRT_IRQ_R {
        MRT_IRQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Standard counter/timer 0 interrupt."]
    #[inline(always)]
    pub fn ctimer0_irq(&self) -> CTIMER0_IRQ_R {
        CTIMER0_IRQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Standard counter/timer 1 interrupt."]
    #[inline(always)]
    pub fn ctimer1_irq(&self) -> CTIMER1_IRQ_R {
        CTIMER1_IRQ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SCTimer/PWM interrupt."]
    #[inline(always)]
    pub fn sct_irq(&self) -> SCT_IRQ_R {
        SCT_IRQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Standard counter/timer 3 interrupt."]
    #[inline(always)]
    pub fn ctimer3_irq(&self) -> CTIMER3_IRQ_R {
        CTIMER3_IRQ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm0_irq(&self) -> FLEXCOMM0_IRQ_R {
        FLEXCOMM0_IRQ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm1_irq(&self) -> FLEXCOMM1_IRQ_R {
        FLEXCOMM1_IRQ_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm2_irq(&self) -> FLEXCOMM2_IRQ_R {
        FLEXCOMM2_IRQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm3_irq(&self) -> FLEXCOMM3_IRQ_R {
        FLEXCOMM3_IRQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm4_irq(&self) -> FLEXCOMM4_IRQ_R {
        FLEXCOMM4_IRQ_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm5_irq(&self) -> FLEXCOMM5_IRQ_R {
        FLEXCOMM5_IRQ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm6_irq(&self) -> FLEXCOMM6_IRQ_R {
        FLEXCOMM6_IRQ_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm7_irq(&self) -> FLEXCOMM7_IRQ_R {
        FLEXCOMM7_IRQ_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - General Purpose ADC interrupt."]
    #[inline(always)]
    pub fn adc_irq(&self) -> ADC_IRQ_R {
        ADC_IRQ_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Analog Comparator interrupt."]
    #[inline(always)]
    pub fn acmp_irq(&self) -> ACMP_IRQ_R {
        ACMP_IRQ_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB Full Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub fn usb0_needclk(&self) -> USB0_NEEDCLK_R {
        USB0_NEEDCLK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - USB Full Speed Controller interrupt."]
    #[inline(always)]
    pub fn usb0_irq(&self) -> USB0_IRQ_R {
        USB0_IRQ_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
    #[inline(always)]
    pub fn rtc_irq(&self) -> RTC_IRQ_R {
        RTC_IRQ_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Mailbox interrupt."]
    #[inline(always)]
    pub fn mailbox_irq(&self) -> MAILBOX_IRQ_R {
        MAILBOX_IRQ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
    #[inline(always)]
    pub fn sys_irq(&mut self) -> SYS_IRQ_W {
        SYS_IRQ_W { w: self }
    }
    #[doc = "Bit 1 - System DMA 0 (non-secure) interrupt."]
    #[inline(always)]
    pub fn sdma0_irq(&mut self) -> SDMA0_IRQ_W {
        SDMA0_IRQ_W { w: self }
    }
    #[doc = "Bit 2 - GPIO Group 0 interrupt."]
    #[inline(always)]
    pub fn gpio_globalint0_irq(&mut self) -> GPIO_GLOBALINT0_IRQ_W {
        GPIO_GLOBALINT0_IRQ_W { w: self }
    }
    #[doc = "Bit 3 - GPIO Group 1 interrupt."]
    #[inline(always)]
    pub fn gpio_globalint1_irq(&mut self) -> GPIO_GLOBALINT1_IRQ_W {
        GPIO_GLOBALINT1_IRQ_W { w: self }
    }
    #[doc = "Bit 4 - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq0(&mut self) -> GPIO_INT0_IRQ0_W {
        GPIO_INT0_IRQ0_W { w: self }
    }
    #[doc = "Bit 5 - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq1(&mut self) -> GPIO_INT0_IRQ1_W {
        GPIO_INT0_IRQ1_W { w: self }
    }
    #[doc = "Bit 6 - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq2(&mut self) -> GPIO_INT0_IRQ2_W {
        GPIO_INT0_IRQ2_W { w: self }
    }
    #[doc = "Bit 7 - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq3(&mut self) -> GPIO_INT0_IRQ3_W {
        GPIO_INT0_IRQ3_W { w: self }
    }
    #[doc = "Bit 8 - Micro Tick Timer interrupt."]
    #[inline(always)]
    pub fn utick_irq(&mut self) -> UTICK_IRQ_W {
        UTICK_IRQ_W { w: self }
    }
    #[doc = "Bit 9 - Multi-Rate Timer interrupt."]
    #[inline(always)]
    pub fn mrt_irq(&mut self) -> MRT_IRQ_W {
        MRT_IRQ_W { w: self }
    }
    #[doc = "Bit 10 - Standard counter/timer 0 interrupt."]
    #[inline(always)]
    pub fn ctimer0_irq(&mut self) -> CTIMER0_IRQ_W {
        CTIMER0_IRQ_W { w: self }
    }
    #[doc = "Bit 11 - Standard counter/timer 1 interrupt."]
    #[inline(always)]
    pub fn ctimer1_irq(&mut self) -> CTIMER1_IRQ_W {
        CTIMER1_IRQ_W { w: self }
    }
    #[doc = "Bit 12 - SCTimer/PWM interrupt."]
    #[inline(always)]
    pub fn sct_irq(&mut self) -> SCT_IRQ_W {
        SCT_IRQ_W { w: self }
    }
    #[doc = "Bit 13 - Standard counter/timer 3 interrupt."]
    #[inline(always)]
    pub fn ctimer3_irq(&mut self) -> CTIMER3_IRQ_W {
        CTIMER3_IRQ_W { w: self }
    }
    #[doc = "Bit 14 - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm0_irq(&mut self) -> FLEXCOMM0_IRQ_W {
        FLEXCOMM0_IRQ_W { w: self }
    }
    #[doc = "Bit 15 - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm1_irq(&mut self) -> FLEXCOMM1_IRQ_W {
        FLEXCOMM1_IRQ_W { w: self }
    }
    #[doc = "Bit 16 - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm2_irq(&mut self) -> FLEXCOMM2_IRQ_W {
        FLEXCOMM2_IRQ_W { w: self }
    }
    #[doc = "Bit 17 - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm3_irq(&mut self) -> FLEXCOMM3_IRQ_W {
        FLEXCOMM3_IRQ_W { w: self }
    }
    #[doc = "Bit 18 - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm4_irq(&mut self) -> FLEXCOMM4_IRQ_W {
        FLEXCOMM4_IRQ_W { w: self }
    }
    #[doc = "Bit 19 - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm5_irq(&mut self) -> FLEXCOMM5_IRQ_W {
        FLEXCOMM5_IRQ_W { w: self }
    }
    #[doc = "Bit 20 - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm6_irq(&mut self) -> FLEXCOMM6_IRQ_W {
        FLEXCOMM6_IRQ_W { w: self }
    }
    #[doc = "Bit 21 - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm7_irq(&mut self) -> FLEXCOMM7_IRQ_W {
        FLEXCOMM7_IRQ_W { w: self }
    }
    #[doc = "Bit 22 - General Purpose ADC interrupt."]
    #[inline(always)]
    pub fn adc_irq(&mut self) -> ADC_IRQ_W {
        ADC_IRQ_W { w: self }
    }
    #[doc = "Bit 23 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Bit 24 - Analog Comparator interrupt."]
    #[inline(always)]
    pub fn acmp_irq(&mut self) -> ACMP_IRQ_W {
        ACMP_IRQ_W { w: self }
    }
    #[doc = "Bit 25 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 26 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 27 - USB Full Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub fn usb0_needclk(&mut self) -> USB0_NEEDCLK_W {
        USB0_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 28 - USB Full Speed Controller interrupt."]
    #[inline(always)]
    pub fn usb0_irq(&mut self) -> USB0_IRQ_W {
        USB0_IRQ_W { w: self }
    }
    #[doc = "Bit 29 - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
    #[inline(always)]
    pub fn rtc_irq(&mut self) -> RTC_IRQ_W {
        RTC_IRQ_W { w: self }
    }
    #[doc = "Bit 30 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 31 - Mailbox interrupt."]
    #[inline(always)]
    pub fn mailbox_irq(&mut self) -> MAILBOX_IRQ_W {
        MAILBOX_IRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure Interrupt mask for CPU1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_cpu_int_mask0](index.html) module"]
pub struct SEC_CPU_INT_MASK0_SPEC;
impl crate::RegisterSpec for SEC_CPU_INT_MASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_cpu_int_mask0::R](R) reader structure"]
impl crate::Readable for SEC_CPU_INT_MASK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_cpu_int_mask0::W](W) writer structure"]
impl crate::Writable for SEC_CPU_INT_MASK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_CPU_INT_MASK0 to value 0xffff_ffff"]
impl crate::Resettable for SEC_CPU_INT_MASK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
