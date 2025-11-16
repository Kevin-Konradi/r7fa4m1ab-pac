#[doc = "Register `USBCKCR` reader"]
pub type R = crate::R<UsbckcrSpec>;
#[doc = "Register `USBCKCR` writer"]
pub type W = crate::W<UsbckcrSpec>;
#[doc = "USB Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbclksel {
    #[doc = "0: PLL(Value after reset)"]
    _0 = 0,
    #[doc = "1: HOCO"]
    _1 = 1,
}
impl From<Usbclksel> for bool {
    #[inline(always)]
    fn from(variant: Usbclksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCLKSEL` reader - USB Clock Source Select"]
pub type UsbclkselR = crate::BitReader<Usbclksel>;
impl UsbclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbclksel {
        match self.bits {
            false => Usbclksel::_0,
            true => Usbclksel::_1,
        }
    }
    #[doc = "PLL(Value after reset)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Usbclksel::_0
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Usbclksel::_1
    }
}
#[doc = "Field `USBCLKSEL` writer - USB Clock Source Select"]
pub type UsbclkselW<'a, REG> = crate::BitWriter<'a, REG, Usbclksel>;
impl<'a, REG> UsbclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL(Value after reset)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbclksel::_0)
    }
    #[doc = "HOCO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbclksel::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USB Clock Source Select"]
    #[inline(always)]
    pub fn usbclksel(&self) -> UsbclkselR {
        UsbclkselR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCKCR")
            .field("usbclksel", &self.usbclksel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Source Select"]
    #[inline(always)]
    pub fn usbclksel(&mut self) -> UsbclkselW<UsbckcrSpec> {
        UsbclkselW::new(self, 0)
    }
}
#[doc = "USB Clock Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbckcrSpec;
impl crate::RegisterSpec for UsbckcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbckcr::R`](R) reader structure"]
impl crate::Readable for UsbckcrSpec {}
#[doc = "`write(|w| ..)` method takes [`usbckcr::W`](W) writer structure"]
impl crate::Writable for UsbckcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBCKCR to value 0"]
impl crate::Resettable for UsbckcrSpec {}
