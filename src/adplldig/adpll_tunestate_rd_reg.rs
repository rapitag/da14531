#[doc = "Register `ADPLL_TUNESTATE_RD_REG` reader"]
pub struct R(crate::R<ADPLL_TUNESTATE_RD_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_TUNESTATE_RD_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_TUNESTATE_RD_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_TUNESTATE_RD_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_TUNESTATE_RD_REG` writer"]
pub struct W(crate::W<ADPLL_TUNESTATE_RD_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_TUNESTATE_RD_REG_SPEC>;
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
impl From<crate::W<ADPLL_TUNESTATE_RD_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_TUNESTATE_RD_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMRVAL` reader - "]
pub struct TMRVAL_R(crate::FieldReader<u16, u16>);
impl TMRVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TMRVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRVAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TUNE_STATE` reader - "]
pub struct TUNE_STATE_R(crate::FieldReader<u8, u8>);
impl TUNE_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TUNE_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TUNE_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 4:13"]
    #[inline(always)]
    pub fn tmrval(&self) -> TMRVAL_R {
        TMRVAL_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tune_state(&self) -> TUNE_STATE_R {
        TUNE_STATE_R::new((self.bits & 0x0f) as u8)
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_tunestate_rd_reg](index.html) module"]
pub struct ADPLL_TUNESTATE_RD_REG_SPEC;
impl crate::RegisterSpec for ADPLL_TUNESTATE_RD_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_tunestate_rd_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_TUNESTATE_RD_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_tunestate_rd_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_TUNESTATE_RD_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_TUNESTATE_RD_REG to value 0"]
impl crate::Resettable for ADPLL_TUNESTATE_RD_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
