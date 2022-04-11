#[doc = "Register `OTPC_PADDR_REG` reader"]
pub struct R(crate::R<OTPC_PADDR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTPC_PADDR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTPC_PADDR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTPC_PADDR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTPC_PADDR_REG` writer"]
pub struct W(crate::W<OTPC_PADDR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTPC_PADDR_REG_SPEC>;
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
impl From<crate::W<OTPC_PADDR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTPC_PADDR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTPC_PADDR` reader - The OTPC_PADDR_REG and the OTPC_PWORD_REG consist the PBUF buffer that keeps the information that will be programmed in the OTP, by using the PROG mode. The PBUF holds the address (OTPC_PADDR_REG) and the data (OTPC_PWORD_REG) of each of the programming requests that are applied in the OTP memory. The OTPC_PADDR_REG refers to a word address. The OTPC_PADDR_REG has to be writen after the OTP_PWORD_REG and only if the OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=1. The register is read only for as long the PBUF is not empty (OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=0). A writting to the OTPC_PADDR_REG triggers the controller to start the programming procedure (only if the PROG mode is active)."]
pub struct OTPC_PADDR_R(crate::FieldReader<u16, u16>);
impl OTPC_PADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        OTPC_PADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_PADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_PADDR` writer - The OTPC_PADDR_REG and the OTPC_PWORD_REG consist the PBUF buffer that keeps the information that will be programmed in the OTP, by using the PROG mode. The PBUF holds the address (OTPC_PADDR_REG) and the data (OTPC_PWORD_REG) of each of the programming requests that are applied in the OTP memory. The OTPC_PADDR_REG refers to a word address. The OTPC_PADDR_REG has to be writen after the OTP_PWORD_REG and only if the OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=1. The register is read only for as long the PBUF is not empty (OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=0). A writting to the OTPC_PADDR_REG triggers the controller to start the programming procedure (only if the PROG mode is active)."]
pub struct OTPC_PADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_PADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - The OTPC_PADDR_REG and the OTPC_PWORD_REG consist the PBUF buffer that keeps the information that will be programmed in the OTP, by using the PROG mode. The PBUF holds the address (OTPC_PADDR_REG) and the data (OTPC_PWORD_REG) of each of the programming requests that are applied in the OTP memory. The OTPC_PADDR_REG refers to a word address. The OTPC_PADDR_REG has to be writen after the OTP_PWORD_REG and only if the OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=1. The register is read only for as long the PBUF is not empty (OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=0). A writting to the OTPC_PADDR_REG triggers the controller to start the programming procedure (only if the PROG mode is active)."]
    #[inline(always)]
    pub fn otpc_paddr(&self) -> OTPC_PADDR_R {
        OTPC_PADDR_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - The OTPC_PADDR_REG and the OTPC_PWORD_REG consist the PBUF buffer that keeps the information that will be programmed in the OTP, by using the PROG mode. The PBUF holds the address (OTPC_PADDR_REG) and the data (OTPC_PWORD_REG) of each of the programming requests that are applied in the OTP memory. The OTPC_PADDR_REG refers to a word address. The OTPC_PADDR_REG has to be writen after the OTP_PWORD_REG and only if the OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=1. The register is read only for as long the PBUF is not empty (OTPC_STAT_REG\\[OTPC_STAT_PBUF_EMPTY\\]=0). A writting to the OTPC_PADDR_REG triggers the controller to start the programming procedure (only if the PROG mode is active)."]
    #[inline(always)]
    pub fn otpc_paddr(&mut self) -> OTPC_PADDR_W {
        OTPC_PADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The address of the word that will be programmed, when the PROG mode is used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpc_paddr_reg](index.html) module"]
pub struct OTPC_PADDR_REG_SPEC;
impl crate::RegisterSpec for OTPC_PADDR_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otpc_paddr_reg::R](R) reader structure"]
impl crate::Readable for OTPC_PADDR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otpc_paddr_reg::W](W) writer structure"]
impl crate::Writable for OTPC_PADDR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTPC_PADDR_REG to value 0"]
impl crate::Resettable for OTPC_PADDR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
