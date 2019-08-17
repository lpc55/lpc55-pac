#[doc = "Reader of register COMP_INT_CTRL"]
pub type R = crate::R<u32, super::COMP_INT_CTRL>;
#[doc = "Writer for register COMP_INT_CTRL"]
pub type W = crate::W<u32, super::COMP_INT_CTRL>;
#[doc = "Register COMP_INT_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP_INT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `INT_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_ENABLE_A {
    #[doc = "interrupt disable."]
    INT_DISABLE,
    #[doc = "interrupt enable."]
    INT_ENABLE,
}
impl From<INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: INT_ENABLE_A) -> Self {
        match variant {
            INT_ENABLE_A::INT_DISABLE => false,
            INT_ENABLE_A::INT_ENABLE => true,
        }
    }
}
#[doc = "Reader of field `INT_ENABLE`"]
pub type INT_ENABLE_R = crate::R<bool, INT_ENABLE_A>;
impl INT_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_ENABLE_A {
        match self.bits {
            false => INT_ENABLE_A::INT_DISABLE,
            true => INT_ENABLE_A::INT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `INT_DISABLE`"]
    #[inline(always)]
    pub fn is_int_disable(&self) -> bool {
        *self == INT_ENABLE_A::INT_DISABLE
    }
    #[doc = "Checks if the value of the field is `INT_ENABLE`"]
    #[inline(always)]
    pub fn is_int_enable(&self) -> bool {
        *self == INT_ENABLE_A::INT_ENABLE
    }
}
#[doc = "Write proxy for field `INT_ENABLE`"]
pub struct INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "interrupt disable."]
    #[inline(always)]
    pub fn int_disable(self) -> &'a mut W {
        self.variant(INT_ENABLE_A::INT_DISABLE)
    }
    #[doc = "interrupt enable."]
    #[inline(always)]
    pub fn int_enable(self) -> &'a mut W {
        self.variant(INT_ENABLE_A::INT_ENABLE)
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
#[doc = "Possible values of the field `INT_CLEAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_CLEAR_A {
    #[doc = "No effect."]
    NONE,
    #[doc = "Clear the interrupt. Self-cleared bit."]
    CLEAR,
}
impl From<INT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: INT_CLEAR_A) -> Self {
        match variant {
            INT_CLEAR_A::NONE => false,
            INT_CLEAR_A::CLEAR => true,
        }
    }
}
#[doc = "Reader of field `INT_CLEAR`"]
pub type INT_CLEAR_R = crate::R<bool, INT_CLEAR_A>;
impl INT_CLEAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_CLEAR_A {
        match self.bits {
            false => INT_CLEAR_A::NONE,
            true => INT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == INT_CLEAR_A::NONE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == INT_CLEAR_A::CLEAR
    }
}
#[doc = "Write proxy for field `INT_CLEAR`"]
pub struct INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_CLEAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(INT_CLEAR_A::NONE)
    }
    #[doc = "Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(INT_CLEAR_A::CLEAR)
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
#[doc = "Possible values of the field `INT_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_CTRL_A {
    #[doc = "The analog comparator interrupt edge sensitive is disabled."]
    EDGE_DISABLE,
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    LVL_DISABLE,
    #[doc = "analog comparator interrupt is rising edge sensitive."]
    EDGE_RISING,
    #[doc = "Analog Comparator interrupt is high level sensitive."]
    LVL_HIGH,
    #[doc = "analog comparator interrupt is falling edge sensitive."]
    EDGE_FALLING,
    #[doc = "Analog Comparator interrupt is low level sensitive."]
    LVL_LOW,
    #[doc = "analog comparator interrupt is rising and falling edge sensitive."]
    EDGE_BOTH,
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    LVL_DIS2,
}
impl From<INT_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: INT_CTRL_A) -> Self {
        match variant {
            INT_CTRL_A::EDGE_DISABLE => 0,
            INT_CTRL_A::LVL_DISABLE => 1,
            INT_CTRL_A::EDGE_RISING => 2,
            INT_CTRL_A::LVL_HIGH => 3,
            INT_CTRL_A::EDGE_FALLING => 4,
            INT_CTRL_A::LVL_LOW => 5,
            INT_CTRL_A::EDGE_BOTH => 6,
            INT_CTRL_A::LVL_DIS2 => 7,
        }
    }
}
#[doc = "Reader of field `INT_CTRL`"]
pub type INT_CTRL_R = crate::R<u8, INT_CTRL_A>;
impl INT_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_CTRL_A {
        match self.bits {
            0 => INT_CTRL_A::EDGE_DISABLE,
            1 => INT_CTRL_A::LVL_DISABLE,
            2 => INT_CTRL_A::EDGE_RISING,
            3 => INT_CTRL_A::LVL_HIGH,
            4 => INT_CTRL_A::EDGE_FALLING,
            5 => INT_CTRL_A::LVL_LOW,
            6 => INT_CTRL_A::EDGE_BOTH,
            7 => INT_CTRL_A::LVL_DIS2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGE_DISABLE`"]
    #[inline(always)]
    pub fn is_edge_disable(&self) -> bool {
        *self == INT_CTRL_A::EDGE_DISABLE
    }
    #[doc = "Checks if the value of the field is `LVL_DISABLE`"]
    #[inline(always)]
    pub fn is_lvl_disable(&self) -> bool {
        *self == INT_CTRL_A::LVL_DISABLE
    }
    #[doc = "Checks if the value of the field is `EDGE_RISING`"]
    #[inline(always)]
    pub fn is_edge_rising(&self) -> bool {
        *self == INT_CTRL_A::EDGE_RISING
    }
    #[doc = "Checks if the value of the field is `LVL_HIGH`"]
    #[inline(always)]
    pub fn is_lvl_high(&self) -> bool {
        *self == INT_CTRL_A::LVL_HIGH
    }
    #[doc = "Checks if the value of the field is `EDGE_FALLING`"]
    #[inline(always)]
    pub fn is_edge_falling(&self) -> bool {
        *self == INT_CTRL_A::EDGE_FALLING
    }
    #[doc = "Checks if the value of the field is `LVL_LOW`"]
    #[inline(always)]
    pub fn is_lvl_low(&self) -> bool {
        *self == INT_CTRL_A::LVL_LOW
    }
    #[doc = "Checks if the value of the field is `EDGE_BOTH`"]
    #[inline(always)]
    pub fn is_edge_both(&self) -> bool {
        *self == INT_CTRL_A::EDGE_BOTH
    }
    #[doc = "Checks if the value of the field is `LVL_DIS2`"]
    #[inline(always)]
    pub fn is_lvl_dis2(&self) -> bool {
        *self == INT_CTRL_A::LVL_DIS2
    }
}
#[doc = "Write proxy for field `INT_CTRL`"]
pub struct INT_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_CTRL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The analog comparator interrupt edge sensitive is disabled."]
    #[inline(always)]
    pub fn edge_disable(self) -> &'a mut W {
        self.variant(INT_CTRL_A::EDGE_DISABLE)
    }
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    #[inline(always)]
    pub fn lvl_disable(self) -> &'a mut W {
        self.variant(INT_CTRL_A::LVL_DISABLE)
    }
    #[doc = "analog comparator interrupt is rising edge sensitive."]
    #[inline(always)]
    pub fn edge_rising(self) -> &'a mut W {
        self.variant(INT_CTRL_A::EDGE_RISING)
    }
    #[doc = "Analog Comparator interrupt is high level sensitive."]
    #[inline(always)]
    pub fn lvl_high(self) -> &'a mut W {
        self.variant(INT_CTRL_A::LVL_HIGH)
    }
    #[doc = "analog comparator interrupt is falling edge sensitive."]
    #[inline(always)]
    pub fn edge_falling(self) -> &'a mut W {
        self.variant(INT_CTRL_A::EDGE_FALLING)
    }
    #[doc = "Analog Comparator interrupt is low level sensitive."]
    #[inline(always)]
    pub fn lvl_low(self) -> &'a mut W {
        self.variant(INT_CTRL_A::LVL_LOW)
    }
    #[doc = "analog comparator interrupt is rising and falling edge sensitive."]
    #[inline(always)]
    pub fn edge_both(self) -> &'a mut W {
        self.variant(INT_CTRL_A::EDGE_BOTH)
    }
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    #[inline(always)]
    pub fn lvl_dis2(self) -> &'a mut W {
        self.variant(INT_CTRL_A::LVL_DIS2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `INT_SOURCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_SOURCE_A {
    #[doc = "Select Analog Comparator filtered output as input for interrupt detection."]
    FILTER_INT,
    #[doc = "Select Analog Comparator raw output (unfiltered) as input for interrupt detection. Must be used when Analog comparator is used as wake up source in Power down mode."]
    RAW_INT,
}
impl From<INT_SOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: INT_SOURCE_A) -> Self {
        match variant {
            INT_SOURCE_A::FILTER_INT => false,
            INT_SOURCE_A::RAW_INT => true,
        }
    }
}
#[doc = "Reader of field `INT_SOURCE`"]
pub type INT_SOURCE_R = crate::R<bool, INT_SOURCE_A>;
impl INT_SOURCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_SOURCE_A {
        match self.bits {
            false => INT_SOURCE_A::FILTER_INT,
            true => INT_SOURCE_A::RAW_INT,
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_INT`"]
    #[inline(always)]
    pub fn is_filter_int(&self) -> bool {
        *self == INT_SOURCE_A::FILTER_INT
    }
    #[doc = "Checks if the value of the field is `RAW_INT`"]
    #[inline(always)]
    pub fn is_raw_int(&self) -> bool {
        *self == INT_SOURCE_A::RAW_INT
    }
}
#[doc = "Write proxy for field `INT_SOURCE`"]
pub struct INT_SOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_SOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_SOURCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Select Analog Comparator filtered output as input for interrupt detection."]
    #[inline(always)]
    pub fn filter_int(self) -> &'a mut W {
        self.variant(INT_SOURCE_A::FILTER_INT)
    }
    #[doc = "Select Analog Comparator raw output (unfiltered) as input for interrupt detection. Must be used when Analog comparator is used as wake up source in Power down mode."]
    #[inline(always)]
    pub fn raw_int(self) -> &'a mut W {
        self.variant(INT_SOURCE_A::RAW_INT)
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
impl R {
    #[doc = "Bit 0 - Analog Comparator interrupt enable control:."]
    #[inline(always)]
    pub fn int_enable(&self) -> INT_ENABLE_R {
        INT_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator interrupt clear."]
    #[inline(always)]
    pub fn int_clear(&self) -> INT_CLEAR_R {
        INT_CLEAR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Comparator interrupt type selector:."]
    #[inline(always)]
    pub fn int_ctrl(&self) -> INT_CTRL_R {
        INT_CTRL_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
    #[inline(always)]
    pub fn int_source(&self) -> INT_SOURCE_R {
        INT_SOURCE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator interrupt enable control:."]
    #[inline(always)]
    pub fn int_enable(&mut self) -> INT_ENABLE_W {
        INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Analog Comparator interrupt clear."]
    #[inline(always)]
    pub fn int_clear(&mut self) -> INT_CLEAR_W {
        INT_CLEAR_W { w: self }
    }
    #[doc = "Bits 2:4 - Comparator interrupt type selector:."]
    #[inline(always)]
    pub fn int_ctrl(&mut self) -> INT_CTRL_W {
        INT_CTRL_W { w: self }
    }
    #[doc = "Bit 5 - Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
    #[inline(always)]
    pub fn int_source(&mut self) -> INT_SOURCE_W {
        INT_SOURCE_W { w: self }
    }
}
