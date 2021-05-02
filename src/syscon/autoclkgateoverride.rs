#[doc = "Register `AUTOCLKGATEOVERRIDE` reader"]
pub struct R(crate::R<AUTOCLKGATEOVERRIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOCLKGATEOVERRIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AUTOCLKGATEOVERRIDE_SPEC>> for R {
    fn from(reader: crate::R<AUTOCLKGATEOVERRIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOCLKGATEOVERRIDE` writer"]
pub struct W(crate::W<AUTOCLKGATEOVERRIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOCLKGATEOVERRIDE_SPEC>;
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
impl core::convert::From<crate::W<AUTOCLKGATEOVERRIDE_SPEC>> for W {
    fn from(writer: crate::W<AUTOCLKGATEOVERRIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control automatic clock gating of ROM controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROM_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<ROM_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM` reader - Control automatic clock gating of ROM controller."]
pub struct ROM_R(crate::FieldReader<bool, ROM_A>);
impl ROM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_A {
        match self.bits {
            false => ROM_A::DISABLE,
            true => ROM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ROM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ROM_A::ENABLE
    }
}
impl core::ops::Deref for ROM_R {
    type Target = crate::FieldReader<bool, ROM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM` writer - Control automatic clock gating of ROM controller."]
pub struct ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROM_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROM_A::ENABLE)
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
#[doc = "Control automatic clock gating of RAMX controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMX_CTRL_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<RAMX_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RAMX_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMX_CTRL` reader - Control automatic clock gating of RAMX controller."]
pub struct RAMX_CTRL_R(crate::FieldReader<bool, RAMX_CTRL_A>);
impl RAMX_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAMX_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMX_CTRL_A {
        match self.bits {
            false => RAMX_CTRL_A::DISABLE,
            true => RAMX_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RAMX_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RAMX_CTRL_A::ENABLE
    }
}
impl core::ops::Deref for RAMX_CTRL_R {
    type Target = crate::FieldReader<bool, RAMX_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMX_CTRL` writer - Control automatic clock gating of RAMX controller."]
pub struct RAMX_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMX_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMX_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAMX_CTRL_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAMX_CTRL_A::ENABLE)
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
#[doc = "Control automatic clock gating of RAM0 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM0_CTRL_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<RAM0_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RAM0_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM0_CTRL` reader - Control automatic clock gating of RAM0 controller."]
pub struct RAM0_CTRL_R(crate::FieldReader<bool, RAM0_CTRL_A>);
impl RAM0_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM0_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM0_CTRL_A {
        match self.bits {
            false => RAM0_CTRL_A::DISABLE,
            true => RAM0_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RAM0_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RAM0_CTRL_A::ENABLE
    }
}
impl core::ops::Deref for RAM0_CTRL_R {
    type Target = crate::FieldReader<bool, RAM0_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM0_CTRL` writer - Control automatic clock gating of RAM0 controller."]
pub struct RAM0_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM0_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM0_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM0_CTRL_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM0_CTRL_A::ENABLE)
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
#[doc = "Control automatic clock gating of RAM1 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM1_CTRL_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<RAM1_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RAM1_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM1_CTRL` reader - Control automatic clock gating of RAM1 controller."]
pub struct RAM1_CTRL_R(crate::FieldReader<bool, RAM1_CTRL_A>);
impl RAM1_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM1_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM1_CTRL_A {
        match self.bits {
            false => RAM1_CTRL_A::DISABLE,
            true => RAM1_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RAM1_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RAM1_CTRL_A::ENABLE
    }
}
impl core::ops::Deref for RAM1_CTRL_R {
    type Target = crate::FieldReader<bool, RAM1_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM1_CTRL` writer - Control automatic clock gating of RAM1 controller."]
pub struct RAM1_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM1_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM1_CTRL_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM1_CTRL_A::ENABLE)
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
#[doc = "Control automatic clock gating of RAM2 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM2_CTRL_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<RAM2_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RAM2_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM2_CTRL` reader - Control automatic clock gating of RAM2 controller."]
pub struct RAM2_CTRL_R(crate::FieldReader<bool, RAM2_CTRL_A>);
impl RAM2_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM2_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM2_CTRL_A {
        match self.bits {
            false => RAM2_CTRL_A::DISABLE,
            true => RAM2_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RAM2_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RAM2_CTRL_A::ENABLE
    }
}
impl core::ops::Deref for RAM2_CTRL_R {
    type Target = crate::FieldReader<bool, RAM2_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM2_CTRL` writer - Control automatic clock gating of RAM2 controller."]
pub struct RAM2_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM2_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM2_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM2_CTRL_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM2_CTRL_A::ENABLE)
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
#[doc = "Control automatic clock gating of RAM3 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM3_CTRL_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<RAM3_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RAM3_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM3_CTRL` reader - Control automatic clock gating of RAM3 controller."]
pub struct RAM3_CTRL_R(crate::FieldReader<bool, RAM3_CTRL_A>);
impl RAM3_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM3_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM3_CTRL_A {
        match self.bits {
            false => RAM3_CTRL_A::DISABLE,
            true => RAM3_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RAM3_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RAM3_CTRL_A::ENABLE
    }
}
impl core::ops::Deref for RAM3_CTRL_R {
    type Target = crate::FieldReader<bool, RAM3_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM3_CTRL` writer - Control automatic clock gating of RAM3 controller."]
pub struct RAM3_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM3_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM3_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM3_CTRL_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM3_CTRL_A::ENABLE)
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
#[doc = "Control automatic clock gating of RAM4 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM4_CTRL_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<RAM4_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RAM4_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM4_CTRL` reader - Control automatic clock gating of RAM4 controller."]
pub struct RAM4_CTRL_R(crate::FieldReader<bool, RAM4_CTRL_A>);
impl RAM4_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM4_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM4_CTRL_A {
        match self.bits {
            false => RAM4_CTRL_A::DISABLE,
            true => RAM4_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RAM4_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RAM4_CTRL_A::ENABLE
    }
}
impl core::ops::Deref for RAM4_CTRL_R {
    type Target = crate::FieldReader<bool, RAM4_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM4_CTRL` writer - Control automatic clock gating of RAM4 controller."]
pub struct RAM4_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM4_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM4_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM4_CTRL_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM4_CTRL_A::ENABLE)
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
#[doc = "Control automatic clock gating of synchronous bridge controller 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC0_APB_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<SYNC0_APB_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC0_APB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC0_APB` reader - Control automatic clock gating of synchronous bridge controller 0."]
pub struct SYNC0_APB_R(crate::FieldReader<bool, SYNC0_APB_A>);
impl SYNC0_APB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNC0_APB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC0_APB_A {
        match self.bits {
            false => SYNC0_APB_A::DISABLE,
            true => SYNC0_APB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SYNC0_APB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SYNC0_APB_A::ENABLE
    }
}
impl core::ops::Deref for SYNC0_APB_R {
    type Target = crate::FieldReader<bool, SYNC0_APB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC0_APB` writer - Control automatic clock gating of synchronous bridge controller 0."]
pub struct SYNC0_APB_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC0_APB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC0_APB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYNC0_APB_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYNC0_APB_A::ENABLE)
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
#[doc = "Control automatic clock gating of synchronous bridge controller 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC1_APB_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<SYNC1_APB_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC1_APB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC1_APB` reader - Control automatic clock gating of synchronous bridge controller 1."]
pub struct SYNC1_APB_R(crate::FieldReader<bool, SYNC1_APB_A>);
impl SYNC1_APB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNC1_APB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC1_APB_A {
        match self.bits {
            false => SYNC1_APB_A::DISABLE,
            true => SYNC1_APB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SYNC1_APB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SYNC1_APB_A::ENABLE
    }
}
impl core::ops::Deref for SYNC1_APB_R {
    type Target = crate::FieldReader<bool, SYNC1_APB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC1_APB` writer - Control automatic clock gating of synchronous bridge controller 1."]
pub struct SYNC1_APB_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC1_APB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC1_APB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYNC1_APB_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYNC1_APB_A::ENABLE)
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
#[doc = "Control automatic clock gating of CRCGEN controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCGEN_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<CRCGEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCGEN` reader - Control automatic clock gating of CRCGEN controller."]
pub struct CRCGEN_R(crate::FieldReader<bool, CRCGEN_A>);
impl CRCGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCGEN_A {
        match self.bits {
            false => CRCGEN_A::DISABLE,
            true => CRCGEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CRCGEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CRCGEN_A::ENABLE
    }
}
impl core::ops::Deref for CRCGEN_R {
    type Target = crate::FieldReader<bool, CRCGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCGEN` writer - Control automatic clock gating of CRCGEN controller."]
pub struct CRCGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRCGEN_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CRCGEN_A::ENABLE)
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
#[doc = "Control automatic clock gating of DMA0 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA0_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<SDMA0_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA0` reader - Control automatic clock gating of DMA0 controller."]
pub struct SDMA0_R(crate::FieldReader<bool, SDMA0_A>);
impl SDMA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMA0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA0_A {
        match self.bits {
            false => SDMA0_A::DISABLE,
            true => SDMA0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SDMA0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SDMA0_A::ENABLE
    }
}
impl core::ops::Deref for SDMA0_R {
    type Target = crate::FieldReader<bool, SDMA0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMA0` writer - Control automatic clock gating of DMA0 controller."]
pub struct SDMA0_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMA0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA0_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDMA0_A::ENABLE)
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
#[doc = "Control automatic clock gating of DMA1 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMA1_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<SDMA1_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA1` reader - Control automatic clock gating of DMA1 controller."]
pub struct SDMA1_R(crate::FieldReader<bool, SDMA1_A>);
impl SDMA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMA1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA1_A {
        match self.bits {
            false => SDMA1_A::DISABLE,
            true => SDMA1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SDMA1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SDMA1_A::ENABLE
    }
}
impl core::ops::Deref for SDMA1_R {
    type Target = crate::FieldReader<bool, SDMA1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMA1` writer - Control automatic clock gating of DMA1 controller."]
pub struct SDMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMA1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA1_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDMA1_A::ENABLE)
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
#[doc = "Control automatic clock gating of USB controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<USB0_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0` reader - Control automatic clock gating of USB controller."]
pub struct USB0_R(crate::FieldReader<bool, USB0_A>);
impl USB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_A {
        match self.bits {
            false => USB0_A::DISABLE,
            true => USB0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == USB0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == USB0_A::ENABLE
    }
}
impl core::ops::Deref for USB0_R {
    type Target = crate::FieldReader<bool, USB0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0` writer - Control automatic clock gating of USB controller."]
pub struct USB0_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_A::ENABLE)
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
#[doc = "Control automatic clock gating of synchronous system controller registers bank.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCON_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<SYSCON_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCON` reader - Control automatic clock gating of synchronous system controller registers bank."]
pub struct SYSCON_R(crate::FieldReader<bool, SYSCON_A>);
impl SYSCON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCON_A {
        match self.bits {
            false => SYSCON_A::DISABLE,
            true => SYSCON_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SYSCON_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SYSCON_A::ENABLE
    }
}
impl core::ops::Deref for SYSCON_R {
    type Target = crate::FieldReader<bool, SYSCON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCON` writer - Control automatic clock gating of synchronous system controller registers bank."]
pub struct SYSCON_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYSCON_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYSCON_A::ENABLE)
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
#[doc = "The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ENABLEUPDATE_AW {
    #[doc = "0: Bit Fields 0 - 15 of this register are not updated"]
    DISABLE = 0,
    #[doc = "49374: Bit Fields 0 - 15 of this register are updated"]
    ENABLE = 49374,
}
impl From<ENABLEUPDATE_AW> for u16 {
    #[inline(always)]
    fn from(variant: ENABLEUPDATE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `ENABLEUPDATE` writer - The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect."]
pub struct ENABLEUPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLEUPDATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLEUPDATE_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Bit Fields 0 - 15 of this register are not updated"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLEUPDATE_AW::DISABLE)
    }
    #[doc = "Bit Fields 0 - 15 of this register are updated"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLEUPDATE_AW::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Control automatic clock gating of ROM controller."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control automatic clock gating of RAMX controller."]
    #[inline(always)]
    pub fn ramx_ctrl(&self) -> RAMX_CTRL_R {
        RAMX_CTRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Control automatic clock gating of RAM0 controller."]
    #[inline(always)]
    pub fn ram0_ctrl(&self) -> RAM0_CTRL_R {
        RAM0_CTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control automatic clock gating of RAM1 controller."]
    #[inline(always)]
    pub fn ram1_ctrl(&self) -> RAM1_CTRL_R {
        RAM1_CTRL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control automatic clock gating of RAM2 controller."]
    #[inline(always)]
    pub fn ram2_ctrl(&self) -> RAM2_CTRL_R {
        RAM2_CTRL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Control automatic clock gating of RAM3 controller."]
    #[inline(always)]
    pub fn ram3_ctrl(&self) -> RAM3_CTRL_R {
        RAM3_CTRL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Control automatic clock gating of RAM4 controller."]
    #[inline(always)]
    pub fn ram4_ctrl(&self) -> RAM4_CTRL_R {
        RAM4_CTRL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Control automatic clock gating of synchronous bridge controller 0."]
    #[inline(always)]
    pub fn sync0_apb(&self) -> SYNC0_APB_R {
        SYNC0_APB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Control automatic clock gating of synchronous bridge controller 1."]
    #[inline(always)]
    pub fn sync1_apb(&self) -> SYNC1_APB_R {
        SYNC1_APB_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Control automatic clock gating of CRCGEN controller."]
    #[inline(always)]
    pub fn crcgen(&self) -> CRCGEN_R {
        CRCGEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Control automatic clock gating of DMA0 controller."]
    #[inline(always)]
    pub fn sdma0(&self) -> SDMA0_R {
        SDMA0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Control automatic clock gating of DMA1 controller."]
    #[inline(always)]
    pub fn sdma1(&self) -> SDMA1_R {
        SDMA1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Control automatic clock gating of USB controller."]
    #[inline(always)]
    pub fn usb0(&self) -> USB0_R {
        USB0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Control automatic clock gating of synchronous system controller registers bank."]
    #[inline(always)]
    pub fn syscon(&self) -> SYSCON_R {
        SYSCON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control automatic clock gating of ROM controller."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 1 - Control automatic clock gating of RAMX controller."]
    #[inline(always)]
    pub fn ramx_ctrl(&mut self) -> RAMX_CTRL_W {
        RAMX_CTRL_W { w: self }
    }
    #[doc = "Bit 2 - Control automatic clock gating of RAM0 controller."]
    #[inline(always)]
    pub fn ram0_ctrl(&mut self) -> RAM0_CTRL_W {
        RAM0_CTRL_W { w: self }
    }
    #[doc = "Bit 3 - Control automatic clock gating of RAM1 controller."]
    #[inline(always)]
    pub fn ram1_ctrl(&mut self) -> RAM1_CTRL_W {
        RAM1_CTRL_W { w: self }
    }
    #[doc = "Bit 4 - Control automatic clock gating of RAM2 controller."]
    #[inline(always)]
    pub fn ram2_ctrl(&mut self) -> RAM2_CTRL_W {
        RAM2_CTRL_W { w: self }
    }
    #[doc = "Bit 5 - Control automatic clock gating of RAM3 controller."]
    #[inline(always)]
    pub fn ram3_ctrl(&mut self) -> RAM3_CTRL_W {
        RAM3_CTRL_W { w: self }
    }
    #[doc = "Bit 6 - Control automatic clock gating of RAM4 controller."]
    #[inline(always)]
    pub fn ram4_ctrl(&mut self) -> RAM4_CTRL_W {
        RAM4_CTRL_W { w: self }
    }
    #[doc = "Bit 7 - Control automatic clock gating of synchronous bridge controller 0."]
    #[inline(always)]
    pub fn sync0_apb(&mut self) -> SYNC0_APB_W {
        SYNC0_APB_W { w: self }
    }
    #[doc = "Bit 8 - Control automatic clock gating of synchronous bridge controller 1."]
    #[inline(always)]
    pub fn sync1_apb(&mut self) -> SYNC1_APB_W {
        SYNC1_APB_W { w: self }
    }
    #[doc = "Bit 11 - Control automatic clock gating of CRCGEN controller."]
    #[inline(always)]
    pub fn crcgen(&mut self) -> CRCGEN_W {
        CRCGEN_W { w: self }
    }
    #[doc = "Bit 12 - Control automatic clock gating of DMA0 controller."]
    #[inline(always)]
    pub fn sdma0(&mut self) -> SDMA0_W {
        SDMA0_W { w: self }
    }
    #[doc = "Bit 13 - Control automatic clock gating of DMA1 controller."]
    #[inline(always)]
    pub fn sdma1(&mut self) -> SDMA1_W {
        SDMA1_W { w: self }
    }
    #[doc = "Bit 14 - Control automatic clock gating of USB controller."]
    #[inline(always)]
    pub fn usb0(&mut self) -> USB0_W {
        USB0_W { w: self }
    }
    #[doc = "Bit 15 - Control automatic clock gating of synchronous system controller registers bank."]
    #[inline(always)]
    pub fn syscon(&mut self) -> SYSCON_W {
        SYSCON_W { w: self }
    }
    #[doc = "Bits 16:31 - The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect."]
    #[inline(always)]
    pub fn enableupdate(&mut self) -> ENABLEUPDATE_W {
        ENABLEUPDATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control automatic clock gating\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autoclkgateoverride](index.html) module"]
pub struct AUTOCLKGATEOVERRIDE_SPEC;
impl crate::RegisterSpec for AUTOCLKGATEOVERRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autoclkgateoverride::R](R) reader structure"]
impl crate::Readable for AUTOCLKGATEOVERRIDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autoclkgateoverride::W](W) writer structure"]
impl crate::Writable for AUTOCLKGATEOVERRIDE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUTOCLKGATEOVERRIDE to value 0xffff"]
impl crate::Resettable for AUTOCLKGATEOVERRIDE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
