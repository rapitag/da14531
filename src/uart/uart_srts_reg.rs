#[doc = "Register `UART_SRTS_REG` reader"]
pub struct R(crate::R<UART_SRTS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SRTS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SRTS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SRTS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_SRTS_REG` writer"]
pub struct W(crate::W<UART_SRTS_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SRTS_REG_SPEC>;
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
impl From<crate::W<UART_SRTS_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SRTS_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_SHADOW_REQUEST_TO_SEND` reader - Shadow Request to Send. This is a shadow register for the RTS bit (MCR\\[1\\]), this can be used to remove the burden of having to performing a read-modify-write on the MCR. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART Ctrl is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
= 0), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high. In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
= 1) and FIFOs enable (FCR\\[0\\]
= 1), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). Note that in Loopback mode (MCR\\[4\\]
= 1), the rts_n output is held inactive-high while the value of this location is internally looped back to an input."]
pub struct UART_SHADOW_REQUEST_TO_SEND_R(crate::FieldReader<bool, bool>);
impl UART_SHADOW_REQUEST_TO_SEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_SHADOW_REQUEST_TO_SEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SHADOW_REQUEST_TO_SEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_SHADOW_REQUEST_TO_SEND` writer - Shadow Request to Send. This is a shadow register for the RTS bit (MCR\\[1\\]), this can be used to remove the burden of having to performing a read-modify-write on the MCR. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART Ctrl is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
= 0), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high. In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
= 1) and FIFOs enable (FCR\\[0\\]
= 1), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). Note that in Loopback mode (MCR\\[4\\]
= 1), the rts_n output is held inactive-high while the value of this location is internally looped back to an input."]
pub struct UART_SHADOW_REQUEST_TO_SEND_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SHADOW_REQUEST_TO_SEND_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u16 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Shadow Request to Send. This is a shadow register for the RTS bit (MCR\\[1\\]), this can be used to remove the burden of having to performing a read-modify-write on the MCR. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART Ctrl is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
= 0), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high. In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
= 1) and FIFOs enable (FCR\\[0\\]
= 1), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). Note that in Loopback mode (MCR\\[4\\]
= 1), the rts_n output is held inactive-high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn uart_shadow_request_to_send(&self) -> UART_SHADOW_REQUEST_TO_SEND_R {
        UART_SHADOW_REQUEST_TO_SEND_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow Request to Send. This is a shadow register for the RTS bit (MCR\\[1\\]), this can be used to remove the burden of having to performing a read-modify-write on the MCR. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART Ctrl is ready to exchange data. When Auto RTS Flow Control is not enabled (MCR\\[5\\]
= 0), the rts_n signal is set low by programming MCR\\[1\\]
(RTS) to a high. In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\]
= 1) and FIFOs enable (FCR\\[0\\]
= 1), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). Note that in Loopback mode (MCR\\[4\\]
= 1), the rts_n output is held inactive-high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn uart_shadow_request_to_send(&mut self) -> UART_SHADOW_REQUEST_TO_SEND_W {
        UART_SHADOW_REQUEST_TO_SEND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow Request to Send\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_srts_reg](index.html) module"]
pub struct UART_SRTS_REG_SPEC;
impl crate::RegisterSpec for UART_SRTS_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart_srts_reg::R](R) reader structure"]
impl crate::Readable for UART_SRTS_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_srts_reg::W](W) writer structure"]
impl crate::Writable for UART_SRTS_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_SRTS_REG to value 0"]
impl crate::Resettable for UART_SRTS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
