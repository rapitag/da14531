#[doc = "Register `GPIO_INT_LEVEL_CTRL_REG` reader"]
pub struct R(crate::R<GPIO_INT_LEVEL_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_LEVEL_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_LEVEL_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_LEVEL_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_LEVEL_CTRL_REG` writer"]
pub struct W(crate::W<GPIO_INT_LEVEL_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_LEVEL_CTRL_REG_SPEC>;
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
impl From<crate::W<GPIO_INT_LEVEL_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_LEVEL_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDGE_LEVELn4` reader - see EDGE_LEVELn0, but for GPIO IRQ4"]
pub struct EDGE_LEVELN4_R(crate::FieldReader<bool, bool>);
impl EDGE_LEVELN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDGE_LEVELN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE_LEVELN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE_LEVELn4` writer - see EDGE_LEVELn0, but for GPIO IRQ4"]
pub struct EDGE_LEVELN4_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_LEVELN4_W<'a> {
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
#[doc = "Field `EDGE_LEVELn3` reader - see EDGE_LEVELn0, but for GPIO IRQ3"]
pub struct EDGE_LEVELN3_R(crate::FieldReader<bool, bool>);
impl EDGE_LEVELN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDGE_LEVELN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE_LEVELN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE_LEVELn3` writer - see EDGE_LEVELn0, but for GPIO IRQ3"]
pub struct EDGE_LEVELN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_LEVELN3_W<'a> {
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
#[doc = "Field `EDGE_LEVELn2` reader - see EDGE_LEVELn0, but for GPIO IRQ2"]
pub struct EDGE_LEVELN2_R(crate::FieldReader<bool, bool>);
impl EDGE_LEVELN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDGE_LEVELN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE_LEVELN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE_LEVELn2` writer - see EDGE_LEVELn0, but for GPIO IRQ2"]
pub struct EDGE_LEVELN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_LEVELN2_W<'a> {
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
#[doc = "Field `EDGE_LEVELn1` reader - see EDGE_LEVELn0, but for GPIO IRQ1"]
pub struct EDGE_LEVELN1_R(crate::FieldReader<bool, bool>);
impl EDGE_LEVELN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDGE_LEVELN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE_LEVELN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE_LEVELn1` writer - see EDGE_LEVELn0, but for GPIO IRQ1"]
pub struct EDGE_LEVELN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_LEVELN1_W<'a> {
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
#[doc = "Field `EDGE_LEVELn0` reader - 0: do not wait for key release after interrupt was reset for GPIO IRQ0, so a new interrupt can be initiated immediately 1: wait for key release after interrupt was reset for IRQ0"]
pub struct EDGE_LEVELN0_R(crate::FieldReader<bool, bool>);
impl EDGE_LEVELN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDGE_LEVELN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGE_LEVELN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE_LEVELn0` writer - 0: do not wait for key release after interrupt was reset for GPIO IRQ0, so a new interrupt can be initiated immediately 1: wait for key release after interrupt was reset for IRQ0"]
pub struct EDGE_LEVELN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_LEVELN0_W<'a> {
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
#[doc = "Field `INPUT_LEVEL4` reader - see INPUT_LEVEL0, but for GPIO IRQ4"]
pub struct INPUT_LEVEL4_R(crate::FieldReader<bool, bool>);
impl INPUT_LEVEL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INPUT_LEVEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUT_LEVEL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUT_LEVEL4` writer - see INPUT_LEVEL0, but for GPIO IRQ4"]
pub struct INPUT_LEVEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_LEVEL4_W<'a> {
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
#[doc = "Field `INPUT_LEVEL3` reader - see INPUT_LEVEL0, but for GPIO IRQ3"]
pub struct INPUT_LEVEL3_R(crate::FieldReader<bool, bool>);
impl INPUT_LEVEL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INPUT_LEVEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUT_LEVEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUT_LEVEL3` writer - see INPUT_LEVEL0, but for GPIO IRQ3"]
pub struct INPUT_LEVEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_LEVEL3_W<'a> {
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
#[doc = "Field `INPUT_LEVEL2` reader - see INPUT_LEVEL0, but for GPIO IRQ2"]
pub struct INPUT_LEVEL2_R(crate::FieldReader<bool, bool>);
impl INPUT_LEVEL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INPUT_LEVEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUT_LEVEL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUT_LEVEL2` writer - see INPUT_LEVEL0, but for GPIO IRQ2"]
pub struct INPUT_LEVEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_LEVEL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u16 & 1) << 2);
        self.w
    }
}
#[doc = "Field `INPUT_LEVEL1` reader - see INPUT_LEVEL0, but for GPIO IRQ1"]
pub struct INPUT_LEVEL1_R(crate::FieldReader<bool, bool>);
impl INPUT_LEVEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INPUT_LEVEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUT_LEVEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUT_LEVEL1` writer - see INPUT_LEVEL0, but for GPIO IRQ1"]
pub struct INPUT_LEVEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_LEVEL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u16 & 1) << 1);
        self.w
    }
}
#[doc = "Field `INPUT_LEVEL0` reader - 0 = selected input will generate GPIO IRQ0 if that input is high. 1 = selected input will generate GPIO IRQ0 if that input is low."]
pub struct INPUT_LEVEL0_R(crate::FieldReader<bool, bool>);
impl INPUT_LEVEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INPUT_LEVEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUT_LEVEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPUT_LEVEL0` writer - 0 = selected input will generate GPIO IRQ0 if that input is high. 1 = selected input will generate GPIO IRQ0 if that input is low."]
pub struct INPUT_LEVEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_LEVEL0_W<'a> {
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
    #[doc = "Bit 9 - see EDGE_LEVELn0, but for GPIO IRQ4"]
    #[inline(always)]
    pub fn edge_leveln4(&self) -> EDGE_LEVELN4_R {
        EDGE_LEVELN4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - see EDGE_LEVELn0, but for GPIO IRQ3"]
    #[inline(always)]
    pub fn edge_leveln3(&self) -> EDGE_LEVELN3_R {
        EDGE_LEVELN3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - see EDGE_LEVELn0, but for GPIO IRQ2"]
    #[inline(always)]
    pub fn edge_leveln2(&self) -> EDGE_LEVELN2_R {
        EDGE_LEVELN2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - see EDGE_LEVELn0, but for GPIO IRQ1"]
    #[inline(always)]
    pub fn edge_leveln1(&self) -> EDGE_LEVELN1_R {
        EDGE_LEVELN1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: do not wait for key release after interrupt was reset for GPIO IRQ0, so a new interrupt can be initiated immediately 1: wait for key release after interrupt was reset for IRQ0"]
    #[inline(always)]
    pub fn edge_leveln0(&self) -> EDGE_LEVELN0_R {
        EDGE_LEVELN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - see INPUT_LEVEL0, but for GPIO IRQ4"]
    #[inline(always)]
    pub fn input_level4(&self) -> INPUT_LEVEL4_R {
        INPUT_LEVEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - see INPUT_LEVEL0, but for GPIO IRQ3"]
    #[inline(always)]
    pub fn input_level3(&self) -> INPUT_LEVEL3_R {
        INPUT_LEVEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - see INPUT_LEVEL0, but for GPIO IRQ2"]
    #[inline(always)]
    pub fn input_level2(&self) -> INPUT_LEVEL2_R {
        INPUT_LEVEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - see INPUT_LEVEL0, but for GPIO IRQ1"]
    #[inline(always)]
    pub fn input_level1(&self) -> INPUT_LEVEL1_R {
        INPUT_LEVEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - 0 = selected input will generate GPIO IRQ0 if that input is high. 1 = selected input will generate GPIO IRQ0 if that input is low."]
    #[inline(always)]
    pub fn input_level0(&self) -> INPUT_LEVEL0_R {
        INPUT_LEVEL0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - see EDGE_LEVELn0, but for GPIO IRQ4"]
    #[inline(always)]
    pub fn edge_leveln4(&mut self) -> EDGE_LEVELN4_W {
        EDGE_LEVELN4_W { w: self }
    }
    #[doc = "Bit 8 - see EDGE_LEVELn0, but for GPIO IRQ3"]
    #[inline(always)]
    pub fn edge_leveln3(&mut self) -> EDGE_LEVELN3_W {
        EDGE_LEVELN3_W { w: self }
    }
    #[doc = "Bit 7 - see EDGE_LEVELn0, but for GPIO IRQ2"]
    #[inline(always)]
    pub fn edge_leveln2(&mut self) -> EDGE_LEVELN2_W {
        EDGE_LEVELN2_W { w: self }
    }
    #[doc = "Bit 6 - see EDGE_LEVELn0, but for GPIO IRQ1"]
    #[inline(always)]
    pub fn edge_leveln1(&mut self) -> EDGE_LEVELN1_W {
        EDGE_LEVELN1_W { w: self }
    }
    #[doc = "Bit 5 - 0: do not wait for key release after interrupt was reset for GPIO IRQ0, so a new interrupt can be initiated immediately 1: wait for key release after interrupt was reset for IRQ0"]
    #[inline(always)]
    pub fn edge_leveln0(&mut self) -> EDGE_LEVELN0_W {
        EDGE_LEVELN0_W { w: self }
    }
    #[doc = "Bit 4 - see INPUT_LEVEL0, but for GPIO IRQ4"]
    #[inline(always)]
    pub fn input_level4(&mut self) -> INPUT_LEVEL4_W {
        INPUT_LEVEL4_W { w: self }
    }
    #[doc = "Bit 3 - see INPUT_LEVEL0, but for GPIO IRQ3"]
    #[inline(always)]
    pub fn input_level3(&mut self) -> INPUT_LEVEL3_W {
        INPUT_LEVEL3_W { w: self }
    }
    #[doc = "Bit 2 - see INPUT_LEVEL0, but for GPIO IRQ2"]
    #[inline(always)]
    pub fn input_level2(&mut self) -> INPUT_LEVEL2_W {
        INPUT_LEVEL2_W { w: self }
    }
    #[doc = "Bit 1 - see INPUT_LEVEL0, but for GPIO IRQ1"]
    #[inline(always)]
    pub fn input_level1(&mut self) -> INPUT_LEVEL1_W {
        INPUT_LEVEL1_W { w: self }
    }
    #[doc = "Bit 0 - 0 = selected input will generate GPIO IRQ0 if that input is high. 1 = selected input will generate GPIO IRQ0 if that input is low."]
    #[inline(always)]
    pub fn input_level0(&mut self) -> INPUT_LEVEL0_W {
        INPUT_LEVEL0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "high or low level select for GPIO interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_level_ctrl_reg](index.html) module"]
pub struct GPIO_INT_LEVEL_CTRL_REG_SPEC;
impl crate::RegisterSpec for GPIO_INT_LEVEL_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gpio_int_level_ctrl_reg::R](R) reader structure"]
impl crate::Readable for GPIO_INT_LEVEL_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_level_ctrl_reg::W](W) writer structure"]
impl crate::Writable for GPIO_INT_LEVEL_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INT_LEVEL_CTRL_REG to value 0"]
impl crate::Resettable for GPIO_INT_LEVEL_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
