#[doc = "Register `GP_ADC_CTRL3_REG` reader"]
pub struct R(crate::R<GP_ADC_CTRL3_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_ADC_CTRL3_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_ADC_CTRL3_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_ADC_CTRL3_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_ADC_CTRL3_REG` writer"]
pub struct W(crate::W<GP_ADC_CTRL3_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_ADC_CTRL3_REG_SPEC>;
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
impl From<crate::W<GP_ADC_CTRL3_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_ADC_CTRL3_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GP_ADC_INTERVAL` reader - Defines the interval between two ADC conversions in case GP_ADC_CONT is set. 0: No extra delay between two conversions. 1: 1.024 ms interval between two conversions. 2: 2.048 ms interval between two conversions. 255: 261.12 ms interval between two conversions."]
pub struct GP_ADC_INTERVAL_R(crate::FieldReader<u8, u8>);
impl GP_ADC_INTERVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GP_ADC_INTERVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_INTERVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_INTERVAL` writer - Defines the interval between two ADC conversions in case GP_ADC_CONT is set. 0: No extra delay between two conversions. 1: 1.024 ms interval between two conversions. 2: 2.048 ms interval between two conversions. 255: 261.12 ms interval between two conversions."]
pub struct GP_ADC_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `GP_ADC_EN_DEL` reader - Defines the delay for enabling the ADC after enabling the LDO. 0: Not allowed 1: 4x ADC_CLK period. n: n*4x ADC_CLK period."]
pub struct GP_ADC_EN_DEL_R(crate::FieldReader<u8, u8>);
impl GP_ADC_EN_DEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GP_ADC_EN_DEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_EN_DEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_EN_DEL` writer - Defines the delay for enabling the ADC after enabling the LDO. 0: Not allowed 1: 4x ADC_CLK period. n: n*4x ADC_CLK period."]
pub struct GP_ADC_EN_DEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_EN_DEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Defines the interval between two ADC conversions in case GP_ADC_CONT is set. 0: No extra delay between two conversions. 1: 1.024 ms interval between two conversions. 2: 2.048 ms interval between two conversions. 255: 261.12 ms interval between two conversions."]
    #[inline(always)]
    pub fn gp_adc_interval(&self) -> GP_ADC_INTERVAL_R {
        GP_ADC_INTERVAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Defines the delay for enabling the ADC after enabling the LDO. 0: Not allowed 1: 4x ADC_CLK period. n: n*4x ADC_CLK period."]
    #[inline(always)]
    pub fn gp_adc_en_del(&self) -> GP_ADC_EN_DEL_R {
        GP_ADC_EN_DEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Defines the interval between two ADC conversions in case GP_ADC_CONT is set. 0: No extra delay between two conversions. 1: 1.024 ms interval between two conversions. 2: 2.048 ms interval between two conversions. 255: 261.12 ms interval between two conversions."]
    #[inline(always)]
    pub fn gp_adc_interval(&mut self) -> GP_ADC_INTERVAL_W {
        GP_ADC_INTERVAL_W { w: self }
    }
    #[doc = "Bits 0:7 - Defines the delay for enabling the ADC after enabling the LDO. 0: Not allowed 1: 4x ADC_CLK period. n: n*4x ADC_CLK period."]
    #[inline(always)]
    pub fn gp_adc_en_del(&mut self) -> GP_ADC_EN_DEL_W {
        GP_ADC_EN_DEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose ADC Third Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_adc_ctrl3_reg](index.html) module"]
pub struct GP_ADC_CTRL3_REG_SPEC;
impl crate::RegisterSpec for GP_ADC_CTRL3_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gp_adc_ctrl3_reg::R](R) reader structure"]
impl crate::Readable for GP_ADC_CTRL3_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_adc_ctrl3_reg::W](W) writer structure"]
impl crate::Writable for GP_ADC_CTRL3_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_ADC_CTRL3_REG to value 0x40"]
impl crate::Resettable for GP_ADC_CTRL3_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
