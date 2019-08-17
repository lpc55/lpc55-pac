#[doc = "Reader of register ANALOG_CTRL_CFG"]
pub type R = crate::R<u32, super::ANALOG_CTRL_CFG>;
#[doc = "Writer for register ANALOG_CTRL_CFG"]
pub type W = crate::W<u32, super::ANALOG_CTRL_CFG>;
#[doc = "Register ANALOG_CTRL_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::ANALOG_CTRL_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `FRO192M_TRIM_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO192M_TRIM_SRC_A {
    #[doc = "FRO192M trimming and 'Enable' comes from eFUSE."]
    EFUSE,
    #[doc = "FRO192M trimming and 'Enable' comes from FRO192M_CTRL registers."]
    FRO192MCTRL,
}
impl From<FRO192M_TRIM_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FRO192M_TRIM_SRC_A) -> Self {
        match variant {
            FRO192M_TRIM_SRC_A::EFUSE => false,
            FRO192M_TRIM_SRC_A::FRO192MCTRL => true,
        }
    }
}
#[doc = "Reader of field `FRO192M_TRIM_SRC`"]
pub type FRO192M_TRIM_SRC_R = crate::R<bool, FRO192M_TRIM_SRC_A>;
impl FRO192M_TRIM_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO192M_TRIM_SRC_A {
        match self.bits {
            false => FRO192M_TRIM_SRC_A::EFUSE,
            true => FRO192M_TRIM_SRC_A::FRO192MCTRL,
        }
    }
    #[doc = "Checks if the value of the field is `EFUSE`"]
    #[inline(always)]
    pub fn is_efuse(&self) -> bool {
        *self == FRO192M_TRIM_SRC_A::EFUSE
    }
    #[doc = "Checks if the value of the field is `FRO192MCTRL`"]
    #[inline(always)]
    pub fn is_fro192mctrl(&self) -> bool {
        *self == FRO192M_TRIM_SRC_A::FRO192MCTRL
    }
}
#[doc = "Write proxy for field `FRO192M_TRIM_SRC`"]
pub struct FRO192M_TRIM_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO192M_TRIM_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRO192M_TRIM_SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FRO192M trimming and 'Enable' comes from eFUSE."]
    #[inline(always)]
    pub fn efuse(self) -> &'a mut W {
        self.variant(FRO192M_TRIM_SRC_A::EFUSE)
    }
    #[doc = "FRO192M trimming and 'Enable' comes from FRO192M_CTRL registers."]
    #[inline(always)]
    pub fn fro192mctrl(self) -> &'a mut W {
        self.variant(FRO192M_TRIM_SRC_A::FRO192MCTRL)
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
impl R {
    #[doc = "Bit 0 - FRO192M trimming and 'Enable' source."]
    #[inline(always)]
    pub fn fro192m_trim_src(&self) -> FRO192M_TRIM_SRC_R {
        FRO192M_TRIM_SRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FRO192M trimming and 'Enable' source."]
    #[inline(always)]
    pub fn fro192m_trim_src(&mut self) -> FRO192M_TRIM_SRC_W {
        FRO192M_TRIM_SRC_W { w: self }
    }
}
