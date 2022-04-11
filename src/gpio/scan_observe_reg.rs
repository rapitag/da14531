#[doc = "Register `SCAN_OBSERVE_REG` reader"]
pub struct R(crate::R<SCAN_OBSERVE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCAN_OBSERVE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCAN_OBSERVE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCAN_OBSERVE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCAN_OBSERVE_REG` writer"]
pub struct W(crate::W<SCAN_OBSERVE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCAN_OBSERVE_REG_SPEC>;
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
impl From<crate::W<SCAN_OBSERVE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCAN_OBSERVE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCAN_FEEDBACK_MUX` reader - "]
pub struct SCAN_FEEDBACK_MUX_R(crate::FieldReader<u16, u16>);
impl SCAN_FEEDBACK_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SCAN_FEEDBACK_MUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCAN_FEEDBACK_MUX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn scan_feedback_mux(&self) -> SCAN_FEEDBACK_MUX_R {
        SCAN_FEEDBACK_MUX_R::new(self.bits)
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan_observe_reg](index.html) module"]
pub struct SCAN_OBSERVE_REG_SPEC;
impl crate::RegisterSpec for SCAN_OBSERVE_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [scan_observe_reg::R](R) reader structure"]
impl crate::Readable for SCAN_OBSERVE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scan_observe_reg::W](W) writer structure"]
impl crate::Writable for SCAN_OBSERVE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCAN_OBSERVE_REG to value 0"]
impl crate::Resettable for SCAN_OBSERVE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
