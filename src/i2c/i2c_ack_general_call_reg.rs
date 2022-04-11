#[doc = "Register `I2C_ACK_GENERAL_CALL_REG` reader"]
pub struct R(crate::R<I2C_ACK_GENERAL_CALL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_ACK_GENERAL_CALL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_ACK_GENERAL_CALL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_ACK_GENERAL_CALL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_ACK_GENERAL_CALL_REG` writer"]
pub struct W(crate::W<I2C_ACK_GENERAL_CALL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_ACK_GENERAL_CALL_REG_SPEC>;
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
impl From<crate::W<I2C_ACK_GENERAL_CALL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_ACK_GENERAL_CALL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACK_GEN_CALL` reader - ACK General Call. When set to 1, I2C Ctrl responds with a ACK (by asserting ic_data_oe) when it receives a General Call. When set to 0, the controller does not generate General Call interrupts."]
pub struct ACK_GEN_CALL_R(crate::FieldReader<bool, bool>);
impl ACK_GEN_CALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACK_GEN_CALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_GEN_CALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK_GEN_CALL` writer - ACK General Call. When set to 1, I2C Ctrl responds with a ACK (by asserting ic_data_oe) when it receives a General Call. When set to 0, the controller does not generate General Call interrupts."]
pub struct ACK_GEN_CALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_GEN_CALL_W<'a> {
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
    #[doc = "Bit 0 - ACK General Call. When set to 1, I2C Ctrl responds with a ACK (by asserting ic_data_oe) when it receives a General Call. When set to 0, the controller does not generate General Call interrupts."]
    #[inline(always)]
    pub fn ack_gen_call(&self) -> ACK_GEN_CALL_R {
        ACK_GEN_CALL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACK General Call. When set to 1, I2C Ctrl responds with a ACK (by asserting ic_data_oe) when it receives a General Call. When set to 0, the controller does not generate General Call interrupts."]
    #[inline(always)]
    pub fn ack_gen_call(&mut self) -> ACK_GEN_CALL_W {
        ACK_GEN_CALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C ACK General Call Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ack_general_call_reg](index.html) module"]
pub struct I2C_ACK_GENERAL_CALL_REG_SPEC;
impl crate::RegisterSpec for I2C_ACK_GENERAL_CALL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_ack_general_call_reg::R](R) reader structure"]
impl crate::Readable for I2C_ACK_GENERAL_CALL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_ack_general_call_reg::W](W) writer structure"]
impl crate::Writable for I2C_ACK_GENERAL_CALL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_ACK_GENERAL_CALL_REG to value 0"]
impl crate::Resettable for I2C_ACK_GENERAL_CALL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
