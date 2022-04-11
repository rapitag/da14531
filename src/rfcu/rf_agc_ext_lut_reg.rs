#[doc = "Register `RF_AGC_EXT_LUT_REG` reader"]
pub struct R(crate::R<RF_AGC_EXT_LUT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_AGC_EXT_LUT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_AGC_EXT_LUT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_AGC_EXT_LUT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_AGC_EXT_LUT_REG` writer"]
pub struct W(crate::W<RF_AGC_EXT_LUT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_AGC_EXT_LUT_REG_SPEC>;
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
impl From<crate::W<RF_AGC_EXT_LUT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_AGC_EXT_LUT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGC_EXT_LUT` reader - "]
pub struct AGC_EXT_LUT_R(crate::FieldReader<u16, u16>);
impl AGC_EXT_LUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        AGC_EXT_LUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AGC_EXT_LUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AGC_EXT_LUT` writer - "]
pub struct AGC_EXT_LUT_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_EXT_LUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn agc_ext_lut(&self) -> AGC_EXT_LUT_R {
        AGC_EXT_LUT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn agc_ext_lut(&mut self) -> AGC_EXT_LUT_W {
        AGC_EXT_LUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_agc_ext_lut_reg](index.html) module"]
pub struct RF_AGC_EXT_LUT_REG_SPEC;
impl crate::RegisterSpec for RF_AGC_EXT_LUT_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_agc_ext_lut_reg::R](R) reader structure"]
impl crate::Readable for RF_AGC_EXT_LUT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_agc_ext_lut_reg::W](W) writer structure"]
impl crate::Writable for RF_AGC_EXT_LUT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_AGC_EXT_LUT_REG to value 0"]
impl crate::Resettable for RF_AGC_EXT_LUT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
