#[doc = "Register `XTALRDY_STAT_REG` reader"]
pub struct R(crate::R<XTALRDY_STAT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTALRDY_STAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTALRDY_STAT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTALRDY_STAT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTALRDY_STAT_REG` writer"]
pub struct W(crate::W<XTALRDY_STAT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTALRDY_STAT_REG_SPEC>;
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
impl From<crate::W<XTALRDY_STAT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTALRDY_STAT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTALRDY_STAT` reader - "]
pub struct XTALRDY_STAT_R(crate::FieldReader<u8, u8>);
impl XTALRDY_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTALRDY_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTALRDY_STAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn xtalrdy_stat(&self) -> XTALRDY_STAT_R {
        XTALRDY_STAT_R::new((self.bits & 0xff) as u8)
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalrdy_stat_reg](index.html) module"]
pub struct XTALRDY_STAT_REG_SPEC;
impl crate::RegisterSpec for XTALRDY_STAT_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xtalrdy_stat_reg::R](R) reader structure"]
impl crate::Readable for XTALRDY_STAT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtalrdy_stat_reg::W](W) writer structure"]
impl crate::Writable for XTALRDY_STAT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTALRDY_STAT_REG to value 0"]
impl crate::Resettable for XTALRDY_STAT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
