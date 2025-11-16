#[doc = "Register `CTSUCHTRC1` reader"]
pub type R = crate::R<Ctsuchtrc1Spec>;
#[doc = "Register `CTSUCHTRC1` writer"]
pub type W = crate::W<Ctsuchtrc1Spec>;
#[doc = "CTSU Channel Transmit/Receive Control 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctsuchtrc1 {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<Ctsuchtrc1> for u8 {
    #[inline(always)]
    fn from(variant: Ctsuchtrc1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctsuchtrc1 {
    type Ux = u8;
}
impl crate::IsEnum for Ctsuchtrc1 {}
#[doc = "Field `CTSUCHTRC1` reader - CTSU Channel Transmit/Receive Control 1"]
pub type Ctsuchtrc1R = crate::FieldReader<Ctsuchtrc1>;
impl Ctsuchtrc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctsuchtrc1> {
        match self.bits {
            0 => Some(Ctsuchtrc1::_0),
            1 => Some(Ctsuchtrc1::_1),
            _ => None,
        }
    }
    #[doc = "Reception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsuchtrc1::_0
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsuchtrc1::_1
    }
}
#[doc = "Field `CTSUCHTRC1` writer - CTSU Channel Transmit/Receive Control 1"]
pub type Ctsuchtrc1W<'a, REG> = crate::FieldWriter<'a, REG, 8, Ctsuchtrc1>;
impl<'a, REG> Ctsuchtrc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuchtrc1::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsuchtrc1::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Transmit/Receive Control 1"]
    #[inline(always)]
    pub fn ctsuchtrc1(&self) -> Ctsuchtrc1R {
        Ctsuchtrc1R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTSUCHTRC1")
            .field("ctsuchtrc1", &self.ctsuchtrc1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Transmit/Receive Control 1"]
    #[inline(always)]
    pub fn ctsuchtrc1(&mut self) -> Ctsuchtrc1W<Ctsuchtrc1Spec> {
        Ctsuchtrc1W::new(self, 0)
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctsuchtrc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctsuchtrc1Spec;
impl crate::RegisterSpec for Ctsuchtrc1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchtrc1::R`](R) reader structure"]
impl crate::Readable for Ctsuchtrc1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctsuchtrc1::W`](W) writer structure"]
impl crate::Writable for Ctsuchtrc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTSUCHTRC1 to value 0"]
impl crate::Resettable for Ctsuchtrc1Spec {}
