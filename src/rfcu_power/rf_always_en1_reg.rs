#[doc = "Register `RF_ALWAYS_EN1_REG` reader"]
pub struct R(crate::R<RF_ALWAYS_EN1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ALWAYS_EN1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_ALWAYS_EN1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_ALWAYS_EN1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_ALWAYS_EN1_REG` writer"]
pub struct W(crate::W<RF_ALWAYS_EN1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ALWAYS_EN1_REG_SPEC>;
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
impl From<crate::W<RF_ALWAYS_EN1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_ALWAYS_EN1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALW_EN_ADPLLDIG_EN` reader - "]
pub struct ALW_EN_ADPLLDIG_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADPLLDIG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADPLLDIG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADPLLDIG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADPLLDIG_EN` writer - "]
pub struct ALW_EN_ADPLLDIG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADPLLDIG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "Field `ALW_EN_ADPLLDIG_RST` reader - "]
pub struct ALW_EN_ADPLLDIG_RST_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADPLLDIG_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADPLLDIG_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADPLLDIG_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADPLLDIG_RST` writer - "]
pub struct ALW_EN_ADPLLDIG_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADPLLDIG_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `ALW_EN_ADPLL_CLK_EN` reader - "]
pub struct ALW_EN_ADPLL_CLK_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADPLL_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADPLL_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADPLL_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADPLL_CLK_EN` writer - "]
pub struct ALW_EN_ADPLL_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADPLL_CLK_EN_W<'a> {
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
#[doc = "Field `ALW_EN_ADPLL_DCO_EN` reader - "]
pub struct ALW_EN_ADPLL_DCO_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADPLL_DCO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADPLL_DCO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADPLL_DCO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADPLL_DCO_EN` writer - "]
pub struct ALW_EN_ADPLL_DCO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADPLL_DCO_EN_W<'a> {
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
#[doc = "Field `ALW_EN_ADC_EN` reader - "]
pub struct ALW_EN_ADC_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADC_EN` writer - "]
pub struct ALW_EN_ADC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADC_EN_W<'a> {
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
#[doc = "Field `ALW_EN_ADC_CLK_EN` reader - "]
pub struct ALW_EN_ADC_CLK_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADC_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADC_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADC_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADC_CLK_EN` writer - "]
pub struct ALW_EN_ADC_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADC_CLK_EN_W<'a> {
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
#[doc = "Field `ALW_EN_IFF_BIAS_SH_OPEN` reader - "]
pub struct ALW_EN_IFF_BIAS_SH_OPEN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_IFF_BIAS_SH_OPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_IFF_BIAS_SH_OPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_IFF_BIAS_SH_OPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_IFF_BIAS_SH_OPEN` writer - "]
pub struct ALW_EN_IFF_BIAS_SH_OPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_IFF_BIAS_SH_OPEN_W<'a> {
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
#[doc = "Field `ALW_EN_IFF_EN` reader - "]
pub struct ALW_EN_IFF_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_IFF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_IFF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_IFF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_IFF_EN` writer - "]
pub struct ALW_EN_IFF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_IFF_EN_W<'a> {
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
#[doc = "Field `ALW_EN_MIX_BIAS_SH_OPEN` reader - "]
pub struct ALW_EN_MIX_BIAS_SH_OPEN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_MIX_BIAS_SH_OPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_MIX_BIAS_SH_OPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_MIX_BIAS_SH_OPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_MIX_BIAS_SH_OPEN` writer - "]
pub struct ALW_EN_MIX_BIAS_SH_OPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_MIX_BIAS_SH_OPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `ALW_EN_MIX_EN` reader - "]
pub struct ALW_EN_MIX_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_MIX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_MIX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_MIX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_MIX_EN` writer - "]
pub struct ALW_EN_MIX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_MIX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `ALW_EN_LNA_CGM_EN` reader - "]
pub struct ALW_EN_LNA_CGM_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_LNA_CGM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_LNA_CGM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_LNA_CGM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_LNA_CGM_EN` writer - "]
pub struct ALW_EN_LNA_CGM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_LNA_CGM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `ALW_EN_LNA_CORE_EN` reader - "]
pub struct ALW_EN_LNA_CORE_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_LNA_CORE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_LNA_CORE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_LNA_CORE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_LNA_CORE_EN` writer - "]
pub struct ALW_EN_LNA_CORE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_LNA_CORE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `ALW_EN_PA_EN` reader - "]
pub struct ALW_EN_PA_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_PA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_PA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_PA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_PA_EN` writer - "]
pub struct ALW_EN_PA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_PA_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `ALW_EN_PA_RAMP_EN` reader - "]
pub struct ALW_EN_PA_RAMP_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_PA_RAMP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_PA_RAMP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_PA_RAMP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_PA_RAMP_EN` writer - "]
pub struct ALW_EN_PA_RAMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_PA_RAMP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `ALW_EN_RFIO_BIAS_SH_OPEN` reader - "]
pub struct ALW_EN_RFIO_BIAS_SH_OPEN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_RFIO_BIAS_SH_OPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_RFIO_BIAS_SH_OPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_RFIO_BIAS_SH_OPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_RFIO_BIAS_SH_OPEN` writer - "]
pub struct ALW_EN_RFIO_BIAS_SH_OPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_RFIO_BIAS_SH_OPEN_W<'a> {
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
#[doc = "Field `ALW_EN_RFIO_BIAS_EN` reader - "]
pub struct ALW_EN_RFIO_BIAS_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_RFIO_BIAS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_RFIO_BIAS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_RFIO_BIAS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_RFIO_BIAS_EN` writer - "]
pub struct ALW_EN_RFIO_BIAS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_RFIO_BIAS_EN_W<'a> {
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
#[doc = "Field `ALW_EN_RFIO_TX_HARM_EN` reader - "]
pub struct ALW_EN_RFIO_TX_HARM_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_RFIO_TX_HARM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_RFIO_TX_HARM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_RFIO_TX_HARM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_RFIO_TX_HARM_EN` writer - "]
pub struct ALW_EN_RFIO_TX_HARM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_RFIO_TX_HARM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `ALW_EN_RFIO_TX_EN` reader - "]
pub struct ALW_EN_RFIO_TX_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_RFIO_TX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_RFIO_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_RFIO_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_RFIO_TX_EN` writer - "]
pub struct ALW_EN_RFIO_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_RFIO_TX_EN_W<'a> {
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
#[doc = "Field `ALW_EN_RFIO_RX_EN` reader - "]
pub struct ALW_EN_RFIO_RX_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_RFIO_RX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_RFIO_RX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_RFIO_RX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_RFIO_RX_EN` writer - "]
pub struct ALW_EN_RFIO_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_RFIO_RX_EN_W<'a> {
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
#[doc = "Field `ALW_EN_ADPLLDIG_LDO_LP` reader - "]
pub struct ALW_EN_ADPLLDIG_LDO_LP_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADPLLDIG_LDO_LP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADPLLDIG_LDO_LP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADPLLDIG_LDO_LP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADPLLDIG_LDO_LP` writer - "]
pub struct ALW_EN_ADPLLDIG_LDO_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADPLLDIG_LDO_LP_W<'a> {
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
#[doc = "Field `ALW_EN_ADPLLDIG_LDO_ACTIVERDY` reader - "]
pub struct ALW_EN_ADPLLDIG_LDO_ACTIVERDY_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADPLLDIG_LDO_ACTIVERDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADPLLDIG_LDO_ACTIVERDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADPLLDIG_LDO_ACTIVERDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADPLLDIG_LDO_ACTIVERDY` writer - "]
pub struct ALW_EN_ADPLLDIG_LDO_ACTIVERDY_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADPLLDIG_LDO_ACTIVERDY_W<'a> {
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
#[doc = "Field `ALW_EN_LNA_LDO_ZERO` reader - "]
pub struct ALW_EN_LNA_LDO_ZERO_R(crate::FieldReader<bool, bool>);
impl ALW_EN_LNA_LDO_ZERO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_LNA_LDO_ZERO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_LNA_LDO_ZERO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_LNA_LDO_ZERO` writer - "]
pub struct ALW_EN_LNA_LDO_ZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_LNA_LDO_ZERO_W<'a> {
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
#[doc = "Field `ALW_EN_LDO_ZERO_EN` reader - "]
pub struct ALW_EN_LDO_ZERO_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_LDO_ZERO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_LDO_ZERO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_LDO_ZERO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_LDO_ZERO_EN` writer - "]
pub struct ALW_EN_LDO_ZERO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_LDO_ZERO_EN_W<'a> {
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
#[doc = "Field `ALW_EN_ADPLL_DCO_LDO_EN` reader - "]
pub struct ALW_EN_ADPLL_DCO_LDO_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADPLL_DCO_LDO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADPLL_DCO_LDO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADPLL_DCO_LDO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADPLL_DCO_LDO_EN` writer - "]
pub struct ALW_EN_ADPLL_DCO_LDO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADPLL_DCO_LDO_EN_W<'a> {
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
#[doc = "Field `ALW_EN_ADPLL_DTC_LDO_EN` reader - "]
pub struct ALW_EN_ADPLL_DTC_LDO_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADPLL_DTC_LDO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADPLL_DTC_LDO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADPLL_DTC_LDO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADPLL_DTC_LDO_EN` writer - "]
pub struct ALW_EN_ADPLL_DTC_LDO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADPLL_DTC_LDO_EN_W<'a> {
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
#[doc = "Field `ALW_EN_ADPLL_TDC_LDO_EN` reader - "]
pub struct ALW_EN_ADPLL_TDC_LDO_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_ADPLL_TDC_LDO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_ADPLL_TDC_LDO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_ADPLL_TDC_LDO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_ADPLL_TDC_LDO_EN` writer - "]
pub struct ALW_EN_ADPLL_TDC_LDO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_ADPLL_TDC_LDO_EN_W<'a> {
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
#[doc = "Field `ALW_EN_IFFADC_LDO_EN` reader - "]
pub struct ALW_EN_IFFADC_LDO_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_IFFADC_LDO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_IFFADC_LDO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_IFFADC_LDO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_IFFADC_LDO_EN` writer - "]
pub struct ALW_EN_IFFADC_LDO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_IFFADC_LDO_EN_W<'a> {
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
#[doc = "Field `ALW_EN_IFF_LDO_EN` reader - "]
pub struct ALW_EN_IFF_LDO_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_IFF_LDO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_IFF_LDO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_IFF_LDO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_IFF_LDO_EN` writer - "]
pub struct ALW_EN_IFF_LDO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_IFF_LDO_EN_W<'a> {
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
#[doc = "Field `ALW_EN_MIX_LDO_EN` reader - "]
pub struct ALW_EN_MIX_LDO_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_MIX_LDO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_MIX_LDO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_MIX_LDO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_MIX_LDO_EN` writer - "]
pub struct ALW_EN_MIX_LDO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_MIX_LDO_EN_W<'a> {
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
#[doc = "Field `ALW_EN_LNA_LDO_EN` reader - "]
pub struct ALW_EN_LNA_LDO_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_LNA_LDO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_LNA_LDO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_LNA_LDO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_LNA_LDO_EN` writer - "]
pub struct ALW_EN_LNA_LDO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_LNA_LDO_EN_W<'a> {
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
#[doc = "Field `ALW_EN_PA_LDO_EN` reader - "]
pub struct ALW_EN_PA_LDO_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_PA_LDO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_PA_LDO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_PA_LDO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_PA_LDO_EN` writer - "]
pub struct ALW_EN_PA_LDO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_PA_LDO_EN_W<'a> {
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
#[doc = "Field `ALW_EN_RFIO_LDO_EN` reader - "]
pub struct ALW_EN_RFIO_LDO_EN_R(crate::FieldReader<bool, bool>);
impl ALW_EN_RFIO_LDO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALW_EN_RFIO_LDO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALW_EN_RFIO_LDO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALW_EN_RFIO_LDO_EN` writer - "]
pub struct ALW_EN_RFIO_LDO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALW_EN_RFIO_LDO_EN_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn alw_en_adplldig_en(&self) -> ALW_EN_ADPLLDIG_EN_R {
        ALW_EN_ADPLLDIG_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn alw_en_adplldig_rst(&self) -> ALW_EN_ADPLLDIG_RST_R {
        ALW_EN_ADPLLDIG_RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn alw_en_adpll_clk_en(&self) -> ALW_EN_ADPLL_CLK_EN_R {
        ALW_EN_ADPLL_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn alw_en_adpll_dco_en(&self) -> ALW_EN_ADPLL_DCO_EN_R {
        ALW_EN_ADPLL_DCO_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn alw_en_adc_en(&self) -> ALW_EN_ADC_EN_R {
        ALW_EN_ADC_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn alw_en_adc_clk_en(&self) -> ALW_EN_ADC_CLK_EN_R {
        ALW_EN_ADC_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn alw_en_iff_bias_sh_open(&self) -> ALW_EN_IFF_BIAS_SH_OPEN_R {
        ALW_EN_IFF_BIAS_SH_OPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn alw_en_iff_en(&self) -> ALW_EN_IFF_EN_R {
        ALW_EN_IFF_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn alw_en_mix_bias_sh_open(&self) -> ALW_EN_MIX_BIAS_SH_OPEN_R {
        ALW_EN_MIX_BIAS_SH_OPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn alw_en_mix_en(&self) -> ALW_EN_MIX_EN_R {
        ALW_EN_MIX_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn alw_en_lna_cgm_en(&self) -> ALW_EN_LNA_CGM_EN_R {
        ALW_EN_LNA_CGM_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn alw_en_lna_core_en(&self) -> ALW_EN_LNA_CORE_EN_R {
        ALW_EN_LNA_CORE_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn alw_en_pa_en(&self) -> ALW_EN_PA_EN_R {
        ALW_EN_PA_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn alw_en_pa_ramp_en(&self) -> ALW_EN_PA_RAMP_EN_R {
        ALW_EN_PA_RAMP_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn alw_en_rfio_bias_sh_open(&self) -> ALW_EN_RFIO_BIAS_SH_OPEN_R {
        ALW_EN_RFIO_BIAS_SH_OPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn alw_en_rfio_bias_en(&self) -> ALW_EN_RFIO_BIAS_EN_R {
        ALW_EN_RFIO_BIAS_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn alw_en_rfio_tx_harm_en(&self) -> ALW_EN_RFIO_TX_HARM_EN_R {
        ALW_EN_RFIO_TX_HARM_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn alw_en_rfio_tx_en(&self) -> ALW_EN_RFIO_TX_EN_R {
        ALW_EN_RFIO_TX_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn alw_en_rfio_rx_en(&self) -> ALW_EN_RFIO_RX_EN_R {
        ALW_EN_RFIO_RX_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn alw_en_adplldig_ldo_lp(&self) -> ALW_EN_ADPLLDIG_LDO_LP_R {
        ALW_EN_ADPLLDIG_LDO_LP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn alw_en_adplldig_ldo_activerdy(&self) -> ALW_EN_ADPLLDIG_LDO_ACTIVERDY_R {
        ALW_EN_ADPLLDIG_LDO_ACTIVERDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn alw_en_lna_ldo_zero(&self) -> ALW_EN_LNA_LDO_ZERO_R {
        ALW_EN_LNA_LDO_ZERO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn alw_en_ldo_zero_en(&self) -> ALW_EN_LDO_ZERO_EN_R {
        ALW_EN_LDO_ZERO_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn alw_en_adpll_dco_ldo_en(&self) -> ALW_EN_ADPLL_DCO_LDO_EN_R {
        ALW_EN_ADPLL_DCO_LDO_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn alw_en_adpll_dtc_ldo_en(&self) -> ALW_EN_ADPLL_DTC_LDO_EN_R {
        ALW_EN_ADPLL_DTC_LDO_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn alw_en_adpll_tdc_ldo_en(&self) -> ALW_EN_ADPLL_TDC_LDO_EN_R {
        ALW_EN_ADPLL_TDC_LDO_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn alw_en_iffadc_ldo_en(&self) -> ALW_EN_IFFADC_LDO_EN_R {
        ALW_EN_IFFADC_LDO_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn alw_en_iff_ldo_en(&self) -> ALW_EN_IFF_LDO_EN_R {
        ALW_EN_IFF_LDO_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn alw_en_mix_ldo_en(&self) -> ALW_EN_MIX_LDO_EN_R {
        ALW_EN_MIX_LDO_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn alw_en_lna_ldo_en(&self) -> ALW_EN_LNA_LDO_EN_R {
        ALW_EN_LNA_LDO_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn alw_en_pa_ldo_en(&self) -> ALW_EN_PA_LDO_EN_R {
        ALW_EN_PA_LDO_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn alw_en_rfio_ldo_en(&self) -> ALW_EN_RFIO_LDO_EN_R {
        ALW_EN_RFIO_LDO_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn alw_en_adplldig_en(&mut self) -> ALW_EN_ADPLLDIG_EN_W {
        ALW_EN_ADPLLDIG_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn alw_en_adplldig_rst(&mut self) -> ALW_EN_ADPLLDIG_RST_W {
        ALW_EN_ADPLLDIG_RST_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn alw_en_adpll_clk_en(&mut self) -> ALW_EN_ADPLL_CLK_EN_W {
        ALW_EN_ADPLL_CLK_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn alw_en_adpll_dco_en(&mut self) -> ALW_EN_ADPLL_DCO_EN_W {
        ALW_EN_ADPLL_DCO_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn alw_en_adc_en(&mut self) -> ALW_EN_ADC_EN_W {
        ALW_EN_ADC_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn alw_en_adc_clk_en(&mut self) -> ALW_EN_ADC_CLK_EN_W {
        ALW_EN_ADC_CLK_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn alw_en_iff_bias_sh_open(&mut self) -> ALW_EN_IFF_BIAS_SH_OPEN_W {
        ALW_EN_IFF_BIAS_SH_OPEN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn alw_en_iff_en(&mut self) -> ALW_EN_IFF_EN_W {
        ALW_EN_IFF_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn alw_en_mix_bias_sh_open(&mut self) -> ALW_EN_MIX_BIAS_SH_OPEN_W {
        ALW_EN_MIX_BIAS_SH_OPEN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn alw_en_mix_en(&mut self) -> ALW_EN_MIX_EN_W {
        ALW_EN_MIX_EN_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn alw_en_lna_cgm_en(&mut self) -> ALW_EN_LNA_CGM_EN_W {
        ALW_EN_LNA_CGM_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn alw_en_lna_core_en(&mut self) -> ALW_EN_LNA_CORE_EN_W {
        ALW_EN_LNA_CORE_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn alw_en_pa_en(&mut self) -> ALW_EN_PA_EN_W {
        ALW_EN_PA_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn alw_en_pa_ramp_en(&mut self) -> ALW_EN_PA_RAMP_EN_W {
        ALW_EN_PA_RAMP_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn alw_en_rfio_bias_sh_open(&mut self) -> ALW_EN_RFIO_BIAS_SH_OPEN_W {
        ALW_EN_RFIO_BIAS_SH_OPEN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn alw_en_rfio_bias_en(&mut self) -> ALW_EN_RFIO_BIAS_EN_W {
        ALW_EN_RFIO_BIAS_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn alw_en_rfio_tx_harm_en(&mut self) -> ALW_EN_RFIO_TX_HARM_EN_W {
        ALW_EN_RFIO_TX_HARM_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn alw_en_rfio_tx_en(&mut self) -> ALW_EN_RFIO_TX_EN_W {
        ALW_EN_RFIO_TX_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn alw_en_rfio_rx_en(&mut self) -> ALW_EN_RFIO_RX_EN_W {
        ALW_EN_RFIO_RX_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn alw_en_adplldig_ldo_lp(&mut self) -> ALW_EN_ADPLLDIG_LDO_LP_W {
        ALW_EN_ADPLLDIG_LDO_LP_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn alw_en_adplldig_ldo_activerdy(&mut self) -> ALW_EN_ADPLLDIG_LDO_ACTIVERDY_W {
        ALW_EN_ADPLLDIG_LDO_ACTIVERDY_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn alw_en_lna_ldo_zero(&mut self) -> ALW_EN_LNA_LDO_ZERO_W {
        ALW_EN_LNA_LDO_ZERO_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn alw_en_ldo_zero_en(&mut self) -> ALW_EN_LDO_ZERO_EN_W {
        ALW_EN_LDO_ZERO_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn alw_en_adpll_dco_ldo_en(&mut self) -> ALW_EN_ADPLL_DCO_LDO_EN_W {
        ALW_EN_ADPLL_DCO_LDO_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn alw_en_adpll_dtc_ldo_en(&mut self) -> ALW_EN_ADPLL_DTC_LDO_EN_W {
        ALW_EN_ADPLL_DTC_LDO_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn alw_en_adpll_tdc_ldo_en(&mut self) -> ALW_EN_ADPLL_TDC_LDO_EN_W {
        ALW_EN_ADPLL_TDC_LDO_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn alw_en_iffadc_ldo_en(&mut self) -> ALW_EN_IFFADC_LDO_EN_W {
        ALW_EN_IFFADC_LDO_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn alw_en_iff_ldo_en(&mut self) -> ALW_EN_IFF_LDO_EN_W {
        ALW_EN_IFF_LDO_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn alw_en_mix_ldo_en(&mut self) -> ALW_EN_MIX_LDO_EN_W {
        ALW_EN_MIX_LDO_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn alw_en_lna_ldo_en(&mut self) -> ALW_EN_LNA_LDO_EN_W {
        ALW_EN_LNA_LDO_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn alw_en_pa_ldo_en(&mut self) -> ALW_EN_PA_LDO_EN_W {
        ALW_EN_PA_LDO_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn alw_en_rfio_ldo_en(&mut self) -> ALW_EN_RFIO_LDO_EN_W {
        ALW_EN_RFIO_LDO_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_always_en1_reg](index.html) module"]
pub struct RF_ALWAYS_EN1_REG_SPEC;
impl crate::RegisterSpec for RF_ALWAYS_EN1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_always_en1_reg::R](R) reader structure"]
impl crate::Readable for RF_ALWAYS_EN1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_always_en1_reg::W](W) writer structure"]
impl crate::Writable for RF_ALWAYS_EN1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_ALWAYS_EN1_REG to value 0"]
impl crate::Resettable for RF_ALWAYS_EN1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
