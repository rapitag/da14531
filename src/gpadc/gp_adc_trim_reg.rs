#[doc = "Register `GP_ADC_TRIM_REG` reader"]
pub struct R(crate::R<GP_ADC_TRIM_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_ADC_TRIM_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_ADC_TRIM_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_ADC_TRIM_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_ADC_TRIM_REG` writer"]
pub struct W(crate::W<GP_ADC_TRIM_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_ADC_TRIM_REG_SPEC>;
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
impl From<crate::W<GP_ADC_TRIM_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_ADC_TRIM_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GP_ADC_LDO_LEVEL` reader - GPADC LDO level 0: 825mV 1: 850mV 2: 875mV 3: 900mV (reset) 4: 925mV (default) 5: 950mV 6: 975mV 7:1000mV"]
pub struct GP_ADC_LDO_LEVEL_R(crate::FieldReader<u8, u8>);
impl GP_ADC_LDO_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GP_ADC_LDO_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_LDO_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_LDO_LEVEL` writer - GPADC LDO level 0: 825mV 1: 850mV 2: 875mV 3: 900mV (reset) 4: 925mV (default) 5: 950mV 6: 975mV 7:1000mV"]
pub struct GP_ADC_LDO_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_LDO_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u16 & 7) << 4);
        self.w
    }
}
#[doc = "Field `GP_ADC_OFFS_SH_VREF` reader - Offset Shifter common-mode reference fine trimming: 2mV/LSB Default = mid-scale at 1000"]
pub struct GP_ADC_OFFS_SH_VREF_R(crate::FieldReader<u8, u8>);
impl GP_ADC_OFFS_SH_VREF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GP_ADC_OFFS_SH_VREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GP_ADC_OFFS_SH_VREF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GP_ADC_OFFS_SH_VREF` writer - Offset Shifter common-mode reference fine trimming: 2mV/LSB Default = mid-scale at 1000"]
pub struct GP_ADC_OFFS_SH_VREF_W<'a> {
    w: &'a mut W,
}
impl<'a> GP_ADC_OFFS_SH_VREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u16 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - GPADC LDO level 0: 825mV 1: 850mV 2: 875mV 3: 900mV (reset) 4: 925mV (default) 5: 950mV 6: 975mV 7:1000mV"]
    #[inline(always)]
    pub fn gp_adc_ldo_level(&self) -> GP_ADC_LDO_LEVEL_R {
        GP_ADC_LDO_LEVEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 0:3 - Offset Shifter common-mode reference fine trimming: 2mV/LSB Default = mid-scale at 1000"]
    #[inline(always)]
    pub fn gp_adc_offs_sh_vref(&self) -> GP_ADC_OFFS_SH_VREF_R {
        GP_ADC_OFFS_SH_VREF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - GPADC LDO level 0: 825mV 1: 850mV 2: 875mV 3: 900mV (reset) 4: 925mV (default) 5: 950mV 6: 975mV 7:1000mV"]
    #[inline(always)]
    pub fn gp_adc_ldo_level(&mut self) -> GP_ADC_LDO_LEVEL_W {
        GP_ADC_LDO_LEVEL_W { w: self }
    }
    #[doc = "Bits 0:3 - Offset Shifter common-mode reference fine trimming: 2mV/LSB Default = mid-scale at 1000"]
    #[inline(always)]
    pub fn gp_adc_offs_sh_vref(&mut self) -> GP_ADC_OFFS_SH_VREF_W {
        GP_ADC_OFFS_SH_VREF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose ADC Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_adc_trim_reg](index.html) module"]
pub struct GP_ADC_TRIM_REG_SPEC;
impl crate::RegisterSpec for GP_ADC_TRIM_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gp_adc_trim_reg::R](R) reader structure"]
impl crate::Readable for GP_ADC_TRIM_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_adc_trim_reg::W](W) writer structure"]
impl crate::Writable for GP_ADC_TRIM_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_ADC_TRIM_REG to value 0x38"]
impl crate::Resettable for GP_ADC_TRIM_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x38
    }
}
