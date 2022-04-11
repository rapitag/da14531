#[doc = "Register `RF_LNA_CTRL1_REG` reader"]
pub struct R(crate::R<RF_LNA_CTRL1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_LNA_CTRL1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_LNA_CTRL1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_LNA_CTRL1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_LNA_CTRL1_REG` writer"]
pub struct W(crate::W<RF_LNA_CTRL1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_LNA_CTRL1_REG_SPEC>;
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
impl From<crate::W<RF_LNA_CTRL1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_LNA_CTRL1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LNA_TRIM_GAIN4_HP` reader - "]
pub struct LNA_TRIM_GAIN4_HP_R(crate::FieldReader<u8, u8>);
impl LNA_TRIM_GAIN4_HP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_TRIM_GAIN4_HP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_TRIM_GAIN4_HP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_TRIM_GAIN4_HP` writer - "]
pub struct LNA_TRIM_GAIN4_HP_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_TRIM_GAIN4_HP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `LNA_TRIM_GAIN3_HP` reader - "]
pub struct LNA_TRIM_GAIN3_HP_R(crate::FieldReader<u8, u8>);
impl LNA_TRIM_GAIN3_HP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_TRIM_GAIN3_HP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_TRIM_GAIN3_HP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_TRIM_GAIN3_HP` writer - "]
pub struct LNA_TRIM_GAIN3_HP_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_TRIM_GAIN3_HP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | ((value as u32 & 0x1f) << 15);
        self.w
    }
}
#[doc = "Field `LNA_TRIM_GAIN2_HP` reader - "]
pub struct LNA_TRIM_GAIN2_HP_R(crate::FieldReader<u8, u8>);
impl LNA_TRIM_GAIN2_HP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_TRIM_GAIN2_HP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_TRIM_GAIN2_HP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_TRIM_GAIN2_HP` writer - "]
pub struct LNA_TRIM_GAIN2_HP_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_TRIM_GAIN2_HP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `LNA_TRIM_GAIN1_HP` reader - "]
pub struct LNA_TRIM_GAIN1_HP_R(crate::FieldReader<u8, u8>);
impl LNA_TRIM_GAIN1_HP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_TRIM_GAIN1_HP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_TRIM_GAIN1_HP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_TRIM_GAIN1_HP` writer - "]
pub struct LNA_TRIM_GAIN1_HP_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_TRIM_GAIN1_HP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `LNA_TRIM_GAIN0_HP` reader - "]
pub struct LNA_TRIM_GAIN0_HP_R(crate::FieldReader<u8, u8>);
impl LNA_TRIM_GAIN0_HP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_TRIM_GAIN0_HP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_TRIM_GAIN0_HP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_TRIM_GAIN0_HP` writer - "]
pub struct LNA_TRIM_GAIN0_HP_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_TRIM_GAIN0_HP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn lna_trim_gain4_hp(&self) -> LNA_TRIM_GAIN4_HP_R {
        LNA_TRIM_GAIN4_HP_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn lna_trim_gain3_hp(&self) -> LNA_TRIM_GAIN3_HP_R {
        LNA_TRIM_GAIN3_HP_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn lna_trim_gain2_hp(&self) -> LNA_TRIM_GAIN2_HP_R {
        LNA_TRIM_GAIN2_HP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn lna_trim_gain1_hp(&self) -> LNA_TRIM_GAIN1_HP_R {
        LNA_TRIM_GAIN1_HP_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lna_trim_gain0_hp(&self) -> LNA_TRIM_GAIN0_HP_R {
        LNA_TRIM_GAIN0_HP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn lna_trim_gain4_hp(&mut self) -> LNA_TRIM_GAIN4_HP_W {
        LNA_TRIM_GAIN4_HP_W { w: self }
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn lna_trim_gain3_hp(&mut self) -> LNA_TRIM_GAIN3_HP_W {
        LNA_TRIM_GAIN3_HP_W { w: self }
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn lna_trim_gain2_hp(&mut self) -> LNA_TRIM_GAIN2_HP_W {
        LNA_TRIM_GAIN2_HP_W { w: self }
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn lna_trim_gain1_hp(&mut self) -> LNA_TRIM_GAIN1_HP_W {
        LNA_TRIM_GAIN1_HP_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn lna_trim_gain0_hp(&mut self) -> LNA_TRIM_GAIN0_HP_W {
        LNA_TRIM_GAIN0_HP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_lna_ctrl1_reg](index.html) module"]
pub struct RF_LNA_CTRL1_REG_SPEC;
impl crate::RegisterSpec for RF_LNA_CTRL1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_lna_ctrl1_reg::R](R) reader structure"]
impl crate::Readable for RF_LNA_CTRL1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_lna_ctrl1_reg::W](W) writer structure"]
impl crate::Writable for RF_LNA_CTRL1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_LNA_CTRL1_REG to value 0x0108_4210"]
impl crate::Resettable for RF_LNA_CTRL1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0108_4210
    }
}
