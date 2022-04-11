#[doc = "Register `CLK_RC32M_REG` reader"]
pub struct R(crate::R<CLK_RC32M_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_RC32M_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_RC32M_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_RC32M_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_RC32M_REG` writer"]
pub struct W(crate::W<CLK_RC32M_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_RC32M_REG_SPEC>;
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
impl From<crate::W<CLK_RC32M_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_RC32M_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RC32M_COSC` reader - C-adjust of RC-oscillator A higher value of COSC results in a lower frequency"]
pub struct RC32M_COSC_R(crate::FieldReader<u8, u8>);
impl RC32M_COSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RC32M_COSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_COSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC32M_COSC` writer - C-adjust of RC-oscillator A higher value of COSC results in a lower frequency"]
pub struct RC32M_COSC_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_COSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u16 & 0x0f) << 7);
        self.w
    }
}
#[doc = "Field `RC32M_RANGE` reader - Coarse adjust A higher value of RANGE results in a higher frequency, values 2 and 3 are equal"]
pub struct RC32M_RANGE_R(crate::FieldReader<u8, u8>);
impl RC32M_RANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RC32M_RANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_RANGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC32M_RANGE` writer - Coarse adjust A higher value of RANGE results in a higher frequency, values 2 and 3 are equal"]
pub struct RC32M_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_RANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 5)) | ((value as u16 & 3) << 5);
        self.w
    }
}
#[doc = "Field `RC32M_BIAS` reader - Bias adjustment"]
pub struct RC32M_BIAS_R(crate::FieldReader<u8, u8>);
impl RC32M_BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RC32M_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_BIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC32M_BIAS` writer - Bias adjustment"]
pub struct RC32M_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_BIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u16 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `RC32M_DISABLE` reader - Instantly disables the 32MHz RC oscillator Disabling of the oscillator during sleep happens automatically."]
pub struct RC32M_DISABLE_R(crate::FieldReader<bool, bool>);
impl RC32M_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RC32M_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32M_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC32M_DISABLE` writer - Instantly disables the 32MHz RC oscillator Disabling of the oscillator during sleep happens automatically."]
pub struct RC32M_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32M_DISABLE_W<'a> {
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
    #[doc = "Bits 7:10 - C-adjust of RC-oscillator A higher value of COSC results in a lower frequency"]
    #[inline(always)]
    pub fn rc32m_cosc(&self) -> RC32M_COSC_R {
        RC32M_COSC_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - Coarse adjust A higher value of RANGE results in a higher frequency, values 2 and 3 are equal"]
    #[inline(always)]
    pub fn rc32m_range(&self) -> RC32M_RANGE_R {
        RC32M_RANGE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 1:4 - Bias adjustment"]
    #[inline(always)]
    pub fn rc32m_bias(&self) -> RC32M_BIAS_R {
        RC32M_BIAS_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - Instantly disables the 32MHz RC oscillator Disabling of the oscillator during sleep happens automatically."]
    #[inline(always)]
    pub fn rc32m_disable(&self) -> RC32M_DISABLE_R {
        RC32M_DISABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 7:10 - C-adjust of RC-oscillator A higher value of COSC results in a lower frequency"]
    #[inline(always)]
    pub fn rc32m_cosc(&mut self) -> RC32M_COSC_W {
        RC32M_COSC_W { w: self }
    }
    #[doc = "Bits 5:6 - Coarse adjust A higher value of RANGE results in a higher frequency, values 2 and 3 are equal"]
    #[inline(always)]
    pub fn rc32m_range(&mut self) -> RC32M_RANGE_W {
        RC32M_RANGE_W { w: self }
    }
    #[doc = "Bits 1:4 - Bias adjustment"]
    #[inline(always)]
    pub fn rc32m_bias(&mut self) -> RC32M_BIAS_W {
        RC32M_BIAS_W { w: self }
    }
    #[doc = "Bit 0 - Instantly disables the 32MHz RC oscillator Disabling of the oscillator during sleep happens automatically."]
    #[inline(always)]
    pub fn rc32m_disable(&mut self) -> RC32M_DISABLE_W {
        RC32M_DISABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast RC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_rc32m_reg](index.html) module"]
pub struct CLK_RC32M_REG_SPEC;
impl crate::RegisterSpec for CLK_RC32M_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [clk_rc32m_reg::R](R) reader structure"]
impl crate::Readable for CLK_RC32M_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_rc32m_reg::W](W) writer structure"]
impl crate::Writable for CLK_RC32M_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_RC32M_REG to value 0x078e"]
impl crate::Resettable for CLK_RC32M_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x078e
    }
}
