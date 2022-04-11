#[doc = "Register `CLK_AMBA_REG` reader"]
pub struct R(crate::R<CLK_AMBA_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_AMBA_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_AMBA_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_AMBA_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_AMBA_REG` writer"]
pub struct W(crate::W<CLK_AMBA_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_AMBA_REG_SPEC>;
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
impl From<crate::W<CLK_AMBA_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_AMBA_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTP_ENABLE` reader - Clock enable for OTP controller"]
pub struct OTP_ENABLE_R(crate::FieldReader<bool, bool>);
impl OTP_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP_ENABLE` writer - Clock enable for OTP controller"]
pub struct OTP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u16 & 1) << 7);
        self.w
    }
}
#[doc = "Field `PCLK_DIV` reader - APB interface clock (PCLK). Divider is cascaded with HCLK_DIV. PCLK is HCLK divided by: 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8"]
pub struct PCLK_DIV_R(crate::FieldReader<u8, u8>);
impl PCLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PCLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_DIV` writer - APB interface clock (PCLK). Divider is cascaded with HCLK_DIV. PCLK is HCLK divided by: 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8"]
pub struct PCLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u16 & 3) << 4);
        self.w
    }
}
#[doc = "Field `HCLK_DIV` reader - AHB interface and microprocessor clock (HCLK). HCLK is source clock divided by: 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8"]
pub struct HCLK_DIV_R(crate::FieldReader<u8, u8>);
impl HCLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HCLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCLK_DIV` writer - AHB interface and microprocessor clock (HCLK). HCLK is source clock divided by: 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8"]
pub struct HCLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u16 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Clock enable for OTP controller"]
    #[inline(always)]
    pub fn otp_enable(&self) -> OTP_ENABLE_R {
        OTP_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:5 - APB interface clock (PCLK). Divider is cascaded with HCLK_DIV. PCLK is HCLK divided by: 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8"]
    #[inline(always)]
    pub fn pclk_div(&self) -> PCLK_DIV_R {
        PCLK_DIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 0:1 - AHB interface and microprocessor clock (HCLK). HCLK is source clock divided by: 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8"]
    #[inline(always)]
    pub fn hclk_div(&self) -> HCLK_DIV_R {
        HCLK_DIV_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Clock enable for OTP controller"]
    #[inline(always)]
    pub fn otp_enable(&mut self) -> OTP_ENABLE_W {
        OTP_ENABLE_W { w: self }
    }
    #[doc = "Bits 4:5 - APB interface clock (PCLK). Divider is cascaded with HCLK_DIV. PCLK is HCLK divided by: 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8"]
    #[inline(always)]
    pub fn pclk_div(&mut self) -> PCLK_DIV_W {
        PCLK_DIV_W { w: self }
    }
    #[doc = "Bits 0:1 - AHB interface and microprocessor clock (HCLK). HCLK is source clock divided by: 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8"]
    #[inline(always)]
    pub fn hclk_div(&mut self) -> HCLK_DIV_W {
        HCLK_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HCLK, PCLK, divider and clock gates\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_amba_reg](index.html) module"]
pub struct CLK_AMBA_REG_SPEC;
impl crate::RegisterSpec for CLK_AMBA_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [clk_amba_reg::R](R) reader structure"]
impl crate::Readable for CLK_AMBA_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_amba_reg::W](W) writer structure"]
impl crate::Writable for CLK_AMBA_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_AMBA_REG to value 0"]
impl crate::Resettable for CLK_AMBA_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
