#[doc = "Register `GP_ADC_RESULT_REG` reader"]
pub struct R(crate::R<GP_ADC_RESULT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_ADC_RESULT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_ADC_RESULT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_ADC_RESULT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_ADC_RESULT_REG` writer"]
pub struct W(crate::W<GP_ADC_RESULT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_ADC_RESULT_REG_SPEC>;
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
impl From<crate::W<GP_ADC_RESULT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_ADC_RESULT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GP_ADC_VAL` reader - Returns the 10 up to 16 bits linear value of the last AD conversion. The upper 10 bits are always valid, the lower 6 bits are only valid in case oversampling has been applied. Two samples results in one extra bit and 64 samples results in six extra bits."]
pub struct GP_ADC_VAL_R(crate::FieldReader<u16, u16>);
impl GP_ADC_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GP_ADC_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_VAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Returns the 10 up to 16 bits linear value of the last AD conversion. The upper 10 bits are always valid, the lower 6 bits are only valid in case oversampling has been applied. Two samples results in one extra bit and 64 samples results in six extra bits."]
    #[inline(always)]
    pub fn gp_adc_val(&self) -> GP_ADC_VAL_R {
        GP_ADC_VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose ADC Result Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_adc_result_reg](index.html) module"]
pub struct GP_ADC_RESULT_REG_SPEC;
impl crate::RegisterSpec for GP_ADC_RESULT_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gp_adc_result_reg::R](R) reader structure"]
impl crate::Readable for GP_ADC_RESULT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_adc_result_reg::W](W) writer structure"]
impl crate::Writable for GP_ADC_RESULT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_ADC_RESULT_REG to value 0"]
impl crate::Resettable for GP_ADC_RESULT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
