#[doc = "Register `CLK_FREQ_TRIM_REG` reader"]
pub struct R(crate::R<CLK_FREQ_TRIM_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_FREQ_TRIM_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_FREQ_TRIM_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_FREQ_TRIM_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_FREQ_TRIM_REG` writer"]
pub struct W(crate::W<CLK_FREQ_TRIM_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_FREQ_TRIM_REG_SPEC>;
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
impl From<crate::W<CLK_FREQ_TRIM_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_FREQ_TRIM_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32M_TRIM` reader - Xtal frequency fine trimming register. 0x00: highest frequency 0xFF: lowest frequency"]
pub struct XTAL32M_TRIM_R(crate::FieldReader<u8, u8>);
impl XTAL32M_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32M_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32M_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32M_TRIM` writer - Xtal frequency fine trimming register. 0x00: highest frequency 0xFF: lowest frequency"]
pub struct XTAL32M_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32M_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Xtal frequency fine trimming register. 0x00: highest frequency 0xFF: lowest frequency"]
    #[inline(always)]
    pub fn xtal32m_trim(&self) -> XTAL32M_TRIM_R {
        XTAL32M_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Xtal frequency fine trimming register. 0x00: highest frequency 0xFF: lowest frequency"]
    #[inline(always)]
    pub fn xtal32m_trim(&mut self) -> XTAL32M_TRIM_W {
        XTAL32M_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Xtal frequency trimming register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_freq_trim_reg](index.html) module"]
pub struct CLK_FREQ_TRIM_REG_SPEC;
impl crate::RegisterSpec for CLK_FREQ_TRIM_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [clk_freq_trim_reg::R](R) reader structure"]
impl crate::Readable for CLK_FREQ_TRIM_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_freq_trim_reg::W](W) writer structure"]
impl crate::Writable for CLK_FREQ_TRIM_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_FREQ_TRIM_REG to value 0"]
impl crate::Resettable for CLK_FREQ_TRIM_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
