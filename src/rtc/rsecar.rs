#[doc = "Register `RSECAR` reader"]
pub type R = crate::R<RsecarSpec>;
#[doc = "Register `RSECAR` writer"]
pub type W = crate::W<RsecarSpec>;
#[doc = "Field `SEC1` reader - 1-Second Value for the ones place of seconds"]
pub type Sec1R = crate::FieldReader;
#[doc = "Field `SEC1` writer - 1-Second Value for the ones place of seconds"]
pub type Sec1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEC10` reader - 10-Seconds Value for the tens place of seconds"]
pub type Sec10R = crate::FieldReader;
#[doc = "Field `SEC10` writer - 10-Seconds Value for the tens place of seconds"]
pub type Sec10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Compare enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb {
    #[doc = "0: The register value is not compared with the RSECCNT counter value."]
    _0 = 0,
    #[doc = "1: The register value is compared with the RSECCNT counter value."]
    _1 = 1,
}
impl From<Enb> for bool {
    #[inline(always)]
    fn from(variant: Enb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENB` reader - Compare enable"]
pub type EnbR = crate::BitReader<Enb>;
impl EnbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enb {
        match self.bits {
            false => Enb::_0,
            true => Enb::_1,
        }
    }
    #[doc = "The register value is not compared with the RSECCNT counter value."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enb::_0
    }
    #[doc = "The register value is compared with the RSECCNT counter value."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Enb::_1
    }
}
#[doc = "Field `ENB` writer - Compare enable"]
pub type EnbW<'a, REG> = crate::BitWriter<'a, REG, Enb>;
impl<'a, REG> EnbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The register value is not compared with the RSECCNT counter value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_0)
    }
    #[doc = "The register value is compared with the RSECCNT counter value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - 1-Second Value for the ones place of seconds"]
    #[inline(always)]
    pub fn sec1(&self) -> Sec1R {
        Sec1R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Seconds Value for the tens place of seconds"]
    #[inline(always)]
    pub fn sec10(&self) -> Sec10R {
        Sec10R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSECAR")
            .field("enb", &self.enb())
            .field("sec10", &self.sec10())
            .field("sec1", &self.sec1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Second Value for the ones place of seconds"]
    #[inline(always)]
    pub fn sec1(&mut self) -> Sec1W<RsecarSpec> {
        Sec1W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 10-Seconds Value for the tens place of seconds"]
    #[inline(always)]
    pub fn sec10(&mut self) -> Sec10W<RsecarSpec> {
        Sec10W::new(self, 4)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&mut self) -> EnbW<RsecarSpec> {
        EnbW::new(self, 7)
    }
}
#[doc = "Second Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsecar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsecar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsecarSpec;
impl crate::RegisterSpec for RsecarSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rsecar::R`](R) reader structure"]
impl crate::Readable for RsecarSpec {}
#[doc = "`write(|w| ..)` method takes [`rsecar::W`](W) writer structure"]
impl crate::Writable for RsecarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSECAR to value 0"]
impl crate::Resettable for RsecarSpec {}
