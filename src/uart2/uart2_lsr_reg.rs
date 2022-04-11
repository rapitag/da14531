#[doc = "Register `UART2_LSR_REG` reader"]
pub struct R(crate::R<UART2_LSR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART2_LSR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART2_LSR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART2_LSR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART2_LSR_REG` writer"]
pub struct W(crate::W<UART2_LSR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART2_LSR_REG_SPEC>;
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
impl From<crate::W<UART2_LSR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART2_LSR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_RFE` reader - Receiver FIFO Error bit. This bit is only relevant when FIFOs are enabled (FCR\\[0\\]
set to one). This is used to indicate if there is at least one parity error, framing error, or break indication in the FIFO. 0 = no error in RX FIFO 1 = error in RX FIFO This bit is cleared when the LSR is read and the character with the error is at the top of the receiver FIFO and there are no subsequent errors in the FIFO."]
pub struct UART_RFE_R(crate::FieldReader<bool, bool>);
impl UART_RFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_RFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_RFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_TEMT` reader - Transmitter Empty bit. If FIFOs enabled (FCR\\[0\\]
set to one), this bit is set whenever the Transmitter Shift Register and the FIFO are both empty. If FIFOs are disabled, this bit is set whenever the Transmitter Holding Register(THR) and the Transmitter Shift Register are both empty."]
pub struct UART_TEMT_R(crate::FieldReader<bool, bool>);
impl UART_TEMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_TEMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_TEMT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_THRE` reader - Transmit Holding Register Empty bit. If THRE mode is disabled (IER\\[7\\]
set to zero) and regardless of FIFO's being implemented/enabled or not, this bit indicates that the THR or TX FIFO is empty. This bit is set whenever data is transferred from the THR or TX FIFO to the transmitter shift register and no new data has been written to the THR or TX FIFO. This also causes a THRE Interrupt to occur, if the THRE Interrupt is enabled. If both modes are active (IER\\[7\\]
set to one and FCR\\[0\\]
set to one respectively), the functionality is switched to indicate the transmitter FIFO is full, and no longer controls THRE interrupts, which are then controlled by the FCR\\[5:4\\]
threshold setting."]
pub struct UART_THRE_R(crate::FieldReader<bool, bool>);
impl UART_THRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_THRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_THRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_BI` reader - Break Interrupt bit. This is used to indicate the detection of a break sequence on the serial input data. If in UART mode (SIR_MODE == Disabled), it is set whenever the serial input, sin, is held in a logic '0' state for longer than the sum of start time + data bits + parity + stop bits. If in infrared mode (SIR_MODE == Enabled), it is set whenever the serial input, sir_in, is continuously pulsed to logic '0' for longer than the sum of start time + data bits + parity + stop bits. A break condition on serial input causes one and only one character, consisting of all zeros, to be received by the UART. In the FIFO mode, the character associated with the break condition is carried through the FIFO and is revealed when the character is at the top of the FIFO. Reading the LSR clears the BI bit. In the non-FIFO mode, the BI indication occurs immediately and persists until the LSR is read."]
pub struct UART_BI_R(crate::FieldReader<bool, bool>);
impl UART_BI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_BI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_BI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_FE` reader - Framing Error bit. This is used to indicate the occurrence of a framing error in the receiver. A framing error occurs when the receiver does not detect a valid STOP bit in the received data. In the FIFO mode, since the framing error is associated with a character received, it is revealed when the character with the framing error is at the top of the FIFO. When a framing error occurs, the UART tries to resynchronize. It does this by assuming that the error was due to the start bit of the next character and then continues receiving the other bit i.e. data, and/or parity and stop. It should be noted that the Framing Error (FE) bit (LSR\\[3\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]). 0 = no framing error 1 = framing error Reading the LSR clears the FE bit."]
pub struct UART_FE_R(crate::FieldReader<bool, bool>);
impl UART_FE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_PE` reader - Parity Error bit. This is used to indicate the occurrence of a parity error in the receiver if the Parity Enable (PEN) bit (LCR\\[3\\]) is set. In the FIFO mode, since the parity error is associated with a character received, it is revealed when the character with the parity error arrives at the top of the FIFO. It should be noted that the Parity Error (PE) bit (LSR\\[2\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]). 0 = no parity error 1 = parity error Reading the LSR clears the PE bit."]
pub struct UART_PE_R(crate::FieldReader<bool, bool>);
impl UART_PE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_OE` reader - Overrun error bit. This is used to indicate the occurrence of an overrun error. This occurs if a new data character was received before the previous data was read. In the non-FIFO mode, the OE bit is set when a new character arrives in the receiver before the previous character was read from the RBR. When this happens, the data in the RBR is overwritten. In the FIFO mode, an overrun error occurs when the FIFO is full and a new character arrives at the receiver. The data in the FIFO is retained and the data in the receive shift register is lost. 0 = no overrun error 1 = overrun error Reading the LSR clears the OE bit."]
pub struct UART_OE_R(crate::FieldReader<bool, bool>);
impl UART_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_DR` reader - Data Ready bit. This is used to indicate that the receiver contains at least one character in the RBR or the receiver FIFO. 0 = no data ready 1 = data ready This bit is cleared when the RBR is read in non-FIFO mode, or when the receiver FIFO is empty, in FIFO mode."]
pub struct UART_DR_R(crate::FieldReader<bool, bool>);
impl UART_DR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_DR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_DR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - Receiver FIFO Error bit. This bit is only relevant when FIFOs are enabled (FCR\\[0\\]
set to one). This is used to indicate if there is at least one parity error, framing error, or break indication in the FIFO. 0 = no error in RX FIFO 1 = error in RX FIFO This bit is cleared when the LSR is read and the character with the error is at the top of the receiver FIFO and there are no subsequent errors in the FIFO."]
    #[inline(always)]
    pub fn uart_rfe(&self) -> UART_RFE_R {
        UART_RFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty bit. If FIFOs enabled (FCR\\[0\\]
set to one), this bit is set whenever the Transmitter Shift Register and the FIFO are both empty. If FIFOs are disabled, this bit is set whenever the Transmitter Holding Register(THR) and the Transmitter Shift Register are both empty."]
    #[inline(always)]
    pub fn uart_temt(&self) -> UART_TEMT_R {
        UART_TEMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Holding Register Empty bit. If THRE mode is disabled (IER\\[7\\]
set to zero) and regardless of FIFO's being implemented/enabled or not, this bit indicates that the THR or TX FIFO is empty. This bit is set whenever data is transferred from the THR or TX FIFO to the transmitter shift register and no new data has been written to the THR or TX FIFO. This also causes a THRE Interrupt to occur, if the THRE Interrupt is enabled. If both modes are active (IER\\[7\\]
set to one and FCR\\[0\\]
set to one respectively), the functionality is switched to indicate the transmitter FIFO is full, and no longer controls THRE interrupts, which are then controlled by the FCR\\[5:4\\]
threshold setting."]
    #[inline(always)]
    pub fn uart_thre(&self) -> UART_THRE_R {
        UART_THRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt bit. This is used to indicate the detection of a break sequence on the serial input data. If in UART mode (SIR_MODE == Disabled), it is set whenever the serial input, sin, is held in a logic '0' state for longer than the sum of start time + data bits + parity + stop bits. If in infrared mode (SIR_MODE == Enabled), it is set whenever the serial input, sir_in, is continuously pulsed to logic '0' for longer than the sum of start time + data bits + parity + stop bits. A break condition on serial input causes one and only one character, consisting of all zeros, to be received by the UART. In the FIFO mode, the character associated with the break condition is carried through the FIFO and is revealed when the character is at the top of the FIFO. Reading the LSR clears the BI bit. In the non-FIFO mode, the BI indication occurs immediately and persists until the LSR is read."]
    #[inline(always)]
    pub fn uart_bi(&self) -> UART_BI_R {
        UART_BI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error bit. This is used to indicate the occurrence of a framing error in the receiver. A framing error occurs when the receiver does not detect a valid STOP bit in the received data. In the FIFO mode, since the framing error is associated with a character received, it is revealed when the character with the framing error is at the top of the FIFO. When a framing error occurs, the UART tries to resynchronize. It does this by assuming that the error was due to the start bit of the next character and then continues receiving the other bit i.e. data, and/or parity and stop. It should be noted that the Framing Error (FE) bit (LSR\\[3\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]). 0 = no framing error 1 = framing error Reading the LSR clears the FE bit."]
    #[inline(always)]
    pub fn uart_fe(&self) -> UART_FE_R {
        UART_FE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error bit. This is used to indicate the occurrence of a parity error in the receiver if the Parity Enable (PEN) bit (LCR\\[3\\]) is set. In the FIFO mode, since the parity error is associated with a character received, it is revealed when the character with the parity error arrives at the top of the FIFO. It should be noted that the Parity Error (PE) bit (LSR\\[2\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]). 0 = no parity error 1 = parity error Reading the LSR clears the PE bit."]
    #[inline(always)]
    pub fn uart_pe(&self) -> UART_PE_R {
        UART_PE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun error bit. This is used to indicate the occurrence of an overrun error. This occurs if a new data character was received before the previous data was read. In the non-FIFO mode, the OE bit is set when a new character arrives in the receiver before the previous character was read from the RBR. When this happens, the data in the RBR is overwritten. In the FIFO mode, an overrun error occurs when the FIFO is full and a new character arrives at the receiver. The data in the FIFO is retained and the data in the receive shift register is lost. 0 = no overrun error 1 = overrun error Reading the LSR clears the OE bit."]
    #[inline(always)]
    pub fn uart_oe(&self) -> UART_OE_R {
        UART_OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Data Ready bit. This is used to indicate that the receiver contains at least one character in the RBR or the receiver FIFO. 0 = no data ready 1 = data ready This bit is cleared when the RBR is read in non-FIFO mode, or when the receiver FIFO is empty, in FIFO mode."]
    #[inline(always)]
    pub fn uart_dr(&self) -> UART_DR_R {
        UART_DR_R::new((self.bits & 1) != 0)
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
#[doc = "Line Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart2_lsr_reg](index.html) module"]
pub struct UART2_LSR_REG_SPEC;
impl crate::RegisterSpec for UART2_LSR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart2_lsr_reg::R](R) reader structure"]
impl crate::Readable for UART2_LSR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart2_lsr_reg::W](W) writer structure"]
impl crate::Writable for UART2_LSR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART2_LSR_REG to value 0x60"]
impl crate::Resettable for UART2_LSR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x60
    }
}
