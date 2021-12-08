#[doc = "Register `FMCCR` reader"]
pub struct R(crate::R<FMCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMCCR` writer"]
pub struct W(crate::W<FMCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMCCR_SPEC>;
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
impl From<crate::W<FMCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Instruction fetch configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FETCHCFG_A {
    #[doc = "0: Instruction fetches from flash are not buffered."]
    NOBUF = 0,
    #[doc = "1: One buffer is used for all instruction fetches."]
    ONEBUF = 1,
    #[doc = "2: All buffers may be used for instruction fetches."]
    ALLBUF = 2,
}
impl From<FETCHCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: FETCHCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FETCHCFG` reader - Instruction fetch configuration."]
pub struct FETCHCFG_R(crate::FieldReader<u8, FETCHCFG_A>);
impl FETCHCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FETCHCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FETCHCFG_A> {
        match self.bits {
            0 => Some(FETCHCFG_A::NOBUF),
            1 => Some(FETCHCFG_A::ONEBUF),
            2 => Some(FETCHCFG_A::ALLBUF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOBUF`"]
    #[inline(always)]
    pub fn is_nobuf(&self) -> bool {
        **self == FETCHCFG_A::NOBUF
    }
    #[doc = "Checks if the value of the field is `ONEBUF`"]
    #[inline(always)]
    pub fn is_onebuf(&self) -> bool {
        **self == FETCHCFG_A::ONEBUF
    }
    #[doc = "Checks if the value of the field is `ALLBUF`"]
    #[inline(always)]
    pub fn is_allbuf(&self) -> bool {
        **self == FETCHCFG_A::ALLBUF
    }
}
impl core::ops::Deref for FETCHCFG_R {
    type Target = crate::FieldReader<u8, FETCHCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FETCHCFG` writer - Instruction fetch configuration."]
pub struct FETCHCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> FETCHCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FETCHCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Instruction fetches from flash are not buffered."]
    #[inline(always)]
    pub fn nobuf(self) -> &'a mut W {
        self.variant(FETCHCFG_A::NOBUF)
    }
    #[doc = "One buffer is used for all instruction fetches."]
    #[inline(always)]
    pub fn onebuf(self) -> &'a mut W {
        self.variant(FETCHCFG_A::ONEBUF)
    }
    #[doc = "All buffers may be used for instruction fetches."]
    #[inline(always)]
    pub fn allbuf(self) -> &'a mut W {
        self.variant(FETCHCFG_A::ALLBUF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Data read configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATACFG_A {
    #[doc = "0: Data accesses from flash are not buffered."]
    NOBUF = 0,
    #[doc = "1: One buffer is used for all data accesses."]
    ONEBUF = 1,
    #[doc = "2: All buffers can be used for data accesses."]
    ALLBUF = 2,
}
impl From<DATACFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DATACFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATACFG` reader - Data read configuration."]
pub struct DATACFG_R(crate::FieldReader<u8, DATACFG_A>);
impl DATACFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATACFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATACFG_A> {
        match self.bits {
            0 => Some(DATACFG_A::NOBUF),
            1 => Some(DATACFG_A::ONEBUF),
            2 => Some(DATACFG_A::ALLBUF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOBUF`"]
    #[inline(always)]
    pub fn is_nobuf(&self) -> bool {
        **self == DATACFG_A::NOBUF
    }
    #[doc = "Checks if the value of the field is `ONEBUF`"]
    #[inline(always)]
    pub fn is_onebuf(&self) -> bool {
        **self == DATACFG_A::ONEBUF
    }
    #[doc = "Checks if the value of the field is `ALLBUF`"]
    #[inline(always)]
    pub fn is_allbuf(&self) -> bool {
        **self == DATACFG_A::ALLBUF
    }
}
impl core::ops::Deref for DATACFG_R {
    type Target = crate::FieldReader<u8, DATACFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATACFG` writer - Data read configuration."]
pub struct DATACFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DATACFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATACFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Data accesses from flash are not buffered."]
    #[inline(always)]
    pub fn nobuf(self) -> &'a mut W {
        self.variant(DATACFG_A::NOBUF)
    }
    #[doc = "One buffer is used for all data accesses."]
    #[inline(always)]
    pub fn onebuf(self) -> &'a mut W {
        self.variant(DATACFG_A::ONEBUF)
    }
    #[doc = "All buffers can be used for data accesses."]
    #[inline(always)]
    pub fn allbuf(self) -> &'a mut W {
        self.variant(DATACFG_A::ALLBUF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Acceleration enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCEL_A {
    #[doc = "0: Flash acceleration is disabled."]
    DISABLE = 0,
    #[doc = "1: Flash acceleration is enabled."]
    ENABLE = 1,
}
impl From<ACCEL_A> for bool {
    #[inline(always)]
    fn from(variant: ACCEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCEL` reader - Acceleration enable."]
pub struct ACCEL_R(crate::FieldReader<bool, ACCEL_A>);
impl ACCEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACCEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCEL_A {
        match self.bits {
            false => ACCEL_A::DISABLE,
            true => ACCEL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ACCEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ACCEL_A::ENABLE
    }
}
impl core::ops::Deref for ACCEL_R {
    type Target = crate::FieldReader<bool, ACCEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCEL` writer - Acceleration enable."]
pub struct ACCEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACCEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash acceleration is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACCEL_A::DISABLE)
    }
    #[doc = "Flash acceleration is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ACCEL_A::ENABLE)
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
#[doc = "Prefetch enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREFEN_A {
    #[doc = "0: No instruction prefetch is performed."]
    DISABLE = 0,
    #[doc = "1: Instruction prefetch is enabled."]
    ENABLE = 1,
}
impl From<PREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: PREFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREFEN` reader - Prefetch enable."]
pub struct PREFEN_R(crate::FieldReader<bool, PREFEN_A>);
impl PREFEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PREFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREFEN_A {
        match self.bits {
            false => PREFEN_A::DISABLE,
            true => PREFEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PREFEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PREFEN_A::ENABLE
    }
}
impl core::ops::Deref for PREFEN_R {
    type Target = crate::FieldReader<bool, PREFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREFEN` writer - Prefetch enable."]
pub struct PREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No instruction prefetch is performed."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PREFEN_A::DISABLE)
    }
    #[doc = "Instruction prefetch is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PREFEN_A::ENABLE)
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
#[doc = "Prefetch override.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREFOVR_A {
    #[doc = "0: Any previously initiated prefetch will be completed."]
    NORMAL = 0,
    #[doc = "1: Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    OVERRIDE = 1,
}
impl From<PREFOVR_A> for bool {
    #[inline(always)]
    fn from(variant: PREFOVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREFOVR` reader - Prefetch override."]
pub struct PREFOVR_R(crate::FieldReader<bool, PREFOVR_A>);
impl PREFOVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PREFOVR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREFOVR_A {
        match self.bits {
            false => PREFOVR_A::NORMAL,
            true => PREFOVR_A::OVERRIDE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == PREFOVR_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        **self == PREFOVR_A::OVERRIDE
    }
}
impl core::ops::Deref for PREFOVR_R {
    type Target = crate::FieldReader<bool, PREFOVR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREFOVR` writer - Prefetch override."]
pub struct PREFOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFOVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREFOVR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Any previously initiated prefetch will be completed."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(PREFOVR_A::NORMAL)
    }
    #[doc = "Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    #[inline(always)]
    pub fn override_(self) -> &'a mut W {
        self.variant(PREFOVR_A::OVERRIDE)
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
#[doc = "Flash memory access time.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASHTIM_A {
    #[doc = "0: 1 system clock flash access time (for system clock rates up to 11 MHz)."]
    FLASHTIM0 = 0,
    #[doc = "1: 2 system clocks flash access time (for system clock rates up to 22 MHz)."]
    FLASHTIM1 = 1,
    #[doc = "2: 3 system clocks flash access time (for system clock rates up to 33 MHz)."]
    FLASHTIM2 = 2,
    #[doc = "3: 4 system clocks flash access time (for system clock rates up to 44 MHz)."]
    FLASHTIM3 = 3,
    #[doc = "4: 5 system clocks flash access time (for system clock rates up to 55 MHz)."]
    FLASHTIM4 = 4,
    #[doc = "5: 6 system clocks flash access time (for system clock rates up to 66 MHz)."]
    FLASHTIM5 = 5,
    #[doc = "6: 7 system clocks flash access time (for system clock rates up to 77 MHz)."]
    FLASHTIM6 = 6,
    #[doc = "7: 8 system clocks flash access time (for system clock rates up to 88 MHz)."]
    FLASHTIM7 = 7,
    #[doc = "8: 9 system clocks flash access time (for system clock rates up to 100 MHz)."]
    FLASHTIM8 = 8,
    #[doc = "9: 10 system clocks flash access time (for system clock rates up to 115 MHz)."]
    FLASHTIM9 = 9,
    #[doc = "10: 11 system clocks flash access time (for system clock rates up to 130 MHz)."]
    FLASHTIM10 = 10,
    #[doc = "11: 12 system clocks flash access time (for system clock rates up to 150 MHz)."]
    FLASHTIM11 = 11,
}
impl From<FLASHTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASHTIM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASHTIM` reader - Flash memory access time."]
pub struct FLASHTIM_R(crate::FieldReader<u8, FLASHTIM_A>);
impl FLASHTIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLASHTIM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASHTIM_A> {
        match self.bits {
            0 => Some(FLASHTIM_A::FLASHTIM0),
            1 => Some(FLASHTIM_A::FLASHTIM1),
            2 => Some(FLASHTIM_A::FLASHTIM2),
            3 => Some(FLASHTIM_A::FLASHTIM3),
            4 => Some(FLASHTIM_A::FLASHTIM4),
            5 => Some(FLASHTIM_A::FLASHTIM5),
            6 => Some(FLASHTIM_A::FLASHTIM6),
            7 => Some(FLASHTIM_A::FLASHTIM7),
            8 => Some(FLASHTIM_A::FLASHTIM8),
            9 => Some(FLASHTIM_A::FLASHTIM9),
            10 => Some(FLASHTIM_A::FLASHTIM10),
            11 => Some(FLASHTIM_A::FLASHTIM11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASHTIM0`"]
    #[inline(always)]
    pub fn is_flashtim0(&self) -> bool {
        **self == FLASHTIM_A::FLASHTIM0
    }
    #[doc = "Checks if the value of the field is `FLASHTIM1`"]
    #[inline(always)]
    pub fn is_flashtim1(&self) -> bool {
        **self == FLASHTIM_A::FLASHTIM1
    }
    #[doc = "Checks if the value of the field is `FLASHTIM2`"]
    #[inline(always)]
    pub fn is_flashtim2(&self) -> bool {
        **self == FLASHTIM_A::FLASHTIM2
    }
    #[doc = "Checks if the value of the field is `FLASHTIM3`"]
    #[inline(always)]
    pub fn is_flashtim3(&self) -> bool {
        **self == FLASHTIM_A::FLASHTIM3
    }
    #[doc = "Checks if the value of the field is `FLASHTIM4`"]
    #[inline(always)]
    pub fn is_flashtim4(&self) -> bool {
        **self == FLASHTIM_A::FLASHTIM4
    }
    #[doc = "Checks if the value of the field is `FLASHTIM5`"]
    #[inline(always)]
    pub fn is_flashtim5(&self) -> bool {
        **self == FLASHTIM_A::FLASHTIM5
    }
    #[doc = "Checks if the value of the field is `FLASHTIM6`"]
    #[inline(always)]
    pub fn is_flashtim6(&self) -> bool {
        **self == FLASHTIM_A::FLASHTIM6
    }
    #[doc = "Checks if the value of the field is `FLASHTIM7`"]
    #[inline(always)]
    pub fn is_flashtim7(&self) -> bool {
        **self == FLASHTIM_A::FLASHTIM7
    }
    #[doc = "Checks if the value of the field is `FLASHTIM8`"]
    #[inline(always)]
    pub fn is_flashtim8(&self) -> bool {
        **self == FLASHTIM_A::FLASHTIM8
    }
    #[doc = "Checks if the value of the field is `FLASHTIM9`"]
    #[inline(always)]
    pub fn is_flashtim9(&self) -> bool {
        **self == FLASHTIM_A::FLASHTIM9
    }
    #[doc = "Checks if the value of the field is `FLASHTIM10`"]
    #[inline(always)]
    pub fn is_flashtim10(&self) -> bool {
        **self == FLASHTIM_A::FLASHTIM10
    }
    #[doc = "Checks if the value of the field is `FLASHTIM11`"]
    #[inline(always)]
    pub fn is_flashtim11(&self) -> bool {
        **self == FLASHTIM_A::FLASHTIM11
    }
}
impl core::ops::Deref for FLASHTIM_R {
    type Target = crate::FieldReader<u8, FLASHTIM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHTIM` writer - Flash memory access time."]
pub struct FLASHTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHTIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 system clock flash access time (for system clock rates up to 11 MHz)."]
    #[inline(always)]
    pub fn flashtim0(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM0)
    }
    #[doc = "2 system clocks flash access time (for system clock rates up to 22 MHz)."]
    #[inline(always)]
    pub fn flashtim1(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM1)
    }
    #[doc = "3 system clocks flash access time (for system clock rates up to 33 MHz)."]
    #[inline(always)]
    pub fn flashtim2(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM2)
    }
    #[doc = "4 system clocks flash access time (for system clock rates up to 44 MHz)."]
    #[inline(always)]
    pub fn flashtim3(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM3)
    }
    #[doc = "5 system clocks flash access time (for system clock rates up to 55 MHz)."]
    #[inline(always)]
    pub fn flashtim4(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM4)
    }
    #[doc = "6 system clocks flash access time (for system clock rates up to 66 MHz)."]
    #[inline(always)]
    pub fn flashtim5(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM5)
    }
    #[doc = "7 system clocks flash access time (for system clock rates up to 77 MHz)."]
    #[inline(always)]
    pub fn flashtim6(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM6)
    }
    #[doc = "8 system clocks flash access time (for system clock rates up to 88 MHz)."]
    #[inline(always)]
    pub fn flashtim7(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM7)
    }
    #[doc = "9 system clocks flash access time (for system clock rates up to 100 MHz)."]
    #[inline(always)]
    pub fn flashtim8(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM8)
    }
    #[doc = "10 system clocks flash access time (for system clock rates up to 115 MHz)."]
    #[inline(always)]
    pub fn flashtim9(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM9)
    }
    #[doc = "11 system clocks flash access time (for system clock rates up to 130 MHz)."]
    #[inline(always)]
    pub fn flashtim10(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM10)
    }
    #[doc = "12 system clocks flash access time (for system clock rates up to 150 MHz)."]
    #[inline(always)]
    pub fn flashtim11(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Instruction fetch configuration."]
    #[inline(always)]
    pub fn fetchcfg(&self) -> FETCHCFG_R {
        FETCHCFG_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Data read configuration."]
    #[inline(always)]
    pub fn datacfg(&self) -> DATACFG_R {
        DATACFG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Acceleration enable."]
    #[inline(always)]
    pub fn accel(&self) -> ACCEL_R {
        ACCEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Prefetch enable."]
    #[inline(always)]
    pub fn prefen(&self) -> PREFEN_R {
        PREFEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Prefetch override."]
    #[inline(always)]
    pub fn prefovr(&self) -> PREFOVR_R {
        PREFOVR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Flash memory access time."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Instruction fetch configuration."]
    #[inline(always)]
    pub fn fetchcfg(&mut self) -> FETCHCFG_W {
        FETCHCFG_W { w: self }
    }
    #[doc = "Bits 2:3 - Data read configuration."]
    #[inline(always)]
    pub fn datacfg(&mut self) -> DATACFG_W {
        DATACFG_W { w: self }
    }
    #[doc = "Bit 4 - Acceleration enable."]
    #[inline(always)]
    pub fn accel(&mut self) -> ACCEL_W {
        ACCEL_W { w: self }
    }
    #[doc = "Bit 5 - Prefetch enable."]
    #[inline(always)]
    pub fn prefen(&mut self) -> PREFEN_W {
        PREFEN_W { w: self }
    }
    #[doc = "Bit 6 - Prefetch override."]
    #[inline(always)]
    pub fn prefovr(&mut self) -> PREFOVR_W {
        PREFOVR_W { w: self }
    }
    #[doc = "Bits 12:15 - Flash memory access time."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> FLASHTIM_W {
        FLASHTIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmccr](index.html) module"]
pub struct FMCCR_SPEC;
impl crate::RegisterSpec for FMCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmccr::R](R) reader structure"]
impl crate::Readable for FMCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmccr::W](W) writer structure"]
impl crate::Writable for FMCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMCCR to value 0x2000"]
impl crate::Resettable for FMCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
