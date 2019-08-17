#[doc = "Reader of register TST"]
pub type R = crate::R<u32, super::TST>;
#[doc = "Writer for register TST"]
pub type W = crate::W<u32, super::TST>;
#[doc = "Register TST `reset()`'s with value 0"]
impl crate::ResetValue for super::TST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CST_LONG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CST_LONG_A {
    #[doc = "Normal sample time. Minimum sample time of 3 ADCK cycles."]
    CST_LONG_0,
    #[doc = "Increased sample time. 67 ADCK cycles total sample time."]
    CST_LONG_1,
}
impl From<CST_LONG_A> for bool {
    #[inline(always)]
    fn from(variant: CST_LONG_A) -> Self {
        match variant {
            CST_LONG_A::CST_LONG_0 => false,
            CST_LONG_A::CST_LONG_1 => true,
        }
    }
}
#[doc = "Reader of field `CST_LONG`"]
pub type CST_LONG_R = crate::R<bool, CST_LONG_A>;
impl CST_LONG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CST_LONG_A {
        match self.bits {
            false => CST_LONG_A::CST_LONG_0,
            true => CST_LONG_A::CST_LONG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CST_LONG_0`"]
    #[inline(always)]
    pub fn is_cst_long_0(&self) -> bool {
        *self == CST_LONG_A::CST_LONG_0
    }
    #[doc = "Checks if the value of the field is `CST_LONG_1`"]
    #[inline(always)]
    pub fn is_cst_long_1(&self) -> bool {
        *self == CST_LONG_A::CST_LONG_1
    }
}
#[doc = "Write proxy for field `CST_LONG`"]
pub struct CST_LONG_W<'a> {
    w: &'a mut W,
}
impl<'a> CST_LONG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CST_LONG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal sample time. Minimum sample time of 3 ADCK cycles."]
    #[inline(always)]
    pub fn cst_long_0(self) -> &'a mut W {
        self.variant(CST_LONG_A::CST_LONG_0)
    }
    #[doc = "Increased sample time. 67 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn cst_long_1(self) -> &'a mut W {
        self.variant(CST_LONG_A::CST_LONG_1)
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
#[doc = "Possible values of the field `FOFFM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFFM_A {
    #[doc = "Normal operation. No forced offset."]
    FOFFM_0,
    #[doc = "Test configuration. Forced positive offset on MDAC."]
    FOFFM_1,
}
impl From<FOFFM_A> for bool {
    #[inline(always)]
    fn from(variant: FOFFM_A) -> Self {
        match variant {
            FOFFM_A::FOFFM_0 => false,
            FOFFM_A::FOFFM_1 => true,
        }
    }
}
#[doc = "Reader of field `FOFFM`"]
pub type FOFFM_R = crate::R<bool, FOFFM_A>;
impl FOFFM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFFM_A {
        match self.bits {
            false => FOFFM_A::FOFFM_0,
            true => FOFFM_A::FOFFM_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFFM_0`"]
    #[inline(always)]
    pub fn is_foffm_0(&self) -> bool {
        *self == FOFFM_A::FOFFM_0
    }
    #[doc = "Checks if the value of the field is `FOFFM_1`"]
    #[inline(always)]
    pub fn is_foffm_1(&self) -> bool {
        *self == FOFFM_A::FOFFM_1
    }
}
#[doc = "Write proxy for field `FOFFM`"]
pub struct FOFFM_W<'a> {
    w: &'a mut W,
}
impl<'a> FOFFM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOFFM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn foffm_0(self) -> &'a mut W {
        self.variant(FOFFM_A::FOFFM_0)
    }
    #[doc = "Test configuration. Forced positive offset on MDAC."]
    #[inline(always)]
    pub fn foffm_1(self) -> &'a mut W {
        self.variant(FOFFM_A::FOFFM_1)
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
#[doc = "Possible values of the field `FOFFP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFFP_A {
    #[doc = "Normal operation. No forced offset."]
    FOFFP_0,
    #[doc = "Test configuration. Forced positive offset on PDAC."]
    FOFFP_1,
}
impl From<FOFFP_A> for bool {
    #[inline(always)]
    fn from(variant: FOFFP_A) -> Self {
        match variant {
            FOFFP_A::FOFFP_0 => false,
            FOFFP_A::FOFFP_1 => true,
        }
    }
}
#[doc = "Reader of field `FOFFP`"]
pub type FOFFP_R = crate::R<bool, FOFFP_A>;
impl FOFFP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFFP_A {
        match self.bits {
            false => FOFFP_A::FOFFP_0,
            true => FOFFP_A::FOFFP_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFFP_0`"]
    #[inline(always)]
    pub fn is_foffp_0(&self) -> bool {
        *self == FOFFP_A::FOFFP_0
    }
    #[doc = "Checks if the value of the field is `FOFFP_1`"]
    #[inline(always)]
    pub fn is_foffp_1(&self) -> bool {
        *self == FOFFP_A::FOFFP_1
    }
}
#[doc = "Write proxy for field `FOFFP`"]
pub struct FOFFP_W<'a> {
    w: &'a mut W,
}
impl<'a> FOFFP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOFFP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn foffp_0(self) -> &'a mut W {
        self.variant(FOFFP_A::FOFFP_0)
    }
    #[doc = "Test configuration. Forced positive offset on PDAC."]
    #[inline(always)]
    pub fn foffp_1(self) -> &'a mut W {
        self.variant(FOFFP_A::FOFFP_1)
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
#[doc = "Possible values of the field `FOFFM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFFM2_A {
    #[doc = "Normal operation. No forced offset."]
    FOFFM2_0,
    #[doc = "Test configuration. Forced negative offset on MDAC."]
    FOFFM2_1,
}
impl From<FOFFM2_A> for bool {
    #[inline(always)]
    fn from(variant: FOFFM2_A) -> Self {
        match variant {
            FOFFM2_A::FOFFM2_0 => false,
            FOFFM2_A::FOFFM2_1 => true,
        }
    }
}
#[doc = "Reader of field `FOFFM2`"]
pub type FOFFM2_R = crate::R<bool, FOFFM2_A>;
impl FOFFM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFFM2_A {
        match self.bits {
            false => FOFFM2_A::FOFFM2_0,
            true => FOFFM2_A::FOFFM2_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFFM2_0`"]
    #[inline(always)]
    pub fn is_foffm2_0(&self) -> bool {
        *self == FOFFM2_A::FOFFM2_0
    }
    #[doc = "Checks if the value of the field is `FOFFM2_1`"]
    #[inline(always)]
    pub fn is_foffm2_1(&self) -> bool {
        *self == FOFFM2_A::FOFFM2_1
    }
}
#[doc = "Write proxy for field `FOFFM2`"]
pub struct FOFFM2_W<'a> {
    w: &'a mut W,
}
impl<'a> FOFFM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOFFM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn foffm2_0(self) -> &'a mut W {
        self.variant(FOFFM2_A::FOFFM2_0)
    }
    #[doc = "Test configuration. Forced negative offset on MDAC."]
    #[inline(always)]
    pub fn foffm2_1(self) -> &'a mut W {
        self.variant(FOFFM2_A::FOFFM2_1)
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
#[doc = "Possible values of the field `FOFFP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFFP2_A {
    #[doc = "Normal operation. No forced offset."]
    FOFFP2_0,
    #[doc = "Test configuration. Forced negative offset on PDAC."]
    FOFFP2_1,
}
impl From<FOFFP2_A> for bool {
    #[inline(always)]
    fn from(variant: FOFFP2_A) -> Self {
        match variant {
            FOFFP2_A::FOFFP2_0 => false,
            FOFFP2_A::FOFFP2_1 => true,
        }
    }
}
#[doc = "Reader of field `FOFFP2`"]
pub type FOFFP2_R = crate::R<bool, FOFFP2_A>;
impl FOFFP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFFP2_A {
        match self.bits {
            false => FOFFP2_A::FOFFP2_0,
            true => FOFFP2_A::FOFFP2_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFFP2_0`"]
    #[inline(always)]
    pub fn is_foffp2_0(&self) -> bool {
        *self == FOFFP2_A::FOFFP2_0
    }
    #[doc = "Checks if the value of the field is `FOFFP2_1`"]
    #[inline(always)]
    pub fn is_foffp2_1(&self) -> bool {
        *self == FOFFP2_A::FOFFP2_1
    }
}
#[doc = "Write proxy for field `FOFFP2`"]
pub struct FOFFP2_W<'a> {
    w: &'a mut W,
}
impl<'a> FOFFP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOFFP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn foffp2_0(self) -> &'a mut W {
        self.variant(FOFFP2_A::FOFFP2_0)
    }
    #[doc = "Test configuration. Forced negative offset on PDAC."]
    #[inline(always)]
    pub fn foffp2_1(self) -> &'a mut W {
        self.variant(FOFFP2_A::FOFFP2_1)
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
#[doc = "Possible values of the field `TESTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TESTEN_A {
    #[doc = "Normal operation. Test configuration not enabled."]
    TESTEN_0,
    #[doc = "Hardware BIST Test in progress."]
    TESTEN_1,
}
impl From<TESTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TESTEN_A) -> Self {
        match variant {
            TESTEN_A::TESTEN_0 => false,
            TESTEN_A::TESTEN_1 => true,
        }
    }
}
#[doc = "Reader of field `TESTEN`"]
pub type TESTEN_R = crate::R<bool, TESTEN_A>;
impl TESTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TESTEN_A {
        match self.bits {
            false => TESTEN_A::TESTEN_0,
            true => TESTEN_A::TESTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TESTEN_0`"]
    #[inline(always)]
    pub fn is_testen_0(&self) -> bool {
        *self == TESTEN_A::TESTEN_0
    }
    #[doc = "Checks if the value of the field is `TESTEN_1`"]
    #[inline(always)]
    pub fn is_testen_1(&self) -> bool {
        *self == TESTEN_A::TESTEN_1
    }
}
#[doc = "Write proxy for field `TESTEN`"]
pub struct TESTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TESTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation. Test configuration not enabled."]
    #[inline(always)]
    pub fn testen_0(self) -> &'a mut W {
        self.variant(TESTEN_A::TESTEN_0)
    }
    #[doc = "Hardware BIST Test in progress."]
    #[inline(always)]
    pub fn testen_1(self) -> &'a mut W {
        self.variant(TESTEN_A::TESTEN_1)
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
impl R {
    #[doc = "Bit 0 - Calibration Sample Time Long"]
    #[inline(always)]
    pub fn cst_long(&self) -> CST_LONG_R {
        CST_LONG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Force M-side positive offset"]
    #[inline(always)]
    pub fn foffm(&self) -> FOFFM_R {
        FOFFM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Force P-side positive offset"]
    #[inline(always)]
    pub fn foffp(&self) -> FOFFP_R {
        FOFFP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Force M-side negative offset"]
    #[inline(always)]
    pub fn foffm2(&self) -> FOFFM2_R {
        FOFFM2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Force P-side negative offset"]
    #[inline(always)]
    pub fn foffp2(&self) -> FOFFP2_R {
        FOFFP2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable test configuration"]
    #[inline(always)]
    pub fn testen(&self) -> TESTEN_R {
        TESTEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration Sample Time Long"]
    #[inline(always)]
    pub fn cst_long(&mut self) -> CST_LONG_W {
        CST_LONG_W { w: self }
    }
    #[doc = "Bit 8 - Force M-side positive offset"]
    #[inline(always)]
    pub fn foffm(&mut self) -> FOFFM_W {
        FOFFM_W { w: self }
    }
    #[doc = "Bit 9 - Force P-side positive offset"]
    #[inline(always)]
    pub fn foffp(&mut self) -> FOFFP_W {
        FOFFP_W { w: self }
    }
    #[doc = "Bit 10 - Force M-side negative offset"]
    #[inline(always)]
    pub fn foffm2(&mut self) -> FOFFM2_W {
        FOFFM2_W { w: self }
    }
    #[doc = "Bit 11 - Force P-side negative offset"]
    #[inline(always)]
    pub fn foffp2(&mut self) -> FOFFP2_W {
        FOFFP2_W { w: self }
    }
    #[doc = "Bit 23 - Enable test configuration"]
    #[inline(always)]
    pub fn testen(&mut self) -> TESTEN_W {
        TESTEN_W { w: self }
    }
}
