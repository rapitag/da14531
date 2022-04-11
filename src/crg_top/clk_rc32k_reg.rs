#[doc = "Register `CLK_RC32K_REG` reader"]
pub struct R(crate::R<CLK_RC32K_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_RC32K_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_RC32K_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_RC32K_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_RC32K_REG` writer"]
pub struct W(crate::W<CLK_RC32K_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_RC32K_REG_SPEC>;
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
impl From<crate::W<CLK_RC32K_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_RC32K_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RC32K_TRIM` reader - 0000 = lowest frequency 0111 = default 1111 = highest frequency"]
pub struct RC32K_TRIM_R(crate::FieldReader<u8, u8>);
impl RC32K_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RC32K_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32K_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC32K_TRIM` writer - 0000 = lowest frequency 0111 = default 1111 = highest frequency"]
pub struct RC32K_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u16 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `RC32K_DISABLE` reader - Instantly disables the 32kHz RC oscillator Sleep cycles cannot happen with this clock disabled."]
pub struct RC32K_DISABLE_R(crate::FieldReader<bool, bool>);
impl RC32K_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RC32K_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC32K_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC32K_DISABLE` writer - Instantly disables the 32kHz RC oscillator Sleep cycles cannot happen with this clock disabled."]
pub struct RC32K_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32K_DISABLE_W<'a> {
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
    #[doc = "Bits 1:4 - 0000 = lowest frequency 0111 = default 1111 = highest frequency"]
    #[inline(always)]
    pub fn rc32k_trim(&self) -> RC32K_TRIM_R {
        RC32K_TRIM_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - Instantly disables the 32kHz RC oscillator Sleep cycles cannot happen with this clock disabled."]
    #[inline(always)]
    pub fn rc32k_disable(&self) -> RC32K_DISABLE_R {
        RC32K_DISABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:4 - 0000 = lowest frequency 0111 = default 1111 = highest frequency"]
    #[inline(always)]
    pub fn rc32k_trim(&mut self) -> RC32K_TRIM_W {
        RC32K_TRIM_W { w: self }
    }
    #[doc = "Bit 0 - Instantly disables the 32kHz RC oscillator Sleep cycles cannot happen with this clock disabled."]
    #[inline(always)]
    pub fn rc32k_disable(&mut self) -> RC32K_DISABLE_W {
        RC32K_DISABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32 kHz RC oscillator register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_rc32k_reg](index.html) module"]
pub struct CLK_RC32K_REG_SPEC;
impl crate::RegisterSpec for CLK_RC32K_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [clk_rc32k_reg::R](R) reader structure"]
impl crate::Readable for CLK_RC32K_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_rc32k_reg::W](W) writer structure"]
impl crate::Writable for CLK_RC32K_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_RC32K_REG to value 0x0e"]
impl crate::Resettable for CLK_RC32K_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0e
    }
}
