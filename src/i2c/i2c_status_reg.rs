#[doc = "Register `I2C_STATUS_REG` reader"]
pub struct R(crate::R<I2C_STATUS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_STATUS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_STATUS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_STATUS_REG` writer"]
pub struct W(crate::W<I2C_STATUS_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_STATUS_REG_SPEC>;
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
impl From<crate::W<I2C_STATUS_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_STATUS_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_ACTIVITY` reader - Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set. 0: Slave FSM is in IDLE state so the Slave part of the controller is not Active 1: Slave FSM is not in IDLE state so the Slave part of the controller is Active"]
pub struct SLV_ACTIVITY_R(crate::FieldReader<bool, bool>);
impl SLV_ACTIVITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_ACTIVITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_ACTIVITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST_ACTIVITY` reader - Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set. 0: Master FSM is in IDLE state so the Master part of the controller is not Active 1: Master FSM is not in IDLE state so the Master part of the controller is Active"]
pub struct MST_ACTIVITY_R(crate::FieldReader<bool, bool>);
impl MST_ACTIVITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MST_ACTIVITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_ACTIVITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFF` reader - Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. 0: Receive FIFO is not full 1: Receive FIFO is full"]
pub struct RFF_R(crate::FieldReader<bool, bool>);
impl RFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFNE` reader - Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. 0: Receive FIFO is empty 1: Receive FIFO is not empty"]
pub struct RFNE_R(crate::FieldReader<bool, bool>);
impl RFNE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. 0: Transmit FIFO is not empty 1: Transmit FIFO is empty"]
pub struct TFE_R(crate::FieldReader<bool, bool>);
impl TFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFNF` reader - Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. 0: Transmit FIFO is full 1: Transmit FIFO is not full"]
pub struct TFNF_R(crate::FieldReader<bool, bool>);
impl TFNF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFNF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_ACTIVITY` reader - I2C Activity Status."]
pub struct I2C_ACTIVITY_R(crate::FieldReader<bool, bool>);
impl I2C_ACTIVITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_ACTIVITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_ACTIVITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 6 - Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set. 0: Slave FSM is in IDLE state so the Slave part of the controller is not Active 1: Slave FSM is not in IDLE state so the Slave part of the controller is Active"]
    #[inline(always)]
    pub fn slv_activity(&self) -> SLV_ACTIVITY_R {
        SLV_ACTIVITY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set. 0: Master FSM is in IDLE state so the Master part of the controller is not Active 1: Master FSM is not in IDLE state so the Master part of the controller is Active"]
    #[inline(always)]
    pub fn mst_activity(&self) -> MST_ACTIVITY_R {
        MST_ACTIVITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. 0: Receive FIFO is not full 1: Receive FIFO is full"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. 0: Receive FIFO is empty 1: Receive FIFO is not empty"]
    #[inline(always)]
    pub fn rfne(&self) -> RFNE_R {
        RFNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. 0: Transmit FIFO is not empty 1: Transmit FIFO is empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. 0: Transmit FIFO is full 1: Transmit FIFO is not full"]
    #[inline(always)]
    pub fn tfnf(&self) -> TFNF_R {
        TFNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - I2C Activity Status."]
    #[inline(always)]
    pub fn i2c_activity(&self) -> I2C_ACTIVITY_R {
        I2C_ACTIVITY_R::new((self.bits & 1) != 0)
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
#[doc = "I2C Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_status_reg](index.html) module"]
pub struct I2C_STATUS_REG_SPEC;
impl crate::RegisterSpec for I2C_STATUS_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_status_reg::R](R) reader structure"]
impl crate::Readable for I2C_STATUS_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_status_reg::W](W) writer structure"]
impl crate::Writable for I2C_STATUS_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_STATUS_REG to value 0x06"]
impl crate::Resettable for I2C_STATUS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
