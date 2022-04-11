#[doc = "Register `ROMBIST_RESULTL_REG` reader"]
pub struct R(crate::R<ROMBIST_RESULTL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROMBIST_RESULTL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROMBIST_RESULTL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROMBIST_RESULTL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROMBIST_RESULTL_REG` writer"]
pub struct W(crate::W<ROMBIST_RESULTL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROMBIST_RESULTL_REG_SPEC>;
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
impl From<crate::W<ROMBIST_RESULTL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROMBIST_RESULTL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROMBIST_RESULTL` reader - "]
pub struct ROMBIST_RESULTL_R(crate::FieldReader<u16, u16>);
impl ROMBIST_RESULTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ROMBIST_RESULTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROMBIST_RESULTL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rombist_resultl(&self) -> ROMBIST_RESULTL_R {
        ROMBIST_RESULTL_R::new(self.bits)
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rombist_resultl_reg](index.html) module"]
pub struct ROMBIST_RESULTL_REG_SPEC;
impl crate::RegisterSpec for ROMBIST_RESULTL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rombist_resultl_reg::R](R) reader structure"]
impl crate::Readable for ROMBIST_RESULTL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rombist_resultl_reg::W](W) writer structure"]
impl crate::Writable for ROMBIST_RESULTL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROMBIST_RESULTL_REG to value 0"]
impl crate::Resettable for ROMBIST_RESULTL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
