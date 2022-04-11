#[doc = "Register `XTAL32M_TESTCTRL1_REG` reader"]
pub struct R(crate::R<XTAL32M_TESTCTRL1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32M_TESTCTRL1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32M_TESTCTRL1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32M_TESTCTRL1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL32M_TESTCTRL1_REG` writer"]
pub struct W(crate::W<XTAL32M_TESTCTRL1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32M_TESTCTRL1_REG_SPEC>;
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
impl From<crate::W<XTAL32M_TESTCTRL1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32M_TESTCTRL1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC_TRIM_CAP_BIAS` reader - "]
pub struct OSC_TRIM_CAP_BIAS_R(crate::FieldReader<bool, bool>);
impl OSC_TRIM_CAP_BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC_TRIM_CAP_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_TRIM_CAP_BIAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC_TRIM_CAP_BIAS` writer - "]
pub struct OSC_TRIM_CAP_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_TRIM_CAP_BIAS_W<'a> {
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
#[doc = "Field `RFCLK_SEL_ADPLL_ADC_TO_GPIO` reader - "]
pub struct RFCLK_SEL_ADPLL_ADC_TO_GPIO_R(crate::FieldReader<bool, bool>);
impl RFCLK_SEL_ADPLL_ADC_TO_GPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFCLK_SEL_ADPLL_ADC_TO_GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCLK_SEL_ADPLL_ADC_TO_GPIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCLK_SEL_ADPLL_ADC_TO_GPIO` writer - "]
pub struct RFCLK_SEL_ADPLL_ADC_TO_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCLK_SEL_ADPLL_ADC_TO_GPIO_W<'a> {
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
#[doc = "Field `RFCLK_ADC_TO_GPIO` reader - "]
pub struct RFCLK_ADC_TO_GPIO_R(crate::FieldReader<bool, bool>);
impl RFCLK_ADC_TO_GPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFCLK_ADC_TO_GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCLK_ADC_TO_GPIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCLK_ADC_TO_GPIO` writer - "]
pub struct RFCLK_ADC_TO_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCLK_ADC_TO_GPIO_W<'a> {
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
#[doc = "Field `RFCLK_ADPLL_TO_GPIO` reader - "]
pub struct RFCLK_ADPLL_TO_GPIO_R(crate::FieldReader<bool, bool>);
impl RFCLK_ADPLL_TO_GPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFCLK_ADPLL_TO_GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCLK_ADPLL_TO_GPIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCLK_ADPLL_TO_GPIO` writer - "]
pub struct RFCLK_ADPLL_TO_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCLK_ADPLL_TO_GPIO_W<'a> {
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
#[doc = "Field `PROG_VREF_SEL` reader - "]
pub struct PROG_VREF_SEL_R(crate::FieldReader<bool, bool>);
impl PROG_VREF_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROG_VREF_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROG_VREF_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROG_VREF_SEL` writer - "]
pub struct PROG_VREF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_VREF_SEL_W<'a> {
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
#[doc = "Field `VARICAP_TEST_SEL_XTAL` reader - "]
pub struct VARICAP_TEST_SEL_XTAL_R(crate::FieldReader<bool, bool>);
impl VARICAP_TEST_SEL_XTAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VARICAP_TEST_SEL_XTAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VARICAP_TEST_SEL_XTAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VARICAP_TEST_SEL_XTAL` writer - "]
pub struct VARICAP_TEST_SEL_XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VARICAP_TEST_SEL_XTAL_W<'a> {
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
#[doc = "Field `VARICAP_TEST_ENABLE` reader - "]
pub struct VARICAP_TEST_ENABLE_R(crate::FieldReader<bool, bool>);
impl VARICAP_TEST_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VARICAP_TEST_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VARICAP_TEST_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VARICAP_TEST_ENABLE` writer - "]
pub struct VARICAP_TEST_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> VARICAP_TEST_ENABLE_W<'a> {
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
#[doc = "Field `LDO_VREF_HOLD_OVERRIDE` reader - "]
pub struct LDO_VREF_HOLD_OVERRIDE_R(crate::FieldReader<bool, bool>);
impl LDO_VREF_HOLD_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_VREF_HOLD_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_VREF_HOLD_OVERRIDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_VREF_HOLD_OVERRIDE` writer - "]
pub struct LDO_VREF_HOLD_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_VREF_HOLD_OVERRIDE_W<'a> {
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
#[doc = "Field `DISABLE_TM_CLK` reader - "]
pub struct DISABLE_TM_CLK_R(crate::FieldReader<bool, bool>);
impl DISABLE_TM_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISABLE_TM_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_TM_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE_TM_CLK` writer - "]
pub struct DISABLE_TM_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_TM_CLK_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn osc_trim_cap_bias(&self) -> OSC_TRIM_CAP_BIAS_R {
        OSC_TRIM_CAP_BIAS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rfclk_sel_adpll_adc_to_gpio(&self) -> RFCLK_SEL_ADPLL_ADC_TO_GPIO_R {
        RFCLK_SEL_ADPLL_ADC_TO_GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rfclk_adc_to_gpio(&self) -> RFCLK_ADC_TO_GPIO_R {
        RFCLK_ADC_TO_GPIO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rfclk_adpll_to_gpio(&self) -> RFCLK_ADPLL_TO_GPIO_R {
        RFCLK_ADPLL_TO_GPIO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn prog_vref_sel(&self) -> PROG_VREF_SEL_R {
        PROG_VREF_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn varicap_test_sel_xtal(&self) -> VARICAP_TEST_SEL_XTAL_R {
        VARICAP_TEST_SEL_XTAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn varicap_test_enable(&self) -> VARICAP_TEST_ENABLE_R {
        VARICAP_TEST_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo_vref_hold_override(&self) -> LDO_VREF_HOLD_OVERRIDE_R {
        LDO_VREF_HOLD_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn disable_tm_clk(&self) -> DISABLE_TM_CLK_R {
        DISABLE_TM_CLK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn osc_trim_cap_bias(&mut self) -> OSC_TRIM_CAP_BIAS_W {
        OSC_TRIM_CAP_BIAS_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rfclk_sel_adpll_adc_to_gpio(&mut self) -> RFCLK_SEL_ADPLL_ADC_TO_GPIO_W {
        RFCLK_SEL_ADPLL_ADC_TO_GPIO_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rfclk_adc_to_gpio(&mut self) -> RFCLK_ADC_TO_GPIO_W {
        RFCLK_ADC_TO_GPIO_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rfclk_adpll_to_gpio(&mut self) -> RFCLK_ADPLL_TO_GPIO_W {
        RFCLK_ADPLL_TO_GPIO_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn prog_vref_sel(&mut self) -> PROG_VREF_SEL_W {
        PROG_VREF_SEL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn varicap_test_sel_xtal(&mut self) -> VARICAP_TEST_SEL_XTAL_W {
        VARICAP_TEST_SEL_XTAL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn varicap_test_enable(&mut self) -> VARICAP_TEST_ENABLE_W {
        VARICAP_TEST_ENABLE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo_vref_hold_override(&mut self) -> LDO_VREF_HOLD_OVERRIDE_W {
        LDO_VREF_HOLD_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn disable_tm_clk(&mut self) -> DISABLE_TM_CLK_W {
        DISABLE_TM_CLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32m_testctrl1_reg](index.html) module"]
pub struct XTAL32M_TESTCTRL1_REG_SPEC;
impl crate::RegisterSpec for XTAL32M_TESTCTRL1_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xtal32m_testctrl1_reg::R](R) reader structure"]
impl crate::Readable for XTAL32M_TESTCTRL1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32m_testctrl1_reg::W](W) writer structure"]
impl crate::Writable for XTAL32M_TESTCTRL1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL32M_TESTCTRL1_REG to value 0"]
impl crate::Resettable for XTAL32M_TESTCTRL1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
