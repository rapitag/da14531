#[doc = "Register `ADPLL_ANATST_RD_REG` reader"]
pub struct R(crate::R<ADPLL_ANATST_RD_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_ANATST_RD_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_ANATST_RD_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_ANATST_RD_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_ANATST_RD_REG` writer"]
pub struct W(crate::W<ADPLL_ANATST_RD_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_ANATST_RD_REG_SPEC>;
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
impl From<crate::W<ADPLL_ANATST_RD_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_ANATST_RD_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANATSTSPARE_IN` reader - "]
pub struct ANATSTSPARE_IN_R(crate::FieldReader<u16, u16>);
impl ANATSTSPARE_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ANATSTSPARE_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANATSTSPARE_IN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn anatstspare_in(&self) -> ANATSTSPARE_IN_R {
        ANATSTSPARE_IN_R::new((self.bits & 0xffff) as u16)
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_anatst_rd_reg](index.html) module"]
pub struct ADPLL_ANATST_RD_REG_SPEC;
impl crate::RegisterSpec for ADPLL_ANATST_RD_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_anatst_rd_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_ANATST_RD_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_anatst_rd_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_ANATST_RD_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_ANATST_RD_REG to value 0"]
impl crate::Resettable for ADPLL_ANATST_RD_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
