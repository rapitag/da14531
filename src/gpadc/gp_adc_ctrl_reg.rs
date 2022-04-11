#[doc = "Register `GP_ADC_CTRL_REG` reader"]
pub struct R(crate::R<GP_ADC_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_ADC_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_ADC_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_ADC_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_ADC_CTRL_REG` writer"]
pub struct W(crate::W<GP_ADC_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_ADC_CTRL_REG_SPEC>;
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
impl From<crate::W<GP_ADC_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_ADC_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIE_TEMP_EN` reader - Enables the die-temperature sensor. Output can be measured on GPADC input 4."]
pub struct DIE_TEMP_EN_R(crate::FieldReader<bool, bool>);
impl DIE_TEMP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIE_TEMP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIE_TEMP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIE_TEMP_EN` writer - Enables the die-temperature sensor. Output can be measured on GPADC input 4."]
pub struct DIE_TEMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIE_TEMP_EN_W<'a> {
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
#[doc = "Field `GP_ADC_OFFS_SH_GAIN_SEL` reader - "]
pub struct GP_ADC_OFFS_SH_GAIN_SEL_R(crate::FieldReader<bool, bool>);
impl GP_ADC_OFFS_SH_GAIN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_OFFS_SH_GAIN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_OFFS_SH_GAIN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_OFFS_SH_GAIN_SEL` writer - "]
pub struct GP_ADC_OFFS_SH_GAIN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_OFFS_SH_GAIN_SEL_W<'a> {
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
#[doc = "Field `GP_ADC_LDO_HOLD` reader - 0: GPADC LDO tracking bandgap reference 1: GPADC LDO hold sampled bandgap reference"]
pub struct GP_ADC_LDO_HOLD_R(crate::FieldReader<bool, bool>);
impl GP_ADC_LDO_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_LDO_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_LDO_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_LDO_HOLD` writer - 0: GPADC LDO tracking bandgap reference 1: GPADC LDO hold sampled bandgap reference"]
pub struct GP_ADC_LDO_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_LDO_HOLD_W<'a> {
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
#[doc = "Field `GP_ADC_CHOP` reader - 0: Chopper mode off 1: Chopper mode enabled. Takes two samples with opposite GP_ADC_SIGN to cancel the internal offset voltage of the ADC; Highly recommended for DC-measurements."]
pub struct GP_ADC_CHOP_R(crate::FieldReader<bool, bool>);
impl GP_ADC_CHOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_CHOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_CHOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_CHOP` writer - 0: Chopper mode off 1: Chopper mode enabled. Takes two samples with opposite GP_ADC_SIGN to cancel the internal offset voltage of the ADC; Highly recommended for DC-measurements."]
pub struct GP_ADC_CHOP_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_CHOP_W<'a> {
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
#[doc = "Field `GP_ADC_SIGN` reader - 0: Default 1: Conversion with opposite sign at input and output to cancel out the internal offset of the ADC and low-frequency"]
pub struct GP_ADC_SIGN_R(crate::FieldReader<bool, bool>);
impl GP_ADC_SIGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_SIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_SIGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_SIGN` writer - 0: Default 1: Conversion with opposite sign at input and output to cancel out the internal offset of the ADC and low-frequency"]
pub struct GP_ADC_SIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_SIGN_W<'a> {
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
#[doc = "Field `GP_ADC_MUTE` reader - 0: Normal operation 1: Mute ADC input. Takes sample at mid-scale (to dertermine the internal offset and/or noise of the ADC with regards to VDD_REF which is also sampled by the ADC)."]
pub struct GP_ADC_MUTE_R(crate::FieldReader<bool, bool>);
impl GP_ADC_MUTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_MUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_MUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_MUTE` writer - 0: Normal operation 1: Mute ADC input. Takes sample at mid-scale (to dertermine the internal offset and/or noise of the ADC with regards to VDD_REF which is also sampled by the ADC)."]
pub struct GP_ADC_MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_MUTE_W<'a> {
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
#[doc = "Field `GP_ADC_SE` reader - 0: Differential mode 1: Single ended mode"]
pub struct GP_ADC_SE_R(crate::FieldReader<bool, bool>);
impl GP_ADC_SE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_SE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_SE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_SE` writer - 0: Differential mode 1: Single ended mode"]
pub struct GP_ADC_SE_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_SE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u16 & 1) << 6);
        self.w
    }
}
#[doc = "Field `GP_ADC_MINT` reader - 0: Disable (mask) GP_ADC_INT. 1: Enable GP_ADC_INT to ICU."]
pub struct GP_ADC_MINT_R(crate::FieldReader<bool, bool>);
impl GP_ADC_MINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_MINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_MINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_MINT` writer - 0: Disable (mask) GP_ADC_INT. 1: Enable GP_ADC_INT to ICU."]
pub struct GP_ADC_MINT_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_MINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u16 & 1) << 5);
        self.w
    }
}
#[doc = "Field `GP_ADC_INT` reader - 1: AD conversion ready and has generated an interrupt. Must be cleared by writing any value to GP_ADC_CLEAR_INT_REG."]
pub struct GP_ADC_INT_R(crate::FieldReader<bool, bool>);
impl GP_ADC_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_DMA_EN` reader - 0: DMA functionality disabled 1: DMA functionality enabled"]
pub struct GP_ADC_DMA_EN_R(crate::FieldReader<bool, bool>);
impl GP_ADC_DMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_DMA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_DMA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_DMA_EN` writer - 0: DMA functionality disabled 1: DMA functionality enabled"]
pub struct GP_ADC_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_DMA_EN_W<'a> {
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
#[doc = "Field `GP_ADC_CONT` reader - 0: Manual ADC mode, a single result will be generated after setting the GP_ADC_START bit. 1: Continuous ADC mode, new ADC results will be constantly stored in GP_ADC_RESULT_REG. Still GP_ADC_START has to be set to start the execution. The time between conversions is configurable with GP_ADC_INTERVAL."]
pub struct GP_ADC_CONT_R(crate::FieldReader<bool, bool>);
impl GP_ADC_CONT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_CONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_CONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_CONT` writer - 0: Manual ADC mode, a single result will be generated after setting the GP_ADC_START bit. 1: Continuous ADC mode, new ADC results will be constantly stored in GP_ADC_RESULT_REG. Still GP_ADC_START has to be set to start the execution. The time between conversions is configurable with GP_ADC_INTERVAL."]
pub struct GP_ADC_CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_CONT_W<'a> {
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
#[doc = "Field `GP_ADC_START` reader - 0: ADC conversion ready. 1: If a 1 is written, the ADC starts a conversion. After the conversion this bit will be set to 0 and the GP_ADC_INT bit will be set. It is not allowed to write this bit while it is not (yet) zero."]
pub struct GP_ADC_START_R(crate::FieldReader<bool, bool>);
impl GP_ADC_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_START` writer - 0: ADC conversion ready. 1: If a 1 is written, the ADC starts a conversion. After the conversion this bit will be set to 0 and the GP_ADC_INT bit will be set. It is not allowed to write this bit while it is not (yet) zero."]
pub struct GP_ADC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_START_W<'a> {
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
#[doc = "Field `GP_ADC_EN` reader - 0: LDO is off and ADC is disabled.. 1: LDO is turned on and afterwards the ADC is enabled."]
pub struct GP_ADC_EN_R(crate::FieldReader<bool, bool>);
impl GP_ADC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GP_ADC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_EN` writer - 0: LDO is off and ADC is disabled.. 1: LDO is turned on and afterwards the ADC is enabled."]
pub struct GP_ADC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_EN_W<'a> {
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
    #[doc = "Bit 12 - Enables the die-temperature sensor. Output can be measured on GPADC input 4."]
    #[inline(always)]
    pub fn die_temp_en(&self) -> DIE_TEMP_EN_R {
        DIE_TEMP_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gp_adc_offs_sh_gain_sel(&self) -> GP_ADC_OFFS_SH_GAIN_SEL_R {
        GP_ADC_OFFS_SH_GAIN_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - 0: GPADC LDO tracking bandgap reference 1: GPADC LDO hold sampled bandgap reference"]
    #[inline(always)]
    pub fn gp_adc_ldo_hold(&self) -> GP_ADC_LDO_HOLD_R {
        GP_ADC_LDO_HOLD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: Chopper mode off 1: Chopper mode enabled. Takes two samples with opposite GP_ADC_SIGN to cancel the internal offset voltage of the ADC; Highly recommended for DC-measurements."]
    #[inline(always)]
    pub fn gp_adc_chop(&self) -> GP_ADC_CHOP_R {
        GP_ADC_CHOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: Default 1: Conversion with opposite sign at input and output to cancel out the internal offset of the ADC and low-frequency"]
    #[inline(always)]
    pub fn gp_adc_sign(&self) -> GP_ADC_SIGN_R {
        GP_ADC_SIGN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: Normal operation 1: Mute ADC input. Takes sample at mid-scale (to dertermine the internal offset and/or noise of the ADC with regards to VDD_REF which is also sampled by the ADC)."]
    #[inline(always)]
    pub fn gp_adc_mute(&self) -> GP_ADC_MUTE_R {
        GP_ADC_MUTE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: Differential mode 1: Single ended mode"]
    #[inline(always)]
    pub fn gp_adc_se(&self) -> GP_ADC_SE_R {
        GP_ADC_SE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: Disable (mask) GP_ADC_INT. 1: Enable GP_ADC_INT to ICU."]
    #[inline(always)]
    pub fn gp_adc_mint(&self) -> GP_ADC_MINT_R {
        GP_ADC_MINT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: AD conversion ready and has generated an interrupt. Must be cleared by writing any value to GP_ADC_CLEAR_INT_REG."]
    #[inline(always)]
    pub fn gp_adc_int(&self) -> GP_ADC_INT_R {
        GP_ADC_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: DMA functionality disabled 1: DMA functionality enabled"]
    #[inline(always)]
    pub fn gp_adc_dma_en(&self) -> GP_ADC_DMA_EN_R {
        GP_ADC_DMA_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: Manual ADC mode, a single result will be generated after setting the GP_ADC_START bit. 1: Continuous ADC mode, new ADC results will be constantly stored in GP_ADC_RESULT_REG. Still GP_ADC_START has to be set to start the execution. The time between conversions is configurable with GP_ADC_INTERVAL."]
    #[inline(always)]
    pub fn gp_adc_cont(&self) -> GP_ADC_CONT_R {
        GP_ADC_CONT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - 0: ADC conversion ready. 1: If a 1 is written, the ADC starts a conversion. After the conversion this bit will be set to 0 and the GP_ADC_INT bit will be set. It is not allowed to write this bit while it is not (yet) zero."]
    #[inline(always)]
    pub fn gp_adc_start(&self) -> GP_ADC_START_R {
        GP_ADC_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - 0: LDO is off and ADC is disabled.. 1: LDO is turned on and afterwards the ADC is enabled."]
    #[inline(always)]
    pub fn gp_adc_en(&self) -> GP_ADC_EN_R {
        GP_ADC_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Enables the die-temperature sensor. Output can be measured on GPADC input 4."]
    #[inline(always)]
    pub fn die_temp_en(&mut self) -> DIE_TEMP_EN_W {
        DIE_TEMP_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gp_adc_offs_sh_gain_sel(&mut self) -> GP_ADC_OFFS_SH_GAIN_SEL_W {
        GP_ADC_OFFS_SH_GAIN_SEL_W { w: self }
    }
    #[doc = "Bit 10 - 0: GPADC LDO tracking bandgap reference 1: GPADC LDO hold sampled bandgap reference"]
    #[inline(always)]
    pub fn gp_adc_ldo_hold(&mut self) -> GP_ADC_LDO_HOLD_W {
        GP_ADC_LDO_HOLD_W { w: self }
    }
    #[doc = "Bit 9 - 0: Chopper mode off 1: Chopper mode enabled. Takes two samples with opposite GP_ADC_SIGN to cancel the internal offset voltage of the ADC; Highly recommended for DC-measurements."]
    #[inline(always)]
    pub fn gp_adc_chop(&mut self) -> GP_ADC_CHOP_W {
        GP_ADC_CHOP_W { w: self }
    }
    #[doc = "Bit 8 - 0: Default 1: Conversion with opposite sign at input and output to cancel out the internal offset of the ADC and low-frequency"]
    #[inline(always)]
    pub fn gp_adc_sign(&mut self) -> GP_ADC_SIGN_W {
        GP_ADC_SIGN_W { w: self }
    }
    #[doc = "Bit 7 - 0: Normal operation 1: Mute ADC input. Takes sample at mid-scale (to dertermine the internal offset and/or noise of the ADC with regards to VDD_REF which is also sampled by the ADC)."]
    #[inline(always)]
    pub fn gp_adc_mute(&mut self) -> GP_ADC_MUTE_W {
        GP_ADC_MUTE_W { w: self }
    }
    #[doc = "Bit 6 - 0: Differential mode 1: Single ended mode"]
    #[inline(always)]
    pub fn gp_adc_se(&mut self) -> GP_ADC_SE_W {
        GP_ADC_SE_W { w: self }
    }
    #[doc = "Bit 5 - 0: Disable (mask) GP_ADC_INT. 1: Enable GP_ADC_INT to ICU."]
    #[inline(always)]
    pub fn gp_adc_mint(&mut self) -> GP_ADC_MINT_W {
        GP_ADC_MINT_W { w: self }
    }
    #[doc = "Bit 3 - 0: DMA functionality disabled 1: DMA functionality enabled"]
    #[inline(always)]
    pub fn gp_adc_dma_en(&mut self) -> GP_ADC_DMA_EN_W {
        GP_ADC_DMA_EN_W { w: self }
    }
    #[doc = "Bit 2 - 0: Manual ADC mode, a single result will be generated after setting the GP_ADC_START bit. 1: Continuous ADC mode, new ADC results will be constantly stored in GP_ADC_RESULT_REG. Still GP_ADC_START has to be set to start the execution. The time between conversions is configurable with GP_ADC_INTERVAL."]
    #[inline(always)]
    pub fn gp_adc_cont(&mut self) -> GP_ADC_CONT_W {
        GP_ADC_CONT_W { w: self }
    }
    #[doc = "Bit 1 - 0: ADC conversion ready. 1: If a 1 is written, the ADC starts a conversion. After the conversion this bit will be set to 0 and the GP_ADC_INT bit will be set. It is not allowed to write this bit while it is not (yet) zero."]
    #[inline(always)]
    pub fn gp_adc_start(&mut self) -> GP_ADC_START_W {
        GP_ADC_START_W { w: self }
    }
    #[doc = "Bit 0 - 0: LDO is off and ADC is disabled.. 1: LDO is turned on and afterwards the ADC is enabled."]
    #[inline(always)]
    pub fn gp_adc_en(&mut self) -> GP_ADC_EN_W {
        GP_ADC_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose ADC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_adc_ctrl_reg](index.html) module"]
pub struct GP_ADC_CTRL_REG_SPEC;
impl crate::RegisterSpec for GP_ADC_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gp_adc_ctrl_reg::R](R) reader structure"]
impl crate::Readable for GP_ADC_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_adc_ctrl_reg::W](W) writer structure"]
impl crate::Writable for GP_ADC_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_ADC_CTRL_REG to value 0"]
impl crate::Resettable for GP_ADC_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
