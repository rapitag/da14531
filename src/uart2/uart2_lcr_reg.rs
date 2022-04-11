#[doc = "Register `UART2_LCR_REG` reader"]
pub struct R(crate::R<UART2_LCR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART2_LCR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART2_LCR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART2_LCR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART2_LCR_REG` writer"]
pub struct W(crate::W<UART2_LCR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART2_LCR_REG_SPEC>;
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
impl From<crate::W<UART2_LCR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART2_LCR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_DLAB` reader - Divisor Latch Access Bit.Writeable only when UART is not busy (USR\\[0\\]
is zero). This bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART. This bit must be cleared after initial baud rate setup in order to access other registers."]
pub struct UART_DLAB_R(crate::FieldReader<bool, bool>);
impl UART_DLAB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_DLAB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_DLAB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_DLAB` writer - Divisor Latch Access Bit.Writeable only when UART is not busy (USR\\[0\\]
is zero). This bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART. This bit must be cleared after initial baud rate setup in order to access other registers."]
pub struct UART_DLAB_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DLAB_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u16 & 1) << 7);
        self.w
    }
}
#[doc = "Field `UART_BC` reader - Break Control Bit. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If active (MCR\\[6\\]
set to one) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
pub struct UART_BC_R(crate::FieldReader<bool, bool>);
impl UART_BC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_BC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_BC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_BC` writer - Break Control Bit. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If active (MCR\\[6\\]
set to one) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
pub struct UART_BC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_BC_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u16 & 1) << 6);
        self.w
    }
}
#[doc = "Field `UART_EPS` reader - Even Parity Select. Writeable only when UART is not busy (USR\\[0\\]
is zero). This is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
pub struct UART_EPS_R(crate::FieldReader<bool, bool>);
impl UART_EPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_EPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_EPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_EPS` writer - Even Parity Select. Writeable only when UART is not busy (USR\\[0\\]
is zero). This is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
pub struct UART_EPS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_EPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u16 & 1) << 4);
        self.w
    }
}
#[doc = "Field `UART_PEN` reader - Parity Enable. Writeable only when UART is not busy (USR\\[0\\]
is zero) This bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively. 0 = parity disabled 1 = parity enabled"]
pub struct UART_PEN_R(crate::FieldReader<bool, bool>);
impl UART_PEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_PEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_PEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_PEN` writer - Parity Enable. Writeable only when UART is not busy (USR\\[0\\]
is zero) This bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively. 0 = parity disabled 1 = parity enabled"]
pub struct UART_PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_PEN_W<'a> {
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
#[doc = "Field `UART_STOP` reader - Number of stop bits. Writeable only when UART is not busy (USR\\[0\\]
is zero). This is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data. If set to one and the data bits are set to 5 (LCR\\[1:0\\]
set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit. 0 = 1 stop bit 1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit"]
pub struct UART_STOP_R(crate::FieldReader<bool, bool>);
impl UART_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_STOP` writer - Number of stop bits. Writeable only when UART is not busy (USR\\[0\\]
is zero). This is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data. If set to one and the data bits are set to 5 (LCR\\[1:0\\]
set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit. 0 = 1 stop bit 1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit"]
pub struct UART_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_STOP_W<'a> {
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
#[doc = "Field `UART_DLS` reader - Data Length Select.Writeable only when UART is not busy (USR\\[0\\]
is zero). This is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows: 00 = 5 bits 01 = 6 bits 10 = 7 bits 11 = 8 bits"]
pub struct UART_DLS_R(crate::FieldReader<u8, u8>);
impl UART_DLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART_DLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_DLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_DLS` writer - Data Length Select.Writeable only when UART is not busy (USR\\[0\\]
is zero). This is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows: 00 = 5 bits 01 = 6 bits 10 = 7 bits 11 = 8 bits"]
pub struct UART_DLS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u16 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Divisor Latch Access Bit.Writeable only when UART is not busy (USR\\[0\\]
is zero). This bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART. This bit must be cleared after initial baud rate setup in order to access other registers."]
    #[inline(always)]
    pub fn uart_dlab(&self) -> UART_DLAB_R {
        UART_DLAB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Break Control Bit. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If active (MCR\\[6\\]
set to one) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
    #[inline(always)]
    pub fn uart_bc(&self) -> UART_BC_R {
        UART_BC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - Even Parity Select. Writeable only when UART is not busy (USR\\[0\\]
is zero). This is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
    #[inline(always)]
    pub fn uart_eps(&self) -> UART_EPS_R {
        UART_EPS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable. Writeable only when UART is not busy (USR\\[0\\]
is zero) This bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively. 0 = parity disabled 1 = parity enabled"]
    #[inline(always)]
    pub fn uart_pen(&self) -> UART_PEN_R {
        UART_PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Number of stop bits. Writeable only when UART is not busy (USR\\[0\\]
is zero). This is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data. If set to one and the data bits are set to 5 (LCR\\[1:0\\]
set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit. 0 = 1 stop bit 1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit"]
    #[inline(always)]
    pub fn uart_stop(&self) -> UART_STOP_R {
        UART_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Data Length Select.Writeable only when UART is not busy (USR\\[0\\]
is zero). This is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows: 00 = 5 bits 01 = 6 bits 10 = 7 bits 11 = 8 bits"]
    #[inline(always)]
    pub fn uart_dls(&self) -> UART_DLS_R {
        UART_DLS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Divisor Latch Access Bit.Writeable only when UART is not busy (USR\\[0\\]
is zero). This bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART. This bit must be cleared after initial baud rate setup in order to access other registers."]
    #[inline(always)]
    pub fn uart_dlab(&mut self) -> UART_DLAB_W {
        UART_DLAB_W { w: self }
    }
    #[doc = "Bit 6 - Break Control Bit. This is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If active (MCR\\[6\\]
set to one) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
    #[inline(always)]
    pub fn uart_bc(&mut self) -> UART_BC_W {
        UART_BC_W { w: self }
    }
    #[doc = "Bit 4 - Even Parity Select. Writeable only when UART is not busy (USR\\[0\\]
is zero). This is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
    #[inline(always)]
    pub fn uart_eps(&mut self) -> UART_EPS_W {
        UART_EPS_W { w: self }
    }
    #[doc = "Bit 3 - Parity Enable. Writeable only when UART is not busy (USR\\[0\\]
is zero) This bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively. 0 = parity disabled 1 = parity enabled"]
    #[inline(always)]
    pub fn uart_pen(&mut self) -> UART_PEN_W {
        UART_PEN_W { w: self }
    }
    #[doc = "Bit 2 - Number of stop bits. Writeable only when UART is not busy (USR\\[0\\]
is zero). This is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data. If set to one and the data bits are set to 5 (LCR\\[1:0\\]
set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit. 0 = 1 stop bit 1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit"]
    #[inline(always)]
    pub fn uart_stop(&mut self) -> UART_STOP_W {
        UART_STOP_W { w: self }
    }
    #[doc = "Bits 0:1 - Data Length Select.Writeable only when UART is not busy (USR\\[0\\]
is zero). This is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows: 00 = 5 bits 01 = 6 bits 10 = 7 bits 11 = 8 bits"]
    #[inline(always)]
    pub fn uart_dls(&mut self) -> UART_DLS_W {
        UART_DLS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart2_lcr_reg](index.html) module"]
pub struct UART2_LCR_REG_SPEC;
impl crate::RegisterSpec for UART2_LCR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart2_lcr_reg::R](R) reader structure"]
impl crate::Readable for UART2_LCR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart2_lcr_reg::W](W) writer structure"]
impl crate::Writable for UART2_LCR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART2_LCR_REG to value 0"]
impl crate::Resettable for UART2_LCR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
