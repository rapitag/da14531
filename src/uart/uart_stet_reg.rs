#[doc = "Register `UART_STET_REG` reader"]
pub struct R(crate::R<UART_STET_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_STET_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_STET_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_STET_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_STET_REG` writer"]
pub struct W(crate::W<UART_STET_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_STET_REG_SPEC>;
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
impl From<crate::W<UART_STET_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_STET_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_SHADOW_TX_EMPTY_TRIGGER` reader - Shadow TX Empty Trigger. This is a shadow register for the TX empty trigger bits (FCR\\[5:4\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the TX empty trigger bit gets updated. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO ¼ full 11 = FIFO ½ full"]
pub struct UART_SHADOW_TX_EMPTY_TRIGGER_R(crate::FieldReader<u8, u8>);
impl UART_SHADOW_TX_EMPTY_TRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART_SHADOW_TX_EMPTY_TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SHADOW_TX_EMPTY_TRIGGER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_SHADOW_TX_EMPTY_TRIGGER` writer - Shadow TX Empty Trigger. This is a shadow register for the TX empty trigger bits (FCR\\[5:4\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the TX empty trigger bit gets updated. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO ¼ full 11 = FIFO ½ full"]
pub struct UART_SHADOW_TX_EMPTY_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SHADOW_TX_EMPTY_TRIGGER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u16 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Shadow TX Empty Trigger. This is a shadow register for the TX empty trigger bits (FCR\\[5:4\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the TX empty trigger bit gets updated. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO ¼ full 11 = FIFO ½ full"]
    #[inline(always)]
    pub fn uart_shadow_tx_empty_trigger(&self) -> UART_SHADOW_TX_EMPTY_TRIGGER_R {
        UART_SHADOW_TX_EMPTY_TRIGGER_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Shadow TX Empty Trigger. This is a shadow register for the TX empty trigger bits (FCR\\[5:4\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the TX empty trigger bit gets updated. This is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO ¼ full 11 = FIFO ½ full"]
    #[inline(always)]
    pub fn uart_shadow_tx_empty_trigger(&mut self) -> UART_SHADOW_TX_EMPTY_TRIGGER_W {
        UART_SHADOW_TX_EMPTY_TRIGGER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow TX Empty Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_stet_reg](index.html) module"]
pub struct UART_STET_REG_SPEC;
impl crate::RegisterSpec for UART_STET_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart_stet_reg::R](R) reader structure"]
impl crate::Readable for UART_STET_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_stet_reg::W](W) writer structure"]
impl crate::Writable for UART_STET_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_STET_REG to value 0"]
impl crate::Resettable for UART_STET_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
