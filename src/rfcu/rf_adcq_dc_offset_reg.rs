#[doc = "Register `RF_ADCQ_DC_OFFSET_REG` reader"]
pub struct R(crate::R<RF_ADCQ_DC_OFFSET_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ADCQ_DC_OFFSET_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_ADCQ_DC_OFFSET_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_ADCQ_DC_OFFSET_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_ADCQ_DC_OFFSET_REG` writer"]
pub struct W(crate::W<RF_ADCQ_DC_OFFSET_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ADCQ_DC_OFFSET_REG_SPEC>;
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
impl From<crate::W<RF_ADCQ_DC_OFFSET_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_ADCQ_DC_OFFSET_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_OFFN_Q_RD` reader - "]
pub struct ADC_OFFN_Q_RD_R(crate::FieldReader<u16, u16>);
impl ADC_OFFN_Q_RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADC_OFFN_Q_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_OFFN_Q_RD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_OFFP_Q_RD` reader - "]
pub struct ADC_OFFP_Q_RD_R(crate::FieldReader<u16, u16>);
impl ADC_OFFP_Q_RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADC_OFFP_Q_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_OFFP_Q_RD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn adc_offn_q_rd(&self) -> ADC_OFFN_Q_RD_R {
        ADC_OFFN_Q_RD_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn adc_offp_q_rd(&self) -> ADC_OFFP_Q_RD_R {
        ADC_OFFP_Q_RD_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_adcq_dc_offset_reg](index.html) module"]
pub struct RF_ADCQ_DC_OFFSET_REG_SPEC;
impl crate::RegisterSpec for RF_ADCQ_DC_OFFSET_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_adcq_dc_offset_reg::R](R) reader structure"]
impl crate::Readable for RF_ADCQ_DC_OFFSET_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_adcq_dc_offset_reg::W](W) writer structure"]
impl crate::Writable for RF_ADCQ_DC_OFFSET_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_ADCQ_DC_OFFSET_REG to value 0x0002_0100"]
impl crate::Resettable for RF_ADCQ_DC_OFFSET_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0100
    }
}
