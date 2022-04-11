#[doc = "Register `RF_PORT_POL_REG` reader"]
pub struct R(crate::R<RF_PORT_POL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_PORT_POL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_PORT_POL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_PORT_POL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_PORT_POL_REG` writer"]
pub struct W(crate::W<RF_PORT_POL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_PORT_POL_REG_SPEC>;
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
impl From<crate::W<RF_PORT_POL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_PORT_POL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RF_PORT4_POL` reader - "]
pub struct RF_PORT4_POL_R(crate::FieldReader<bool, bool>);
impl RF_PORT4_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_PORT4_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_PORT4_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_PORT4_POL` writer - "]
pub struct RF_PORT4_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_PORT4_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `RF_PORT3_POL` reader - "]
pub struct RF_PORT3_POL_R(crate::FieldReader<bool, bool>);
impl RF_PORT3_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_PORT3_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_PORT3_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_PORT3_POL` writer - "]
pub struct RF_PORT3_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_PORT3_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `RF_PORT2_POL` reader - "]
pub struct RF_PORT2_POL_R(crate::FieldReader<bool, bool>);
impl RF_PORT2_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_PORT2_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_PORT2_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_PORT2_POL` writer - "]
pub struct RF_PORT2_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_PORT2_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `RF_PORT1_POL` reader - "]
pub struct RF_PORT1_POL_R(crate::FieldReader<bool, bool>);
impl RF_PORT1_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_PORT1_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_PORT1_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_PORT1_POL` writer - "]
pub struct RF_PORT1_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_PORT1_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `RF_PORT0_POL` reader - "]
pub struct RF_PORT0_POL_R(crate::FieldReader<bool, bool>);
impl RF_PORT0_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_PORT0_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_PORT0_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_PORT0_POL` writer - "]
pub struct RF_PORT0_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_PORT0_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rf_port4_pol(&self) -> RF_PORT4_POL_R {
        RF_PORT4_POL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_port3_pol(&self) -> RF_PORT3_POL_R {
        RF_PORT3_POL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rf_port2_pol(&self) -> RF_PORT2_POL_R {
        RF_PORT2_POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_port1_pol(&self) -> RF_PORT1_POL_R {
        RF_PORT1_POL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rf_port0_pol(&self) -> RF_PORT0_POL_R {
        RF_PORT0_POL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rf_port4_pol(&mut self) -> RF_PORT4_POL_W {
        RF_PORT4_POL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_port3_pol(&mut self) -> RF_PORT3_POL_W {
        RF_PORT3_POL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rf_port2_pol(&mut self) -> RF_PORT2_POL_W {
        RF_PORT2_POL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_port1_pol(&mut self) -> RF_PORT1_POL_W {
        RF_PORT1_POL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rf_port0_pol(&mut self) -> RF_PORT0_POL_W {
        RF_PORT0_POL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_port_pol_reg](index.html) module"]
pub struct RF_PORT_POL_REG_SPEC;
impl crate::RegisterSpec for RF_PORT_POL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_port_pol_reg::R](R) reader structure"]
impl crate::Readable for RF_PORT_POL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_port_pol_reg::W](W) writer structure"]
impl crate::Writable for RF_PORT_POL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_PORT_POL_REG to value 0"]
impl crate::Resettable for RF_PORT_POL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
