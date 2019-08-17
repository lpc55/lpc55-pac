#[doc = "Reader of register CPUCTRL"]
pub type R = crate::R<u32, super::CPUCTRL>;
#[doc = "Writer for register CPUCTRL"]
pub type W = crate::W<u32, super::CPUCTRL>;
#[doc = "Register CPUCTRL `reset()`'s with value 0x2c"]
impl crate::ResetValue for super::CPUCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2c
    }
}
#[doc = "Possible values of the field `CPU1CLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1CLKEN_A {
    #[doc = "The CPU1 clock is not enabled."]
    DISABLE,
    #[doc = "The CPU1 clock is enabled."]
    ENABLE,
}
impl From<CPU1CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1CLKEN_A) -> Self {
        match variant {
            CPU1CLKEN_A::DISABLE => false,
            CPU1CLKEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CPU1CLKEN`"]
pub type CPU1CLKEN_R = crate::R<bool, CPU1CLKEN_A>;
impl CPU1CLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1CLKEN_A {
        match self.bits {
            false => CPU1CLKEN_A::DISABLE,
            true => CPU1CLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU1CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU1CLKEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `CPU1CLKEN`"]
pub struct CPU1CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1CLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1CLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CPU1 clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU1CLKEN_A::DISABLE)
    }
    #[doc = "The CPU1 clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU1CLKEN_A::ENABLE)
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
#[doc = "Possible values of the field `CPU1RSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1RSTEN_A {
    #[doc = "The CPU1 is not being reset."]
    RELEASED,
    #[doc = "The CPU1 is being reset."]
    ASSERTED,
}
impl From<CPU1RSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1RSTEN_A) -> Self {
        match variant {
            CPU1RSTEN_A::RELEASED => false,
            CPU1RSTEN_A::ASSERTED => true,
        }
    }
}
#[doc = "Reader of field `CPU1RSTEN`"]
pub type CPU1RSTEN_R = crate::R<bool, CPU1RSTEN_A>;
impl CPU1RSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1RSTEN_A {
        match self.bits {
            false => CPU1RSTEN_A::RELEASED,
            true => CPU1RSTEN_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == CPU1RSTEN_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == CPU1RSTEN_A::ASSERTED
    }
}
#[doc = "Write proxy for field `CPU1RSTEN`"]
pub struct CPU1RSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1RSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1RSTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CPU1 is not being reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(CPU1RSTEN_A::RELEASED)
    }
    #[doc = "The CPU1 is being reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(CPU1RSTEN_A::ASSERTED)
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
    #[doc = "Bit 3 - CPU1 clock enable."]
    #[inline(always)]
    pub fn cpu1clken(&self) -> CPU1CLKEN_R {
        CPU1CLKEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CPU1 reset."]
    #[inline(always)]
    pub fn cpu1rsten(&self) -> CPU1RSTEN_R {
        CPU1RSTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - CPU1 clock enable."]
    #[inline(always)]
    pub fn cpu1clken(&mut self) -> CPU1CLKEN_W {
        CPU1CLKEN_W { w: self }
    }
    #[doc = "Bit 5 - CPU1 reset."]
    #[inline(always)]
    pub fn cpu1rsten(&mut self) -> CPU1RSTEN_W {
        CPU1RSTEN_W { w: self }
    }
}
