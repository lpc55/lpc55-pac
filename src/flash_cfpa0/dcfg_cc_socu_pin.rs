#[doc = "Register `DCFG_CC_SOCU_PIN` reader"]
pub struct R(crate::R<DCFG_CC_SOCU_PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCFG_CC_SOCU_PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCFG_CC_SOCU_PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCFG_CC_SOCU_PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCFG_CC_SOCU_PIN` writer"]
pub struct W(crate::W<DCFG_CC_SOCU_PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCFG_CC_SOCU_PIN_SPEC>;
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
impl From<crate::W<DCFG_CC_SOCU_PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCFG_CC_SOCU_PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Non Secure non-invasive debug enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NIDEN_A {
    #[doc = "0: Use DAP to enable"]
    VALUE_0 = 0,
    #[doc = "1: Fixed state"]
    VALUE_1 = 1,
}
impl From<NIDEN_A> for bool {
    #[inline(always)]
    fn from(variant: NIDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDEN` reader - Non Secure non-invasive debug enable"]
pub struct NIDEN_R(crate::FieldReader<bool, NIDEN_A>);
impl NIDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NIDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NIDEN_A {
        match self.bits {
            false => NIDEN_A::VALUE_0,
            true => NIDEN_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == NIDEN_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == NIDEN_A::VALUE_1
    }
}
impl core::ops::Deref for NIDEN_R {
    type Target = crate::FieldReader<bool, NIDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NIDEN` writer - Non Secure non-invasive debug enable"]
pub struct NIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NIDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(NIDEN_A::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(NIDEN_A::VALUE_1)
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
#[doc = "Non Secure debug enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGEN_A {
    #[doc = "0: Use DAP to enable"]
    VALUE_0 = 0,
    #[doc = "1: Fixed state"]
    VALUE_1 = 1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN` reader - Non Secure debug enable"]
pub struct DBGEN_R(crate::FieldReader<bool, DBGEN_A>);
impl DBGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::VALUE_0,
            true => DBGEN_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == DBGEN_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == DBGEN_A::VALUE_1
    }
}
impl core::ops::Deref for DBGEN_R {
    type Target = crate::FieldReader<bool, DBGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGEN` writer - Non Secure debug enable"]
pub struct DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(DBGEN_A::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(DBGEN_A::VALUE_1)
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
#[doc = "Secure non-invasive debug enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPNIDEN_A {
    #[doc = "0: Use DAP to enable"]
    VALUE_0 = 0,
    #[doc = "1: Fixed state"]
    VALUE_1 = 1,
}
impl From<SPNIDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPNIDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPNIDEN` reader - Secure non-invasive debug enable"]
pub struct SPNIDEN_R(crate::FieldReader<bool, SPNIDEN_A>);
impl SPNIDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPNIDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPNIDEN_A {
        match self.bits {
            false => SPNIDEN_A::VALUE_0,
            true => SPNIDEN_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == SPNIDEN_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == SPNIDEN_A::VALUE_1
    }
}
impl core::ops::Deref for SPNIDEN_R {
    type Target = crate::FieldReader<bool, SPNIDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPNIDEN` writer - Secure non-invasive debug enable"]
pub struct SPNIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPNIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPNIDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SPNIDEN_A::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(SPNIDEN_A::VALUE_1)
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
#[doc = "Secure invasive debug enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIDEN_A {
    #[doc = "0: Use DAP to enable"]
    VALUE_0 = 0,
    #[doc = "1: Fixed state"]
    VALUE_1 = 1,
}
impl From<SPIDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPIDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIDEN` reader - Secure invasive debug enable"]
pub struct SPIDEN_R(crate::FieldReader<bool, SPIDEN_A>);
impl SPIDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIDEN_A {
        match self.bits {
            false => SPIDEN_A::VALUE_0,
            true => SPIDEN_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == SPIDEN_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == SPIDEN_A::VALUE_1
    }
}
impl core::ops::Deref for SPIDEN_R {
    type Target = crate::FieldReader<bool, SPIDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIDEN` writer - Secure invasive debug enable"]
pub struct SPIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(SPIDEN_A::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(SPIDEN_A::VALUE_1)
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
#[doc = "JTAG TAP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAPEN_A {
    #[doc = "0: Use DAP to enable"]
    VALUE_0 = 0,
    #[doc = "1: Fixed state"]
    VALUE_1 = 1,
}
impl From<TAPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TAPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAPEN` reader - JTAG TAP enable"]
pub struct TAPEN_R(crate::FieldReader<bool, TAPEN_A>);
impl TAPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAPEN_A {
        match self.bits {
            false => TAPEN_A::VALUE_0,
            true => TAPEN_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == TAPEN_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == TAPEN_A::VALUE_1
    }
}
impl core::ops::Deref for TAPEN_R {
    type Target = crate::FieldReader<bool, TAPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAPEN` writer - JTAG TAP enable"]
pub struct TAPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(TAPEN_A::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(TAPEN_A::VALUE_1)
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
#[doc = "CPU1 (Micro cortex M33) invasive debug enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1_DBGEN_A {
    #[doc = "0: Use DAP to enable"]
    VALUE_0 = 0,
    #[doc = "1: Fixed state"]
    VALUE_1 = 1,
}
impl From<CPU1_DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1_DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1_DBGEN` reader - CPU1 (Micro cortex M33) invasive debug enable"]
pub struct CPU1_DBGEN_R(crate::FieldReader<bool, CPU1_DBGEN_A>);
impl CPU1_DBGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPU1_DBGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1_DBGEN_A {
        match self.bits {
            false => CPU1_DBGEN_A::VALUE_0,
            true => CPU1_DBGEN_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == CPU1_DBGEN_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == CPU1_DBGEN_A::VALUE_1
    }
}
impl core::ops::Deref for CPU1_DBGEN_R {
    type Target = crate::FieldReader<bool, CPU1_DBGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU1_DBGEN` writer - CPU1 (Micro cortex M33) invasive debug enable"]
pub struct CPU1_DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1_DBGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1_DBGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(CPU1_DBGEN_A::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(CPU1_DBGEN_A::VALUE_1)
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
#[doc = "ISP Boot Command enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISP_CMD_EN_A {
    #[doc = "0: Use DAP to enable"]
    VALUE_0 = 0,
    #[doc = "1: Fixed state"]
    VALUE_1 = 1,
}
impl From<ISP_CMD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ISP_CMD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISP_CMD_EN` reader - ISP Boot Command enable"]
pub struct ISP_CMD_EN_R(crate::FieldReader<bool, ISP_CMD_EN_A>);
impl ISP_CMD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISP_CMD_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISP_CMD_EN_A {
        match self.bits {
            false => ISP_CMD_EN_A::VALUE_0,
            true => ISP_CMD_EN_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == ISP_CMD_EN_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == ISP_CMD_EN_A::VALUE_1
    }
}
impl core::ops::Deref for ISP_CMD_EN_R {
    type Target = crate::FieldReader<bool, ISP_CMD_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISP_CMD_EN` writer - ISP Boot Command enable"]
pub struct ISP_CMD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISP_CMD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISP_CMD_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(ISP_CMD_EN_A::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(ISP_CMD_EN_A::VALUE_1)
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
#[doc = "FA Command enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FA_CMD_EN_A {
    #[doc = "0: Use DAP to enable"]
    VALUE_0 = 0,
    #[doc = "1: Fixed state"]
    VALUE_1 = 1,
}
impl From<FA_CMD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FA_CMD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FA_CMD_EN` reader - FA Command enable"]
pub struct FA_CMD_EN_R(crate::FieldReader<bool, FA_CMD_EN_A>);
impl FA_CMD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FA_CMD_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FA_CMD_EN_A {
        match self.bits {
            false => FA_CMD_EN_A::VALUE_0,
            true => FA_CMD_EN_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == FA_CMD_EN_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == FA_CMD_EN_A::VALUE_1
    }
}
impl core::ops::Deref for FA_CMD_EN_R {
    type Target = crate::FieldReader<bool, FA_CMD_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FA_CMD_EN` writer - FA Command enable"]
pub struct FA_CMD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FA_CMD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FA_CMD_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(FA_CMD_EN_A::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(FA_CMD_EN_A::VALUE_1)
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
#[doc = "Flash Mass Erase Command enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ME_CMD_EN_A {
    #[doc = "0: Use DAP to enable"]
    VALUE_0 = 0,
    #[doc = "1: Fixed state"]
    VALUE_1 = 1,
}
impl From<ME_CMD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ME_CMD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ME_CMD_EN` reader - Flash Mass Erase Command enable"]
pub struct ME_CMD_EN_R(crate::FieldReader<bool, ME_CMD_EN_A>);
impl ME_CMD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ME_CMD_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ME_CMD_EN_A {
        match self.bits {
            false => ME_CMD_EN_A::VALUE_0,
            true => ME_CMD_EN_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == ME_CMD_EN_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == ME_CMD_EN_A::VALUE_1
    }
}
impl core::ops::Deref for ME_CMD_EN_R {
    type Target = crate::FieldReader<bool, ME_CMD_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ME_CMD_EN` writer - Flash Mass Erase Command enable"]
pub struct ME_CMD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ME_CMD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ME_CMD_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(ME_CMD_EN_A::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(ME_CMD_EN_A::VALUE_1)
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
#[doc = "CPU1 (Micro cortex M33) non-invasive debug enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1_NIDEN_A {
    #[doc = "0: Use DAP to enable"]
    VALUE_0 = 0,
    #[doc = "1: Fixed state"]
    VALUE_1 = 1,
}
impl From<CPU1_NIDEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1_NIDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1_NIDEN` reader - CPU1 (Micro cortex M33) non-invasive debug enable"]
pub struct CPU1_NIDEN_R(crate::FieldReader<bool, CPU1_NIDEN_A>);
impl CPU1_NIDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPU1_NIDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1_NIDEN_A {
        match self.bits {
            false => CPU1_NIDEN_A::VALUE_0,
            true => CPU1_NIDEN_A::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        **self == CPU1_NIDEN_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        **self == CPU1_NIDEN_A::VALUE_1
    }
}
impl core::ops::Deref for CPU1_NIDEN_R {
    type Target = crate::FieldReader<bool, CPU1_NIDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU1_NIDEN` writer - CPU1 (Micro cortex M33) non-invasive debug enable"]
pub struct CPU1_NIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1_NIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1_NIDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use DAP to enable"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(CPU1_NIDEN_A::VALUE_0)
    }
    #[doc = "Fixed state"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(CPU1_NIDEN_A::VALUE_1)
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
#[doc = "Field `UUID_CHECK` reader - Enforce UUID match during Debug authentication."]
pub struct UUID_CHECK_R(crate::FieldReader<bool, bool>);
impl UUID_CHECK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UUID_CHECK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UUID_CHECK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UUID_CHECK` writer - Enforce UUID match during Debug authentication."]
pub struct UUID_CHECK_W<'a> {
    w: &'a mut W,
}
impl<'a> UUID_CHECK_W<'a> {
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
#[doc = "Field `INVERSE_VALUE` reader - inverse value of bits \\[15:0\\]"]
pub struct INVERSE_VALUE_R(crate::FieldReader<u16, u16>);
impl INVERSE_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        INVERSE_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVERSE_VALUE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVERSE_VALUE` writer - inverse value of bits \\[15:0\\]"]
pub struct INVERSE_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> INVERSE_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Non Secure non-invasive debug enable"]
    #[inline(always)]
    pub fn niden(&self) -> NIDEN_R {
        NIDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Non Secure debug enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Secure non-invasive debug enable"]
    #[inline(always)]
    pub fn spniden(&self) -> SPNIDEN_R {
        SPNIDEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Secure invasive debug enable"]
    #[inline(always)]
    pub fn spiden(&self) -> SPIDEN_R {
        SPIDEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - JTAG TAP enable"]
    #[inline(always)]
    pub fn tapen(&self) -> TAPEN_R {
        TAPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CPU1 (Micro cortex M33) invasive debug enable"]
    #[inline(always)]
    pub fn cpu1_dbgen(&self) -> CPU1_DBGEN_R {
        CPU1_DBGEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ISP Boot Command enable"]
    #[inline(always)]
    pub fn isp_cmd_en(&self) -> ISP_CMD_EN_R {
        ISP_CMD_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FA Command enable"]
    #[inline(always)]
    pub fn fa_cmd_en(&self) -> FA_CMD_EN_R {
        FA_CMD_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash Mass Erase Command enable"]
    #[inline(always)]
    pub fn me_cmd_en(&self) -> ME_CMD_EN_R {
        ME_CMD_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CPU1 (Micro cortex M33) non-invasive debug enable"]
    #[inline(always)]
    pub fn cpu1_niden(&self) -> CPU1_NIDEN_R {
        CPU1_NIDEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enforce UUID match during Debug authentication."]
    #[inline(always)]
    pub fn uuid_check(&self) -> UUID_CHECK_R {
        UUID_CHECK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&self) -> INVERSE_VALUE_R {
        INVERSE_VALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Non Secure non-invasive debug enable"]
    #[inline(always)]
    pub fn niden(&mut self) -> NIDEN_W {
        NIDEN_W { w: self }
    }
    #[doc = "Bit 1 - Non Secure debug enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
    }
    #[doc = "Bit 2 - Secure non-invasive debug enable"]
    #[inline(always)]
    pub fn spniden(&mut self) -> SPNIDEN_W {
        SPNIDEN_W { w: self }
    }
    #[doc = "Bit 3 - Secure invasive debug enable"]
    #[inline(always)]
    pub fn spiden(&mut self) -> SPIDEN_W {
        SPIDEN_W { w: self }
    }
    #[doc = "Bit 4 - JTAG TAP enable"]
    #[inline(always)]
    pub fn tapen(&mut self) -> TAPEN_W {
        TAPEN_W { w: self }
    }
    #[doc = "Bit 5 - CPU1 (Micro cortex M33) invasive debug enable"]
    #[inline(always)]
    pub fn cpu1_dbgen(&mut self) -> CPU1_DBGEN_W {
        CPU1_DBGEN_W { w: self }
    }
    #[doc = "Bit 6 - ISP Boot Command enable"]
    #[inline(always)]
    pub fn isp_cmd_en(&mut self) -> ISP_CMD_EN_W {
        ISP_CMD_EN_W { w: self }
    }
    #[doc = "Bit 7 - FA Command enable"]
    #[inline(always)]
    pub fn fa_cmd_en(&mut self) -> FA_CMD_EN_W {
        FA_CMD_EN_W { w: self }
    }
    #[doc = "Bit 8 - Flash Mass Erase Command enable"]
    #[inline(always)]
    pub fn me_cmd_en(&mut self) -> ME_CMD_EN_W {
        ME_CMD_EN_W { w: self }
    }
    #[doc = "Bit 9 - CPU1 (Micro cortex M33) non-invasive debug enable"]
    #[inline(always)]
    pub fn cpu1_niden(&mut self) -> CPU1_NIDEN_W {
        CPU1_NIDEN_W { w: self }
    }
    #[doc = "Bit 15 - Enforce UUID match during Debug authentication."]
    #[inline(always)]
    pub fn uuid_check(&mut self) -> UUID_CHECK_W {
        UUID_CHECK_W { w: self }
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&mut self) -> INVERSE_VALUE_W {
        INVERSE_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg_cc_socu_pin](index.html) module"]
pub struct DCFG_CC_SOCU_PIN_SPEC;
impl crate::RegisterSpec for DCFG_CC_SOCU_PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcfg_cc_socu_pin::R](R) reader structure"]
impl crate::Readable for DCFG_CC_SOCU_PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcfg_cc_socu_pin::W](W) writer structure"]
impl crate::Writable for DCFG_CC_SOCU_PIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCFG_CC_SOCU_PIN to value 0"]
impl crate::Resettable for DCFG_CC_SOCU_PIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
