#[doc = "Register `CTSUCHTRC2` reader"]
pub type R = crate::R<Ctsuchtrc2Spec>;
#[doc = "Register `CTSUCHTRC2` writer"]
pub type W = crate::W<Ctsuchtrc2Spec>;
#[doc = "CTSU Channel Transmit/Receive Control 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsuchtrc2 {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<Ctsuchtrc2> for u8 {
    #[inline(always)]
    fn from(variant: Ctsuchtrc2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsuchtrc2 {
    type Ux = u8;
}
impl crate::IsEnum for Ctsuchtrc2 {}
#[doc = "Field `CTSUCHTRC2` reader - CTSU Channel Transmit/Receive Control 2"]
pub type Ctsuchtrc2R = crate::FieldReader<Ctsuchtrc2>;
impl Ctsuchtrc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctsuchtrc2> {
        match self.bits {
            0 => Some(Ctsuchtrc2::_0),
            1 => Some(Ctsuchtrc2::_1),
            _ => None,
        }
    }
    #[doc = "Reception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsuchtrc2::_0
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsuchtrc2::_1
    }
}
#[doc = "Field `CTSUCHTRC2` writer - CTSU Channel Transmit/Receive Control 2"]
pub type Ctsuchtrc2W<'a, REG> = crate::FieldWriter<'a, REG, 8, Ctsuchtrc2>;
impl<'a, REG> Ctsuchtrc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuchtrc2::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuchtrc2::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Transmit/Receive Control 2"]
    #[inline(always)]
    pub fn ctsuchtrc2(&self) -> Ctsuchtrc2R {
        Ctsuchtrc2R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUCHTRC2")
            .field("ctsuchtrc2", &self.ctsuchtrc2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Transmit/Receive Control 2"]
    #[inline(always)]
    pub fn ctsuchtrc2(&mut self) -> Ctsuchtrc2W<Ctsuchtrc2Spec> {
        Ctsuchtrc2W::new(self, 0)
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsuchtrc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsuchtrc2Spec;
impl crate::RegisterSpec for Ctsuchtrc2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchtrc2::R`](R) reader structure"]
impl crate::Readable for Ctsuchtrc2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsuchtrc2::W`](W) writer structure"]
impl crate::Writable for Ctsuchtrc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUCHTRC2 to value 0"]
impl crate::Resettable for Ctsuchtrc2Spec {}
