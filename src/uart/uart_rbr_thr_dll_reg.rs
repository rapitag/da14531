#[doc = "Register `UART_RBR_THR_DLL_REG` reader"]
pub struct R(crate::R<UART_RBR_THR_DLL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_RBR_THR_DLL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_RBR_THR_DLL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_RBR_THR_DLL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_RBR_THR_DLL_REG` writer"]
pub struct W(crate::W<UART_RBR_THR_DLL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_RBR_THR_DLL_REG_SPEC>;
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
impl From<crate::W<UART_RBR_THR_DLL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_RBR_THR_DLL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBR_THR_DLL` reader - Receive Buffer Register: (RBR). This register contains the data byte received on the serial input port (sin) in UART mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\]
set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\]
set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Transmit Holding Register: (THR) This register contains data to be transmitted on the serial output port (sout) in UART mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO's are disabled (FCR\\[0\\]
set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO's are enabled (FCR\\[0\\]
set to one) and THRE is set, 16 number of characters of data may be written to the THR before the FIFO is full. Any attempt to write data when the FIFO is full results in the write data being lost. Divisor Latch (Low): (DLL) This register makes up the lower 8-bits of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set. The output baud rate is equal to the serial clock (sclk) frequency divided by sixteen times the value of the baud rate divisor, as follows: baud rate = (serial clock freq) / (16 * divisor) Note that with the Divisor Latch Registers (DLL and DLH) set to zero, the baud clock is disabled and no serial communications will occur. Also, once the Divisor Latch is set, at least 8 clock cycles of the slowest UART clock should be allowed to pass before transmitting or receiving data. For the Divisor Latch (High) bits, see register UART_IER_DLH_REG."]
pub struct RBR_THR_DLL_R(crate::FieldReader<u8, u8>);
impl RBR_THR_DLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBR_THR_DLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBR_THR_DLL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBR_THR_DLL` writer - Receive Buffer Register: (RBR). This register contains the data byte received on the serial input port (sin) in UART mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\]
set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\]
set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Transmit Holding Register: (THR) This register contains data to be transmitted on the serial output port (sout) in UART mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO's are disabled (FCR\\[0\\]
set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO's are enabled (FCR\\[0\\]
set to one) and THRE is set, 16 number of characters of data may be written to the THR before the FIFO is full. Any attempt to write data when the FIFO is full results in the write data being lost. Divisor Latch (Low): (DLL) This register makes up the lower 8-bits of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set. The output baud rate is equal to the serial clock (sclk) frequency divided by sixteen times the value of the baud rate divisor, as follows: baud rate = (serial clock freq) / (16 * divisor) Note that with the Divisor Latch Registers (DLL and DLH) set to zero, the baud clock is disabled and no serial communications will occur. Also, once the Divisor Latch is set, at least 8 clock cycles of the slowest UART clock should be allowed to pass before transmitting or receiving data. For the Divisor Latch (High) bits, see register UART_IER_DLH_REG."]
pub struct RBR_THR_DLL_W<'a> {
    w: &'a mut W,
}
impl<'a> RBR_THR_DLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Buffer Register: (RBR). This register contains the data byte received on the serial input port (sin) in UART mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\]
set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\]
set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Transmit Holding Register: (THR) This register contains data to be transmitted on the serial output port (sout) in UART mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO's are disabled (FCR\\[0\\]
set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO's are enabled (FCR\\[0\\]
set to one) and THRE is set, 16 number of characters of data may be written to the THR before the FIFO is full. Any attempt to write data when the FIFO is full results in the write data being lost. Divisor Latch (Low): (DLL) This register makes up the lower 8-bits of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set. The output baud rate is equal to the serial clock (sclk) frequency divided by sixteen times the value of the baud rate divisor, as follows: baud rate = (serial clock freq) / (16 * divisor) Note that with the Divisor Latch Registers (DLL and DLH) set to zero, the baud clock is disabled and no serial communications will occur. Also, once the Divisor Latch is set, at least 8 clock cycles of the slowest UART clock should be allowed to pass before transmitting or receiving data. For the Divisor Latch (High) bits, see register UART_IER_DLH_REG."]
    #[inline(always)]
    pub fn rbr_thr_dll(&self) -> RBR_THR_DLL_R {
        RBR_THR_DLL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Buffer Register: (RBR). This register contains the data byte received on the serial input port (sin) in UART mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\]
set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\]
set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Transmit Holding Register: (THR) This register contains data to be transmitted on the serial output port (sout) in UART mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO's are disabled (FCR\\[0\\]
set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO's are enabled (FCR\\[0\\]
set to one) and THRE is set, 16 number of characters of data may be written to the THR before the FIFO is full. Any attempt to write data when the FIFO is full results in the write data being lost. Divisor Latch (Low): (DLL) This register makes up the lower 8-bits of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set. The output baud rate is equal to the serial clock (sclk) frequency divided by sixteen times the value of the baud rate divisor, as follows: baud rate = (serial clock freq) / (16 * divisor) Note that with the Divisor Latch Registers (DLL and DLH) set to zero, the baud clock is disabled and no serial communications will occur. Also, once the Divisor Latch is set, at least 8 clock cycles of the slowest UART clock should be allowed to pass before transmitting or receiving data. For the Divisor Latch (High) bits, see register UART_IER_DLH_REG."]
    #[inline(always)]
    pub fn rbr_thr_dll(&mut self) -> RBR_THR_DLL_W {
        RBR_THR_DLL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Buffer Register/Transmit Holding Register/Divisor Latch Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rbr_thr_dll_reg](index.html) module"]
pub struct UART_RBR_THR_DLL_REG_SPEC;
impl crate::RegisterSpec for UART_RBR_THR_DLL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart_rbr_thr_dll_reg::R](R) reader structure"]
impl crate::Readable for UART_RBR_THR_DLL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_rbr_thr_dll_reg::W](W) writer structure"]
impl crate::Writable for UART_RBR_THR_DLL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_RBR_THR_DLL_REG to value 0"]
impl crate::Resettable for UART_RBR_THR_DLL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
