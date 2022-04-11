#[doc = "Register `I2C_ENABLE_REG` reader"]
pub struct R(crate::R<I2C_ENABLE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_ENABLE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_ENABLE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_ENABLE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_ENABLE_REG` writer"]
pub struct W(crate::W<I2C_ENABLE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_ENABLE_REG_SPEC>;
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
impl From<crate::W<I2C_ENABLE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_ENABLE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_ABORT` reader - 0= ABORT not initiated or ABORT done 1= ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation."]
pub struct I2C_ABORT_R(crate::FieldReader<bool, bool>);
impl I2C_ABORT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_ABORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_ABORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_ABORT` writer - 0= ABORT not initiated or ABORT done 1= ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation."]
pub struct I2C_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ABORT_W<'a> {
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
#[doc = "Field `CTRL_ENABLE` reader - Controls whether the controller is enabled. 0: Disables the controller (TX and RX FIFOs are held in an erased state) 1: Enables the controller Software can disable the controller while it is active. However, it is important that care be taken to ensure that the controller is disabled properly. When the controller is disabled, the following occurs: * The TX FIFO and RX FIFO get flushed. * Status bits in the IC_INTR_STAT register are still active until the controller goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the controller stops the current transfer at the end of the current byte and does not acknowledge the transfer. There is a two ic_clk delay when enabling or disabling the controller"]
pub struct CTRL_ENABLE_R(crate::FieldReader<bool, bool>);
impl CTRL_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTRL_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRL_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRL_ENABLE` writer - Controls whether the controller is enabled. 0: Disables the controller (TX and RX FIFOs are held in an erased state) 1: Enables the controller Software can disable the controller while it is active. However, it is important that care be taken to ensure that the controller is disabled properly. When the controller is disabled, the following occurs: * The TX FIFO and RX FIFO get flushed. * Status bits in the IC_INTR_STAT register are still active until the controller goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the controller stops the current transfer at the end of the current byte and does not acknowledge the transfer. There is a two ic_clk delay when enabling or disabling the controller"]
pub struct CTRL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_W<'a> {
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
    #[doc = "Bit 1 - 0= ABORT not initiated or ABORT done 1= ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation."]
    #[inline(always)]
    pub fn i2c_abort(&self) -> I2C_ABORT_R {
        I2C_ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Controls whether the controller is enabled. 0: Disables the controller (TX and RX FIFOs are held in an erased state) 1: Enables the controller Software can disable the controller while it is active. However, it is important that care be taken to ensure that the controller is disabled properly. When the controller is disabled, the following occurs: * The TX FIFO and RX FIFO get flushed. * Status bits in the IC_INTR_STAT register are still active until the controller goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the controller stops the current transfer at the end of the current byte and does not acknowledge the transfer. There is a two ic_clk delay when enabling or disabling the controller"]
    #[inline(always)]
    pub fn ctrl_enable(&self) -> CTRL_ENABLE_R {
        CTRL_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 0= ABORT not initiated or ABORT done 1= ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation."]
    #[inline(always)]
    pub fn i2c_abort(&mut self) -> I2C_ABORT_W {
        I2C_ABORT_W { w: self }
    }
    #[doc = "Bit 0 - Controls whether the controller is enabled. 0: Disables the controller (TX and RX FIFOs are held in an erased state) 1: Enables the controller Software can disable the controller while it is active. However, it is important that care be taken to ensure that the controller is disabled properly. When the controller is disabled, the following occurs: * The TX FIFO and RX FIFO get flushed. * Status bits in the IC_INTR_STAT register are still active until the controller goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the controller stops the current transfer at the end of the current byte and does not acknowledge the transfer. There is a two ic_clk delay when enabling or disabling the controller"]
    #[inline(always)]
    pub fn ctrl_enable(&mut self) -> CTRL_ENABLE_W {
        CTRL_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_enable_reg](index.html) module"]
pub struct I2C_ENABLE_REG_SPEC;
impl crate::RegisterSpec for I2C_ENABLE_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_enable_reg::R](R) reader structure"]
impl crate::Readable for I2C_ENABLE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_enable_reg::W](W) writer structure"]
impl crate::Writable for I2C_ENABLE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_ENABLE_REG to value 0"]
impl crate::Resettable for I2C_ENABLE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
