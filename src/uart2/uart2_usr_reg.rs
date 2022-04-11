#[doc = "Register `UART2_USR_REG` reader"]
pub struct R(crate::R<UART2_USR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART2_USR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART2_USR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART2_USR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART2_USR_REG` writer"]
pub struct W(crate::W<UART2_USR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART2_USR_REG_SPEC>;
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
impl From<crate::W<UART2_USR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART2_USR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_RFF` reader - Receive FIFO Full. This is used to indicate that the receive FIFO is completely full. 0 = Receive FIFO not full 1 = Receive FIFO Full This bit is cleared when the RX FIFO is no longer full."]
pub struct UART_RFF_R(crate::FieldReader<bool, bool>);
impl UART_RFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_RFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_RFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_RFNE` reader - Receive FIFO Not Empty. This is used to indicate that the receive FIFO contains one or more entries. 0 = Receive FIFO is empty 1 = Receive FIFO is not empty This bit is cleared when the RX FIFO is empty."]
pub struct UART_RFNE_R(crate::FieldReader<bool, bool>);
impl UART_RFNE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_RFNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_RFNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_TFE` reader - Transmit FIFO Empty. This is used to indicate that the transmit FIFO is completely empty. 0 = Transmit FIFO is not empty 1 = Transmit FIFO is empty This bit is cleared when the TX FIFO is no longer empty."]
pub struct UART_TFE_R(crate::FieldReader<bool, bool>);
impl UART_TFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_TFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_TFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_TFNF` reader - Transmit FIFO Not Full. This is used to indicate that the transmit FIFO in not full. 0 = Transmit FIFO is full 1 = Transmit FIFO is not full This bit is cleared when the TX FIFO is full."]
pub struct UART_TFNF_R(crate::FieldReader<bool, bool>);
impl UART_TFNF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_TFNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_TFNF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_BUSY` reader - UART Busy. This indicates that a serial transfer is in progress, when cleared indicates that the DW_apb_uart is idle or inactive. 0 - DW_apb_uart is idle or inactive 1 - DW_apb_uart is busy (actively transferring data) Note that it is possible for the UART Busy bit to be cleared even though a new character may have been sent from another device. That is, if the DW_apb_uart has no data in the THR and RBR and there is no transmission in progress and a start bit of a new character has just reached the DW_apb_uart. This is due to the fact that a valid start is not seen until the middle of the bit period and this duration is dependent on the baud divisor that has been programmed. If a second system clock has been implemented (CLOCK_MODE == Enabled) the assertion of this bit will also be delayed by several cycles of the slower clock."]
pub struct UART_BUSY_R(crate::FieldReader<bool, bool>);
impl UART_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Receive FIFO Full. This is used to indicate that the receive FIFO is completely full. 0 = Receive FIFO not full 1 = Receive FIFO Full This bit is cleared when the RX FIFO is no longer full."]
    #[inline(always)]
    pub fn uart_rff(&self) -> UART_RFF_R {
        UART_RFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Not Empty. This is used to indicate that the receive FIFO contains one or more entries. 0 = Receive FIFO is empty 1 = Receive FIFO is not empty This bit is cleared when the RX FIFO is empty."]
    #[inline(always)]
    pub fn uart_rfne(&self) -> UART_RFNE_R {
        UART_RFNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Empty. This is used to indicate that the transmit FIFO is completely empty. 0 = Transmit FIFO is not empty 1 = Transmit FIFO is empty This bit is cleared when the TX FIFO is no longer empty."]
    #[inline(always)]
    pub fn uart_tfe(&self) -> UART_TFE_R {
        UART_TFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full. This is used to indicate that the transmit FIFO in not full. 0 = Transmit FIFO is full 1 = Transmit FIFO is not full This bit is cleared when the TX FIFO is full."]
    #[inline(always)]
    pub fn uart_tfnf(&self) -> UART_TFNF_R {
        UART_TFNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - UART Busy. This indicates that a serial transfer is in progress, when cleared indicates that the DW_apb_uart is idle or inactive. 0 - DW_apb_uart is idle or inactive 1 - DW_apb_uart is busy (actively transferring data) Note that it is possible for the UART Busy bit to be cleared even though a new character may have been sent from another device. That is, if the DW_apb_uart has no data in the THR and RBR and there is no transmission in progress and a start bit of a new character has just reached the DW_apb_uart. This is due to the fact that a valid start is not seen until the middle of the bit period and this duration is dependent on the baud divisor that has been programmed. If a second system clock has been implemented (CLOCK_MODE == Enabled) the assertion of this bit will also be delayed by several cycles of the slower clock."]
    #[inline(always)]
    pub fn uart_busy(&self) -> UART_BUSY_R {
        UART_BUSY_R::new((self.bits & 1) != 0)
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
#[doc = "UART Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart2_usr_reg](index.html) module"]
pub struct UART2_USR_REG_SPEC;
impl crate::RegisterSpec for UART2_USR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart2_usr_reg::R](R) reader structure"]
impl crate::Readable for UART2_USR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart2_usr_reg::W](W) writer structure"]
impl crate::Writable for UART2_USR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART2_USR_REG to value 0x06"]
impl crate::Resettable for UART2_USR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
