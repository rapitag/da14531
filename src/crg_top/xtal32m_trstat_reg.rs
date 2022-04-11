#[doc = "Register `XTAL32M_TRSTAT_REG` reader"]
pub struct R(crate::R<XTAL32M_TRSTAT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32M_TRSTAT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32M_TRSTAT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32M_TRSTAT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL32M_TRSTAT_REG` writer"]
pub struct W(crate::W<XTAL32M_TRSTAT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32M_TRSTAT_REG_SPEC>;
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
impl From<crate::W<XTAL32M_TRSTAT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32M_TRSTAT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32M_TRSTAT` reader - Reads value of the current XTAL trimming"]
pub struct XTAL32M_TRSTAT_R(crate::FieldReader<u8, u8>);
impl XTAL32M_TRSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32M_TRSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32M_TRSTAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Reads value of the current XTAL trimming"]
    #[inline(always)]
    pub fn xtal32m_trstat(&self) -> XTAL32M_TRSTAT_R {
        XTAL32M_TRSTAT_R::new((self.bits & 0xff) as u8)
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
#[doc = "Read back value of current XTAL trimming\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32m_trstat_reg](index.html) module"]
pub struct XTAL32M_TRSTAT_REG_SPEC;
impl crate::RegisterSpec for XTAL32M_TRSTAT_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xtal32m_trstat_reg::R](R) reader structure"]
impl crate::Readable for XTAL32M_TRSTAT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32m_trstat_reg::W](W) writer structure"]
impl crate::Writable for XTAL32M_TRSTAT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL32M_TRSTAT_REG to value 0"]
impl crate::Resettable for XTAL32M_TRSTAT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
