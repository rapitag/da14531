#[doc = "Register `RF_LNA_CTRL3_REG` reader"]
pub struct R(crate::R<RF_LNA_CTRL3_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_LNA_CTRL3_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_LNA_CTRL3_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_LNA_CTRL3_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_LNA_CTRL3_REG` writer"]
pub struct W(crate::W<RF_LNA_CTRL3_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_LNA_CTRL3_REG_SPEC>;
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
impl From<crate::W<RF_LNA_CTRL3_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_LNA_CTRL3_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LNA_SPARE` reader - "]
pub struct LNA_SPARE_R(crate::FieldReader<u8, u8>);
impl LNA_SPARE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_SPARE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_SPARE` writer - "]
pub struct LNA_SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `LNA_MODE_GAIN4_LP` reader - "]
pub struct LNA_MODE_GAIN4_LP_R(crate::FieldReader<u8, u8>);
impl LNA_MODE_GAIN4_LP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_MODE_GAIN4_LP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_MODE_GAIN4_LP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_MODE_GAIN4_LP` writer - "]
pub struct LNA_MODE_GAIN4_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_MODE_GAIN4_LP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `LNA_MODE_GAIN3_LP` reader - "]
pub struct LNA_MODE_GAIN3_LP_R(crate::FieldReader<u8, u8>);
impl LNA_MODE_GAIN3_LP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_MODE_GAIN3_LP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_MODE_GAIN3_LP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_MODE_GAIN3_LP` writer - "]
pub struct LNA_MODE_GAIN3_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_MODE_GAIN3_LP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `LNA_MODE_GAIN2_LP` reader - "]
pub struct LNA_MODE_GAIN2_LP_R(crate::FieldReader<u8, u8>);
impl LNA_MODE_GAIN2_LP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_MODE_GAIN2_LP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_MODE_GAIN2_LP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_MODE_GAIN2_LP` writer - "]
pub struct LNA_MODE_GAIN2_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_MODE_GAIN2_LP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `LNA_MODE_GAIN1_LP` reader - "]
pub struct LNA_MODE_GAIN1_LP_R(crate::FieldReader<u8, u8>);
impl LNA_MODE_GAIN1_LP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_MODE_GAIN1_LP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_MODE_GAIN1_LP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_MODE_GAIN1_LP` writer - "]
pub struct LNA_MODE_GAIN1_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_MODE_GAIN1_LP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `LNA_MODE_GAIN0_LP` reader - "]
pub struct LNA_MODE_GAIN0_LP_R(crate::FieldReader<u8, u8>);
impl LNA_MODE_GAIN0_LP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_MODE_GAIN0_LP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_MODE_GAIN0_LP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_MODE_GAIN0_LP` writer - "]
pub struct LNA_MODE_GAIN0_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_MODE_GAIN0_LP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `LNA_TRIM_CASC` reader - "]
pub struct LNA_TRIM_CASC_R(crate::FieldReader<u8, u8>);
impl LNA_TRIM_CASC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_TRIM_CASC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_TRIM_CASC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_TRIM_CASC` writer - "]
pub struct LNA_TRIM_CASC_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_TRIM_CASC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lna_spare(&self) -> LNA_SPARE_R {
        LNA_SPARE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lna_mode_gain4_lp(&self) -> LNA_MODE_GAIN4_LP_R {
        LNA_MODE_GAIN4_LP_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lna_mode_gain3_lp(&self) -> LNA_MODE_GAIN3_LP_R {
        LNA_MODE_GAIN3_LP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lna_mode_gain2_lp(&self) -> LNA_MODE_GAIN2_LP_R {
        LNA_MODE_GAIN2_LP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lna_mode_gain1_lp(&self) -> LNA_MODE_GAIN1_LP_R {
        LNA_MODE_GAIN1_LP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lna_mode_gain0_lp(&self) -> LNA_MODE_GAIN0_LP_R {
        LNA_MODE_GAIN0_LP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn lna_trim_casc(&self) -> LNA_TRIM_CASC_R {
        LNA_TRIM_CASC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lna_spare(&mut self) -> LNA_SPARE_W {
        LNA_SPARE_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lna_mode_gain4_lp(&mut self) -> LNA_MODE_GAIN4_LP_W {
        LNA_MODE_GAIN4_LP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lna_mode_gain3_lp(&mut self) -> LNA_MODE_GAIN3_LP_W {
        LNA_MODE_GAIN3_LP_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lna_mode_gain2_lp(&mut self) -> LNA_MODE_GAIN2_LP_W {
        LNA_MODE_GAIN2_LP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lna_mode_gain1_lp(&mut self) -> LNA_MODE_GAIN1_LP_W {
        LNA_MODE_GAIN1_LP_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lna_mode_gain0_lp(&mut self) -> LNA_MODE_GAIN0_LP_W {
        LNA_MODE_GAIN0_LP_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn lna_trim_casc(&mut self) -> LNA_TRIM_CASC_W {
        LNA_TRIM_CASC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_lna_ctrl3_reg](index.html) module"]
pub struct RF_LNA_CTRL3_REG_SPEC;
impl crate::RegisterSpec for RF_LNA_CTRL3_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_lna_ctrl3_reg::R](R) reader structure"]
impl crate::Readable for RF_LNA_CTRL3_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_lna_ctrl3_reg::W](W) writer structure"]
impl crate::Writable for RF_LNA_CTRL3_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_LNA_CTRL3_REG to value 0x0011_1114"]
impl crate::Resettable for RF_LNA_CTRL3_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0011_1114
    }
}
