#[doc = "Register `UART2_DMASA_REG` reader"]
pub struct R(crate::R<UART2_DMASA_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART2_DMASA_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART2_DMASA_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART2_DMASA_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART2_DMASA_REG` writer"]
pub struct W(crate::W<UART2_DMASA_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART2_DMASA_REG_SPEC>;
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
impl From<crate::W<UART2_DMASA_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART2_DMASA_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMASA` writer - This register is use to perform DMA software acknowledge if a transfer needs to be terminated due to an error condition. For example, if the DMA disables the channel, then the DW_apb_uart should clear its request. This will cause the TX request, TX single, RX request and RX single signals to de-assert. Note that this bit is 'self-clearing' and it is not necessary to clear this bit."]
pub struct DMASA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASA_W<'a> {
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
    #[doc = "Bit 0 - This register is use to perform DMA software acknowledge if a transfer needs to be terminated due to an error condition. For example, if the DMA disables the channel, then the DW_apb_uart should clear its request. This will cause the TX request, TX single, RX request and RX single signals to de-assert. Note that this bit is 'self-clearing' and it is not necessary to clear this bit."]
    #[inline(always)]
    pub fn dmasa(&mut self) -> DMASA_W {
        DMASA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Software Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart2_dmasa_reg](index.html) module"]
pub struct UART2_DMASA_REG_SPEC;
impl crate::RegisterSpec for UART2_DMASA_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart2_dmasa_reg::R](R) reader structure"]
impl crate::Readable for UART2_DMASA_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart2_dmasa_reg::W](W) writer structure"]
impl crate::Writable for UART2_DMASA_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART2_DMASA_REG to value 0"]
impl crate::Resettable for UART2_DMASA_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
