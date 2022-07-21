#[doc = "Register `GP_ADC_SEL_REG` reader"]
pub struct R(crate::R<GP_ADC_SEL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_ADC_SEL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_ADC_SEL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_ADC_SEL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_ADC_SEL_REG` writer"]
pub struct W(crate::W<GP_ADC_SEL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_ADC_SEL_REG_SPEC>;
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
impl From<crate::W<GP_ADC_SEL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_ADC_SEL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GP_ADC_SEL_P` reader - ADC positive input selection. 0: ADC0 (P0\\[1\\]) 1: ADC1 (P0\\[2\\]) 2: ADC2 (P0\\[6\\]) 3: ADC3 (P0\\[7\\]) 4: Temperature Sensor 5: VBAT_HIGH 6: VBAT_LOW 7: VDDD"]
pub struct GP_ADC_SEL_P_R(crate::FieldReader<u8, u8>);
impl GP_ADC_SEL_P_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GP_ADC_SEL_P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_SEL_P_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_SEL_P` writer - ADC positive input selection. 0: ADC0 (P0\\[1\\]) 1: ADC1 (P0\\[2\\]) 2: ADC2 (P0\\[6\\]) 3: ADC3 (P0\\[7\\]) 4: Temperature Sensor 5: VBAT_HIGH 6: VBAT_LOW 7: VDDD"]
pub struct GP_ADC_SEL_P_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_SEL_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u8 & 7) << 4);
        self.w
    }
}
#[doc = "Field `GP_ADC_SEL_N` reader - ADC negative input selection. Differential only (GP_ADC_SE=0). 0: ADC0 (P0\\[1\\]) 1: ADC1 (P0\\[2\\]) 2: ADC2 (P0\\[6\\]) 3: ADC3 (P0\\[7\\]) All other combinations are reserved."]
pub struct GP_ADC_SEL_N_R(crate::FieldReader<u8, u8>);
impl GP_ADC_SEL_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GP_ADC_SEL_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_SEL_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_SEL_N` writer - ADC negative input selection. Differential only (GP_ADC_SE=0). 0: ADC0 (P0\\[1\\]) 1: ADC1 (P0\\[2\\]) 2: ADC2 (P0\\[6\\]) 3: ADC3 (P0\\[7\\]) All other combinations are reserved."]
pub struct GP_ADC_SEL_N_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_SEL_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u8 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - ADC positive input selection. 0: ADC0 (P0\\[1\\]) 1: ADC1 (P0\\[2\\]) 2: ADC2 (P0\\[6\\]) 3: ADC3 (P0\\[7\\]) 4: Temperature Sensor 5: VBAT_HIGH 6: VBAT_LOW 7: VDDD"]
    #[inline(always)]
    pub fn gp_adc_sel_p(&self) -> GP_ADC_SEL_P_R {
        GP_ADC_SEL_P_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 0:2 - ADC negative input selection. Differential only (GP_ADC_SE=0). 0: ADC0 (P0\\[1\\]) 1: ADC1 (P0\\[2\\]) 2: ADC2 (P0\\[6\\]) 3: ADC3 (P0\\[7\\]) All other combinations are reserved."]
    #[inline(always)]
    pub fn gp_adc_sel_n(&self) -> GP_ADC_SEL_N_R {
        GP_ADC_SEL_N_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - ADC positive input selection. 0: ADC0 (P0\\[1\\]) 1: ADC1 (P0\\[2\\]) 2: ADC2 (P0\\[6\\]) 3: ADC3 (P0\\[7\\]) 4: Temperature Sensor 5: VBAT_HIGH 6: VBAT_LOW 7: VDDD"]
    #[inline(always)]
    pub fn gp_adc_sel_p(&mut self) -> GP_ADC_SEL_P_W {
        GP_ADC_SEL_P_W { w: self }
    }
    #[doc = "Bits 0:2 - ADC negative input selection. Differential only (GP_ADC_SE=0). 0: ADC0 (P0\\[1\\]) 1: ADC1 (P0\\[2\\]) 2: ADC2 (P0\\[6\\]) 3: ADC3 (P0\\[7\\]) All other combinations are reserved."]
    #[inline(always)]
    pub fn gp_adc_sel_n(&mut self) -> GP_ADC_SEL_N_W {
        GP_ADC_SEL_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose ADC Input Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_adc_sel_reg](index.html) module"]
pub struct GP_ADC_SEL_REG_SPEC;
impl crate::RegisterSpec for GP_ADC_SEL_REG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gp_adc_sel_reg::R](R) reader structure"]
impl crate::Readable for GP_ADC_SEL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_adc_sel_reg::W](W) writer structure"]
impl crate::Writable for GP_ADC_SEL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_ADC_SEL_REG to value 0"]
impl crate::Resettable for GP_ADC_SEL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
