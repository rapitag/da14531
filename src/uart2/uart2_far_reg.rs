#[doc = "Register `UART2_FAR_REG` reader"]
pub struct R(crate::R<UART2_FAR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART2_FAR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART2_FAR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART2_FAR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART2_FAR_REG` writer"]
pub struct W(crate::W<UART2_FAR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART2_FAR_REG_SPEC>;
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
impl From<crate::W<UART2_FAR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART2_FAR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_FAR` reader - Description: Writes will have no effect when FIFO_ACCESS == No, always readable. This register is use to enable a FIFO access mode for testing, so that the receive FIFO can be written by the master and the transmit FIFO can be read by the master when FIFO's are implemented and enabled. When FIFO's are not implemented or not enabled it allows the RBR to be written by the master and the THR to be read by the master. 0 = FIFO access mode disabled 1 = FIFO access mode enabled Note, that when the FIFO access mode is enabled/disabled, the control portion of the receive FIFO and transmit FIFO is reset and the FIFO's are treated as empty."]
pub struct UART_FAR_R(crate::FieldReader<bool, bool>);
impl UART_FAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_FAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_FAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Description: Writes will have no effect when FIFO_ACCESS == No, always readable. This register is use to enable a FIFO access mode for testing, so that the receive FIFO can be written by the master and the transmit FIFO can be read by the master when FIFO's are implemented and enabled. When FIFO's are not implemented or not enabled it allows the RBR to be written by the master and the THR to be read by the master. 0 = FIFO access mode disabled 1 = FIFO access mode enabled Note, that when the FIFO access mode is enabled/disabled, the control portion of the receive FIFO and transmit FIFO is reset and the FIFO's are treated as empty."]
    #[inline(always)]
    pub fn uart_far(&self) -> UART_FAR_R {
        UART_FAR_R::new((self.bits & 1) != 0)
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
#[doc = "FIFO Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart2_far_reg](index.html) module"]
pub struct UART2_FAR_REG_SPEC;
impl crate::RegisterSpec for UART2_FAR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart2_far_reg::R](R) reader structure"]
impl crate::Readable for UART2_FAR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart2_far_reg::W](W) writer structure"]
impl crate::Writable for UART2_FAR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART2_FAR_REG to value 0"]
impl crate::Resettable for UART2_FAR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
