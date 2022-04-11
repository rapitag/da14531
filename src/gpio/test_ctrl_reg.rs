#[doc = "Register `TEST_CTRL_REG` reader"]
pub struct R(crate::R<TEST_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_CTRL_REG` writer"]
pub struct W(crate::W<TEST_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_CTRL_REG_SPEC>;
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
impl From<crate::W<TEST_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADPLL_SCAN_COMP_EN` reader - "]
pub struct ADPLL_SCAN_COMP_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_SCAN_COMP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_SCAN_COMP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_SCAN_COMP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLL_SCAN_COMP_EN` writer - "]
pub struct ADPLL_SCAN_COMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_SCAN_COMP_EN_W<'a> {
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
#[doc = "Field `ADPLL_SCAN_TEST_EN` reader - "]
pub struct ADPLL_SCAN_TEST_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_SCAN_TEST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_SCAN_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_SCAN_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADPLL_SCAN_TEST_EN` writer - "]
pub struct ADPLL_SCAN_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_SCAN_TEST_EN_W<'a> {
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
#[doc = "Field `CP_CAP_BIAS_TRIM` reader - "]
pub struct CP_CAP_BIAS_TRIM_R(crate::FieldReader<u8, u8>);
impl CP_CAP_BIAS_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CP_CAP_BIAS_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CP_CAP_BIAS_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CP_CAP_BIAS_TRIM` writer - "]
pub struct CP_CAP_BIAS_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CP_CAP_BIAS_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 9)) | ((value as u16 & 3) << 9);
        self.w
    }
}
#[doc = "Field `LDO_CORE_DUMMY_LOAD_ENABLE` reader - "]
pub struct LDO_CORE_DUMMY_LOAD_ENABLE_R(crate::FieldReader<bool, bool>);
impl LDO_CORE_DUMMY_LOAD_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_CORE_DUMMY_LOAD_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_CORE_DUMMY_LOAD_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_CORE_DUMMY_LOAD_ENABLE` writer - "]
pub struct LDO_CORE_DUMMY_LOAD_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_CORE_DUMMY_LOAD_ENABLE_W<'a> {
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
#[doc = "Field `LDO_CORE_CAP_BYPASS` reader - "]
pub struct LDO_CORE_CAP_BYPASS_R(crate::FieldReader<bool, bool>);
impl LDO_CORE_CAP_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDO_CORE_CAP_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO_CORE_CAP_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDO_CORE_CAP_BYPASS` writer - "]
pub struct LDO_CORE_CAP_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO_CORE_CAP_BYPASS_W<'a> {
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
#[doc = "Field `XTAL32M_CAP_TEST_EN` reader - "]
pub struct XTAL32M_CAP_TEST_EN_R(crate::FieldReader<bool, bool>);
impl XTAL32M_CAP_TEST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32M_CAP_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32M_CAP_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32M_CAP_TEST_EN` writer - "]
pub struct XTAL32M_CAP_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32M_CAP_TEST_EN_W<'a> {
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
#[doc = "Field `SHOW_DCDC` reader - "]
pub struct SHOW_DCDC_R(crate::FieldReader<bool, bool>);
impl SHOW_DCDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHOW_DCDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHOW_DCDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHOW_DCDC` writer - "]
pub struct SHOW_DCDC_W<'a> {
    w: &'a mut W,
}
impl<'a> SHOW_DCDC_W<'a> {
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
#[doc = "Field `SHOW_POWER` reader - "]
pub struct SHOW_POWER_R(crate::FieldReader<bool, bool>);
impl SHOW_POWER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHOW_POWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHOW_POWER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHOW_POWER` writer - "]
pub struct SHOW_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> SHOW_POWER_W<'a> {
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
#[doc = "Field `SHOW_CLOCKS` reader - "]
pub struct SHOW_CLOCKS_R(crate::FieldReader<bool, bool>);
impl SHOW_CLOCKS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SHOW_CLOCKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHOW_CLOCKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHOW_CLOCKS` writer - "]
pub struct SHOW_CLOCKS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHOW_CLOCKS_W<'a> {
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
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_scan_comp_en(&self) -> ADPLL_SCAN_COMP_EN_R {
        ADPLL_SCAN_COMP_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn adpll_scan_test_en(&self) -> ADPLL_SCAN_TEST_EN_R {
        ADPLL_SCAN_TEST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn cp_cap_bias_trim(&self) -> CP_CAP_BIAS_TRIM_R {
        CP_CAP_BIAS_TRIM_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ldo_core_dummy_load_enable(&self) -> LDO_CORE_DUMMY_LOAD_ENABLE_R {
        LDO_CORE_DUMMY_LOAD_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ldo_core_cap_bypass(&self) -> LDO_CORE_CAP_BYPASS_R {
        LDO_CORE_CAP_BYPASS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn xtal32m_cap_test_en(&self) -> XTAL32M_CAP_TEST_EN_R {
        XTAL32M_CAP_TEST_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn show_dcdc(&self) -> SHOW_DCDC_R {
        SHOW_DCDC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn show_power(&self) -> SHOW_POWER_R {
        SHOW_POWER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn show_clocks(&self) -> SHOW_CLOCKS_R {
        SHOW_CLOCKS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_scan_comp_en(&mut self) -> ADPLL_SCAN_COMP_EN_W {
        ADPLL_SCAN_COMP_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn adpll_scan_test_en(&mut self) -> ADPLL_SCAN_TEST_EN_W {
        ADPLL_SCAN_TEST_EN_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn cp_cap_bias_trim(&mut self) -> CP_CAP_BIAS_TRIM_W {
        CP_CAP_BIAS_TRIM_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ldo_core_dummy_load_enable(&mut self) -> LDO_CORE_DUMMY_LOAD_ENABLE_W {
        LDO_CORE_DUMMY_LOAD_ENABLE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ldo_core_cap_bypass(&mut self) -> LDO_CORE_CAP_BYPASS_W {
        LDO_CORE_CAP_BYPASS_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn xtal32m_cap_test_en(&mut self) -> XTAL32M_CAP_TEST_EN_W {
        XTAL32M_CAP_TEST_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn show_dcdc(&mut self) -> SHOW_DCDC_W {
        SHOW_DCDC_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn show_power(&mut self) -> SHOW_POWER_W {
        SHOW_POWER_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn show_clocks(&mut self) -> SHOW_CLOCKS_W {
        SHOW_CLOCKS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_ctrl_reg](index.html) module"]
pub struct TEST_CTRL_REG_SPEC;
impl crate::RegisterSpec for TEST_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [test_ctrl_reg::R](R) reader structure"]
impl crate::Readable for TEST_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_ctrl_reg::W](W) writer structure"]
impl crate::Writable for TEST_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEST_CTRL_REG to value 0"]
impl crate::Resettable for TEST_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
