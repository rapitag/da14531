#[doc = "Register `I2C_CON_REG` reader"]
pub struct R(crate::R<I2C_CON_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CON_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CON_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CON_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CON_REG` writer"]
pub struct W(crate::W<I2C_CON_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CON_REG_SPEC>;
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
impl From<crate::W<I2C_CON_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CON_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SLAVE_DISABLE` reader - Slave enabled or disabled after reset is applied, which means software does not have to configure the slave. 0=slave is enabled 1=slave is disabled Software should ensure that if this bit is written with '0', then bit 0 should also be written with a '0'."]
pub struct I2C_SLAVE_DISABLE_R(crate::FieldReader<bool, bool>);
impl I2C_SLAVE_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_SLAVE_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_SLAVE_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_SLAVE_DISABLE` writer - Slave enabled or disabled after reset is applied, which means software does not have to configure the slave. 0=slave is enabled 1=slave is disabled Software should ensure that if this bit is written with '0', then bit 0 should also be written with a '0'."]
pub struct I2C_SLAVE_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLAVE_DISABLE_W<'a> {
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
#[doc = "Field `I2C_RESTART_EN` reader - Determines whether RESTART conditions may be sent when acting as a master 0= disable 1=enable"]
pub struct I2C_RESTART_EN_R(crate::FieldReader<bool, bool>);
impl I2C_RESTART_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_RESTART_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_RESTART_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_RESTART_EN` writer - Determines whether RESTART conditions may be sent when acting as a master 0= disable 1=enable"]
pub struct I2C_RESTART_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RESTART_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u16 & 1) << 5);
        self.w
    }
}
#[doc = "Field `I2C_10BITADDR_MASTER` reader - Controls whether the controller starts its transfers in 7- or 10-bit addressing mode when acting as a master. 0= 7-bit addressing 1= 10-bit addressing"]
pub struct I2C_10BITADDR_MASTER_R(crate::FieldReader<bool, bool>);
impl I2C_10BITADDR_MASTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_10BITADDR_MASTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_10BITADDR_MASTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_10BITADDR_MASTER` writer - Controls whether the controller starts its transfers in 7- or 10-bit addressing mode when acting as a master. 0= 7-bit addressing 1= 10-bit addressing"]
pub struct I2C_10BITADDR_MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_10BITADDR_MASTER_W<'a> {
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
#[doc = "Field `I2C_10BITADDR_SLAVE` reader - When acting as a slave, this bit controls whether the controller responds to 7- or 10-bit addresses. 0= 7-bit addressing 1= 10-bit addressing"]
pub struct I2C_10BITADDR_SLAVE_R(crate::FieldReader<bool, bool>);
impl I2C_10BITADDR_SLAVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_10BITADDR_SLAVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_10BITADDR_SLAVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_10BITADDR_SLAVE` writer - When acting as a slave, this bit controls whether the controller responds to 7- or 10-bit addresses. 0= 7-bit addressing 1= 10-bit addressing"]
pub struct I2C_10BITADDR_SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_10BITADDR_SLAVE_W<'a> {
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
#[doc = "Field `I2C_SPEED` reader - These bits control at which speed the controller operates. 1= standard mode (100 kbit/s) 2= fast mode (400 kbit/s) Note: The actuall speed depends on the pcb traces capacitance as well as on the values of the external pull-up resistorts. For an exact speed match, trimming might be required, by adjusting the values of I2C_SS_SCL_HCNT_REG, I2C_SS_SCL_LCNT_REG, I2C_FS_SCL_HCNT_REG, I2C_FS_SCL_LCNT_REG registers. The reset values of those registers were calculated with the assumption of 4.3kOhms external pull-up resistors."]
pub struct I2C_SPEED_R(crate::FieldReader<u8, u8>);
impl I2C_SPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2C_SPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_SPEED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_SPEED` writer - These bits control at which speed the controller operates. 1= standard mode (100 kbit/s) 2= fast mode (400 kbit/s) Note: The actuall speed depends on the pcb traces capacitance as well as on the values of the external pull-up resistorts. For an exact speed match, trimming might be required, by adjusting the values of I2C_SS_SCL_HCNT_REG, I2C_SS_SCL_LCNT_REG, I2C_FS_SCL_HCNT_REG, I2C_FS_SCL_LCNT_REG registers. The reset values of those registers were calculated with the assumption of 4.3kOhms external pull-up resistors."]
pub struct I2C_SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SPEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 1)) | ((value as u16 & 3) << 1);
        self.w
    }
}
#[doc = "Field `I2C_MASTER_MODE` reader - This bit controls whether the controller master is enabled. 0= master disabled 1= master enabled Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'."]
pub struct I2C_MASTER_MODE_R(crate::FieldReader<bool, bool>);
impl I2C_MASTER_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_MASTER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_MASTER_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_MASTER_MODE` writer - This bit controls whether the controller master is enabled. 0= master disabled 1= master enabled Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'."]
pub struct I2C_MASTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MASTER_MODE_W<'a> {
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
    #[doc = "Bit 6 - Slave enabled or disabled after reset is applied, which means software does not have to configure the slave. 0=slave is enabled 1=slave is disabled Software should ensure that if this bit is written with '0', then bit 0 should also be written with a '0'."]
    #[inline(always)]
    pub fn i2c_slave_disable(&self) -> I2C_SLAVE_DISABLE_R {
        I2C_SLAVE_DISABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Determines whether RESTART conditions may be sent when acting as a master 0= disable 1=enable"]
    #[inline(always)]
    pub fn i2c_restart_en(&self) -> I2C_RESTART_EN_R {
        I2C_RESTART_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Controls whether the controller starts its transfers in 7- or 10-bit addressing mode when acting as a master. 0= 7-bit addressing 1= 10-bit addressing"]
    #[inline(always)]
    pub fn i2c_10bitaddr_master(&self) -> I2C_10BITADDR_MASTER_R {
        I2C_10BITADDR_MASTER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - When acting as a slave, this bit controls whether the controller responds to 7- or 10-bit addresses. 0= 7-bit addressing 1= 10-bit addressing"]
    #[inline(always)]
    pub fn i2c_10bitaddr_slave(&self) -> I2C_10BITADDR_SLAVE_R {
        I2C_10BITADDR_SLAVE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 1:2 - These bits control at which speed the controller operates. 1= standard mode (100 kbit/s) 2= fast mode (400 kbit/s) Note: The actuall speed depends on the pcb traces capacitance as well as on the values of the external pull-up resistorts. For an exact speed match, trimming might be required, by adjusting the values of I2C_SS_SCL_HCNT_REG, I2C_SS_SCL_LCNT_REG, I2C_FS_SCL_HCNT_REG, I2C_FS_SCL_LCNT_REG registers. The reset values of those registers were calculated with the assumption of 4.3kOhms external pull-up resistors."]
    #[inline(always)]
    pub fn i2c_speed(&self) -> I2C_SPEED_R {
        I2C_SPEED_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 0 - This bit controls whether the controller master is enabled. 0= master disabled 1= master enabled Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'."]
    #[inline(always)]
    pub fn i2c_master_mode(&self) -> I2C_MASTER_MODE_R {
        I2C_MASTER_MODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Slave enabled or disabled after reset is applied, which means software does not have to configure the slave. 0=slave is enabled 1=slave is disabled Software should ensure that if this bit is written with '0', then bit 0 should also be written with a '0'."]
    #[inline(always)]
    pub fn i2c_slave_disable(&mut self) -> I2C_SLAVE_DISABLE_W {
        I2C_SLAVE_DISABLE_W { w: self }
    }
    #[doc = "Bit 5 - Determines whether RESTART conditions may be sent when acting as a master 0= disable 1=enable"]
    #[inline(always)]
    pub fn i2c_restart_en(&mut self) -> I2C_RESTART_EN_W {
        I2C_RESTART_EN_W { w: self }
    }
    #[doc = "Bit 4 - Controls whether the controller starts its transfers in 7- or 10-bit addressing mode when acting as a master. 0= 7-bit addressing 1= 10-bit addressing"]
    #[inline(always)]
    pub fn i2c_10bitaddr_master(&mut self) -> I2C_10BITADDR_MASTER_W {
        I2C_10BITADDR_MASTER_W { w: self }
    }
    #[doc = "Bit 3 - When acting as a slave, this bit controls whether the controller responds to 7- or 10-bit addresses. 0= 7-bit addressing 1= 10-bit addressing"]
    #[inline(always)]
    pub fn i2c_10bitaddr_slave(&mut self) -> I2C_10BITADDR_SLAVE_W {
        I2C_10BITADDR_SLAVE_W { w: self }
    }
    #[doc = "Bits 1:2 - These bits control at which speed the controller operates. 1= standard mode (100 kbit/s) 2= fast mode (400 kbit/s) Note: The actuall speed depends on the pcb traces capacitance as well as on the values of the external pull-up resistorts. For an exact speed match, trimming might be required, by adjusting the values of I2C_SS_SCL_HCNT_REG, I2C_SS_SCL_LCNT_REG, I2C_FS_SCL_HCNT_REG, I2C_FS_SCL_LCNT_REG registers. The reset values of those registers were calculated with the assumption of 4.3kOhms external pull-up resistors."]
    #[inline(always)]
    pub fn i2c_speed(&mut self) -> I2C_SPEED_W {
        I2C_SPEED_W { w: self }
    }
    #[doc = "Bit 0 - This bit controls whether the controller master is enabled. 0= master disabled 1= master enabled Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'."]
    #[inline(always)]
    pub fn i2c_master_mode(&mut self) -> I2C_MASTER_MODE_W {
        I2C_MASTER_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_con_reg](index.html) module"]
pub struct I2C_CON_REG_SPEC;
impl crate::RegisterSpec for I2C_CON_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_con_reg::R](R) reader structure"]
impl crate::Readable for I2C_CON_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_con_reg::W](W) writer structure"]
impl crate::Writable for I2C_CON_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CON_REG to value 0x7d"]
impl crate::Resettable for I2C_CON_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7d
    }
}
