#[doc = "Register `RF_RADIO_INIT_REG` reader"]
pub struct R(crate::R<RF_RADIO_INIT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_RADIO_INIT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_RADIO_INIT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_RADIO_INIT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_RADIO_INIT_REG` writer"]
pub struct W(crate::W<RF_RADIO_INIT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_RADIO_INIT_REG_SPEC>;
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
impl From<crate::W<RF_RADIO_INIT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_RADIO_INIT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RADIO_INIT_AUTOCLEAR` reader - "]
pub struct RADIO_INIT_AUTOCLEAR_R(crate::FieldReader<bool, bool>);
impl RADIO_INIT_AUTOCLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RADIO_INIT_AUTOCLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADIO_INIT_AUTOCLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADIO_INIT_AUTOCLEAR` writer - "]
pub struct RADIO_INIT_AUTOCLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RADIO_INIT_AUTOCLEAR_W<'a> {
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
#[doc = "Field `ADPLLDIG_HCLK_DIS` reader - "]
pub struct ADPLLDIG_HCLK_DIS_R(crate::FieldReader<bool, bool>);
impl ADPLLDIG_HCLK_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLLDIG_HCLK_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLLDIG_HCLK_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLLDIG_HCLK_DIS` writer - "]
pub struct ADPLLDIG_HCLK_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLLDIG_HCLK_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `RADIO_REGS_RDY` reader - "]
pub struct RADIO_REGS_RDY_R(crate::FieldReader<bool, bool>);
impl RADIO_REGS_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RADIO_REGS_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADIO_REGS_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADIO_REGS_RDY` writer - "]
pub struct RADIO_REGS_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RADIO_REGS_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `ADPLLDIG_HCLK_EN` reader - "]
pub struct ADPLLDIG_HCLK_EN_R(crate::FieldReader<bool, bool>);
impl ADPLLDIG_HCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLLDIG_HCLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLLDIG_HCLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLLDIG_HCLK_EN` writer - "]
pub struct ADPLLDIG_HCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLLDIG_HCLK_EN_W<'a> {
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
#[doc = "Field `ADPLLDIG_HRESET_N` reader - "]
pub struct ADPLLDIG_HRESET_N_R(crate::FieldReader<bool, bool>);
impl ADPLLDIG_HRESET_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLLDIG_HRESET_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLLDIG_HRESET_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLLDIG_HRESET_N` writer - "]
pub struct ADPLLDIG_HRESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLLDIG_HRESET_N_W<'a> {
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
#[doc = "Field `ADPLLDIG_LDO_EN_WR` reader - "]
pub struct ADPLLDIG_LDO_EN_WR_R(crate::FieldReader<bool, bool>);
impl ADPLLDIG_LDO_EN_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLLDIG_LDO_EN_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLLDIG_LDO_EN_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLLDIG_LDO_EN_WR` writer - "]
pub struct ADPLLDIG_LDO_EN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLLDIG_LDO_EN_WR_W<'a> {
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
#[doc = "Field `ADPLLDIG_LDO_EN_SEL` reader - "]
pub struct ADPLLDIG_LDO_EN_SEL_R(crate::FieldReader<bool, bool>);
impl ADPLLDIG_LDO_EN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLLDIG_LDO_EN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLLDIG_LDO_EN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLLDIG_LDO_EN_SEL` writer - "]
pub struct ADPLLDIG_LDO_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLLDIG_LDO_EN_SEL_W<'a> {
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
#[doc = "Field `ADPLLDIG_PWR_SW1_EN` reader - "]
pub struct ADPLLDIG_PWR_SW1_EN_R(crate::FieldReader<bool, bool>);
impl ADPLLDIG_PWR_SW1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLLDIG_PWR_SW1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLLDIG_PWR_SW1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLLDIG_PWR_SW1_EN` writer - "]
pub struct ADPLLDIG_PWR_SW1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLLDIG_PWR_SW1_EN_W<'a> {
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
#[doc = "Field `RADIO_LDO_EN_WR` reader - "]
pub struct RADIO_LDO_EN_WR_R(crate::FieldReader<bool, bool>);
impl RADIO_LDO_EN_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RADIO_LDO_EN_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADIO_LDO_EN_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADIO_LDO_EN_WR` writer - "]
pub struct RADIO_LDO_EN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> RADIO_LDO_EN_WR_W<'a> {
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
#[doc = "Field `RADIO_LDO_EN_SEL` reader - "]
pub struct RADIO_LDO_EN_SEL_R(crate::FieldReader<bool, bool>);
impl RADIO_LDO_EN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RADIO_LDO_EN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADIO_LDO_EN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADIO_LDO_EN_SEL` writer - "]
pub struct RADIO_LDO_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RADIO_LDO_EN_SEL_W<'a> {
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
#[doc = "Field `RADIO_LDO_EN` reader - "]
pub struct RADIO_LDO_EN_R(crate::FieldReader<bool, bool>);
impl RADIO_LDO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RADIO_LDO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADIO_LDO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADIO_LDO_EN` writer - "]
pub struct RADIO_LDO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RADIO_LDO_EN_W<'a> {
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
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn radio_init_autoclear(&self) -> RADIO_INIT_AUTOCLEAR_R {
        RADIO_INIT_AUTOCLEAR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn adplldig_hclk_dis(&self) -> ADPLLDIG_HCLK_DIS_R {
        ADPLLDIG_HCLK_DIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn radio_regs_rdy(&self) -> RADIO_REGS_RDY_R {
        RADIO_REGS_RDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adplldig_hclk_en(&self) -> ADPLLDIG_HCLK_EN_R {
        ADPLLDIG_HCLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn adplldig_hreset_n(&self) -> ADPLLDIG_HRESET_N_R {
        ADPLLDIG_HRESET_N_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adplldig_ldo_en_wr(&self) -> ADPLLDIG_LDO_EN_WR_R {
        ADPLLDIG_LDO_EN_WR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adplldig_ldo_en_sel(&self) -> ADPLLDIG_LDO_EN_SEL_R {
        ADPLLDIG_LDO_EN_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn adplldig_pwr_sw1_en(&self) -> ADPLLDIG_PWR_SW1_EN_R {
        ADPLLDIG_PWR_SW1_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn radio_ldo_en_wr(&self) -> RADIO_LDO_EN_WR_R {
        RADIO_LDO_EN_WR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn radio_ldo_en_sel(&self) -> RADIO_LDO_EN_SEL_R {
        RADIO_LDO_EN_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn radio_ldo_en(&self) -> RADIO_LDO_EN_R {
        RADIO_LDO_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn radio_init_autoclear(&mut self) -> RADIO_INIT_AUTOCLEAR_W {
        RADIO_INIT_AUTOCLEAR_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn adplldig_hclk_dis(&mut self) -> ADPLLDIG_HCLK_DIS_W {
        ADPLLDIG_HCLK_DIS_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn radio_regs_rdy(&mut self) -> RADIO_REGS_RDY_W {
        RADIO_REGS_RDY_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adplldig_hclk_en(&mut self) -> ADPLLDIG_HCLK_EN_W {
        ADPLLDIG_HCLK_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn adplldig_hreset_n(&mut self) -> ADPLLDIG_HRESET_N_W {
        ADPLLDIG_HRESET_N_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adplldig_ldo_en_wr(&mut self) -> ADPLLDIG_LDO_EN_WR_W {
        ADPLLDIG_LDO_EN_WR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adplldig_ldo_en_sel(&mut self) -> ADPLLDIG_LDO_EN_SEL_W {
        ADPLLDIG_LDO_EN_SEL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn adplldig_pwr_sw1_en(&mut self) -> ADPLLDIG_PWR_SW1_EN_W {
        ADPLLDIG_PWR_SW1_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn radio_ldo_en_wr(&mut self) -> RADIO_LDO_EN_WR_W {
        RADIO_LDO_EN_WR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn radio_ldo_en_sel(&mut self) -> RADIO_LDO_EN_SEL_W {
        RADIO_LDO_EN_SEL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn radio_ldo_en(&mut self) -> RADIO_LDO_EN_W {
        RADIO_LDO_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_radio_init_reg](index.html) module"]
pub struct RF_RADIO_INIT_REG_SPEC;
impl crate::RegisterSpec for RF_RADIO_INIT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_radio_init_reg::R](R) reader structure"]
impl crate::Readable for RF_RADIO_INIT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_radio_init_reg::W](W) writer structure"]
impl crate::Writable for RF_RADIO_INIT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_RADIO_INIT_REG to value 0"]
impl crate::Resettable for RF_RADIO_INIT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
