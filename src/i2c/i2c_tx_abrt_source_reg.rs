#[doc = "Register `I2C_TX_ABRT_SOURCE_REG` reader"]
pub struct R(crate::R<I2C_TX_ABRT_SOURCE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TX_ABRT_SOURCE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TX_ABRT_SOURCE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TX_ABRT_SOURCE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_TX_ABRT_SOURCE_REG` writer"]
pub struct W(crate::W<I2C_TX_ABRT_SOURCE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TX_ABRT_SOURCE_REG_SPEC>;
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
impl From<crate::W<I2C_TX_ABRT_SOURCE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TX_ABRT_SOURCE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ABRT_SLVRD_INTX` reader - 1: When the processor side responds to a slave mode request for data to be transmitted to a remote master and user writes a 1 in CMD (bit 8) of 2IC_DATA_CMD register"]
pub struct ABRT_SLVRD_INTX_R(crate::FieldReader<bool, bool>);
impl ABRT_SLVRD_INTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_SLVRD_INTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_SLVRD_INTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_SLV_ARBLOST` reader - 1: Slave lost the bus while transmitting data to a remote master. I2C_TX_ABRT_SOURCE\\[12\\]
is set at the same time. Note: Even though the slave never \"owns\" the bus, something could go wrong on the bus. This is a fail safe check. For instance, during a data transmission at the low-to-high transition of SCL, if what is on the data bus is not what is supposed to be transmitted, then the controller no longer own the bus."]
pub struct ABRT_SLV_ARBLOST_R(crate::FieldReader<bool, bool>);
impl ABRT_SLV_ARBLOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_SLV_ARBLOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_SLV_ARBLOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_SLVFLUSH_TXFIFO` reader - 1: Slave has received a read command and some data exists in the TX FIFO so the slave issues a TX_ABRT interrupt to flush old data in TX FIFO."]
pub struct ABRT_SLVFLUSH_TXFIFO_R(crate::FieldReader<bool, bool>);
impl ABRT_SLVFLUSH_TXFIFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_SLVFLUSH_TXFIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_SLVFLUSH_TXFIFO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB_LOST` reader - 1: Master has lost arbitration, or if I2C_TX_ABRT_SOURCE\\[14\\]
is also set, then the slave transmitter has lost arbitration. Note: I2C can be both master and slave at the same time."]
pub struct ARB_LOST_R(crate::FieldReader<bool, bool>);
impl ARB_LOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB_LOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB_LOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_MASTER_DIS` reader - 1: User tries to initiate a Master operation with the Master mode disabled."]
pub struct ABRT_MASTER_DIS_R(crate::FieldReader<bool, bool>);
impl ABRT_MASTER_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_MASTER_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_MASTER_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_10B_RD_NORSTRT` reader - 1: The restart is disabled (IC_RESTART_EN bit (I2C_CON\\[5\\]) = 0) and the master sends a read command in 10-bit addressing mode."]
pub struct ABRT_10B_RD_NORSTRT_R(crate::FieldReader<bool, bool>);
impl ABRT_10B_RD_NORSTRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_10B_RD_NORSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_10B_RD_NORSTRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_SBYTE_NORSTRT` reader - To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; restart must be enabled (I2C_CON\\[5\\]=1), the SPECIAL bit must be cleared (I2C_TAR\\[11\\]), or the GC_OR_START bit must be cleared (I2C_TAR\\[10\\]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, bit 9 clears for one cycle and then gets re-asserted. 1: The restart is disabled (IC_RESTART_EN bit (I2C_CON\\[5\\]) = 0) and the user is trying to send a START Byte."]
pub struct ABRT_SBYTE_NORSTRT_R(crate::FieldReader<bool, bool>);
impl ABRT_SBYTE_NORSTRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_SBYTE_NORSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_SBYTE_NORSTRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_HS_NORSTRT` reader - 1: The restart is disabled (IC_RESTART_EN bit (I2C_CON\\[5\\]) = 0) and the user is trying to use the master to transfer data in High Speed mode"]
pub struct ABRT_HS_NORSTRT_R(crate::FieldReader<bool, bool>);
impl ABRT_HS_NORSTRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_HS_NORSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_HS_NORSTRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_SBYTE_ACKDET` reader - 1: Master has sent a START Byte and the START Byte was acknowledged (wrong behavior)."]
pub struct ABRT_SBYTE_ACKDET_R(crate::FieldReader<bool, bool>);
impl ABRT_SBYTE_ACKDET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_SBYTE_ACKDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_SBYTE_ACKDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_HS_ACKDET` reader - 1: Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior)."]
pub struct ABRT_HS_ACKDET_R(crate::FieldReader<bool, bool>);
impl ABRT_HS_ACKDET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_HS_ACKDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_HS_ACKDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_GCALL_READ` reader - 1: the controller in master mode sent a General Call but the user programmed the byte following the General Call to be a read from the bus (IC_DATA_CMD\\[9\\]
is set to 1)."]
pub struct ABRT_GCALL_READ_R(crate::FieldReader<bool, bool>);
impl ABRT_GCALL_READ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_GCALL_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_GCALL_READ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_GCALL_NOACK` reader - 1: the controller in master mode sent a General Call and no slave on the bus acknowledged the General Call."]
pub struct ABRT_GCALL_NOACK_R(crate::FieldReader<bool, bool>);
impl ABRT_GCALL_NOACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_GCALL_NOACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_GCALL_NOACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_TXDATA_NOACK` reader - 1: This is a master-mode only bit. Master has received an acknowledgement for the address, but when it sent data byte(s) following the address, it did not receive an acknowledge from the remote slave(s)."]
pub struct ABRT_TXDATA_NOACK_R(crate::FieldReader<bool, bool>);
impl ABRT_TXDATA_NOACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_TXDATA_NOACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_TXDATA_NOACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_10ADDR2_NOACK` reader - 1: Master is in 10-bit address mode and the second address byte of the 10-bit address was not acknowledged by any slave."]
pub struct ABRT_10ADDR2_NOACK_R(crate::FieldReader<bool, bool>);
impl ABRT_10ADDR2_NOACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_10ADDR2_NOACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_10ADDR2_NOACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_10ADDR1_NOACK` reader - 1: Master is in 10-bit address mode and the first 10-bit address byte was not acknowledged by any slave."]
pub struct ABRT_10ADDR1_NOACK_R(crate::FieldReader<bool, bool>);
impl ABRT_10ADDR1_NOACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_10ADDR1_NOACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_10ADDR1_NOACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRT_7B_ADDR_NOACK` reader - 1: Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave."]
pub struct ABRT_7B_ADDR_NOACK_R(crate::FieldReader<bool, bool>);
impl ABRT_7B_ADDR_NOACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABRT_7B_ADDR_NOACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABRT_7B_ADDR_NOACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 15 - 1: When the processor side responds to a slave mode request for data to be transmitted to a remote master and user writes a 1 in CMD (bit 8) of 2IC_DATA_CMD register"]
    #[inline(always)]
    pub fn abrt_slvrd_intx(&self) -> ABRT_SLVRD_INTX_R {
        ABRT_SLVRD_INTX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Slave lost the bus while transmitting data to a remote master. I2C_TX_ABRT_SOURCE\\[12\\]
is set at the same time. Note: Even though the slave never \"owns\" the bus, something could go wrong on the bus. This is a fail safe check. For instance, during a data transmission at the low-to-high transition of SCL, if what is on the data bus is not what is supposed to be transmitted, then the controller no longer own the bus."]
    #[inline(always)]
    pub fn abrt_slv_arblost(&self) -> ABRT_SLV_ARBLOST_R {
        ABRT_SLV_ARBLOST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Slave has received a read command and some data exists in the TX FIFO so the slave issues a TX_ABRT interrupt to flush old data in TX FIFO."]
    #[inline(always)]
    pub fn abrt_slvflush_txfifo(&self) -> ABRT_SLVFLUSH_TXFIFO_R {
        ABRT_SLVFLUSH_TXFIFO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Master has lost arbitration, or if I2C_TX_ABRT_SOURCE\\[14\\]
is also set, then the slave transmitter has lost arbitration. Note: I2C can be both master and slave at the same time."]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: User tries to initiate a Master operation with the Master mode disabled."]
    #[inline(always)]
    pub fn abrt_master_dis(&self) -> ABRT_MASTER_DIS_R {
        ABRT_MASTER_DIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: The restart is disabled (IC_RESTART_EN bit (I2C_CON\\[5\\]) = 0) and the master sends a read command in 10-bit addressing mode."]
    #[inline(always)]
    pub fn abrt_10b_rd_norstrt(&self) -> ABRT_10B_RD_NORSTRT_R {
        ABRT_10B_RD_NORSTRT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; restart must be enabled (I2C_CON\\[5\\]=1), the SPECIAL bit must be cleared (I2C_TAR\\[11\\]), or the GC_OR_START bit must be cleared (I2C_TAR\\[10\\]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, bit 9 clears for one cycle and then gets re-asserted. 1: The restart is disabled (IC_RESTART_EN bit (I2C_CON\\[5\\]) = 0) and the user is trying to send a START Byte."]
    #[inline(always)]
    pub fn abrt_sbyte_norstrt(&self) -> ABRT_SBYTE_NORSTRT_R {
        ABRT_SBYTE_NORSTRT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: The restart is disabled (IC_RESTART_EN bit (I2C_CON\\[5\\]) = 0) and the user is trying to use the master to transfer data in High Speed mode"]
    #[inline(always)]
    pub fn abrt_hs_norstrt(&self) -> ABRT_HS_NORSTRT_R {
        ABRT_HS_NORSTRT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Master has sent a START Byte and the START Byte was acknowledged (wrong behavior)."]
    #[inline(always)]
    pub fn abrt_sbyte_ackdet(&self) -> ABRT_SBYTE_ACKDET_R {
        ABRT_SBYTE_ACKDET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior)."]
    #[inline(always)]
    pub fn abrt_hs_ackdet(&self) -> ABRT_HS_ACKDET_R {
        ABRT_HS_ACKDET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: the controller in master mode sent a General Call but the user programmed the byte following the General Call to be a read from the bus (IC_DATA_CMD\\[9\\]
is set to 1)."]
    #[inline(always)]
    pub fn abrt_gcall_read(&self) -> ABRT_GCALL_READ_R {
        ABRT_GCALL_READ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: the controller in master mode sent a General Call and no slave on the bus acknowledged the General Call."]
    #[inline(always)]
    pub fn abrt_gcall_noack(&self) -> ABRT_GCALL_NOACK_R {
        ABRT_GCALL_NOACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: This is a master-mode only bit. Master has received an acknowledgement for the address, but when it sent data byte(s) following the address, it did not receive an acknowledge from the remote slave(s)."]
    #[inline(always)]
    pub fn abrt_txdata_noack(&self) -> ABRT_TXDATA_NOACK_R {
        ABRT_TXDATA_NOACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Master is in 10-bit address mode and the second address byte of the 10-bit address was not acknowledged by any slave."]
    #[inline(always)]
    pub fn abrt_10addr2_noack(&self) -> ABRT_10ADDR2_NOACK_R {
        ABRT_10ADDR2_NOACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Master is in 10-bit address mode and the first 10-bit address byte was not acknowledged by any slave."]
    #[inline(always)]
    pub fn abrt_10addr1_noack(&self) -> ABRT_10ADDR1_NOACK_R {
        ABRT_10ADDR1_NOACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - 1: Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave."]
    #[inline(always)]
    pub fn abrt_7b_addr_noack(&self) -> ABRT_7B_ADDR_NOACK_R {
        ABRT_7B_ADDR_NOACK_R::new((self.bits & 1) != 0)
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
#[doc = "I2C Transmit Abort Source Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_tx_abrt_source_reg](index.html) module"]
pub struct I2C_TX_ABRT_SOURCE_REG_SPEC;
impl crate::RegisterSpec for I2C_TX_ABRT_SOURCE_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_tx_abrt_source_reg::R](R) reader structure"]
impl crate::Readable for I2C_TX_ABRT_SOURCE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_tx_abrt_source_reg::W](W) writer structure"]
impl crate::Writable for I2C_TX_ABRT_SOURCE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_TX_ABRT_SOURCE_REG to value 0"]
impl crate::Resettable for I2C_TX_ABRT_SOURCE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
