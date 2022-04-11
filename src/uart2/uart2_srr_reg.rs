#[doc = "Register `UART2_SRR_REG` reader"]
pub struct R(crate::R<UART2_SRR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART2_SRR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART2_SRR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART2_SRR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART2_SRR_REG` writer"]
pub struct W(crate::W<UART2_SRR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART2_SRR_REG_SPEC>;
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
impl From<crate::W<UART2_SRR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART2_SRR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_XFR` writer - XMIT FIFO Reset. This is a shadow register for the XMIT FIFO Reset bit (FCR\\[2\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the transmit FIFO. This resets the control portion of the transmit FIFO and treats the FIFO as empty. Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub struct UART_XFR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_XFR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u16 & 1) << 2);
        self.w
    }
}
#[doc = "Field `UART_RFR` writer - RCVR FIFO Reset. This is a shadow register for the RCVR FIFO Reset bit (FCR\\[1\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the receive FIFO This resets the control portion of the receive FIFO and treats the FIFO as empty. Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
pub struct UART_RFR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RFR_W<'a> {
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
#[doc = "Field `UART_UR` writer - UART Reset. This asynchronously resets the UART Ctrl and synchronously removes the reset assertion. For a two clock implementation both pclk and sclk domains are reset."]
pub struct UART_UR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_UR_W<'a> {
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
impl W {
    #[doc = "Bit 2 - XMIT FIFO Reset. This is a shadow register for the XMIT FIFO Reset bit (FCR\\[2\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the transmit FIFO. This resets the control portion of the transmit FIFO and treats the FIFO as empty. Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    pub fn uart_xfr(&mut self) -> UART_XFR_W {
        UART_XFR_W { w: self }
    }
    #[doc = "Bit 1 - RCVR FIFO Reset. This is a shadow register for the RCVR FIFO Reset bit (FCR\\[1\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the receive FIFO This resets the control portion of the receive FIFO and treats the FIFO as empty. Note that this bit is 'self-clearing'. It is not necessary to clear this bit."]
    #[inline(always)]
    pub fn uart_rfr(&mut self) -> UART_RFR_W {
        UART_RFR_W { w: self }
    }
    #[doc = "Bit 0 - UART Reset. This asynchronously resets the UART Ctrl and synchronously removes the reset assertion. For a two clock implementation both pclk and sclk domains are reset."]
    #[inline(always)]
    pub fn uart_ur(&mut self) -> UART_UR_W {
        UART_UR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Reset Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart2_srr_reg](index.html) module"]
pub struct UART2_SRR_REG_SPEC;
impl crate::RegisterSpec for UART2_SRR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart2_srr_reg::R](R) reader structure"]
impl crate::Readable for UART2_SRR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart2_srr_reg::W](W) writer structure"]
impl crate::Writable for UART2_SRR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART2_SRR_REG to value 0"]
impl crate::Resettable for UART2_SRR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
