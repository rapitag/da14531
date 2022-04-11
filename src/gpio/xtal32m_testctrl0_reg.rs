#[doc = "Register `XTAL32M_TESTCTRL0_REG` reader"]
pub struct R(crate::R<XTAL32M_TESTCTRL0_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32M_TESTCTRL0_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32M_TESTCTRL0_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32M_TESTCTRL0_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL32M_TESTCTRL0_REG` writer"]
pub struct W(crate::W<XTAL32M_TESTCTRL0_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32M_TESTCTRL0_REG_SPEC>;
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
impl From<crate::W<XTAL32M_TESTCTRL0_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32M_TESTCTRL0_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIAS_SAH_HOLD_OVERRIDE` reader - "]
pub struct BIAS_SAH_HOLD_OVERRIDE_R(crate::FieldReader<u8, u8>);
impl BIAS_SAH_HOLD_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BIAS_SAH_HOLD_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_SAH_HOLD_OVERRIDE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_SAH_HOLD_OVERRIDE` writer - "]
pub struct BIAS_SAH_HOLD_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_SAH_HOLD_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u16 & 3) << 14);
        self.w
    }
}
#[doc = "Field `CORE_FREQ_TRIM_SW2_AMP` reader - "]
pub struct CORE_FREQ_TRIM_SW2_AMP_R(crate::FieldReader<u8, u8>);
impl CORE_FREQ_TRIM_SW2_AMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_FREQ_TRIM_SW2_AMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_FREQ_TRIM_SW2_AMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_FREQ_TRIM_SW2_AMP` writer - "]
pub struct CORE_FREQ_TRIM_SW2_AMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_FREQ_TRIM_SW2_AMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 11)) | ((value as u16 & 7) << 11);
        self.w
    }
}
#[doc = "Field `CORE_GM_CURRENT` reader - "]
pub struct CORE_GM_CURRENT_R(crate::FieldReader<bool, bool>);
impl CORE_GM_CURRENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_GM_CURRENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_GM_CURRENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_GM_CURRENT` writer - "]
pub struct CORE_GM_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_GM_CURRENT_W<'a> {
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
#[doc = "Field `CORE_HOLD_AMP_REG_OVERRIDE` reader - "]
pub struct CORE_HOLD_AMP_REG_OVERRIDE_R(crate::FieldReader<u8, u8>);
impl CORE_HOLD_AMP_REG_OVERRIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_HOLD_AMP_REG_OVERRIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_HOLD_AMP_REG_OVERRIDE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_HOLD_AMP_REG_OVERRIDE` writer - "]
pub struct CORE_HOLD_AMP_REG_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_HOLD_AMP_REG_OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u16 & 3) << 8);
        self.w
    }
}
#[doc = "Field `CORE_I2V_TO_TESTBUS` reader - "]
pub struct CORE_I2V_TO_TESTBUS_R(crate::FieldReader<bool, bool>);
impl CORE_I2V_TO_TESTBUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_I2V_TO_TESTBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_I2V_TO_TESTBUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_I2V_TO_TESTBUS` writer - "]
pub struct CORE_I2V_TO_TESTBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_I2V_TO_TESTBUS_W<'a> {
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
#[doc = "Field `CORE_I2V_TO_TESTBUS_10X` reader - "]
pub struct CORE_I2V_TO_TESTBUS_10X_R(crate::FieldReader<bool, bool>);
impl CORE_I2V_TO_TESTBUS_10X_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_I2V_TO_TESTBUS_10X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_I2V_TO_TESTBUS_10X_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_I2V_TO_TESTBUS_10X` writer - "]
pub struct CORE_I2V_TO_TESTBUS_10X_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_I2V_TO_TESTBUS_10X_W<'a> {
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
#[doc = "Field `CORE_MAX_CURRENT` reader - "]
pub struct CORE_MAX_CURRENT_R(crate::FieldReader<bool, bool>);
impl CORE_MAX_CURRENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_MAX_CURRENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_MAX_CURRENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_MAX_CURRENT` writer - "]
pub struct CORE_MAX_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_MAX_CURRENT_W<'a> {
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
#[doc = "Field `CORE_XTAL_DISCHARGE` reader - "]
pub struct CORE_XTAL_DISCHARGE_R(crate::FieldReader<bool, bool>);
impl CORE_XTAL_DISCHARGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_XTAL_DISCHARGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_XTAL_DISCHARGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_XTAL_DISCHARGE` writer - "]
pub struct CORE_XTAL_DISCHARGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_XTAL_DISCHARGE_W<'a> {
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
#[doc = "Field `DCBLOCK_LV_MODE` reader - "]
pub struct DCBLOCK_LV_MODE_R(crate::FieldReader<bool, bool>);
impl DCBLOCK_LV_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCBLOCK_LV_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCBLOCK_LV_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCBLOCK_LV_MODE` writer - "]
pub struct DCBLOCK_LV_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCBLOCK_LV_MODE_W<'a> {
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
#[doc = "Field `DIFFBUF_BYPASS` reader - "]
pub struct DIFFBUF_BYPASS_R(crate::FieldReader<bool, bool>);
impl DIFFBUF_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIFFBUF_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFFBUF_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFFBUF_BYPASS` writer - "]
pub struct DIFFBUF_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFFBUF_BYPASS_W<'a> {
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
#[doc = "Field `OSC_TRIM_OPEN_DISABLE` reader - "]
pub struct OSC_TRIM_OPEN_DISABLE_R(crate::FieldReader<bool, bool>);
impl OSC_TRIM_OPEN_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC_TRIM_OPEN_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_TRIM_OPEN_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC_TRIM_OPEN_DISABLE` writer - "]
pub struct OSC_TRIM_OPEN_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_TRIM_OPEN_DISABLE_W<'a> {
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
#[doc = "Field `SPIKE_FLT_DISABLE` reader - "]
pub struct SPIKE_FLT_DISABLE_R(crate::FieldReader<bool, bool>);
impl SPIKE_FLT_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIKE_FLT_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIKE_FLT_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIKE_FLT_DISABLE` writer - "]
pub struct SPIKE_FLT_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIKE_FLT_DISABLE_W<'a> {
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
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn bias_sah_hold_override(&self) -> BIAS_SAH_HOLD_OVERRIDE_R {
        BIAS_SAH_HOLD_OVERRIDE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn core_freq_trim_sw2_amp(&self) -> CORE_FREQ_TRIM_SW2_AMP_R {
        CORE_FREQ_TRIM_SW2_AMP_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn core_gm_current(&self) -> CORE_GM_CURRENT_R {
        CORE_GM_CURRENT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn core_hold_amp_reg_override(&self) -> CORE_HOLD_AMP_REG_OVERRIDE_R {
        CORE_HOLD_AMP_REG_OVERRIDE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn core_i2v_to_testbus(&self) -> CORE_I2V_TO_TESTBUS_R {
        CORE_I2V_TO_TESTBUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn core_i2v_to_testbus_10x(&self) -> CORE_I2V_TO_TESTBUS_10X_R {
        CORE_I2V_TO_TESTBUS_10X_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn core_max_current(&self) -> CORE_MAX_CURRENT_R {
        CORE_MAX_CURRENT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn core_xtal_discharge(&self) -> CORE_XTAL_DISCHARGE_R {
        CORE_XTAL_DISCHARGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dcblock_lv_mode(&self) -> DCBLOCK_LV_MODE_R {
        DCBLOCK_LV_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn diffbuf_bypass(&self) -> DIFFBUF_BYPASS_R {
        DIFFBUF_BYPASS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn osc_trim_open_disable(&self) -> OSC_TRIM_OPEN_DISABLE_R {
        OSC_TRIM_OPEN_DISABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spike_flt_disable(&self) -> SPIKE_FLT_DISABLE_R {
        SPIKE_FLT_DISABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn bias_sah_hold_override(&mut self) -> BIAS_SAH_HOLD_OVERRIDE_W {
        BIAS_SAH_HOLD_OVERRIDE_W { w: self }
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn core_freq_trim_sw2_amp(&mut self) -> CORE_FREQ_TRIM_SW2_AMP_W {
        CORE_FREQ_TRIM_SW2_AMP_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn core_gm_current(&mut self) -> CORE_GM_CURRENT_W {
        CORE_GM_CURRENT_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn core_hold_amp_reg_override(&mut self) -> CORE_HOLD_AMP_REG_OVERRIDE_W {
        CORE_HOLD_AMP_REG_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn core_i2v_to_testbus(&mut self) -> CORE_I2V_TO_TESTBUS_W {
        CORE_I2V_TO_TESTBUS_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn core_i2v_to_testbus_10x(&mut self) -> CORE_I2V_TO_TESTBUS_10X_W {
        CORE_I2V_TO_TESTBUS_10X_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn core_max_current(&mut self) -> CORE_MAX_CURRENT_W {
        CORE_MAX_CURRENT_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn core_xtal_discharge(&mut self) -> CORE_XTAL_DISCHARGE_W {
        CORE_XTAL_DISCHARGE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dcblock_lv_mode(&mut self) -> DCBLOCK_LV_MODE_W {
        DCBLOCK_LV_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn diffbuf_bypass(&mut self) -> DIFFBUF_BYPASS_W {
        DIFFBUF_BYPASS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn osc_trim_open_disable(&mut self) -> OSC_TRIM_OPEN_DISABLE_W {
        OSC_TRIM_OPEN_DISABLE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spike_flt_disable(&mut self) -> SPIKE_FLT_DISABLE_W {
        SPIKE_FLT_DISABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32m_testctrl0_reg](index.html) module"]
pub struct XTAL32M_TESTCTRL0_REG_SPEC;
impl crate::RegisterSpec for XTAL32M_TESTCTRL0_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xtal32m_testctrl0_reg::R](R) reader structure"]
impl crate::Readable for XTAL32M_TESTCTRL0_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32m_testctrl0_reg::W](W) writer structure"]
impl crate::Writable for XTAL32M_TESTCTRL0_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL32M_TESTCTRL0_REG to value 0x3400"]
impl crate::Resettable for XTAL32M_TESTCTRL0_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3400
    }
}
