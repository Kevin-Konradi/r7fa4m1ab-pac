#[doc = "Register `CTSUCHTRC4` reader"]
pub type R = crate::R<Ctsuchtrc4Spec>;
#[doc = "Register `CTSUCHTRC4` writer"]
pub type W = crate::W<Ctsuchtrc4Spec>;
#[doc = "CTSU Channel Transmit/Receive Control 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsuchac4 {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<Ctsuchac4> for u8 {
    #[inline(always)]
    fn from(variant: Ctsuchac4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsuchac4 {
    type Ux = u8;
}
impl crate::IsEnum for Ctsuchac4 {}
#[doc = "Field `CTSUCHAC4` reader - CTSU Channel Transmit/Receive Control 4"]
pub type Ctsuchac4R = crate::FieldReader<Ctsuchac4>;
impl Ctsuchac4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctsuchac4> {
        match self.bits {
            0 => Some(Ctsuchac4::_0),
            1 => Some(Ctsuchac4::_1),
            _ => None,
        }
    }
    #[doc = "Reception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsuchac4::_0
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsuchac4::_1
    }
}
#[doc = "Field `CTSUCHAC4` writer - CTSU Channel Transmit/Receive Control 4"]
pub type Ctsuchac4W<'a, REG> = crate::FieldWriter<'a, REG, 4, Ctsuchac4>;
impl<'a, REG> Ctsuchac4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuchac4::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuchac4::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - CTSU Channel Transmit/Receive Control 4"]
    #[inline(always)]
    pub fn ctsuchac4(&self) -> Ctsuchac4R {
        Ctsuchac4R::new(self.bits & 0x0f)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUCHTRC4")
            .field("ctsuchac4", &self.ctsuchac4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - CTSU Channel Transmit/Receive Control 4"]
    #[inline(always)]
    pub fn ctsuchac4(&mut self) -> Ctsuchac4W<Ctsuchtrc4Spec> {
        Ctsuchac4W::new(self, 0)
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsuchtrc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsuchtrc4Spec;
impl crate::RegisterSpec for Ctsuchtrc4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchtrc4::R`](R) reader structure"]
impl crate::Readable for Ctsuchtrc4Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsuchtrc4::W`](W) writer structure"]
impl crate::Writable for Ctsuchtrc4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUCHTRC4 to value 0"]
impl crate::Resettable for Ctsuchtrc4Spec {}
