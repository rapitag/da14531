#[doc = "Register `POWER_CTRL_REG` reader"]
pub struct R(crate::R<POWER_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_CTRL_REG` writer"]
pub struct W(crate::W<POWER_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_CTRL_REG_SPEC>;
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
impl From<crate::W<POWER_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBAT_HL_CONNECT_MODE` reader - Sets the control mode fo the switch between VBAT_HIGH and VBAT_LOW 0: Manual (default) 1: Automatic (boost mode only)"]
pub struct VBAT_HL_CONNECT_MODE_R(crate::FieldReader<bool, bool>);
impl VBAT_HL_CONNECT_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBAT_HL_CONNECT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBAT_HL_CONNECT_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBAT_HL_CONNECT_MODE` writer - Sets the control mode fo the switch between VBAT_HIGH and VBAT_LOW 0: Manual (default) 1: Automatic (boost mode only)"]
pub struct VBAT_HL_CONNECT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBAT_HL_CONNECT_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u16 & 1) << 15);
        self.w
    }
}
#[doc = "Field `POR_VBAT_HIGH_HYST_DIS` reader - "]
pub struct POR_VBAT_HIGH_HYST_DIS_R(crate::FieldReader<bool, bool>);
impl POR_VBAT_HIGH_HYST_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_VBAT_HIGH_HYST_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_VBAT_HIGH_HYST_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_VBAT_HIGH_HYST_DIS` writer - "]
pub struct POR_VBAT_HIGH_HYST_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_VBAT_HIGH_HYST_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u16 & 1) << 14);
        self.w
    }
}
#[doc = "Field `POR_VBAT_HIGH_HYST_SEL` reader - "]
pub struct POR_VBAT_HIGH_HYST_SEL_R(crate::FieldReader<bool, bool>);
impl POR_VBAT_HIGH_HYST_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_VBAT_HIGH_HYST_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_VBAT_HIGH_HYST_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_VBAT_HIGH_HYST_SEL` writer - "]
pub struct POR_VBAT_HIGH_HYST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_VBAT_HIGH_HYST_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u16 & 1) << 13);
        self.w
    }
}
#[doc = "Field `POR_VBAT_HIGH_DISABLE` reader - Disable por_vbat_high circuit"]
pub struct POR_VBAT_HIGH_DISABLE_R(crate::FieldReader<bool, bool>);
impl POR_VBAT_HIGH_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_VBAT_HIGH_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_VBAT_HIGH_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_VBAT_HIGH_DISABLE` writer - Disable por_vbat_high circuit"]
pub struct POR_VBAT_HIGH_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_VBAT_HIGH_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u16 & 1) << 12);
        self.w
    }
}
#[doc = "Field `POR_VBAT_LOW_HYST_DIS` reader - "]
pub struct POR_VBAT_LOW_HYST_DIS_R(crate::FieldReader<bool, bool>);
impl POR_VBAT_LOW_HYST_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_VBAT_LOW_HYST_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_VBAT_LOW_HYST_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_VBAT_LOW_HYST_DIS` writer - "]
pub struct POR_VBAT_LOW_HYST_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_VBAT_LOW_HYST_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u16 & 1) << 11);
        self.w
    }
}
#[doc = "Field `POR_VBAT_LOW_HYST_SEL` reader - "]
pub struct POR_VBAT_LOW_HYST_SEL_R(crate::FieldReader<bool, bool>);
impl POR_VBAT_LOW_HYST_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_VBAT_LOW_HYST_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_VBAT_LOW_HYST_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_VBAT_LOW_HYST_SEL` writer - "]
pub struct POR_VBAT_LOW_HYST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_VBAT_LOW_HYST_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u16 & 1) << 10);
        self.w
    }
}
#[doc = "Field `POR_VBAT_LOW_DISABLE` reader - Disable por_vbat_low circuit"]
pub struct POR_VBAT_LOW_DISABLE_R(crate::FieldReader<bool, bool>);
impl POR_VBAT_LOW_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_VBAT_LOW_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_VBAT_LOW_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR_VBAT_LOW_DISABLE` writer - Disable por_vbat_low circuit"]
pub struct POR_VBAT_LOW_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_VBAT_LOW_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u16 & 1) << 9);
        self.w
    }
}
#[doc = "Field `CP_DISABLE` reader - Disables LDO_CORE charge-pump circuit"]
pub struct CP_DISABLE_R(crate::FieldReader<bool, bool>);
impl CP_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CP_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CP_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CP_DISABLE` writer - Disables LDO_CORE charge-pump circuit"]
pub struct CP_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CP_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u16 & 1) << 8);
        self.w
    }
}
#[doc = "Field `LDO_VREF_HOLD_FORCE` reader - Forces LDO references in HOLD mode"]
pub struct LDO_VREF_HOLD_FORCE_R(crate::FieldReader<bool, bool>);
impl LDO_VREF_HOLD_FORCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_VREF_HOLD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_VREF_HOLD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_VREF_HOLD_FORCE` writer - Forces LDO references in HOLD mode"]
pub struct LDO_VREF_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_VREF_HOLD_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u16 & 1) << 7);
        self.w
    }
}
#[doc = "Field `LDO_LOW_CTRL_REG` reader - 00: High-current mode in active, LDO_LOW OFF in sleep 01: LDO_LOW OFF 10: Low-current mode in active, Low-current mode in sleep 11: High-current mode in active, Low-current mode in sleep"]
pub struct LDO_LOW_CTRL_REG_R(crate::FieldReader<u8, u8>);
impl LDO_LOW_CTRL_REG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LDO_LOW_CTRL_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_LOW_CTRL_REG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_LOW_CTRL_REG` writer - 00: High-current mode in active, LDO_LOW OFF in sleep 01: LDO_LOW OFF 10: Low-current mode in active, Low-current mode in sleep 11: High-current mode in active, Low-current mode in sleep"]
pub struct LDO_LOW_CTRL_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_LOW_CTRL_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 5)) | ((value as u16 & 3) << 5);
        self.w
    }
}
#[doc = "Field `LDO_CORE_DISABLE` reader - Disables LDO_CORE"]
pub struct LDO_CORE_DISABLE_R(crate::FieldReader<bool, bool>);
impl LDO_CORE_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_CORE_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_CORE_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_CORE_DISABLE` writer - Disables LDO_CORE"]
pub struct LDO_CORE_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_CORE_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u16 & 1) << 4);
        self.w
    }
}
#[doc = "Field `LDO_CORE_RET_ENABLE` reader - LDO_CORE_RETENTION 0: Disabled 1: Enabled"]
pub struct LDO_CORE_RET_ENABLE_R(crate::FieldReader<bool, bool>);
impl LDO_CORE_RET_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_CORE_RET_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_CORE_RET_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_CORE_RET_ENABLE` writer - LDO_CORE_RETENTION 0: Disabled 1: Enabled"]
pub struct LDO_CORE_RET_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_CORE_RET_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u16 & 1) << 3);
        self.w
    }
}
#[doc = "Field `VBAT_HL_CONNECT` reader - Switch between VBAT_HIGH and VBAT_LOW 0: Open 1: Closed"]
pub struct VBAT_HL_CONNECT_R(crate::FieldReader<bool, bool>);
impl VBAT_HL_CONNECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBAT_HL_CONNECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBAT_HL_CONNECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBAT_HL_CONNECT` writer - Switch between VBAT_HIGH and VBAT_LOW 0: Open 1: Closed"]
pub struct VBAT_HL_CONNECT_W<'a> {
    w: &'a mut W,
}
impl<'a> VBAT_HL_CONNECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u16 & 1) << 2);
        self.w
    }
}
#[doc = "Field `CMP_VBAT_HIGH_OK_ENABLE` reader - Enable cmp_vbat_high_ok"]
pub struct CMP_VBAT_HIGH_OK_ENABLE_R(crate::FieldReader<bool, bool>);
impl CMP_VBAT_HIGH_OK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP_VBAT_HIGH_OK_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_VBAT_HIGH_OK_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_VBAT_HIGH_OK_ENABLE` writer - Enable cmp_vbat_high_ok"]
pub struct CMP_VBAT_HIGH_OK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_VBAT_HIGH_OK_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u16 & 1) << 1);
        self.w
    }
}
#[doc = "Field `CMP_VBAT_HIGH_NOK_ENABLE` reader - Enable cmp_vbat_high_nok"]
pub struct CMP_VBAT_HIGH_NOK_ENABLE_R(crate::FieldReader<bool, bool>);
impl CMP_VBAT_HIGH_NOK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMP_VBAT_HIGH_NOK_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_VBAT_HIGH_NOK_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_VBAT_HIGH_NOK_ENABLE` writer - Enable cmp_vbat_high_nok"]
pub struct CMP_VBAT_HIGH_NOK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_VBAT_HIGH_NOK_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u16 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Sets the control mode fo the switch between VBAT_HIGH and VBAT_LOW 0: Manual (default) 1: Automatic (boost mode only)"]
    #[inline(always)]
    pub fn vbat_hl_connect_mode(&self) -> VBAT_HL_CONNECT_MODE_R {
        VBAT_HL_CONNECT_MODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn por_vbat_high_hyst_dis(&self) -> POR_VBAT_HIGH_HYST_DIS_R {
        POR_VBAT_HIGH_HYST_DIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn por_vbat_high_hyst_sel(&self) -> POR_VBAT_HIGH_HYST_SEL_R {
        POR_VBAT_HIGH_HYST_SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Disable por_vbat_high circuit"]
    #[inline(always)]
    pub fn por_vbat_high_disable(&self) -> POR_VBAT_HIGH_DISABLE_R {
        POR_VBAT_HIGH_DISABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn por_vbat_low_hyst_dis(&self) -> POR_VBAT_LOW_HYST_DIS_R {
        POR_VBAT_LOW_HYST_DIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn por_vbat_low_hyst_sel(&self) -> POR_VBAT_LOW_HYST_SEL_R {
        POR_VBAT_LOW_HYST_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable por_vbat_low circuit"]
    #[inline(always)]
    pub fn por_vbat_low_disable(&self) -> POR_VBAT_LOW_DISABLE_R {
        POR_VBAT_LOW_DISABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Disables LDO_CORE charge-pump circuit"]
    #[inline(always)]
    pub fn cp_disable(&self) -> CP_DISABLE_R {
        CP_DISABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Forces LDO references in HOLD mode"]
    #[inline(always)]
    pub fn ldo_vref_hold_force(&self) -> LDO_VREF_HOLD_FORCE_R {
        LDO_VREF_HOLD_FORCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 00: High-current mode in active, LDO_LOW OFF in sleep 01: LDO_LOW OFF 10: Low-current mode in active, Low-current mode in sleep 11: High-current mode in active, Low-current mode in sleep"]
    #[inline(always)]
    pub fn ldo_low_ctrl_reg(&self) -> LDO_LOW_CTRL_REG_R {
        LDO_LOW_CTRL_REG_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 4 - Disables LDO_CORE"]
    #[inline(always)]
    pub fn ldo_core_disable(&self) -> LDO_CORE_DISABLE_R {
        LDO_CORE_DISABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - LDO_CORE_RETENTION 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn ldo_core_ret_enable(&self) -> LDO_CORE_RET_ENABLE_R {
        LDO_CORE_RET_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Switch between VBAT_HIGH and VBAT_LOW 0: Open 1: Closed"]
    #[inline(always)]
    pub fn vbat_hl_connect(&self) -> VBAT_HL_CONNECT_R {
        VBAT_HL_CONNECT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Enable cmp_vbat_high_ok"]
    #[inline(always)]
    pub fn cmp_vbat_high_ok_enable(&self) -> CMP_VBAT_HIGH_OK_ENABLE_R {
        CMP_VBAT_HIGH_OK_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable cmp_vbat_high_nok"]
    #[inline(always)]
    pub fn cmp_vbat_high_nok_enable(&self) -> CMP_VBAT_HIGH_NOK_ENABLE_R {
        CMP_VBAT_HIGH_NOK_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Sets the control mode fo the switch between VBAT_HIGH and VBAT_LOW 0: Manual (default) 1: Automatic (boost mode only)"]
    #[inline(always)]
    pub fn vbat_hl_connect_mode(&mut self) -> VBAT_HL_CONNECT_MODE_W {
        VBAT_HL_CONNECT_MODE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn por_vbat_high_hyst_dis(&mut self) -> POR_VBAT_HIGH_HYST_DIS_W {
        POR_VBAT_HIGH_HYST_DIS_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn por_vbat_high_hyst_sel(&mut self) -> POR_VBAT_HIGH_HYST_SEL_W {
        POR_VBAT_HIGH_HYST_SEL_W { w: self }
    }
    #[doc = "Bit 12 - Disable por_vbat_high circuit"]
    #[inline(always)]
    pub fn por_vbat_high_disable(&mut self) -> POR_VBAT_HIGH_DISABLE_W {
        POR_VBAT_HIGH_DISABLE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn por_vbat_low_hyst_dis(&mut self) -> POR_VBAT_LOW_HYST_DIS_W {
        POR_VBAT_LOW_HYST_DIS_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn por_vbat_low_hyst_sel(&mut self) -> POR_VBAT_LOW_HYST_SEL_W {
        POR_VBAT_LOW_HYST_SEL_W { w: self }
    }
    #[doc = "Bit 9 - Disable por_vbat_low circuit"]
    #[inline(always)]
    pub fn por_vbat_low_disable(&mut self) -> POR_VBAT_LOW_DISABLE_W {
        POR_VBAT_LOW_DISABLE_W { w: self }
    }
    #[doc = "Bit 8 - Disables LDO_CORE charge-pump circuit"]
    #[inline(always)]
    pub fn cp_disable(&mut self) -> CP_DISABLE_W {
        CP_DISABLE_W { w: self }
    }
    #[doc = "Bit 7 - Forces LDO references in HOLD mode"]
    #[inline(always)]
    pub fn ldo_vref_hold_force(&mut self) -> LDO_VREF_HOLD_FORCE_W {
        LDO_VREF_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bits 5:6 - 00: High-current mode in active, LDO_LOW OFF in sleep 01: LDO_LOW OFF 10: Low-current mode in active, Low-current mode in sleep 11: High-current mode in active, Low-current mode in sleep"]
    #[inline(always)]
    pub fn ldo_low_ctrl_reg(&mut self) -> LDO_LOW_CTRL_REG_W {
        LDO_LOW_CTRL_REG_W { w: self }
    }
    #[doc = "Bit 4 - Disables LDO_CORE"]
    #[inline(always)]
    pub fn ldo_core_disable(&mut self) -> LDO_CORE_DISABLE_W {
        LDO_CORE_DISABLE_W { w: self }
    }
    #[doc = "Bit 3 - LDO_CORE_RETENTION 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn ldo_core_ret_enable(&mut self) -> LDO_CORE_RET_ENABLE_W {
        LDO_CORE_RET_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Switch between VBAT_HIGH and VBAT_LOW 0: Open 1: Closed"]
    #[inline(always)]
    pub fn vbat_hl_connect(&mut self) -> VBAT_HL_CONNECT_W {
        VBAT_HL_CONNECT_W { w: self }
    }
    #[doc = "Bit 1 - Enable cmp_vbat_high_ok"]
    #[inline(always)]
    pub fn cmp_vbat_high_ok_enable(&mut self) -> CMP_VBAT_HIGH_OK_ENABLE_W {
        CMP_VBAT_HIGH_OK_ENABLE_W { w: self }
    }
    #[doc = "Bit 0 - Enable cmp_vbat_high_nok"]
    #[inline(always)]
    pub fn cmp_vbat_high_nok_enable(&mut self) -> CMP_VBAT_HIGH_NOK_ENABLE_W {
        CMP_VBAT_HIGH_NOK_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power management control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_ctrl_reg](index.html) module"]
pub struct POWER_CTRL_REG_SPEC;
impl crate::RegisterSpec for POWER_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [power_ctrl_reg::R](R) reader structure"]
impl crate::Readable for POWER_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_ctrl_reg::W](W) writer structure"]
impl crate::Writable for POWER_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWER_CTRL_REG to value 0x4000"]
impl crate::Resettable for POWER_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000
    }
}
