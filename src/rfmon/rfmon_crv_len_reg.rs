#[doc = "Register `RFMON_CRV_LEN_REG` reader"]
pub struct R(crate::R<RFMON_CRV_LEN_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFMON_CRV_LEN_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFMON_CRV_LEN_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFMON_CRV_LEN_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFMON_CRV_LEN_REG` writer"]
pub struct W(crate::W<RFMON_CRV_LEN_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFMON_CRV_LEN_REG_SPEC>;
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
impl From<crate::W<RFMON_CRV_LEN_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFMON_CRV_LEN_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFMON_CRV_LEN` reader - "]
pub struct RFMON_CRV_LEN_R(crate::FieldReader<u16, u16>);
impl RFMON_CRV_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RFMON_CRV_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFMON_CRV_LEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn rfmon_crv_len(&self) -> RFMON_CRV_LEN_R {
        RFMON_CRV_LEN_R::new((self.bits & 0x3fff) as u16)
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfmon_crv_len_reg](index.html) module"]
pub struct RFMON_CRV_LEN_REG_SPEC;
impl crate::RegisterSpec for RFMON_CRV_LEN_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rfmon_crv_len_reg::R](R) reader structure"]
impl crate::Readable for RFMON_CRV_LEN_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfmon_crv_len_reg::W](W) writer structure"]
impl crate::Writable for RFMON_CRV_LEN_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFMON_CRV_LEN_REG to value 0"]
impl crate::Resettable for RFMON_CRV_LEN_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
