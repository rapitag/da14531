#[doc = "Register `I2C_ENABLE_STATUS_REG` reader"]
pub struct R(crate::R<I2C_ENABLE_STATUS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_ENABLE_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_ENABLE_STATUS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_ENABLE_STATUS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_ENABLE_STATUS_REG` writer"]
pub struct W(crate::W<I2C_ENABLE_STATUS_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_ENABLE_STATUS_REG_SPEC>;
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
impl From<crate::W<I2C_ENABLE_STATUS_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_ENABLE_STATUS_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_RX_DATA_LOST` reader - Slave Received Data Lost. This bit indicates if a Slave-Receiver operation has been aborted with at least one data byte received from an I2C transfer due to the setting of IC_ENABLE from 1 to 0. When read as 1, the controller is deemed to have been actively engaged in an aborted I2C transfer (with matching address) and the data phase of the I2C transfer has been entered, even though a data byte has been responded with a NACK. NOTE: If the remote I2C master terminates the transfer with a STOP condition before the controller has a chance to NACK a transfer, and IC_ENABLE has been set to 0, then this bit is also set to 1. When read as 0, the controller is deemed to have been disabled without being actively involved in the data phase of a Slave-Receiver transfer. NOTE: The CPU can safely read this bit when IC_EN (bit 0) is read as 0."]
pub struct SLV_RX_DATA_LOST_R(crate::FieldReader<bool, bool>);
impl SLV_RX_DATA_LOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RX_DATA_LOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RX_DATA_LOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_DISABLED_WHILE_BUSY` reader - Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting of the IC_ENABLE register from 1 to 0. This bit is set when the CPU writes a 0 to the IC_ENABLE register while: (a) I2C Ctrl is receiving the address byte of the Slave-Transmitter operation from a remote master; OR, (b) address and data bytes of the Slave-Receiver operation from a remote master. When read as 1, the controller is deemed to have forced a NACK during any part of an I2C transfer, irrespective of whether the I2C address matches the slave address set in I2C Ctrl (IC_SAR register) OR if the transfer is completed before IC_ENABLE is set to 0 but has not taken effect. NOTE: If the remote I2C master terminates the transfer with a STOP condition before the the controller has a chance to NACK a transfer, and IC_ENABLE has been set to 0, then this bit will also be set to 1. When read as 0, the controller is deemed to have been disabled when there is master activity, or when the I2C bus is idle. NOTE: The CPU can safely read this bit when IC_EN (bit 0) is read as 0."]
pub struct SLV_DISABLED_WHILE_BUSY_R(crate::FieldReader<bool, bool>);
impl SLV_DISABLED_WHILE_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_DISABLED_WHILE_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_DISABLED_WHILE_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_EN` reader - ic_en Status. This bit always reflects the value driven on the output port ic_en. When read as 1, the controller is deemed to be in an enabled state. When read as 0, the controller is deemed completely inactive. NOTE: The CPU can safely read this bit anytime. When this bit is read as 0, the CPU can safely read SLV_RX_DATA_LOST (bit 2) and SLV_DISABLED_WHILE_BUSY (bit 1)."]
pub struct IC_EN_R(crate::FieldReader<bool, bool>);
impl IC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - Slave Received Data Lost. This bit indicates if a Slave-Receiver operation has been aborted with at least one data byte received from an I2C transfer due to the setting of IC_ENABLE from 1 to 0. When read as 1, the controller is deemed to have been actively engaged in an aborted I2C transfer (with matching address) and the data phase of the I2C transfer has been entered, even though a data byte has been responded with a NACK. NOTE: If the remote I2C master terminates the transfer with a STOP condition before the controller has a chance to NACK a transfer, and IC_ENABLE has been set to 0, then this bit is also set to 1. When read as 0, the controller is deemed to have been disabled without being actively involved in the data phase of a Slave-Receiver transfer. NOTE: The CPU can safely read this bit when IC_EN (bit 0) is read as 0."]
    #[inline(always)]
    pub fn slv_rx_data_lost(&self) -> SLV_RX_DATA_LOST_R {
        SLV_RX_DATA_LOST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting of the IC_ENABLE register from 1 to 0. This bit is set when the CPU writes a 0 to the IC_ENABLE register while: (a) I2C Ctrl is receiving the address byte of the Slave-Transmitter operation from a remote master; OR, (b) address and data bytes of the Slave-Receiver operation from a remote master. When read as 1, the controller is deemed to have forced a NACK during any part of an I2C transfer, irrespective of whether the I2C address matches the slave address set in I2C Ctrl (IC_SAR register) OR if the transfer is completed before IC_ENABLE is set to 0 but has not taken effect. NOTE: If the remote I2C master terminates the transfer with a STOP condition before the the controller has a chance to NACK a transfer, and IC_ENABLE has been set to 0, then this bit will also be set to 1. When read as 0, the controller is deemed to have been disabled when there is master activity, or when the I2C bus is idle. NOTE: The CPU can safely read this bit when IC_EN (bit 0) is read as 0."]
    #[inline(always)]
    pub fn slv_disabled_while_busy(&self) -> SLV_DISABLED_WHILE_BUSY_R {
        SLV_DISABLED_WHILE_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - ic_en Status. This bit always reflects the value driven on the output port ic_en. When read as 1, the controller is deemed to be in an enabled state. When read as 0, the controller is deemed completely inactive. NOTE: The CPU can safely read this bit anytime. When this bit is read as 0, the CPU can safely read SLV_RX_DATA_LOST (bit 2) and SLV_DISABLED_WHILE_BUSY (bit 1)."]
    #[inline(always)]
    pub fn ic_en(&self) -> IC_EN_R {
        IC_EN_R::new((self.bits & 1) != 0)
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
#[doc = "I2C Enable Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_enable_status_reg](index.html) module"]
pub struct I2C_ENABLE_STATUS_REG_SPEC;
impl crate::RegisterSpec for I2C_ENABLE_STATUS_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_enable_status_reg::R](R) reader structure"]
impl crate::Readable for I2C_ENABLE_STATUS_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_enable_status_reg::W](W) writer structure"]
impl crate::Writable for I2C_ENABLE_STATUS_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_ENABLE_STATUS_REG to value 0"]
impl crate::Resettable for I2C_ENABLE_STATUS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
