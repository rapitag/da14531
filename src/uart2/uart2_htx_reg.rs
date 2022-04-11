#[doc = "Register `UART2_HTX_REG` reader"]
pub struct R(crate::R<UART2_HTX_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART2_HTX_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART2_HTX_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART2_HTX_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART2_HTX_REG` writer"]
pub struct W(crate::W<UART2_HTX_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART2_HTX_REG_SPEC>;
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
impl From<crate::W<UART2_HTX_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART2_HTX_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_HALT_TX` reader - This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled. 0 = Halt TX disabled 1 = Halt TX enabled Note, if FIFOs are implemented and not enabled, the setting of the halt TX register has no effect on operation."]
pub struct UART_HALT_TX_R(crate::FieldReader<bool, bool>);
impl UART_HALT_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_HALT_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_HALT_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_HALT_TX` writer - This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled. 0 = Halt TX disabled 1 = Halt TX enabled Note, if FIFOs are implemented and not enabled, the setting of the halt TX register has no effect on operation."]
pub struct UART_HALT_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_HALT_TX_W<'a> {
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
    #[doc = "Bit 0 - This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled. 0 = Halt TX disabled 1 = Halt TX enabled Note, if FIFOs are implemented and not enabled, the setting of the halt TX register has no effect on operation."]
    #[inline(always)]
    pub fn uart_halt_tx(&self) -> UART_HALT_TX_R {
        UART_HALT_TX_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled. 0 = Halt TX disabled 1 = Halt TX enabled Note, if FIFOs are implemented and not enabled, the setting of the halt TX register has no effect on operation."]
    #[inline(always)]
    pub fn uart_halt_tx(&mut self) -> UART_HALT_TX_W {
        UART_HALT_TX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Halt TX\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart2_htx_reg](index.html) module"]
pub struct UART2_HTX_REG_SPEC;
impl crate::RegisterSpec for UART2_HTX_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart2_htx_reg::R](R) reader structure"]
impl crate::Readable for UART2_HTX_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart2_htx_reg::W](W) writer structure"]
impl crate::Writable for UART2_HTX_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART2_HTX_REG to value 0"]
impl crate::Resettable for UART2_HTX_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
