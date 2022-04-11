#[doc = "Register `CLK_PER_REG` reader"]
pub struct R(crate::R<CLK_PER_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_PER_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_PER_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_PER_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_PER_REG` writer"]
pub struct W(crate::W<CLK_PER_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_PER_REG_SPEC>;
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
impl From<crate::W<CLK_PER_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_PER_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUAD_ENABLE` reader - Enable the Quadrature clock"]
pub struct QUAD_ENABLE_R(crate::FieldReader<bool, bool>);
impl QUAD_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QUAD_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUAD_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUAD_ENABLE` writer - Enable the Quadrature clock"]
pub struct QUAD_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> QUAD_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u16 & 1) << 11);
        self.w
    }
}
#[doc = "Field `SPI_ENABLE` reader - Enable SPI clock"]
pub struct SPI_ENABLE_R(crate::FieldReader<bool, bool>);
impl SPI_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_ENABLE` writer - Enable SPI clock"]
pub struct SPI_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u16 & 1) << 10);
        self.w
    }
}
#[doc = "Field `UART1_ENABLE` reader - Enable UART1 clock"]
pub struct UART1_ENABLE_R(crate::FieldReader<bool, bool>);
impl UART1_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART1_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART1_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1_ENABLE` writer - Enable UART1 clock"]
pub struct UART1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_ENABLE_W<'a> {
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
#[doc = "Field `UART2_ENABLE` reader - Enable UART2 clock"]
pub struct UART2_ENABLE_R(crate::FieldReader<bool, bool>);
impl UART2_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART2_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART2_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2_ENABLE` writer - Enable UART2 clock"]
pub struct UART2_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_ENABLE_W<'a> {
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
#[doc = "Field `I2C_ENABLE` reader - Enable I2C clock"]
pub struct I2C_ENABLE_R(crate::FieldReader<bool, bool>);
impl I2C_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_ENABLE` writer - Enable I2C clock"]
pub struct I2C_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ENABLE_W<'a> {
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
#[doc = "Field `WAKEUPCT_ENABLE` reader - Enable Wakeup CaptureTimer clock"]
pub struct WAKEUPCT_ENABLE_R(crate::FieldReader<bool, bool>);
impl WAKEUPCT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPCT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPCT_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPCT_ENABLE` writer - Enable Wakeup CaptureTimer clock"]
pub struct WAKEUPCT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPCT_ENABLE_W<'a> {
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
#[doc = "Field `TMR_ENABLE` reader - Enable TIMER0 and TIMER2 clock"]
pub struct TMR_ENABLE_R(crate::FieldReader<bool, bool>);
impl TMR_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TMR_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR_ENABLE` writer - Enable TIMER0 and TIMER2 clock"]
pub struct TMR_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_ENABLE_W<'a> {
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
#[doc = "Field `TMR_DIV` reader - Division factor for TIMER0 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8"]
pub struct TMR_DIV_R(crate::FieldReader<u8, u8>);
impl TMR_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMR_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR_DIV` writer - Division factor for TIMER0 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8"]
pub struct TMR_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u16 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - Enable the Quadrature clock"]
    #[inline(always)]
    pub fn quad_enable(&self) -> QUAD_ENABLE_R {
        QUAD_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable SPI clock"]
    #[inline(always)]
    pub fn spi_enable(&self) -> SPI_ENABLE_R {
        SPI_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable UART1 clock"]
    #[inline(always)]
    pub fn uart1_enable(&self) -> UART1_ENABLE_R {
        UART1_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable UART2 clock"]
    #[inline(always)]
    pub fn uart2_enable(&self) -> UART2_ENABLE_R {
        UART2_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable I2C clock"]
    #[inline(always)]
    pub fn i2c_enable(&self) -> I2C_ENABLE_R {
        I2C_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Wakeup CaptureTimer clock"]
    #[inline(always)]
    pub fn wakeupct_enable(&self) -> WAKEUPCT_ENABLE_R {
        WAKEUPCT_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable TIMER0 and TIMER2 clock"]
    #[inline(always)]
    pub fn tmr_enable(&self) -> TMR_ENABLE_R {
        TMR_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Division factor for TIMER0 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8"]
    #[inline(always)]
    pub fn tmr_div(&self) -> TMR_DIV_R {
        TMR_DIV_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - Enable the Quadrature clock"]
    #[inline(always)]
    pub fn quad_enable(&mut self) -> QUAD_ENABLE_W {
        QUAD_ENABLE_W { w: self }
    }
    #[doc = "Bit 10 - Enable SPI clock"]
    #[inline(always)]
    pub fn spi_enable(&mut self) -> SPI_ENABLE_W {
        SPI_ENABLE_W { w: self }
    }
    #[doc = "Bit 7 - Enable UART1 clock"]
    #[inline(always)]
    pub fn uart1_enable(&mut self) -> UART1_ENABLE_W {
        UART1_ENABLE_W { w: self }
    }
    #[doc = "Bit 6 - Enable UART2 clock"]
    #[inline(always)]
    pub fn uart2_enable(&mut self) -> UART2_ENABLE_W {
        UART2_ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - Enable I2C clock"]
    #[inline(always)]
    pub fn i2c_enable(&mut self) -> I2C_ENABLE_W {
        I2C_ENABLE_W { w: self }
    }
    #[doc = "Bit 4 - Enable Wakeup CaptureTimer clock"]
    #[inline(always)]
    pub fn wakeupct_enable(&mut self) -> WAKEUPCT_ENABLE_W {
        WAKEUPCT_ENABLE_W { w: self }
    }
    #[doc = "Bit 3 - Enable TIMER0 and TIMER2 clock"]
    #[inline(always)]
    pub fn tmr_enable(&mut self) -> TMR_ENABLE_W {
        TMR_ENABLE_W { w: self }
    }
    #[doc = "Bits 0:1 - Division factor for TIMER0 0x0: divide by 1 0x1: divide by 2 0x2: divide by 4 0x3: divide by 8"]
    #[inline(always)]
    pub fn tmr_div(&mut self) -> TMR_DIV_W {
        TMR_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_per_reg](index.html) module"]
pub struct CLK_PER_REG_SPEC;
impl crate::RegisterSpec for CLK_PER_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [clk_per_reg::R](R) reader structure"]
impl crate::Readable for CLK_PER_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_per_reg::W](W) writer structure"]
impl crate::Writable for CLK_PER_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_PER_REG to value 0x0800"]
impl crate::Resettable for CLK_PER_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}
