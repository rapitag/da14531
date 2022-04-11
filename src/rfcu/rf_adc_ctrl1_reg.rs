#[doc = "Register `RF_ADC_CTRL1_REG` reader"]
pub struct R(crate::R<RF_ADC_CTRL1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ADC_CTRL1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_ADC_CTRL1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_ADC_CTRL1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_ADC_CTRL1_REG` writer"]
pub struct W(crate::W<RF_ADC_CTRL1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ADC_CTRL1_REG_SPEC>;
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
impl From<crate::W<RF_ADC_CTRL1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_ADC_CTRL1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_SIGN` reader - "]
pub struct ADC_SIGN_R(crate::FieldReader<bool, bool>);
impl ADC_SIGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_SIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_SIGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_SIGN` writer - "]
pub struct ADC_SIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SIGN_W<'a> {
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
#[doc = "Field `ADC_MUTE` reader - "]
pub struct ADC_MUTE_R(crate::FieldReader<bool, bool>);
impl ADC_MUTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_MUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_MUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_MUTE` writer - "]
pub struct ADC_MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_MUTE_W<'a> {
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
#[doc = "Field `ADC_DC_OFFSET_SEL` reader - "]
pub struct ADC_DC_OFFSET_SEL_R(crate::FieldReader<bool, bool>);
impl ADC_DC_OFFSET_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_DC_OFFSET_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_DC_OFFSET_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_DC_OFFSET_SEL` writer - "]
pub struct ADC_DC_OFFSET_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DC_OFFSET_SEL_W<'a> {
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
    pub fn adc_sign(&self) -> ADC_SIGN_R {
        ADC_SIGN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_mute(&self) -> ADC_MUTE_R {
        ADC_MUTE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc_dc_offset_sel(&self) -> ADC_DC_OFFSET_SEL_R {
        ADC_DC_OFFSET_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adc_sign(&mut self) -> ADC_SIGN_W {
        ADC_SIGN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_mute(&mut self) -> ADC_MUTE_W {
        ADC_MUTE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc_dc_offset_sel(&mut self) -> ADC_DC_OFFSET_SEL_W {
        ADC_DC_OFFSET_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_adc_ctrl1_reg](index.html) module"]
pub struct RF_ADC_CTRL1_REG_SPEC;
impl crate::RegisterSpec for RF_ADC_CTRL1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_adc_ctrl1_reg::R](R) reader structure"]
impl crate::Readable for RF_ADC_CTRL1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_adc_ctrl1_reg::W](W) writer structure"]
impl crate::Writable for RF_ADC_CTRL1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_ADC_CTRL1_REG to value 0"]
impl crate::Resettable for RF_ADC_CTRL1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
