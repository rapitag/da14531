#[doc = "Register `UART2_MCR_REG` reader"]
pub struct R(crate::R<UART2_MCR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART2_MCR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART2_MCR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART2_MCR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART2_MCR_REG` writer"]
pub struct W(crate::W<UART2_MCR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART2_MCR_REG_SPEC>;
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
impl From<crate::W<UART2_MCR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART2_MCR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_LB` reader - LoopBack Bit. This is used to put the UART into a diagnostic mode for test purposes. If operating in UART mode (SIR_MODE not active, MCR\\[6\\]
set to zero), data on the sout line is held high, while serial data output is looped back to the sin line, internally. In this mode all the interrupts are fully functional. Also, in loopback mode, the modem control inputs (dsr_n, cts_n, ri_n, dcd_n) are disconnected and the modem control outputs (dtr_n, rts_n, out1_n, out2_n) are looped back to the inputs, internally. If operating in infrared mode (SIR_MODE active, MCR\\[6\\]
set to one), data on the sir_out_n line is held low, while serial data output is inverted and looped back to the sir_in line."]
pub struct UART_LB_R(crate::FieldReader<bool, bool>);
impl UART_LB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_LB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_LB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_LB` writer - LoopBack Bit. This is used to put the UART into a diagnostic mode for test purposes. If operating in UART mode (SIR_MODE not active, MCR\\[6\\]
set to zero), data on the sout line is held high, while serial data output is looped back to the sin line, internally. In this mode all the interrupts are fully functional. Also, in loopback mode, the modem control inputs (dsr_n, cts_n, ri_n, dcd_n) are disconnected and the modem control outputs (dtr_n, rts_n, out1_n, out2_n) are looped back to the inputs, internally. If operating in infrared mode (SIR_MODE active, MCR\\[6\\]
set to one), data on the sir_out_n line is held low, while serial data output is inverted and looped back to the sir_in line."]
pub struct UART_LB_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_LB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u16 & 1) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - LoopBack Bit. This is used to put the UART into a diagnostic mode for test purposes. If operating in UART mode (SIR_MODE not active, MCR\\[6\\]
set to zero), data on the sout line is held high, while serial data output is looped back to the sin line, internally. In this mode all the interrupts are fully functional. Also, in loopback mode, the modem control inputs (dsr_n, cts_n, ri_n, dcd_n) are disconnected and the modem control outputs (dtr_n, rts_n, out1_n, out2_n) are looped back to the inputs, internally. If operating in infrared mode (SIR_MODE active, MCR\\[6\\]
set to one), data on the sir_out_n line is held low, while serial data output is inverted and looped back to the sir_in line."]
    #[inline(always)]
    pub fn uart_lb(&self) -> UART_LB_R {
        UART_LB_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - LoopBack Bit. This is used to put the UART into a diagnostic mode for test purposes. If operating in UART mode (SIR_MODE not active, MCR\\[6\\]
set to zero), data on the sout line is held high, while serial data output is looped back to the sin line, internally. In this mode all the interrupts are fully functional. Also, in loopback mode, the modem control inputs (dsr_n, cts_n, ri_n, dcd_n) are disconnected and the modem control outputs (dtr_n, rts_n, out1_n, out2_n) are looped back to the inputs, internally. If operating in infrared mode (SIR_MODE active, MCR\\[6\\]
set to one), data on the sir_out_n line is held low, while serial data output is inverted and looped back to the sir_in line."]
    #[inline(always)]
    pub fn uart_lb(&mut self) -> UART_LB_W {
        UART_LB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Modem Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart2_mcr_reg](index.html) module"]
pub struct UART2_MCR_REG_SPEC;
impl crate::RegisterSpec for UART2_MCR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart2_mcr_reg::R](R) reader structure"]
impl crate::Readable for UART2_MCR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart2_mcr_reg::W](W) writer structure"]
impl crate::Writable for UART2_MCR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART2_MCR_REG to value 0"]
impl crate::Resettable for UART2_MCR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
