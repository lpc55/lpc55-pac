#[doc = "Reader of register AHBCLKCTRL1"]
pub type R = crate::R<u32, super::AHBCLKCTRL1>;
#[doc = "Writer for register AHBCLKCTRL1"]
pub type W = crate::W<u32, super::AHBCLKCTRL1>;
#[doc = "Register AHBCLKCTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AHBCLKCTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `MRT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<MRT_A> for bool {
    #[inline(always)]
    fn from(variant: MRT_A) -> Self {
        match variant {
            MRT_A::DISABLE => false,
            MRT_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `MRT`"]
pub type MRT_R = crate::R<bool, MRT_A>;
impl MRT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_A {
        match self.bits {
            false => MRT_A::DISABLE,
            true => MRT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MRT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MRT_A::ENABLE
    }
}
#[doc = "Write proxy for field `MRT`"]
pub struct MRT_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MRT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MRT_A::ENABLE)
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
#[doc = "Possible values of the field `OSTIMER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSTIMER0_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<OSTIMER0_A> for bool {
    #[inline(always)]
    fn from(variant: OSTIMER0_A) -> Self {
        match variant {
            OSTIMER0_A::DISABLE => false,
            OSTIMER0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `OSTIMER0`"]
pub type OSTIMER0_R = crate::R<bool, OSTIMER0_A>;
impl OSTIMER0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTIMER0_A {
        match self.bits {
            false => OSTIMER0_A::DISABLE,
            true => OSTIMER0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OSTIMER0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OSTIMER0_A::ENABLE
    }
}
#[doc = "Write proxy for field `OSTIMER0`"]
pub struct OSTIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSTIMER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSTIMER0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OSTIMER0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OSTIMER0_A::ENABLE)
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
#[doc = "Possible values of the field `SCT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT0_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<SCT0_A> for bool {
    #[inline(always)]
    fn from(variant: SCT0_A) -> Self {
        match variant {
            SCT0_A::DISABLE => false,
            SCT0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SCT0`"]
pub type SCT0_R = crate::R<bool, SCT0_A>;
impl SCT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT0_A {
        match self.bits {
            false => SCT0_A::DISABLE,
            true => SCT0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCT0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCT0_A::ENABLE
    }
}
#[doc = "Write proxy for field `SCT0`"]
pub struct SCT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCT0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCT0_A::ENABLE)
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
#[doc = "Possible values of the field `SCTIPU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCTIPU_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<SCTIPU_A> for bool {
    #[inline(always)]
    fn from(variant: SCTIPU_A) -> Self {
        match variant {
            SCTIPU_A::DISABLE => false,
            SCTIPU_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `SCTIPU`"]
pub type SCTIPU_R = crate::R<bool, SCTIPU_A>;
impl SCTIPU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCTIPU_A {
        match self.bits {
            false => SCTIPU_A::DISABLE,
            true => SCTIPU_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCTIPU_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCTIPU_A::ENABLE
    }
}
#[doc = "Write proxy for field `SCTIPU`"]
pub struct SCTIPU_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTIPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCTIPU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCTIPU_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCTIPU_A::ENABLE)
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
#[doc = "Possible values of the field `UTICK0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UTICK0_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<UTICK0_A> for bool {
    #[inline(always)]
    fn from(variant: UTICK0_A) -> Self {
        match variant {
            UTICK0_A::DISABLE => false,
            UTICK0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `UTICK0`"]
pub type UTICK0_R = crate::R<bool, UTICK0_A>;
impl UTICK0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTICK0_A {
        match self.bits {
            false => UTICK0_A::DISABLE,
            true => UTICK0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UTICK0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UTICK0_A::ENABLE
    }
}
#[doc = "Write proxy for field `UTICK0`"]
pub struct UTICK0_W<'a> {
    w: &'a mut W,
}
impl<'a> UTICK0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UTICK0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UTICK0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UTICK0_A::ENABLE)
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
#[doc = "Possible values of the field `FC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC0_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<FC0_A> for bool {
    #[inline(always)]
    fn from(variant: FC0_A) -> Self {
        match variant {
            FC0_A::DISABLE => false,
            FC0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FC0`"]
pub type FC0_R = crate::R<bool, FC0_A>;
impl FC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC0_A {
        match self.bits {
            false => FC0_A::DISABLE,
            true => FC0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC0_A::ENABLE
    }
}
#[doc = "Write proxy for field `FC0`"]
pub struct FC0_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC0_A::ENABLE)
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
#[doc = "Possible values of the field `FC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC1_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<FC1_A> for bool {
    #[inline(always)]
    fn from(variant: FC1_A) -> Self {
        match variant {
            FC1_A::DISABLE => false,
            FC1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FC1`"]
pub type FC1_R = crate::R<bool, FC1_A>;
impl FC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC1_A {
        match self.bits {
            false => FC1_A::DISABLE,
            true => FC1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC1_A::ENABLE
    }
}
#[doc = "Write proxy for field `FC1`"]
pub struct FC1_W<'a> {
    w: &'a mut W,
}
impl<'a> FC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC1_A::ENABLE)
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
#[doc = "Possible values of the field `FC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC2_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<FC2_A> for bool {
    #[inline(always)]
    fn from(variant: FC2_A) -> Self {
        match variant {
            FC2_A::DISABLE => false,
            FC2_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FC2`"]
pub type FC2_R = crate::R<bool, FC2_A>;
impl FC2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC2_A {
        match self.bits {
            false => FC2_A::DISABLE,
            true => FC2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC2_A::ENABLE
    }
}
#[doc = "Write proxy for field `FC2`"]
pub struct FC2_W<'a> {
    w: &'a mut W,
}
impl<'a> FC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC2_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC2_A::ENABLE)
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
#[doc = "Possible values of the field `FC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC3_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<FC3_A> for bool {
    #[inline(always)]
    fn from(variant: FC3_A) -> Self {
        match variant {
            FC3_A::DISABLE => false,
            FC3_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FC3`"]
pub type FC3_R = crate::R<bool, FC3_A>;
impl FC3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC3_A {
        match self.bits {
            false => FC3_A::DISABLE,
            true => FC3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC3_A::ENABLE
    }
}
#[doc = "Write proxy for field `FC3`"]
pub struct FC3_W<'a> {
    w: &'a mut W,
}
impl<'a> FC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC3_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC3_A::ENABLE)
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
#[doc = "Possible values of the field `FC4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC4_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<FC4_A> for bool {
    #[inline(always)]
    fn from(variant: FC4_A) -> Self {
        match variant {
            FC4_A::DISABLE => false,
            FC4_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FC4`"]
pub type FC4_R = crate::R<bool, FC4_A>;
impl FC4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC4_A {
        match self.bits {
            false => FC4_A::DISABLE,
            true => FC4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC4_A::ENABLE
    }
}
#[doc = "Write proxy for field `FC4`"]
pub struct FC4_W<'a> {
    w: &'a mut W,
}
impl<'a> FC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC4_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC4_A::ENABLE)
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
#[doc = "Possible values of the field `FC5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC5_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<FC5_A> for bool {
    #[inline(always)]
    fn from(variant: FC5_A) -> Self {
        match variant {
            FC5_A::DISABLE => false,
            FC5_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FC5`"]
pub type FC5_R = crate::R<bool, FC5_A>;
impl FC5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC5_A {
        match self.bits {
            false => FC5_A::DISABLE,
            true => FC5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC5_A::ENABLE
    }
}
#[doc = "Write proxy for field `FC5`"]
pub struct FC5_W<'a> {
    w: &'a mut W,
}
impl<'a> FC5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC5_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC5_A::ENABLE)
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
#[doc = "Possible values of the field `FC6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC6_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<FC6_A> for bool {
    #[inline(always)]
    fn from(variant: FC6_A) -> Self {
        match variant {
            FC6_A::DISABLE => false,
            FC6_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FC6`"]
pub type FC6_R = crate::R<bool, FC6_A>;
impl FC6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC6_A {
        match self.bits {
            false => FC6_A::DISABLE,
            true => FC6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC6_A::ENABLE
    }
}
#[doc = "Write proxy for field `FC6`"]
pub struct FC6_W<'a> {
    w: &'a mut W,
}
impl<'a> FC6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC6_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC6_A::ENABLE)
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
#[doc = "Possible values of the field `FC7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC7_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<FC7_A> for bool {
    #[inline(always)]
    fn from(variant: FC7_A) -> Self {
        match variant {
            FC7_A::DISABLE => false,
            FC7_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `FC7`"]
pub type FC7_R = crate::R<bool, FC7_A>;
impl FC7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC7_A {
        match self.bits {
            false => FC7_A::DISABLE,
            true => FC7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FC7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FC7_A::ENABLE
    }
}
#[doc = "Write proxy for field `FC7`"]
pub struct FC7_W<'a> {
    w: &'a mut W,
}
impl<'a> FC7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FC7_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FC7_A::ENABLE)
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
#[doc = "Possible values of the field `TIMER2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER2_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<TIMER2_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER2_A) -> Self {
        match variant {
            TIMER2_A::DISABLE => false,
            TIMER2_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `TIMER2`"]
pub type TIMER2_R = crate::R<bool, TIMER2_A>;
impl TIMER2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER2_A {
        match self.bits {
            false => TIMER2_A::DISABLE,
            true => TIMER2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIMER2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMER2_A::ENABLE
    }
}
#[doc = "Write proxy for field `TIMER2`"]
pub struct TIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER2_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER2_A::ENABLE)
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
#[doc = "Possible values of the field `USB0_DEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_DEV_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<USB0_DEV_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_DEV_A) -> Self {
        match variant {
            USB0_DEV_A::DISABLE => false,
            USB0_DEV_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `USB0_DEV`"]
pub type USB0_DEV_R = crate::R<bool, USB0_DEV_A>;
impl USB0_DEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_DEV_A {
        match self.bits {
            false => USB0_DEV_A::DISABLE,
            true => USB0_DEV_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB0_DEV_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB0_DEV_A::ENABLE
    }
}
#[doc = "Write proxy for field `USB0_DEV`"]
pub struct USB0_DEV_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_DEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_DEV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_DEV_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_DEV_A::ENABLE)
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
#[doc = "Possible values of the field `TIMER0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER0_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<TIMER0_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER0_A) -> Self {
        match variant {
            TIMER0_A::DISABLE => false,
            TIMER0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `TIMER0`"]
pub type TIMER0_R = crate::R<bool, TIMER0_A>;
impl TIMER0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER0_A {
        match self.bits {
            false => TIMER0_A::DISABLE,
            true => TIMER0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIMER0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMER0_A::ENABLE
    }
}
#[doc = "Write proxy for field `TIMER0`"]
pub struct TIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER0_A::ENABLE)
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
#[doc = "Possible values of the field `TIMER1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER1_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<TIMER1_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER1_A) -> Self {
        match variant {
            TIMER1_A::DISABLE => false,
            TIMER1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `TIMER1`"]
pub type TIMER1_R = crate::R<bool, TIMER1_A>;
impl TIMER1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER1_A {
        match self.bits {
            false => TIMER1_A::DISABLE,
            true => TIMER1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIMER1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMER1_A::ENABLE
    }
}
#[doc = "Write proxy for field `TIMER1`"]
pub struct TIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER1_A::ENABLE)
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
#[doc = "Possible values of the field `PVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVT_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<PVT_A> for bool {
    #[inline(always)]
    fn from(variant: PVT_A) -> Self {
        match variant {
            PVT_A::DISABLE => false,
            PVT_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `PVT`"]
pub type PVT_R = crate::R<bool, PVT_A>;
impl PVT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVT_A {
        match self.bits {
            false => PVT_A::DISABLE,
            true => PVT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PVT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PVT_A::ENABLE
    }
}
#[doc = "Write proxy for field `PVT`"]
pub struct PVT_W<'a> {
    w: &'a mut W,
}
impl<'a> PVT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PVT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PVT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PVT_A::ENABLE)
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
#[doc = "Possible values of the field `EZHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EZHA_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<EZHA_A> for bool {
    #[inline(always)]
    fn from(variant: EZHA_A) -> Self {
        match variant {
            EZHA_A::DISABLE => false,
            EZHA_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `EZHA`"]
pub type EZHA_R = crate::R<bool, EZHA_A>;
impl EZHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EZHA_A {
        match self.bits {
            false => EZHA_A::DISABLE,
            true => EZHA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EZHA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EZHA_A::ENABLE
    }
}
#[doc = "Write proxy for field `EZHA`"]
pub struct EZHA_W<'a> {
    w: &'a mut W,
}
impl<'a> EZHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EZHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EZHA_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EZHA_A::ENABLE)
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
#[doc = "Possible values of the field `EZHB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EZHB_A {
    #[doc = "Disable Clock."]
    DISABLE,
    #[doc = "Enable Clock."]
    ENABLE,
}
impl From<EZHB_A> for bool {
    #[inline(always)]
    fn from(variant: EZHB_A) -> Self {
        match variant {
            EZHB_A::DISABLE => false,
            EZHB_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `EZHB`"]
pub type EZHB_R = crate::R<bool, EZHB_A>;
impl EZHB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EZHB_A {
        match self.bits {
            false => EZHB_A::DISABLE,
            true => EZHB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EZHB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EZHB_A::ENABLE
    }
}
#[doc = "Write proxy for field `EZHB`"]
pub struct EZHB_W<'a> {
    w: &'a mut W,
}
impl<'a> EZHB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EZHB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EZHB_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EZHB_A::ENABLE)
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
    #[doc = "Bit 0 - Enables the clock for the MRT."]
    #[inline(always)]
    pub fn mrt(&self) -> MRT_R {
        MRT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the clock for the OS Timer 0."]
    #[inline(always)]
    pub fn ostimer0(&self) -> OSTIMER0_R {
        OSTIMER0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the clock for the SCT0."]
    #[inline(always)]
    pub fn sct0(&self) -> SCT0_R {
        SCT0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables the clock for the SCTIPU."]
    #[inline(always)]
    pub fn sctipu(&self) -> SCTIPU_R {
        SCTIPU_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the UTICK0."]
    #[inline(always)]
    pub fn utick0(&self) -> UTICK0_R {
        UTICK0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the FC0."]
    #[inline(always)]
    pub fn fc0(&self) -> FC0_R {
        FC0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enables the clock for the FC1."]
    #[inline(always)]
    pub fn fc1(&self) -> FC1_R {
        FC1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the FC2."]
    #[inline(always)]
    pub fn fc2(&self) -> FC2_R {
        FC2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the FC3."]
    #[inline(always)]
    pub fn fc3(&self) -> FC3_R {
        FC3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the FC4."]
    #[inline(always)]
    pub fn fc4(&self) -> FC4_R {
        FC4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the FC5."]
    #[inline(always)]
    pub fn fc5(&self) -> FC5_R {
        FC5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the FC6."]
    #[inline(always)]
    pub fn fc6(&self) -> FC6_R {
        FC6_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the FC7."]
    #[inline(always)]
    pub fn fc7(&self) -> FC7_R {
        FC7_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 2."]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 DEV."]
    #[inline(always)]
    pub fn usb0_dev(&self) -> USB0_DEV_R {
        USB0_DEV_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enables the clock for the Timer 0."]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the Timer 1."]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enables the clock for the PVT."]
    #[inline(always)]
    pub fn pvt(&self) -> PVT_R {
        PVT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enables the clock for the EZH a."]
    #[inline(always)]
    pub fn ezha(&self) -> EZHA_R {
        EZHA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enables the clock for the EZH b."]
    #[inline(always)]
    pub fn ezhb(&self) -> EZHB_R {
        EZHB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the clock for the MRT."]
    #[inline(always)]
    pub fn mrt(&mut self) -> MRT_W {
        MRT_W { w: self }
    }
    #[doc = "Bit 1 - Enables the clock for the OS Timer 0."]
    #[inline(always)]
    pub fn ostimer0(&mut self) -> OSTIMER0_W {
        OSTIMER0_W { w: self }
    }
    #[doc = "Bit 2 - Enables the clock for the SCT0."]
    #[inline(always)]
    pub fn sct0(&mut self) -> SCT0_W {
        SCT0_W { w: self }
    }
    #[doc = "Bit 6 - Enables the clock for the SCTIPU."]
    #[inline(always)]
    pub fn sctipu(&mut self) -> SCTIPU_W {
        SCTIPU_W { w: self }
    }
    #[doc = "Bit 10 - Enables the clock for the UTICK0."]
    #[inline(always)]
    pub fn utick0(&mut self) -> UTICK0_W {
        UTICK0_W { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for the FC0."]
    #[inline(always)]
    pub fn fc0(&mut self) -> FC0_W {
        FC0_W { w: self }
    }
    #[doc = "Bit 12 - Enables the clock for the FC1."]
    #[inline(always)]
    pub fn fc1(&mut self) -> FC1_W {
        FC1_W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the FC2."]
    #[inline(always)]
    pub fn fc2(&mut self) -> FC2_W {
        FC2_W { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for the FC3."]
    #[inline(always)]
    pub fn fc3(&mut self) -> FC3_W {
        FC3_W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for the FC4."]
    #[inline(always)]
    pub fn fc4(&mut self) -> FC4_W {
        FC4_W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the FC5."]
    #[inline(always)]
    pub fn fc5(&mut self) -> FC5_W {
        FC5_W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the FC6."]
    #[inline(always)]
    pub fn fc6(&mut self) -> FC6_W {
        FC6_W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the FC7."]
    #[inline(always)]
    pub fn fc7(&mut self) -> FC7_W {
        FC7_W { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 2."]
    #[inline(always)]
    pub fn timer2(&mut self) -> TIMER2_W {
        TIMER2_W { w: self }
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 DEV."]
    #[inline(always)]
    pub fn usb0_dev(&mut self) -> USB0_DEV_W {
        USB0_DEV_W { w: self }
    }
    #[doc = "Bit 26 - Enables the clock for the Timer 0."]
    #[inline(always)]
    pub fn timer0(&mut self) -> TIMER0_W {
        TIMER0_W { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for the Timer 1."]
    #[inline(always)]
    pub fn timer1(&mut self) -> TIMER1_W {
        TIMER1_W { w: self }
    }
    #[doc = "Bit 28 - Enables the clock for the PVT."]
    #[inline(always)]
    pub fn pvt(&mut self) -> PVT_W {
        PVT_W { w: self }
    }
    #[doc = "Bit 30 - Enables the clock for the EZH a."]
    #[inline(always)]
    pub fn ezha(&mut self) -> EZHA_W {
        EZHA_W { w: self }
    }
    #[doc = "Bit 31 - Enables the clock for the EZH b."]
    #[inline(always)]
    pub fn ezhb(&mut self) -> EZHB_W {
        EZHB_W { w: self }
    }
}
