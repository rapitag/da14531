#[doc = "Register `QDEC_EVENT_CNT_REG` reader"]
pub struct R(crate::R<QDEC_EVENT_CNT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDEC_EVENT_CNT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDEC_EVENT_CNT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDEC_EVENT_CNT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QDEC_EVENT_CNT_REG` writer"]
pub struct W(crate::W<QDEC_EVENT_CNT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDEC_EVENT_CNT_REG_SPEC>;
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
impl From<crate::W<QDEC_EVENT_CNT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDEC_EVENT_CNT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QDEC_EVENT_CNT` reader - Gives the number of events at all channels."]
pub struct QDEC_EVENT_CNT_R(crate::FieldReader<u8, u8>);
impl QDEC_EVENT_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QDEC_EVENT_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_EVENT_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Gives the number of events at all channels."]
    #[inline(always)]
    pub fn qdec_event_cnt(&self) -> QDEC_EVENT_CNT_R {
        QDEC_EVENT_CNT_R::new((self.bits & 0xff) as u8)
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
#[doc = "Event counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdec_event_cnt_reg](index.html) module"]
pub struct QDEC_EVENT_CNT_REG_SPEC;
impl crate::RegisterSpec for QDEC_EVENT_CNT_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [qdec_event_cnt_reg::R](R) reader structure"]
impl crate::Readable for QDEC_EVENT_CNT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdec_event_cnt_reg::W](W) writer structure"]
impl crate::Writable for QDEC_EVENT_CNT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QDEC_EVENT_CNT_REG to value 0"]
impl crate::Resettable for QDEC_EVENT_CNT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}