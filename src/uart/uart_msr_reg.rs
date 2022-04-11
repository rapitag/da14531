#[doc = "Register `UART_MSR_REG` reader"]
pub struct R(crate::R<UART_MSR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_MSR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_MSR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_MSR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_MSR_REG` writer"]
pub struct W(crate::W<UART_MSR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_MSR_REG_SPEC>;
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
impl From<crate::W<UART_MSR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_MSR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_CTS` reader - Clear to Send. This is used to indicate the current state of the modem control line cts_n. This bit is the complement of cts_n. When the Clear to Send input (cts_n) is asserted it is an indication that the modem or data set is ready to exchange data with the UART Ctrl. 0 = cts_n input is de-asserted (logic 1) 1 = cts_n input is asserted (logic 0) In Loopback Mode (MCR\\[4\\]
= 1), CTS is the same as MCR\\[1\\]
(RTS)."]
pub struct UART_CTS_R(crate::FieldReader<bool, bool>);
impl UART_CTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_CTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_CTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Clear to Send. This is used to indicate the current state of the modem control line cts_n. This bit is the complement of cts_n. When the Clear to Send input (cts_n) is asserted it is an indication that the modem or data set is ready to exchange data with the UART Ctrl. 0 = cts_n input is de-asserted (logic 1) 1 = cts_n input is asserted (logic 0) In Loopback Mode (MCR\\[4\\]
= 1), CTS is the same as MCR\\[1\\]
(RTS)."]
    #[inline(always)]
    pub fn uart_cts(&self) -> UART_CTS_R {
        UART_CTS_R::new(((self.bits >> 4) & 1) != 0)
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
#[doc = "Modem Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_msr_reg](index.html) module"]
pub struct UART_MSR_REG_SPEC;
impl crate::RegisterSpec for UART_MSR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart_msr_reg::R](R) reader structure"]
impl crate::Readable for UART_MSR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_msr_reg::W](W) writer structure"]
impl crate::Writable for UART_MSR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_MSR_REG to value 0x10"]
impl crate::Resettable for UART_MSR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
