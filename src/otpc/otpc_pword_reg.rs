#[doc = "Register `OTPC_PWORD_REG` reader"]
pub struct R(crate::R<OTPC_PWORD_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTPC_PWORD_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTPC_PWORD_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTPC_PWORD_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTPC_PWORD_REG` writer"]
pub struct W(crate::W<OTPC_PWORD_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTPC_PWORD_REG_SPEC>;
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
impl From<crate::W<OTPC_PWORD_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTPC_PWORD_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTPC_PWORD` reader - The OTPC_PADDR_REG and the OTPC_PWORD_REG consist the PBUF buffer that keeps the information that will be programmed in the OTP memory, by using the PROG mode. The PBUF holds the address (OTPC_PADDR_REG) and the data (OTPC_PWORD_REG) of each of the programming requests that are applied in the OTP memory. The OTP_PWORD_REG must be written before the OTPC_PADDR_REG and only if OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]
= 1. The register is read only for as long the PBUF is not empty (OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=0)."]
pub struct OTPC_PWORD_R(crate::FieldReader<u32, u32>);
impl OTPC_PWORD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        OTPC_PWORD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_PWORD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_PWORD` writer - The OTPC_PADDR_REG and the OTPC_PWORD_REG consist the PBUF buffer that keeps the information that will be programmed in the OTP memory, by using the PROG mode. The PBUF holds the address (OTPC_PADDR_REG) and the data (OTPC_PWORD_REG) of each of the programming requests that are applied in the OTP memory. The OTP_PWORD_REG must be written before the OTPC_PADDR_REG and only if OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]
= 1. The register is read only for as long the PBUF is not empty (OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=0)."]
pub struct OTPC_PWORD_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_PWORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The OTPC_PADDR_REG and the OTPC_PWORD_REG consist the PBUF buffer that keeps the information that will be programmed in the OTP memory, by using the PROG mode. The PBUF holds the address (OTPC_PADDR_REG) and the data (OTPC_PWORD_REG) of each of the programming requests that are applied in the OTP memory. The OTP_PWORD_REG must be written before the OTPC_PADDR_REG and only if OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]
= 1. The register is read only for as long the PBUF is not empty (OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=0)."]
    #[inline(always)]
    pub fn otpc_pword(&self) -> OTPC_PWORD_R {
        OTPC_PWORD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The OTPC_PADDR_REG and the OTPC_PWORD_REG consist the PBUF buffer that keeps the information that will be programmed in the OTP memory, by using the PROG mode. The PBUF holds the address (OTPC_PADDR_REG) and the data (OTPC_PWORD_REG) of each of the programming requests that are applied in the OTP memory. The OTP_PWORD_REG must be written before the OTPC_PADDR_REG and only if OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]
= 1. The register is read only for as long the PBUF is not empty (OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=0)."]
    #[inline(always)]
    pub fn otpc_pword(&mut self) -> OTPC_PWORD_W {
        OTPC_PWORD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The 32-bit word that will be programmed, when the PROG mode is used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpc_pword_reg](index.html) module"]
pub struct OTPC_PWORD_REG_SPEC;
impl crate::RegisterSpec for OTPC_PWORD_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otpc_pword_reg::R](R) reader structure"]
impl crate::Readable for OTPC_PWORD_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otpc_pword_reg::W](W) writer structure"]
impl crate::Writable for OTPC_PWORD_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTPC_PWORD_REG to value 0"]
impl crate::Resettable for OTPC_PWORD_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
