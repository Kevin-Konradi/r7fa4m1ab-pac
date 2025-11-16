#[doc = "Register `CTSUCHTRC3` reader"]
pub type R = crate::R<Ctsuchtrc3Spec>;
#[doc = "Register `CTSUCHTRC3` writer"]
pub type W = crate::W<Ctsuchtrc3Spec>;
#[doc = "CTSU Channel Transmit/Receive Control 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsuchtrc3 {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<Ctsuchtrc3> for u8 {
    #[inline(always)]
    fn from(variant: Ctsuchtrc3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsuchtrc3 {
    type Ux = u8;
}
impl crate::IsEnum for Ctsuchtrc3 {}
#[doc = "Field `CTSUCHTRC3` reader - CTSU Channel Transmit/Receive Control 3"]
pub type Ctsuchtrc3R = crate::FieldReader<Ctsuchtrc3>;
impl Ctsuchtrc3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctsuchtrc3> {
        match self.bits {
            0 => Some(Ctsuchtrc3::_0),
            1 => Some(Ctsuchtrc3::_1),
            _ => None,
        }
    }
    #[doc = "Reception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsuchtrc3::_0
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsuchtrc3::_1
    }
}
#[doc = "Field `CTSUCHTRC3` writer - CTSU Channel Transmit/Receive Control 3"]
pub type Ctsuchtrc3W<'a, REG> = crate::FieldWriter<'a, REG, 8, Ctsuchtrc3>;
impl<'a, REG> Ctsuchtrc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuchtrc3::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuchtrc3::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Transmit/Receive Control 3"]
    #[inline(always)]
    pub fn ctsuchtrc3(&self) -> Ctsuchtrc3R {
        Ctsuchtrc3R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUCHTRC3")
            .field("ctsuchtrc3", &self.ctsuchtrc3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Transmit/Receive Control 3"]
    #[inline(always)]
    pub fn ctsuchtrc3(&mut self) -> Ctsuchtrc3W<Ctsuchtrc3Spec> {
        Ctsuchtrc3W::new(self, 0)
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsuchtrc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsuchtrc3Spec;
impl crate::RegisterSpec for Ctsuchtrc3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchtrc3::R`](R) reader structure"]
impl crate::Readable for Ctsuchtrc3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsuchtrc3::W`](W) writer structure"]
impl crate::Writable for Ctsuchtrc3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUCHTRC3 to value 0"]
impl crate::Resettable for Ctsuchtrc3Spec {}
