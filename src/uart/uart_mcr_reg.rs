#[doc = "Register `UART_MCR_REG` reader"]
pub struct R(crate::R<UART_MCR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_MCR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_MCR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_MCR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_MCR_REG` writer"]
pub struct W(crate::W<UART_MCR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_MCR_REG_SPEC>;
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
impl From<crate::W<UART_MCR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_MCR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_AFCE` reader - Auto Flow Control Enable. When FIFOs are enabled and the Auto Flow Control Enable (AFCE) bit is set, Auto Flow Control features are enabled. 0 = Auto Flow Control Mode disabled 1 = Auto Flow Control Mode enabled"]
pub struct UART_AFCE_R(crate::FieldReader<bool, bool>);
impl UART_AFCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_AFCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_AFCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_AFCE` writer - Auto Flow Control Enable. When FIFOs are enabled and the Auto Flow Control Enable (AFCE) bit is set, Auto Flow Control features are enabled. 0 = Auto Flow Control Mode disabled 1 = Auto Flow Control Mode enabled"]
pub struct UART_AFCE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_AFCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u16 & 1) << 5);
        self.w
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
#[doc = "Field `UART_RTS` reader - Request to Send. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
set to zero), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high.In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
set to one) and FIFOs enable (FCR\\[0\\]
set to one), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). The rts_n signal is de-asserted when MCR\\[1\\]
is set low. Note that in Loopback mode (MCR\\[4\\]
set to one), the rts_n output is held inactive high while the value of this location is internally looped back to an input."]
pub struct UART_RTS_R(crate::FieldReader<bool, bool>);
impl UART_RTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_RTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_RTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_RTS` writer - Request to Send. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
set to zero), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high.In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
set to one) and FIFOs enable (FCR\\[0\\]
set to one), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). The rts_n signal is de-asserted when MCR\\[1\\]
is set low. Note that in Loopback mode (MCR\\[4\\]
set to one), the rts_n output is held inactive high while the value of this location is internally looped back to an input."]
pub struct UART_RTS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u16 & 1) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Auto Flow Control Enable. When FIFOs are enabled and the Auto Flow Control Enable (AFCE) bit is set, Auto Flow Control features are enabled. 0 = Auto Flow Control Mode disabled 1 = Auto Flow Control Mode enabled"]
    #[inline(always)]
    pub fn uart_afce(&self) -> UART_AFCE_R {
        UART_AFCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - LoopBack Bit. This is used to put the UART into a diagnostic mode for test purposes. If operating in UART mode (SIR_MODE not active, MCR\\[6\\]
set to zero), data on the sout line is held high, while serial data output is looped back to the sin line, internally. In this mode all the interrupts are fully functional. Also, in loopback mode, the modem control inputs (dsr_n, cts_n, ri_n, dcd_n) are disconnected and the modem control outputs (dtr_n, rts_n, out1_n, out2_n) are looped back to the inputs, internally. If operating in infrared mode (SIR_MODE active, MCR\\[6\\]
set to one), data on the sir_out_n line is held low, while serial data output is inverted and looped back to the sir_in line."]
    #[inline(always)]
    pub fn uart_lb(&self) -> UART_LB_R {
        UART_LB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 1 - Request to Send. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
set to zero), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high.In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
set to one) and FIFOs enable (FCR\\[0\\]
set to one), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). The rts_n signal is de-asserted when MCR\\[1\\]
is set low. Note that in Loopback mode (MCR\\[4\\]
set to one), the rts_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn uart_rts(&self) -> UART_RTS_R {
        UART_RTS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Auto Flow Control Enable. When FIFOs are enabled and the Auto Flow Control Enable (AFCE) bit is set, Auto Flow Control features are enabled. 0 = Auto Flow Control Mode disabled 1 = Auto Flow Control Mode enabled"]
    #[inline(always)]
    pub fn uart_afce(&mut self) -> UART_AFCE_W {
        UART_AFCE_W { w: self }
    }
    #[doc = "Bit 4 - LoopBack Bit. This is used to put the UART into a diagnostic mode for test purposes. If operating in UART mode (SIR_MODE not active, MCR\\[6\\]
set to zero), data on the sout line is held high, while serial data output is looped back to the sin line, internally. In this mode all the interrupts are fully functional. Also, in loopback mode, the modem control inputs (dsr_n, cts_n, ri_n, dcd_n) are disconnected and the modem control outputs (dtr_n, rts_n, out1_n, out2_n) are looped back to the inputs, internally. If operating in infrared mode (SIR_MODE active, MCR\\[6\\]
set to one), data on the sir_out_n line is held low, while serial data output is inverted and looped back to the sir_in line."]
    #[inline(always)]
    pub fn uart_lb(&mut self) -> UART_LB_W {
        UART_LB_W { w: self }
    }
    #[doc = "Bit 1 - Request to Send. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
set to zero), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high.In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
set to one) and FIFOs enable (FCR\\[0\\]
set to one), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). The rts_n signal is de-asserted when MCR\\[1\\]
is set low. Note that in Loopback mode (MCR\\[4\\]
set to one), the rts_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn uart_rts(&mut self) -> UART_RTS_W {
        UART_RTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Modem Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_mcr_reg](index.html) module"]
pub struct UART_MCR_REG_SPEC;
impl crate::RegisterSpec for UART_MCR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart_mcr_reg::R](R) reader structure"]
impl crate::Readable for UART_MCR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_mcr_reg::W](W) writer structure"]
impl crate::Writable for UART_MCR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_MCR_REG to value 0"]
impl crate::Resettable for UART_MCR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
