#[doc = "Register `UART_IIR_FCR_REG` reader"]
pub struct R(crate::R<UART_IIR_FCR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_IIR_FCR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_IIR_FCR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_IIR_FCR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_IIR_FCR_REG` writer"]
pub struct W(crate::W<UART_IIR_FCR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_IIR_FCR_REG_SPEC>;
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
impl From<crate::W<UART_IIR_FCR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_IIR_FCR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_FIFOSE_RT` reader - On read FIFO's Enabled (or FIFOSE): This is used to indicate whether the FIFO's are enabled or disabled. 00 = disabled. 11 = enabled. On write RCVR Trigger (or RT):. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt will be generated. In auto flow control mode it is used to determine when the rts_n signal will be de-asserted. It also determines when the dma_rx_req_n signal will be asserted when in certain modes of operation. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO 1/4 full 10 = FIFO 1/2 full 11 = FIFO 2 less than full"]
pub struct UART_FIFOSE_RT_R(crate::FieldReader<u8, u8>);
impl UART_FIFOSE_RT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART_FIFOSE_RT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_FIFOSE_RT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_FIFOSE_RT` writer - On read FIFO's Enabled (or FIFOSE): This is used to indicate whether the FIFO's are enabled or disabled. 00 = disabled. 11 = enabled. On write RCVR Trigger (or RT):. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt will be generated. In auto flow control mode it is used to determine when the rts_n signal will be de-asserted. It also determines when the dma_rx_req_n signal will be asserted when in certain modes of operation. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO 1/4 full 10 = FIFO 1/2 full 11 = FIFO 2 less than full"]
pub struct UART_FIFOSE_RT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FIFOSE_RT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u16 & 3) << 6);
        self.w
    }
}
#[doc = "Field `UART_TET` writer - On read reserved On Write TX Empty Trigger (or TET): This is used to select the empty threshold level at which the THRE Interrupts will be generated when the mode is active. It also determines when the dma_tx_req_n signal will be asserted when in certain modes of operation. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO 1/4 full 11 = FIFO 1/2 full"]
pub struct UART_TET_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_TET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u16 & 3) << 4);
        self.w
    }
}
#[doc = "Field `UART_IID3_DMAM` reader - On Read (Bit3) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write DMA Mode (or DMAM): This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals. 0 = mode 0 1 = mode 1"]
pub struct UART_IID3_DMAM_R(crate::FieldReader<bool, bool>);
impl UART_IID3_DMAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_IID3_DMAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_IID3_DMAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_IID3_DMAM` writer - On Read (Bit3) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write DMA Mode (or DMAM): This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals. 0 = mode 0 1 = mode 1"]
pub struct UART_IID3_DMAM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IID3_DMAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u16 & 1) << 3);
        self.w
    }
}
#[doc = "Field `UART_IID2_XFIFOR` reader - On Read (Bit2) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write XMIT FIFO Reset (or XFIFOR): This resets the control portion of the transmit FIFO and treats the FIFO as empty. Note that this bit is 'self-clearing' and it is not necessary to clear this bit."]
pub struct UART_IID2_XFIFOR_R(crate::FieldReader<bool, bool>);
impl UART_IID2_XFIFOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_IID2_XFIFOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_IID2_XFIFOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_IID2_XFIFOR` writer - On Read (Bit2) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write XMIT FIFO Reset (or XFIFOR): This resets the control portion of the transmit FIFO and treats the FIFO as empty. Note that this bit is 'self-clearing' and it is not necessary to clear this bit."]
pub struct UART_IID2_XFIFOR_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IID2_XFIFOR_W<'a> {
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
#[doc = "Field `UART_IID1_RFIFOE` reader - On Read (Bit1) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write RCVR FIFO Reset (or RFIFOR): This resets the control portion of the receive FIFO and treats the FIFO as empty. Note that this bit is 'self-clearing' and it is not necessary to clear this bit."]
pub struct UART_IID1_RFIFOE_R(crate::FieldReader<bool, bool>);
impl UART_IID1_RFIFOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_IID1_RFIFOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_IID1_RFIFOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_IID1_RFIFOE` writer - On Read (Bit1) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write RCVR FIFO Reset (or RFIFOR): This resets the control portion of the receive FIFO and treats the FIFO as empty. Note that this bit is 'self-clearing' and it is not necessary to clear this bit."]
pub struct UART_IID1_RFIFOE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IID1_RFIFOE_W<'a> {
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
#[doc = "Field `UART_IID0_FIFOE` reader - On Read (Bit0) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write FIFO Enable (or FIFOE): This enables/disables the transmit (XMIT) and receive (RCVR) FIFO's. Whenever the value of this bit is changed both the XMIT and RCVR controller portion of FIFO's will be reset"]
pub struct UART_IID0_FIFOE_R(crate::FieldReader<bool, bool>);
impl UART_IID0_FIFOE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_IID0_FIFOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_IID0_FIFOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_IID0_FIFOE` writer - On Read (Bit0) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write FIFO Enable (or FIFOE): This enables/disables the transmit (XMIT) and receive (RCVR) FIFO's. Whenever the value of this bit is changed both the XMIT and RCVR controller portion of FIFO's will be reset"]
pub struct UART_IID0_FIFOE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_IID0_FIFOE_W<'a> {
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
    #[doc = "Bits 6:7 - On read FIFO's Enabled (or FIFOSE): This is used to indicate whether the FIFO's are enabled or disabled. 00 = disabled. 11 = enabled. On write RCVR Trigger (or RT):. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt will be generated. In auto flow control mode it is used to determine when the rts_n signal will be de-asserted. It also determines when the dma_rx_req_n signal will be asserted when in certain modes of operation. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO 1/4 full 10 = FIFO 1/2 full 11 = FIFO 2 less than full"]
    #[inline(always)]
    pub fn uart_fifose_rt(&self) -> UART_FIFOSE_RT_R {
        UART_FIFOSE_RT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 3 - On Read (Bit3) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write DMA Mode (or DMAM): This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals. 0 = mode 0 1 = mode 1"]
    #[inline(always)]
    pub fn uart_iid3_dmam(&self) -> UART_IID3_DMAM_R {
        UART_IID3_DMAM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - On Read (Bit2) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write XMIT FIFO Reset (or XFIFOR): This resets the control portion of the transmit FIFO and treats the FIFO as empty. Note that this bit is 'self-clearing' and it is not necessary to clear this bit."]
    #[inline(always)]
    pub fn uart_iid2_xfifor(&self) -> UART_IID2_XFIFOR_R {
        UART_IID2_XFIFOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - On Read (Bit1) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write RCVR FIFO Reset (or RFIFOR): This resets the control portion of the receive FIFO and treats the FIFO as empty. Note that this bit is 'self-clearing' and it is not necessary to clear this bit."]
    #[inline(always)]
    pub fn uart_iid1_rfifoe(&self) -> UART_IID1_RFIFOE_R {
        UART_IID1_RFIFOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - On Read (Bit0) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write FIFO Enable (or FIFOE): This enables/disables the transmit (XMIT) and receive (RCVR) FIFO's. Whenever the value of this bit is changed both the XMIT and RCVR controller portion of FIFO's will be reset"]
    #[inline(always)]
    pub fn uart_iid0_fifoe(&self) -> UART_IID0_FIFOE_R {
        UART_IID0_FIFOE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:7 - On read FIFO's Enabled (or FIFOSE): This is used to indicate whether the FIFO's are enabled or disabled. 00 = disabled. 11 = enabled. On write RCVR Trigger (or RT):. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt will be generated. In auto flow control mode it is used to determine when the rts_n signal will be de-asserted. It also determines when the dma_rx_req_n signal will be asserted when in certain modes of operation. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO 1/4 full 10 = FIFO 1/2 full 11 = FIFO 2 less than full"]
    #[inline(always)]
    pub fn uart_fifose_rt(&mut self) -> UART_FIFOSE_RT_W {
        UART_FIFOSE_RT_W { w: self }
    }
    #[doc = "Bits 4:5 - On read reserved On Write TX Empty Trigger (or TET): This is used to select the empty threshold level at which the THRE Interrupts will be generated when the mode is active. It also determines when the dma_tx_req_n signal will be asserted when in certain modes of operation. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO 1/4 full 11 = FIFO 1/2 full"]
    #[inline(always)]
    pub fn uart_tet(&mut self) -> UART_TET_W {
        UART_TET_W { w: self }
    }
    #[doc = "Bit 3 - On Read (Bit3) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write DMA Mode (or DMAM): This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals. 0 = mode 0 1 = mode 1"]
    #[inline(always)]
    pub fn uart_iid3_dmam(&mut self) -> UART_IID3_DMAM_W {
        UART_IID3_DMAM_W { w: self }
    }
    #[doc = "Bit 2 - On Read (Bit2) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write XMIT FIFO Reset (or XFIFOR): This resets the control portion of the transmit FIFO and treats the FIFO as empty. Note that this bit is 'self-clearing' and it is not necessary to clear this bit."]
    #[inline(always)]
    pub fn uart_iid2_xfifor(&mut self) -> UART_IID2_XFIFOR_W {
        UART_IID2_XFIFOR_W { w: self }
    }
    #[doc = "Bit 1 - On Read (Bit1) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write RCVR FIFO Reset (or RFIFOR): This resets the control portion of the receive FIFO and treats the FIFO as empty. Note that this bit is 'self-clearing' and it is not necessary to clear this bit."]
    #[inline(always)]
    pub fn uart_iid1_rfifoe(&mut self) -> UART_IID1_RFIFOE_W {
        UART_IID1_RFIFOE_W { w: self }
    }
    #[doc = "Bit 0 - On Read (Bit0) Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types: 0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout. On Write FIFO Enable (or FIFOE): This enables/disables the transmit (XMIT) and receive (RCVR) FIFO's. Whenever the value of this bit is changed both the XMIT and RCVR controller portion of FIFO's will be reset"]
    #[inline(always)]
    pub fn uart_iid0_fifoe(&mut self) -> UART_IID0_FIFOE_W {
        UART_IID0_FIFOE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Identification Register/FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_iir_fcr_reg](index.html) module"]
pub struct UART_IIR_FCR_REG_SPEC;
impl crate::RegisterSpec for UART_IIR_FCR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart_iir_fcr_reg::R](R) reader structure"]
impl crate::Readable for UART_IIR_FCR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_iir_fcr_reg::W](W) writer structure"]
impl crate::Writable for UART_IIR_FCR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_IIR_FCR_REG to value 0x01"]
impl crate::Resettable for UART_IIR_FCR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
