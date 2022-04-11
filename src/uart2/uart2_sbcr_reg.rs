#[doc = "Register `UART2_SBCR_REG` reader"]
pub struct R(crate::R<UART2_SBCR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART2_SBCR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART2_SBCR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART2_SBCR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART2_SBCR_REG` writer"]
pub struct W(crate::W<UART2_SBCR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART2_SBCR_REG_SPEC>;
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
impl From<crate::W<UART2_SBCR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART2_SBCR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_SHADOW_BREAK_CONTROL` reader - Shadow Break Control Bit. This is a shadow register for the Break bit (LCR\\[6\\]), this can be used to remove the burden of having to performing a read modify write on the LCR. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If SIR_MODE active (MCR\\[6\\]
= 1) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver."]
pub struct UART_SHADOW_BREAK_CONTROL_R(crate::FieldReader<bool, bool>);
impl UART_SHADOW_BREAK_CONTROL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_SHADOW_BREAK_CONTROL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SHADOW_BREAK_CONTROL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_SHADOW_BREAK_CONTROL` writer - Shadow Break Control Bit. This is a shadow register for the Break bit (LCR\\[6\\]), this can be used to remove the burden of having to performing a read modify write on the LCR. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If SIR_MODE active (MCR\\[6\\]
= 1) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver."]
pub struct UART_SHADOW_BREAK_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SHADOW_BREAK_CONTROL_W<'a> {
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
    #[doc = "Bit 0 - Shadow Break Control Bit. This is a shadow register for the Break bit (LCR\\[6\\]), this can be used to remove the burden of having to performing a read modify write on the LCR. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If SIR_MODE active (MCR\\[6\\]
= 1) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver."]
    #[inline(always)]
    pub fn uart_shadow_break_control(&self) -> UART_SHADOW_BREAK_CONTROL_R {
        UART_SHADOW_BREAK_CONTROL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow Break Control Bit. This is a shadow register for the Break bit (LCR\\[6\\]), this can be used to remove the burden of having to performing a read modify write on the LCR. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If SIR_MODE active (MCR\\[6\\]
= 1) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver."]
    #[inline(always)]
    pub fn uart_shadow_break_control(&mut self) -> UART_SHADOW_BREAK_CONTROL_W {
        UART_SHADOW_BREAK_CONTROL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow Break Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart2_sbcr_reg](index.html) module"]
pub struct UART2_SBCR_REG_SPEC;
impl crate::RegisterSpec for UART2_SBCR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart2_sbcr_reg::R](R) reader structure"]
impl crate::Readable for UART2_SBCR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart2_sbcr_reg::W](W) writer structure"]
impl crate::Writable for UART2_SBCR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART2_SBCR_REG to value 0"]
impl crate::Resettable for UART2_SBCR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
