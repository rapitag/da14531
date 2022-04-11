#[doc = "Register `UART_CTR_HIGH_REG` reader"]
pub struct R(crate::R<UART_CTR_HIGH_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_CTR_HIGH_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_CTR_HIGH_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_CTR_HIGH_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_CTR_HIGH_REG` writer"]
pub struct W(crate::W<UART_CTR_HIGH_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_CTR_HIGH_REG_SPEC>;
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
impl From<crate::W<UART_CTR_HIGH_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_CTR_HIGH_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTR` reader - Component Type Register"]
pub struct CTR_R(crate::FieldReader<u16, u16>);
impl CTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Component Type Register"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(self.bits)
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
#[doc = "Component Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ctr_high_reg](index.html) module"]
pub struct UART_CTR_HIGH_REG_SPEC;
impl crate::RegisterSpec for UART_CTR_HIGH_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart_ctr_high_reg::R](R) reader structure"]
impl crate::Readable for UART_CTR_HIGH_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_ctr_high_reg::W](W) writer structure"]
impl crate::Writable for UART_CTR_HIGH_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_CTR_HIGH_REG to value 0x4457"]
impl crate::Resettable for UART_CTR_HIGH_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4457
    }
}
