#[doc = "Register `CLK_RTCDIV_REG` reader"]
pub struct R(crate::R<CLK_RTCDIV_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_RTCDIV_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_RTCDIV_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_RTCDIV_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_RTCDIV_REG` writer"]
pub struct W(crate::W<CLK_RTCDIV_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_RTCDIV_REG_SPEC>;
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
impl From<crate::W<CLK_RTCDIV_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_RTCDIV_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_RESET_REQ` reader - Reset request for the RTC module"]
pub struct RTC_RESET_REQ_R(crate::FieldReader<bool, bool>);
impl RTC_RESET_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_RESET_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_RESET_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_RESET_REQ` writer - Reset request for the RTC module"]
pub struct RTC_RESET_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_RESET_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `RTC_DIV_ENABLE` reader - Enable for the 100 Hz generation for the RTC block"]
pub struct RTC_DIV_ENABLE_R(crate::FieldReader<bool, bool>);
impl RTC_DIV_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_DIV_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_DIV_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_DIV_ENABLE` writer - Enable for the 100 Hz generation for the RTC block"]
pub struct RTC_DIV_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DIV_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `RTC_DIV_DENOM` reader - Selects the denominator for the fractional division: 0b0: 1000 0b1: 1024"]
pub struct RTC_DIV_DENOM_R(crate::FieldReader<bool, bool>);
impl RTC_DIV_DENOM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_DIV_DENOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_DIV_DENOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_DIV_DENOM` writer - Selects the denominator for the fractional division: 0b0: 1000 0b1: 1024"]
pub struct RTC_DIV_DENOM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DIV_DENOM_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `RTC_DIV_INT` reader - Integer divisor part for RTC 100Hz generation"]
pub struct RTC_DIV_INT_R(crate::FieldReader<u16, u16>);
impl RTC_DIV_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RTC_DIV_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_DIV_INT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_DIV_INT` writer - Integer divisor part for RTC 100Hz generation"]
pub struct RTC_DIV_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DIV_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 10)) | ((value as u32 & 0x01ff) << 10);
        self.w
    }
}
#[doc = "Field `RTC_DIV_FRAC` reader - Fractional divisor part for RTC 100Hz generation. if RTC_DIV_DENOM=1, <RTC_DIV_FRAC> out of 1024 cycles will divide by <RTC_DIV_INT+1>, the rest is <RTC_DIV_INT> If RTC_DIV_DENOM=0, <RTC_DIV_FRAC> out of 1000 cycles will divide by <RTC_DIV_INT+1>, the rest is <RTC_DIV_INT>"]
pub struct RTC_DIV_FRAC_R(crate::FieldReader<u16, u16>);
impl RTC_DIV_FRAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RTC_DIV_FRAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_DIV_FRAC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_DIV_FRAC` writer - Fractional divisor part for RTC 100Hz generation. if RTC_DIV_DENOM=1, <RTC_DIV_FRAC> out of 1024 cycles will divide by <RTC_DIV_INT+1>, the rest is <RTC_DIV_INT> If RTC_DIV_DENOM=0, <RTC_DIV_FRAC> out of 1000 cycles will divide by <RTC_DIV_INT+1>, the rest is <RTC_DIV_INT>"]
pub struct RTC_DIV_FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DIV_FRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 21 - Reset request for the RTC module"]
    #[inline(always)]
    pub fn rtc_reset_req(&self) -> RTC_RESET_REQ_R {
        RTC_RESET_REQ_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable for the 100 Hz generation for the RTC block"]
    #[inline(always)]
    pub fn rtc_div_enable(&self) -> RTC_DIV_ENABLE_R {
        RTC_DIV_ENABLE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Selects the denominator for the fractional division: 0b0: 1000 0b1: 1024"]
    #[inline(always)]
    pub fn rtc_div_denom(&self) -> RTC_DIV_DENOM_R {
        RTC_DIV_DENOM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 10:18 - Integer divisor part for RTC 100Hz generation"]
    #[inline(always)]
    pub fn rtc_div_int(&self) -> RTC_DIV_INT_R {
        RTC_DIV_INT_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:9 - Fractional divisor part for RTC 100Hz generation. if RTC_DIV_DENOM=1, <RTC_DIV_FRAC> out of 1024 cycles will divide by <RTC_DIV_INT+1>, the rest is <RTC_DIV_INT> If RTC_DIV_DENOM=0, <RTC_DIV_FRAC> out of 1000 cycles will divide by <RTC_DIV_INT+1>, the rest is <RTC_DIV_INT>"]
    #[inline(always)]
    pub fn rtc_div_frac(&self) -> RTC_DIV_FRAC_R {
        RTC_DIV_FRAC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 21 - Reset request for the RTC module"]
    #[inline(always)]
    pub fn rtc_reset_req(&mut self) -> RTC_RESET_REQ_W {
        RTC_RESET_REQ_W { w: self }
    }
    #[doc = "Bit 20 - Enable for the 100 Hz generation for the RTC block"]
    #[inline(always)]
    pub fn rtc_div_enable(&mut self) -> RTC_DIV_ENABLE_W {
        RTC_DIV_ENABLE_W { w: self }
    }
    #[doc = "Bit 19 - Selects the denominator for the fractional division: 0b0: 1000 0b1: 1024"]
    #[inline(always)]
    pub fn rtc_div_denom(&mut self) -> RTC_DIV_DENOM_W {
        RTC_DIV_DENOM_W { w: self }
    }
    #[doc = "Bits 10:18 - Integer divisor part for RTC 100Hz generation"]
    #[inline(always)]
    pub fn rtc_div_int(&mut self) -> RTC_DIV_INT_W {
        RTC_DIV_INT_W { w: self }
    }
    #[doc = "Bits 0:9 - Fractional divisor part for RTC 100Hz generation. if RTC_DIV_DENOM=1, <RTC_DIV_FRAC> out of 1024 cycles will divide by <RTC_DIV_INT+1>, the rest is <RTC_DIV_INT> If RTC_DIV_DENOM=0, <RTC_DIV_FRAC> out of 1000 cycles will divide by <RTC_DIV_INT+1>, the rest is <RTC_DIV_INT>"]
    #[inline(always)]
    pub fn rtc_div_frac(&mut self) -> RTC_DIV_FRAC_W {
        RTC_DIV_FRAC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divisor for RTC 100Hz clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_rtcdiv_reg](index.html) module"]
pub struct CLK_RTCDIV_REG_SPEC;
impl crate::RegisterSpec for CLK_RTCDIV_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_rtcdiv_reg::R](R) reader structure"]
impl crate::Readable for CLK_RTCDIV_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_rtcdiv_reg::W](W) writer structure"]
impl crate::Writable for CLK_RTCDIV_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_RTCDIV_REG to value 0x0005_1ea8"]
impl crate::Resettable for CLK_RTCDIV_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0005_1ea8
    }
}
