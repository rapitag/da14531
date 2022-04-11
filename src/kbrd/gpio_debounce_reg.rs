#[doc = "Register `GPIO_DEBOUNCE_REG` reader"]
pub struct R(crate::R<GPIO_DEBOUNCE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_DEBOUNCE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_DEBOUNCE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_DEBOUNCE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_DEBOUNCE_REG` writer"]
pub struct W(crate::W<GPIO_DEBOUNCE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_DEBOUNCE_REG_SPEC>;
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
impl From<crate::W<GPIO_DEBOUNCE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_DEBOUNCE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEB_ENABLE_KBRD` reader - enables the debounce counter for the KBRD interface"]
pub struct DEB_ENABLE_KBRD_R(crate::FieldReader<bool, bool>);
impl DEB_ENABLE_KBRD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEB_ENABLE_KBRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEB_ENABLE_KBRD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEB_ENABLE_KBRD` writer - enables the debounce counter for the KBRD interface"]
pub struct DEB_ENABLE_KBRD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEB_ENABLE_KBRD_W<'a> {
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
#[doc = "Field `DEB_ENABLE4` reader - enables the debounce counter for GPIO IRQ4"]
pub struct DEB_ENABLE4_R(crate::FieldReader<bool, bool>);
impl DEB_ENABLE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEB_ENABLE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEB_ENABLE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEB_ENABLE4` writer - enables the debounce counter for GPIO IRQ4"]
pub struct DEB_ENABLE4_W<'a> {
    w: &'a mut W,
}
impl<'a> DEB_ENABLE4_W<'a> {
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
#[doc = "Field `DEB_ENABLE3` reader - enables the debounce counter for GPIO IRQ3"]
pub struct DEB_ENABLE3_R(crate::FieldReader<bool, bool>);
impl DEB_ENABLE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEB_ENABLE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEB_ENABLE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEB_ENABLE3` writer - enables the debounce counter for GPIO IRQ3"]
pub struct DEB_ENABLE3_W<'a> {
    w: &'a mut W,
}
impl<'a> DEB_ENABLE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u16 & 1) << 9);
        self.w
    }
}
#[doc = "Field `DEB_ENABLE2` reader - enables the debounce counter for GPIO IRQ2"]
pub struct DEB_ENABLE2_R(crate::FieldReader<bool, bool>);
impl DEB_ENABLE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEB_ENABLE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEB_ENABLE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEB_ENABLE2` writer - enables the debounce counter for GPIO IRQ2"]
pub struct DEB_ENABLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> DEB_ENABLE2_W<'a> {
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
#[doc = "Field `DEB_ENABLE1` reader - enables the debounce counter for GPIO IRQ1"]
pub struct DEB_ENABLE1_R(crate::FieldReader<bool, bool>);
impl DEB_ENABLE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEB_ENABLE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEB_ENABLE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEB_ENABLE1` writer - enables the debounce counter for GPIO IRQ1"]
pub struct DEB_ENABLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DEB_ENABLE1_W<'a> {
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
#[doc = "Field `DEB_ENABLE0` reader - enables the debounce counter for GPIO IRQ0"]
pub struct DEB_ENABLE0_R(crate::FieldReader<bool, bool>);
impl DEB_ENABLE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEB_ENABLE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEB_ENABLE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEB_ENABLE0` writer - enables the debounce counter for GPIO IRQ0"]
pub struct DEB_ENABLE0_W<'a> {
    w: &'a mut W,
}
impl<'a> DEB_ENABLE0_W<'a> {
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
#[doc = "Field `DEB_VALUE` reader - Keyboard debounce time if enabled. Generate KEYB_INT after specified time. Debounce time: N*1 ms. N =0..63"]
pub struct DEB_VALUE_R(crate::FieldReader<u8, u8>);
impl DEB_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEB_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEB_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEB_VALUE` writer - Keyboard debounce time if enabled. Generate KEYB_INT after specified time. Debounce time: N*1 ms. N =0..63"]
pub struct DEB_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEB_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u16 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - enables the debounce counter for the KBRD interface"]
    #[inline(always)]
    pub fn deb_enable_kbrd(&self) -> DEB_ENABLE_KBRD_R {
        DEB_ENABLE_KBRD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - enables the debounce counter for GPIO IRQ4"]
    #[inline(always)]
    pub fn deb_enable4(&self) -> DEB_ENABLE4_R {
        DEB_ENABLE4_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - enables the debounce counter for GPIO IRQ3"]
    #[inline(always)]
    pub fn deb_enable3(&self) -> DEB_ENABLE3_R {
        DEB_ENABLE3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - enables the debounce counter for GPIO IRQ2"]
    #[inline(always)]
    pub fn deb_enable2(&self) -> DEB_ENABLE2_R {
        DEB_ENABLE2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - enables the debounce counter for GPIO IRQ1"]
    #[inline(always)]
    pub fn deb_enable1(&self) -> DEB_ENABLE1_R {
        DEB_ENABLE1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - enables the debounce counter for GPIO IRQ0"]
    #[inline(always)]
    pub fn deb_enable0(&self) -> DEB_ENABLE0_R {
        DEB_ENABLE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 0:5 - Keyboard debounce time if enabled. Generate KEYB_INT after specified time. Debounce time: N*1 ms. N =0..63"]
    #[inline(always)]
    pub fn deb_value(&self) -> DEB_VALUE_R {
        DEB_VALUE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - enables the debounce counter for the KBRD interface"]
    #[inline(always)]
    pub fn deb_enable_kbrd(&mut self) -> DEB_ENABLE_KBRD_W {
        DEB_ENABLE_KBRD_W { w: self }
    }
    #[doc = "Bit 10 - enables the debounce counter for GPIO IRQ4"]
    #[inline(always)]
    pub fn deb_enable4(&mut self) -> DEB_ENABLE4_W {
        DEB_ENABLE4_W { w: self }
    }
    #[doc = "Bit 9 - enables the debounce counter for GPIO IRQ3"]
    #[inline(always)]
    pub fn deb_enable3(&mut self) -> DEB_ENABLE3_W {
        DEB_ENABLE3_W { w: self }
    }
    #[doc = "Bit 8 - enables the debounce counter for GPIO IRQ2"]
    #[inline(always)]
    pub fn deb_enable2(&mut self) -> DEB_ENABLE2_W {
        DEB_ENABLE2_W { w: self }
    }
    #[doc = "Bit 7 - enables the debounce counter for GPIO IRQ1"]
    #[inline(always)]
    pub fn deb_enable1(&mut self) -> DEB_ENABLE1_W {
        DEB_ENABLE1_W { w: self }
    }
    #[doc = "Bit 6 - enables the debounce counter for GPIO IRQ0"]
    #[inline(always)]
    pub fn deb_enable0(&mut self) -> DEB_ENABLE0_W {
        DEB_ENABLE0_W { w: self }
    }
    #[doc = "Bits 0:5 - Keyboard debounce time if enabled. Generate KEYB_INT after specified time. Debounce time: N*1 ms. N =0..63"]
    #[inline(always)]
    pub fn deb_value(&mut self) -> DEB_VALUE_W {
        DEB_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "debounce counter value for GPIO inputs\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_debounce_reg](index.html) module"]
pub struct GPIO_DEBOUNCE_REG_SPEC;
impl crate::RegisterSpec for GPIO_DEBOUNCE_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gpio_debounce_reg::R](R) reader structure"]
impl crate::Readable for GPIO_DEBOUNCE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_debounce_reg::W](W) writer structure"]
impl crate::Writable for GPIO_DEBOUNCE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_DEBOUNCE_REG to value 0"]
impl crate::Resettable for GPIO_DEBOUNCE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
