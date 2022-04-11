#[doc = "Register `ADPLL_KDTCTDC_CAL_CTRL1_REG` reader"]
pub struct R(crate::R<ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_KDTCTDC_CAL_CTRL1_REG` writer"]
pub struct W(crate::W<ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC>;
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
impl From<crate::W<ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KDTCIC` reader - "]
pub struct KDTCIC_R(crate::FieldReader<u16, u16>);
impl KDTCIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        KDTCIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDTCIC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDTCIC` writer - "]
pub struct KDTCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> KDTCIC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 23)) | ((value as u32 & 0x01ff) << 23);
        self.w
    }
}
#[doc = "Field `KDTCCN_IC` reader - "]
pub struct KDTCCN_IC_R(crate::FieldReader<u8, u8>);
impl KDTCCN_IC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KDTCCN_IC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDTCCN_IC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDTCCN_IC` writer - "]
pub struct KDTCCN_IC_W<'a> {
    w: &'a mut W,
}
impl<'a> KDTCCN_IC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `KDTC_PIPELINE_BYPASS` reader - "]
pub struct KDTC_PIPELINE_BYPASS_R(crate::FieldReader<bool, bool>);
impl KDTC_PIPELINE_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KDTC_PIPELINE_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDTC_PIPELINE_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDTC_PIPELINE_BYPASS` writer - "]
pub struct KDTC_PIPELINE_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> KDTC_PIPELINE_BYPASS_W<'a> {
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
#[doc = "Field `KTDC_IN` reader - "]
pub struct KTDC_IN_R(crate::FieldReader<u16, u16>);
impl KTDC_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        KTDC_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KTDC_IN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KTDC_IN` writer - "]
pub struct KTDC_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> KTDC_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | ((value as u32 & 0x01ff) << 6);
        self.w
    }
}
#[doc = "Field `KDTC_ALPHA` reader - "]
pub struct KDTC_ALPHA_R(crate::FieldReader<u8, u8>);
impl KDTC_ALPHA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KDTC_ALPHA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KDTC_ALPHA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KDTC_ALPHA` writer - "]
pub struct KDTC_ALPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> KDTC_ALPHA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:31"]
    #[inline(always)]
    pub fn kdtcic(&self) -> KDTCIC_R {
        KDTCIC_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn kdtccn_ic(&self) -> KDTCCN_IC_R {
        KDTCCN_IC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn kdtc_pipeline_bypass(&self) -> KDTC_PIPELINE_BYPASS_R {
        KDTC_PIPELINE_BYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 6:14"]
    #[inline(always)]
    pub fn ktdc_in(&self) -> KTDC_IN_R {
        KTDC_IN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn kdtc_alpha(&self) -> KDTC_ALPHA_R {
        KDTC_ALPHA_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 23:31"]
    #[inline(always)]
    pub fn kdtcic(&mut self) -> KDTCIC_W {
        KDTCIC_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn kdtccn_ic(&mut self) -> KDTCCN_IC_W {
        KDTCCN_IC_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn kdtc_pipeline_bypass(&mut self) -> KDTC_PIPELINE_BYPASS_W {
        KDTC_PIPELINE_BYPASS_W { w: self }
    }
    #[doc = "Bits 6:14"]
    #[inline(always)]
    pub fn ktdc_in(&mut self) -> KTDC_IN_W {
        KTDC_IN_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn kdtc_alpha(&mut self) -> KDTC_ALPHA_W {
        KDTC_ALPHA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_kdtctdc_cal_ctrl1_reg](index.html) module"]
pub struct ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC;
impl crate::RegisterSpec for ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_kdtctdc_cal_ctrl1_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_kdtctdc_cal_ctrl1_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_KDTCTDC_CAL_CTRL1_REG to value 0x7fff_ffff"]
impl crate::Resettable for ADPLL_KDTCTDC_CAL_CTRL1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff_ffff
    }
}
