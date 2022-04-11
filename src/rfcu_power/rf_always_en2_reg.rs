#[doc = "Register `RF_ALWAYS_EN2_REG` reader"]
pub struct R(crate::R<RF_ALWAYS_EN2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ALWAYS_EN2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_ALWAYS_EN2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_ALWAYS_EN2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_ALWAYS_EN2_REG` writer"]
pub struct W(crate::W<RF_ALWAYS_EN2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ALWAYS_EN2_REG_SPEC>;
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
impl From<crate::W<RF_ALWAYS_EN2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_ALWAYS_EN2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALW_EN_SPARE5` reader - "]
pub struct ALW_EN_SPARE5_R(crate::FieldReader<bool, bool>);
impl ALW_EN_SPARE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_SPARE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_SPARE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_SPARE5` writer - "]
pub struct ALW_EN_SPARE5_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_SPARE5_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `ALW_EN_SPARE4` reader - "]
pub struct ALW_EN_SPARE4_R(crate::FieldReader<bool, bool>);
impl ALW_EN_SPARE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_SPARE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_SPARE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_SPARE4` writer - "]
pub struct ALW_EN_SPARE4_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_SPARE4_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `ALW_EN_SPARE3` reader - "]
pub struct ALW_EN_SPARE3_R(crate::FieldReader<bool, bool>);
impl ALW_EN_SPARE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_SPARE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_SPARE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_SPARE3` writer - "]
pub struct ALW_EN_SPARE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_SPARE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `ALW_EN_SPARE2` reader - "]
pub struct ALW_EN_SPARE2_R(crate::FieldReader<bool, bool>);
impl ALW_EN_SPARE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_SPARE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_SPARE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_SPARE2` writer - "]
pub struct ALW_EN_SPARE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_SPARE2_W<'a> {
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
#[doc = "Field `ALW_EN_SPARE1` reader - "]
pub struct ALW_EN_SPARE1_R(crate::FieldReader<bool, bool>);
impl ALW_EN_SPARE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_SPARE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_SPARE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_SPARE1` writer - "]
pub struct ALW_EN_SPARE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_SPARE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `ALW_EN_ADPLL_RDY_FOR_DIV` reader - "]
pub struct ALW_EN_ADPLL_RDY_FOR_DIV_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADPLL_RDY_FOR_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADPLL_RDY_FOR_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADPLL_RDY_FOR_DIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADPLL_RDY_FOR_DIV` writer - "]
pub struct ALW_EN_ADPLL_RDY_FOR_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADPLL_RDY_FOR_DIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `ALW_EN_PHY_RDY4BS` reader - "]
pub struct ALW_EN_PHY_RDY4BS_R(crate::FieldReader<bool, bool>);
impl ALW_EN_PHY_RDY4BS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_PHY_RDY4BS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_PHY_RDY4BS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_PHY_RDY4BS` writer - "]
pub struct ALW_EN_PHY_RDY4BS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_PHY_RDY4BS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `ALW_EN_DEM_SIGDETECT_EN` reader - "]
pub struct ALW_EN_DEM_SIGDETECT_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_DEM_SIGDETECT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_DEM_SIGDETECT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_DEM_SIGDETECT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_DEM_SIGDETECT_EN` writer - "]
pub struct ALW_EN_DEM_SIGDETECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_DEM_SIGDETECT_EN_W<'a> {
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
#[doc = "Field `ALW_EN_DEM_AGC_UNFREEZE_EN` reader - "]
pub struct ALW_EN_DEM_AGC_UNFREEZE_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_DEM_AGC_UNFREEZE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_DEM_AGC_UNFREEZE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_DEM_AGC_UNFREEZE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_DEM_AGC_UNFREEZE_EN` writer - "]
pub struct ALW_EN_DEM_AGC_UNFREEZE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_DEM_AGC_UNFREEZE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `ALW_EN_DEM_DC_PARCAL_EN` reader - "]
pub struct ALW_EN_DEM_DC_PARCAL_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_DEM_DC_PARCAL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_DEM_DC_PARCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_DEM_DC_PARCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_DEM_DC_PARCAL_EN` writer - "]
pub struct ALW_EN_DEM_DC_PARCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_DEM_DC_PARCAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `ALW_EN_DEM_EN` reader - "]
pub struct ALW_EN_DEM_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_DEM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_DEM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_DEM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_DEM_EN` writer - "]
pub struct ALW_EN_DEM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_DEM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `ALW_EN_CAL_EN` reader - "]
pub struct ALW_EN_CAL_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_CAL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_CAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_CAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_CAL_EN` writer - "]
pub struct ALW_EN_CAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_CAL_EN_W<'a> {
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
#[doc = "Field `ALW_EN_ADPLL_LOBUF_PA_EN` reader - "]
pub struct ALW_EN_ADPLL_LOBUF_PA_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADPLL_LOBUF_PA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADPLL_LOBUF_PA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADPLL_LOBUF_PA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADPLL_LOBUF_PA_EN` writer - "]
pub struct ALW_EN_ADPLL_LOBUF_PA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADPLL_LOBUF_PA_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `ALW_EN_ADPLL_PAIN_EN` reader - "]
pub struct ALW_EN_ADPLL_PAIN_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADPLL_PAIN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADPLL_PAIN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADPLL_PAIN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADPLL_PAIN_EN` writer - "]
pub struct ALW_EN_ADPLL_PAIN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADPLL_PAIN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `ALW_EN_ADPLLDIG_RX_EN` reader - "]
pub struct ALW_EN_ADPLLDIG_RX_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADPLLDIG_RX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADPLLDIG_RX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADPLLDIG_RX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADPLLDIG_RX_EN` writer - "]
pub struct ALW_EN_ADPLLDIG_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADPLLDIG_RX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn alw_en_spare5(&self) -> ALW_EN_SPARE5_R {
        ALW_EN_SPARE5_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn alw_en_spare4(&self) -> ALW_EN_SPARE4_R {
        ALW_EN_SPARE4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn alw_en_spare3(&self) -> ALW_EN_SPARE3_R {
        ALW_EN_SPARE3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn alw_en_spare2(&self) -> ALW_EN_SPARE2_R {
        ALW_EN_SPARE2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn alw_en_spare1(&self) -> ALW_EN_SPARE1_R {
        ALW_EN_SPARE1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn alw_en_adpll_rdy_for_div(&self) -> ALW_EN_ADPLL_RDY_FOR_DIV_R {
        ALW_EN_ADPLL_RDY_FOR_DIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn alw_en_phy_rdy4bs(&self) -> ALW_EN_PHY_RDY4BS_R {
        ALW_EN_PHY_RDY4BS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn alw_en_dem_sigdetect_en(&self) -> ALW_EN_DEM_SIGDETECT_EN_R {
        ALW_EN_DEM_SIGDETECT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn alw_en_dem_agc_unfreeze_en(&self) -> ALW_EN_DEM_AGC_UNFREEZE_EN_R {
        ALW_EN_DEM_AGC_UNFREEZE_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn alw_en_dem_dc_parcal_en(&self) -> ALW_EN_DEM_DC_PARCAL_EN_R {
        ALW_EN_DEM_DC_PARCAL_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn alw_en_dem_en(&self) -> ALW_EN_DEM_EN_R {
        ALW_EN_DEM_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn alw_en_cal_en(&self) -> ALW_EN_CAL_EN_R {
        ALW_EN_CAL_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn alw_en_adpll_lobuf_pa_en(&self) -> ALW_EN_ADPLL_LOBUF_PA_EN_R {
        ALW_EN_ADPLL_LOBUF_PA_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn alw_en_adpll_pain_en(&self) -> ALW_EN_ADPLL_PAIN_EN_R {
        ALW_EN_ADPLL_PAIN_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn alw_en_adplldig_rx_en(&self) -> ALW_EN_ADPLLDIG_RX_EN_R {
        ALW_EN_ADPLLDIG_RX_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn alw_en_spare5(&mut self) -> ALW_EN_SPARE5_W {
        ALW_EN_SPARE5_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn alw_en_spare4(&mut self) -> ALW_EN_SPARE4_W {
        ALW_EN_SPARE4_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn alw_en_spare3(&mut self) -> ALW_EN_SPARE3_W {
        ALW_EN_SPARE3_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn alw_en_spare2(&mut self) -> ALW_EN_SPARE2_W {
        ALW_EN_SPARE2_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn alw_en_spare1(&mut self) -> ALW_EN_SPARE1_W {
        ALW_EN_SPARE1_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn alw_en_adpll_rdy_for_div(&mut self) -> ALW_EN_ADPLL_RDY_FOR_DIV_W {
        ALW_EN_ADPLL_RDY_FOR_DIV_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn alw_en_phy_rdy4bs(&mut self) -> ALW_EN_PHY_RDY4BS_W {
        ALW_EN_PHY_RDY4BS_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn alw_en_dem_sigdetect_en(&mut self) -> ALW_EN_DEM_SIGDETECT_EN_W {
        ALW_EN_DEM_SIGDETECT_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn alw_en_dem_agc_unfreeze_en(&mut self) -> ALW_EN_DEM_AGC_UNFREEZE_EN_W {
        ALW_EN_DEM_AGC_UNFREEZE_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn alw_en_dem_dc_parcal_en(&mut self) -> ALW_EN_DEM_DC_PARCAL_EN_W {
        ALW_EN_DEM_DC_PARCAL_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn alw_en_dem_en(&mut self) -> ALW_EN_DEM_EN_W {
        ALW_EN_DEM_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn alw_en_cal_en(&mut self) -> ALW_EN_CAL_EN_W {
        ALW_EN_CAL_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn alw_en_adpll_lobuf_pa_en(&mut self) -> ALW_EN_ADPLL_LOBUF_PA_EN_W {
        ALW_EN_ADPLL_LOBUF_PA_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn alw_en_adpll_pain_en(&mut self) -> ALW_EN_ADPLL_PAIN_EN_W {
        ALW_EN_ADPLL_PAIN_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn alw_en_adplldig_rx_en(&mut self) -> ALW_EN_ADPLLDIG_RX_EN_W {
        ALW_EN_ADPLLDIG_RX_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_always_en2_reg](index.html) module"]
pub struct RF_ALWAYS_EN2_REG_SPEC;
impl crate::RegisterSpec for RF_ALWAYS_EN2_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_always_en2_reg::R](R) reader structure"]
impl crate::Readable for RF_ALWAYS_EN2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_always_en2_reg::W](W) writer structure"]
impl crate::Writable for RF_ALWAYS_EN2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_ALWAYS_EN2_REG to value 0"]
impl crate::Resettable for RF_ALWAYS_EN2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
