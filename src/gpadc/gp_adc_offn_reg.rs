#[doc = "Register `GP_ADC_OFFN_REG` reader"]
pub struct R(crate::R<GP_ADC_OFFN_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_ADC_OFFN_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_ADC_OFFN_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_ADC_OFFN_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_ADC_OFFN_REG` writer"]
pub struct W(crate::W<GP_ADC_OFFN_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_ADC_OFFN_REG_SPEC>;
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
impl From<crate::W<GP_ADC_OFFN_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_ADC_OFFN_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GP_ADC_OFFN` reader - Offset adjust of 'negative' array of ADC-network (effective if \"GP_ADC_SE=0\", or \"GP_ADC_SE=1 AND GP_ADC_SIGN=1 OR GP_ADC_CHOP=1\")"]
pub struct GP_ADC_OFFN_R(crate::FieldReader<u16, u16>);
impl GP_ADC_OFFN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GP_ADC_OFFN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_OFFN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_OFFN` writer - Offset adjust of 'negative' array of ADC-network (effective if \"GP_ADC_SE=0\", or \"GP_ADC_SE=1 AND GP_ADC_SIGN=1 OR GP_ADC_CHOP=1\")"]
pub struct GP_ADC_OFFN_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_OFFN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u16 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Offset adjust of 'negative' array of ADC-network (effective if \"GP_ADC_SE=0\", or \"GP_ADC_SE=1 AND GP_ADC_SIGN=1 OR GP_ADC_CHOP=1\")"]
    #[inline(always)]
    pub fn gp_adc_offn(&self) -> GP_ADC_OFFN_R {
        GP_ADC_OFFN_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Offset adjust of 'negative' array of ADC-network (effective if \"GP_ADC_SE=0\", or \"GP_ADC_SE=1 AND GP_ADC_SIGN=1 OR GP_ADC_CHOP=1\")"]
    #[inline(always)]
    pub fn gp_adc_offn(&mut self) -> GP_ADC_OFFN_W {
        GP_ADC_OFFN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose ADC Negative Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_adc_offn_reg](index.html) module"]
pub struct GP_ADC_OFFN_REG_SPEC;
impl crate::RegisterSpec for GP_ADC_OFFN_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gp_adc_offn_reg::R](R) reader structure"]
impl crate::Readable for GP_ADC_OFFN_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_adc_offn_reg::W](W) writer structure"]
impl crate::Writable for GP_ADC_OFFN_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_ADC_OFFN_REG to value 0x0200"]
impl crate::Resettable for GP_ADC_OFFN_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
