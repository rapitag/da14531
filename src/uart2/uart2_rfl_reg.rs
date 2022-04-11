#[doc = "Register `UART2_RFL_REG` reader"]
pub struct R(crate::R<UART2_RFL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART2_RFL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART2_RFL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART2_RFL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART2_RFL_REG` writer"]
pub struct W(crate::W<UART2_RFL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART2_RFL_REG_SPEC>;
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
impl From<crate::W<UART2_RFL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART2_RFL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_RECEIVE_FIFO_LEVEL` reader - Receive FIFO Level. This is indicates the number of data entries in the receive FIFO."]
pub struct UART_RECEIVE_FIFO_LEVEL_R(crate::FieldReader<u8, u8>);
impl UART_RECEIVE_FIFO_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART_RECEIVE_FIFO_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_RECEIVE_FIFO_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Receive FIFO Level. This is indicates the number of data entries in the receive FIFO."]
    #[inline(always)]
    pub fn uart_receive_fifo_level(&self) -> UART_RECEIVE_FIFO_LEVEL_R {
        UART_RECEIVE_FIFO_LEVEL_R::new((self.bits & 0x1f) as u8)
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
#[doc = "Receive FIFO Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart2_rfl_reg](index.html) module"]
pub struct UART2_RFL_REG_SPEC;
impl crate::RegisterSpec for UART2_RFL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart2_rfl_reg::R](R) reader structure"]
impl crate::Readable for UART2_RFL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart2_rfl_reg::W](W) writer structure"]
impl crate::Writable for UART2_RFL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART2_RFL_REG to value 0"]
impl crate::Resettable for UART2_RFL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
