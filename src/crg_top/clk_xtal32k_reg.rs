#[doc = "Register `CLK_XTAL32K_REG` reader"]
pub struct R(crate::R<CLK_XTAL32K_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_XTAL32K_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_XTAL32K_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_XTAL32K_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_XTAL32K_REG` writer"]
pub struct W(crate::W<CLK_XTAL32K_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_XTAL32K_REG_SPEC>;
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
impl From<crate::W<CLK_XTAL32K_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_XTAL32K_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32K_XTAL1_BIAS_DISABLE` reader - "]
pub struct XTAL32K_XTAL1_BIAS_DISABLE_R(crate::FieldReader<bool, bool>);
impl XTAL32K_XTAL1_BIAS_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_XTAL1_BIAS_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_XTAL1_BIAS_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_XTAL1_BIAS_DISABLE` writer - "]
pub struct XTAL32K_XTAL1_BIAS_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_XTAL1_BIAS_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u16 & 1) << 8);
        self.w
    }
}
#[doc = "Field `XTAL32K_DISABLE_AMPREG` reader - Setting this bit disables the amplitude regulation of the XTAL32kHz oscillator. Set this bit to '1' for an external clock to XTAL32Kp Keep this bit '0' with a crystal between XTAL32Kp and XTAL32Km"]
pub struct XTAL32K_DISABLE_AMPREG_R(crate::FieldReader<bool, bool>);
impl XTAL32K_DISABLE_AMPREG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_DISABLE_AMPREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_DISABLE_AMPREG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_DISABLE_AMPREG` writer - Setting this bit disables the amplitude regulation of the XTAL32kHz oscillator. Set this bit to '1' for an external clock to XTAL32Kp Keep this bit '0' with a crystal between XTAL32Kp and XTAL32Km"]
pub struct XTAL32K_DISABLE_AMPREG_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_DISABLE_AMPREG_W<'a> {
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
#[doc = "Field `XTAL32K_CUR` reader - Bias current for the 32kHz XTAL oscillator. 0000 is minimum, 1111 is maximum, 0011 is default. For each application there is an optimal setting for which the start-up behavior is optimal"]
pub struct XTAL32K_CUR_R(crate::FieldReader<u8, u8>);
impl XTAL32K_CUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32K_CUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_CUR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_CUR` writer - Bias current for the 32kHz XTAL oscillator. 0000 is minimum, 1111 is maximum, 0011 is default. For each application there is an optimal setting for which the start-up behavior is optimal"]
pub struct XTAL32K_CUR_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_CUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u16 & 0x0f) << 3);
        self.w
    }
}
#[doc = "Field `XTAL32K_RBIAS` reader - Setting for the bias resistor. 00 is maximum, 11 is minimum. Prefered setting will be provided by Dialog"]
pub struct XTAL32K_RBIAS_R(crate::FieldReader<u8, u8>);
impl XTAL32K_RBIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32K_RBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_RBIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_RBIAS` writer - Setting for the bias resistor. 00 is maximum, 11 is minimum. Prefered setting will be provided by Dialog"]
pub struct XTAL32K_RBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_RBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 1)) | ((value as u16 & 3) << 1);
        self.w
    }
}
#[doc = "Field `XTAL32K_ENABLE` reader - Enables the 32kHz XTAL oscillator. Also set GP_DATA_REG\\[P03_P04_FILT_DIS\\]
= 1 for lowest current consumption."]
pub struct XTAL32K_ENABLE_R(crate::FieldReader<bool, bool>);
impl XTAL32K_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_ENABLE` writer - Enables the 32kHz XTAL oscillator. Also set GP_DATA_REG\\[P03_P04_FILT_DIS\\]
= 1 for lowest current consumption."]
pub struct XTAL32K_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_ENABLE_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn xtal32k_xtal1_bias_disable(&self) -> XTAL32K_XTAL1_BIAS_DISABLE_R {
        XTAL32K_XTAL1_BIAS_DISABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Setting this bit disables the amplitude regulation of the XTAL32kHz oscillator. Set this bit to '1' for an external clock to XTAL32Kp Keep this bit '0' with a crystal between XTAL32Kp and XTAL32Km"]
    #[inline(always)]
    pub fn xtal32k_disable_ampreg(&self) -> XTAL32K_DISABLE_AMPREG_R {
        XTAL32K_DISABLE_AMPREG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 3:6 - Bias current for the 32kHz XTAL oscillator. 0000 is minimum, 1111 is maximum, 0011 is default. For each application there is an optimal setting for which the start-up behavior is optimal"]
    #[inline(always)]
    pub fn xtal32k_cur(&self) -> XTAL32K_CUR_R {
        XTAL32K_CUR_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 1:2 - Setting for the bias resistor. 00 is maximum, 11 is minimum. Prefered setting will be provided by Dialog"]
    #[inline(always)]
    pub fn xtal32k_rbias(&self) -> XTAL32K_RBIAS_R {
        XTAL32K_RBIAS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 0 - Enables the 32kHz XTAL oscillator. Also set GP_DATA_REG\\[P03_P04_FILT_DIS\\]
= 1 for lowest current consumption."]
    #[inline(always)]
    pub fn xtal32k_enable(&self) -> XTAL32K_ENABLE_R {
        XTAL32K_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn xtal32k_xtal1_bias_disable(&mut self) -> XTAL32K_XTAL1_BIAS_DISABLE_W {
        XTAL32K_XTAL1_BIAS_DISABLE_W { w: self }
    }
    #[doc = "Bit 7 - Setting this bit disables the amplitude regulation of the XTAL32kHz oscillator. Set this bit to '1' for an external clock to XTAL32Kp Keep this bit '0' with a crystal between XTAL32Kp and XTAL32Km"]
    #[inline(always)]
    pub fn xtal32k_disable_ampreg(&mut self) -> XTAL32K_DISABLE_AMPREG_W {
        XTAL32K_DISABLE_AMPREG_W { w: self }
    }
    #[doc = "Bits 3:6 - Bias current for the 32kHz XTAL oscillator. 0000 is minimum, 1111 is maximum, 0011 is default. For each application there is an optimal setting for which the start-up behavior is optimal"]
    #[inline(always)]
    pub fn xtal32k_cur(&mut self) -> XTAL32K_CUR_W {
        XTAL32K_CUR_W { w: self }
    }
    #[doc = "Bits 1:2 - Setting for the bias resistor. 00 is maximum, 11 is minimum. Prefered setting will be provided by Dialog"]
    #[inline(always)]
    pub fn xtal32k_rbias(&mut self) -> XTAL32K_RBIAS_W {
        XTAL32K_RBIAS_W { w: self }
    }
    #[doc = "Bit 0 - Enables the 32kHz XTAL oscillator. Also set GP_DATA_REG\\[P03_P04_FILT_DIS\\]
= 1 for lowest current consumption."]
    #[inline(always)]
    pub fn xtal32k_enable(&mut self) -> XTAL32K_ENABLE_W {
        XTAL32K_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32 kHz XTAL oscillator register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_xtal32k_reg](index.html) module"]
pub struct CLK_XTAL32K_REG_SPEC;
impl crate::RegisterSpec for CLK_XTAL32K_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [clk_xtal32k_reg::R](R) reader structure"]
impl crate::Readable for CLK_XTAL32K_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_xtal32k_reg::W](W) writer structure"]
impl crate::Writable for CLK_XTAL32K_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_XTAL32K_REG to value 0x2e"]
impl crate::Resettable for CLK_XTAL32K_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2e
    }
}
