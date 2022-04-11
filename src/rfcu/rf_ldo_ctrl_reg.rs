#[doc = "Register `RF_LDO_CTRL_REG` reader"]
pub struct R(crate::R<RF_LDO_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_LDO_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_LDO_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_LDO_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_LDO_CTRL_REG` writer"]
pub struct W(crate::W<RF_LDO_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_LDO_CTRL_REG_SPEC>;
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
impl From<crate::W<RF_LDO_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_LDO_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDO_DCO_HOLD_OVR_EN` reader - "]
pub struct LDO_DCO_HOLD_OVR_EN_R(crate::FieldReader<bool, bool>);
impl LDO_DCO_HOLD_OVR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_DCO_HOLD_OVR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_DCO_HOLD_OVR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_DCO_HOLD_OVR_EN` writer - "]
pub struct LDO_DCO_HOLD_OVR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_DCO_HOLD_OVR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `LDO_DCO_HOLD_OVR_VAL` reader - "]
pub struct LDO_DCO_HOLD_OVR_VAL_R(crate::FieldReader<bool, bool>);
impl LDO_DCO_HOLD_OVR_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_DCO_HOLD_OVR_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_DCO_HOLD_OVR_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_DCO_HOLD_OVR_VAL` writer - "]
pub struct LDO_DCO_HOLD_OVR_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_DCO_HOLD_OVR_VAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `LDO_DTC_HOLD_OVR_EN` reader - "]
pub struct LDO_DTC_HOLD_OVR_EN_R(crate::FieldReader<bool, bool>);
impl LDO_DTC_HOLD_OVR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_DTC_HOLD_OVR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_DTC_HOLD_OVR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_DTC_HOLD_OVR_EN` writer - "]
pub struct LDO_DTC_HOLD_OVR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_DTC_HOLD_OVR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `LDO_DTC_HOLD_OVR_VAL` reader - "]
pub struct LDO_DTC_HOLD_OVR_VAL_R(crate::FieldReader<bool, bool>);
impl LDO_DTC_HOLD_OVR_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_DTC_HOLD_OVR_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_DTC_HOLD_OVR_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_DTC_HOLD_OVR_VAL` writer - "]
pub struct LDO_DTC_HOLD_OVR_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_DTC_HOLD_OVR_VAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `LDO_RADIO_HOLD_OVR_EN` reader - "]
pub struct LDO_RADIO_HOLD_OVR_EN_R(crate::FieldReader<bool, bool>);
impl LDO_RADIO_HOLD_OVR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_RADIO_HOLD_OVR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_RADIO_HOLD_OVR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_RADIO_HOLD_OVR_EN` writer - "]
pub struct LDO_RADIO_HOLD_OVR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_RADIO_HOLD_OVR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Field `LDO_RADIO_HOLD_OVR_VAL` reader - "]
pub struct LDO_RADIO_HOLD_OVR_VAL_R(crate::FieldReader<bool, bool>);
impl LDO_RADIO_HOLD_OVR_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_RADIO_HOLD_OVR_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_RADIO_HOLD_OVR_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_RADIO_HOLD_OVR_VAL` writer - "]
pub struct LDO_RADIO_HOLD_OVR_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_RADIO_HOLD_OVR_VAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `LDO_VREF_SMPL_TIME` reader - "]
pub struct LDO_VREF_SMPL_TIME_R(crate::FieldReader<u8, u8>);
impl LDO_VREF_SMPL_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LDO_VREF_SMPL_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_VREF_SMPL_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_VREF_SMPL_TIME` writer - "]
pub struct LDO_VREF_SMPL_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_VREF_SMPL_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `LDO_DCO_CONT_ENABLE` reader - "]
pub struct LDO_DCO_CONT_ENABLE_R(crate::FieldReader<bool, bool>);
impl LDO_DCO_CONT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_DCO_CONT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_DCO_CONT_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_DCO_CONT_ENABLE` writer - "]
pub struct LDO_DCO_CONT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_DCO_CONT_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `LDO_DCO_LEVEL` reader - "]
pub struct LDO_DCO_LEVEL_R(crate::FieldReader<u8, u8>);
impl LDO_DCO_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LDO_DCO_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_DCO_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_DCO_LEVEL` writer - "]
pub struct LDO_DCO_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_DCO_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 8)) | ((value as u32 & 7) << 8);
        self.w
    }
}
#[doc = "Field `LDO_DTC_CONT_ENABLE` reader - "]
pub struct LDO_DTC_CONT_ENABLE_R(crate::FieldReader<bool, bool>);
impl LDO_DTC_CONT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_DTC_CONT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_DTC_CONT_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_DTC_CONT_ENABLE` writer - "]
pub struct LDO_DTC_CONT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_DTC_CONT_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `LDO_DTC_LEVEL` reader - "]
pub struct LDO_DTC_LEVEL_R(crate::FieldReader<u8, u8>);
impl LDO_DTC_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LDO_DTC_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_DTC_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_DTC_LEVEL` writer - "]
pub struct LDO_DTC_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_DTC_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u32 & 7) << 4);
        self.w
    }
}
#[doc = "Field `LDO_RADIO_CONT_ENABLE` reader - "]
pub struct LDO_RADIO_CONT_ENABLE_R(crate::FieldReader<bool, bool>);
impl LDO_RADIO_CONT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_RADIO_CONT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_RADIO_CONT_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_RADIO_CONT_ENABLE` writer - "]
pub struct LDO_RADIO_CONT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_RADIO_CONT_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `LDO_RADIO_LEVEL` reader - "]
pub struct LDO_RADIO_LEVEL_R(crate::FieldReader<u8, u8>);
impl LDO_RADIO_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LDO_RADIO_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_RADIO_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_RADIO_LEVEL` writer - "]
pub struct LDO_RADIO_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_RADIO_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ldo_dco_hold_ovr_en(&self) -> LDO_DCO_HOLD_OVR_EN_R {
        LDO_DCO_HOLD_OVR_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo_dco_hold_ovr_val(&self) -> LDO_DCO_HOLD_OVR_VAL_R {
        LDO_DCO_HOLD_OVR_VAL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ldo_dtc_hold_ovr_en(&self) -> LDO_DTC_HOLD_OVR_EN_R {
        LDO_DTC_HOLD_OVR_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ldo_dtc_hold_ovr_val(&self) -> LDO_DTC_HOLD_OVR_VAL_R {
        LDO_DTC_HOLD_OVR_VAL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ldo_radio_hold_ovr_en(&self) -> LDO_RADIO_HOLD_OVR_EN_R {
        LDO_RADIO_HOLD_OVR_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ldo_radio_hold_ovr_val(&self) -> LDO_RADIO_HOLD_OVR_VAL_R {
        LDO_RADIO_HOLD_OVR_VAL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn ldo_vref_smpl_time(&self) -> LDO_VREF_SMPL_TIME_R {
        LDO_VREF_SMPL_TIME_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo_dco_cont_enable(&self) -> LDO_DCO_CONT_ENABLE_R {
        LDO_DCO_CONT_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo_dco_level(&self) -> LDO_DCO_LEVEL_R {
        LDO_DCO_LEVEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ldo_dtc_cont_enable(&self) -> LDO_DTC_CONT_ENABLE_R {
        LDO_DTC_CONT_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo_dtc_level(&self) -> LDO_DTC_LEVEL_R {
        LDO_DTC_LEVEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo_radio_cont_enable(&self) -> LDO_RADIO_CONT_ENABLE_R {
        LDO_RADIO_CONT_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn ldo_radio_level(&self) -> LDO_RADIO_LEVEL_R {
        LDO_RADIO_LEVEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ldo_dco_hold_ovr_en(&mut self) -> LDO_DCO_HOLD_OVR_EN_W {
        LDO_DCO_HOLD_OVR_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo_dco_hold_ovr_val(&mut self) -> LDO_DCO_HOLD_OVR_VAL_W {
        LDO_DCO_HOLD_OVR_VAL_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ldo_dtc_hold_ovr_en(&mut self) -> LDO_DTC_HOLD_OVR_EN_W {
        LDO_DTC_HOLD_OVR_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ldo_dtc_hold_ovr_val(&mut self) -> LDO_DTC_HOLD_OVR_VAL_W {
        LDO_DTC_HOLD_OVR_VAL_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ldo_radio_hold_ovr_en(&mut self) -> LDO_RADIO_HOLD_OVR_EN_W {
        LDO_RADIO_HOLD_OVR_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ldo_radio_hold_ovr_val(&mut self) -> LDO_RADIO_HOLD_OVR_VAL_W {
        LDO_RADIO_HOLD_OVR_VAL_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn ldo_vref_smpl_time(&mut self) -> LDO_VREF_SMPL_TIME_W {
        LDO_VREF_SMPL_TIME_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo_dco_cont_enable(&mut self) -> LDO_DCO_CONT_ENABLE_W {
        LDO_DCO_CONT_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo_dco_level(&mut self) -> LDO_DCO_LEVEL_W {
        LDO_DCO_LEVEL_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ldo_dtc_cont_enable(&mut self) -> LDO_DTC_CONT_ENABLE_W {
        LDO_DTC_CONT_ENABLE_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo_dtc_level(&mut self) -> LDO_DTC_LEVEL_W {
        LDO_DTC_LEVEL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo_radio_cont_enable(&mut self) -> LDO_RADIO_CONT_ENABLE_W {
        LDO_RADIO_CONT_ENABLE_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn ldo_radio_level(&mut self) -> LDO_RADIO_LEVEL_W {
        LDO_RADIO_LEVEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ldo_ctrl_reg](index.html) module"]
pub struct RF_LDO_CTRL_REG_SPEC;
impl crate::RegisterSpec for RF_LDO_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_ldo_ctrl_reg::R](R) reader structure"]
impl crate::Readable for RF_LDO_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_ldo_ctrl_reg::W](W) writer structure"]
impl crate::Writable for RF_LDO_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_LDO_CTRL_REG to value 0x0019_0333"]
impl crate::Resettable for RF_LDO_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0019_0333
    }
}
