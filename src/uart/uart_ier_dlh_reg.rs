#[doc = "Register `UART_IER_DLH_REG` reader"]
pub struct R(crate::R<UART_IER_DLH_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_IER_DLH_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_IER_DLH_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_IER_DLH_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_IER_DLH_REG` writer"]
pub struct W(crate::W<UART_IER_DLH_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_IER_DLH_REG_SPEC>;
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
impl From<crate::W<UART_IER_DLH_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_IER_DLH_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTIME_dlh7` reader - Interrupt Enable Register: PTIME, Programmable THRE Interrupt Mode Enable. This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled. Divisor Latch (High): DLH7, Bit 7 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
pub struct PTIME_DLH7_R(crate::FieldReader<bool, bool>);
impl PTIME_DLH7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PTIME_DLH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTIME_DLH7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTIME_dlh7` writer - Interrupt Enable Register: PTIME, Programmable THRE Interrupt Mode Enable. This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled. Divisor Latch (High): DLH7, Bit 7 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
pub struct PTIME_DLH7_W<'a> {
    w: &'a mut W,
}
impl<'a> PTIME_DLH7_W<'a> {
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
#[doc = "Field `dlh6_4` reader - Divisor Latch (High): DLH6 to DLH4, Bits 6 to 4 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set, otherwise, this field is reserved. See register UART_RBR_THR_DLL_REG."]
pub struct DLH6_4_R(crate::FieldReader<u8, u8>);
impl DLH6_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLH6_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLH6_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dlh6_4` writer - Divisor Latch (High): DLH6 to DLH4, Bits 6 to 4 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set, otherwise, this field is reserved. See register UART_RBR_THR_DLL_REG."]
pub struct DLH6_4_W<'a> {
    w: &'a mut W,
}
impl<'a> DLH6_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u16 & 7) << 4);
        self.w
    }
}
#[doc = "Field `EDSSI_dlh3` reader - Interrupt Enable Register: EDSSI, Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled Divisor Latch (High): DLH3, Bit 3 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
pub struct EDSSI_DLH3_R(crate::FieldReader<bool, bool>);
impl EDSSI_DLH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDSSI_DLH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDSSI_DLH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDSSI_dlh3` writer - Interrupt Enable Register: EDSSI, Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled Divisor Latch (High): DLH3, Bit 3 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
pub struct EDSSI_DLH3_W<'a> {
    w: &'a mut W,
}
impl<'a> EDSSI_DLH3_W<'a> {
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
#[doc = "Field `ELSI_dhl2` reader - Interrupt Enable Register: ELSI, Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled Divisor Latch (High): DLH2, Bit 2 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
pub struct ELSI_DHL2_R(crate::FieldReader<bool, bool>);
impl ELSI_DHL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ELSI_DHL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELSI_DHL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ELSI_dhl2` writer - Interrupt Enable Register: ELSI, Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled Divisor Latch (High): DLH2, Bit 2 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
pub struct ELSI_DHL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ELSI_DHL2_W<'a> {
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
#[doc = "Field `ETBEI_dlh1` reader - Interrupt Enable Register: ETBEI, Enable Transmit Holding Register Empty Interrupt. This is used to enable/disable the generation of Transmitter Holding Register Empty Interrupt. This is the third highest priority interrupt. 0 = disabled 1 = enabled Divisor Latch (High): DLH1, Bit 1 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
pub struct ETBEI_DLH1_R(crate::FieldReader<bool, bool>);
impl ETBEI_DLH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETBEI_DLH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETBEI_DLH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETBEI_dlh1` writer - Interrupt Enable Register: ETBEI, Enable Transmit Holding Register Empty Interrupt. This is used to enable/disable the generation of Transmitter Holding Register Empty Interrupt. This is the third highest priority interrupt. 0 = disabled 1 = enabled Divisor Latch (High): DLH1, Bit 1 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
pub struct ETBEI_DLH1_W<'a> {
    w: &'a mut W,
}
impl<'a> ETBEI_DLH1_W<'a> {
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
#[doc = "Field `ERBFI_dlh0` reader - Interrupt Enable Register: ERBFI, Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFO's enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled Divisor Latch (High): DLH0, Bit 0 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
pub struct ERBFI_DLH0_R(crate::FieldReader<bool, bool>);
impl ERBFI_DLH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERBFI_DLH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERBFI_DLH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERBFI_dlh0` writer - Interrupt Enable Register: ERBFI, Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFO's enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled Divisor Latch (High): DLH0, Bit 0 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
pub struct ERBFI_DLH0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERBFI_DLH0_W<'a> {
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
    #[doc = "Bit 7 - Interrupt Enable Register: PTIME, Programmable THRE Interrupt Mode Enable. This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled. Divisor Latch (High): DLH7, Bit 7 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn ptime_dlh7(&self) -> PTIME_DLH7_R {
        PTIME_DLH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Divisor Latch (High): DLH6 to DLH4, Bits 6 to 4 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set, otherwise, this field is reserved. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn dlh6_4(&self) -> DLH6_4_R {
        DLH6_4_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3 - Interrupt Enable Register: EDSSI, Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled Divisor Latch (High): DLH3, Bit 3 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn edssi_dlh3(&self) -> EDSSI_DLH3_R {
        EDSSI_DLH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Enable Register: ELSI, Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled Divisor Latch (High): DLH2, Bit 2 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn elsi_dhl2(&self) -> ELSI_DHL2_R {
        ELSI_DHL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Enable Register: ETBEI, Enable Transmit Holding Register Empty Interrupt. This is used to enable/disable the generation of Transmitter Holding Register Empty Interrupt. This is the third highest priority interrupt. 0 = disabled 1 = enabled Divisor Latch (High): DLH1, Bit 1 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn etbei_dlh1(&self) -> ETBEI_DLH1_R {
        ETBEI_DLH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Interrupt Enable Register: ERBFI, Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFO's enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled Divisor Latch (High): DLH0, Bit 0 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn erbfi_dlh0(&self) -> ERBFI_DLH0_R {
        ERBFI_DLH0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Interrupt Enable Register: PTIME, Programmable THRE Interrupt Mode Enable. This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled. Divisor Latch (High): DLH7, Bit 7 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn ptime_dlh7(&mut self) -> PTIME_DLH7_W {
        PTIME_DLH7_W { w: self }
    }
    #[doc = "Bits 4:6 - Divisor Latch (High): DLH6 to DLH4, Bits 6 to 4 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set, otherwise, this field is reserved. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn dlh6_4(&mut self) -> DLH6_4_W {
        DLH6_4_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt Enable Register: EDSSI, Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled Divisor Latch (High): DLH3, Bit 3 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn edssi_dlh3(&mut self) -> EDSSI_DLH3_W {
        EDSSI_DLH3_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt Enable Register: ELSI, Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled Divisor Latch (High): DLH2, Bit 2 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn elsi_dhl2(&mut self) -> ELSI_DHL2_W {
        ELSI_DHL2_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Enable Register: ETBEI, Enable Transmit Holding Register Empty Interrupt. This is used to enable/disable the generation of Transmitter Holding Register Empty Interrupt. This is the third highest priority interrupt. 0 = disabled 1 = enabled Divisor Latch (High): DLH1, Bit 1 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn etbei_dlh1(&mut self) -> ETBEI_DLH1_W {
        ETBEI_DLH1_W { w: self }
    }
    #[doc = "Bit 0 - Interrupt Enable Register: ERBFI, Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFO's enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled Divisor Latch (High): DLH0, Bit 0 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn erbfi_dlh0(&mut self) -> ERBFI_DLH0_W {
        ERBFI_DLH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register/Divisor Latch High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ier_dlh_reg](index.html) module"]
pub struct UART_IER_DLH_REG_SPEC;
impl crate::RegisterSpec for UART_IER_DLH_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart_ier_dlh_reg::R](R) reader structure"]
impl crate::Readable for UART_IER_DLH_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_ier_dlh_reg::W](W) writer structure"]
impl crate::Writable for UART_IER_DLH_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_IER_DLH_REG to value 0"]
impl crate::Resettable for UART_IER_DLH_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
