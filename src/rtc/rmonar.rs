#[doc = "Register `RMONAR` reader"]
pub type R = crate::R<RmonarSpec>;
#[doc = "Register `RMONAR` writer"]
pub type W = crate::W<RmonarSpec>;
#[doc = "Field `MON1` reader - 1 Month Value for the ones place of months"]
pub type Mon1R = crate::FieldReader;
#[doc = "Field `MON1` writer - 1 Month Value for the ones place of months"]
pub type Mon1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MON10` reader - 10 Months Value for the tens place of months"]
pub type Mon10R = crate::BitReader;
#[doc = "Field `MON10` writer - 10 Months Value for the tens place of months"]
pub type Mon10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Compare enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enb {
    #[doc = "0: The register value is not compared with the RMONCNT counter value."]
    _0 = 0,
    #[doc = "1: The register value is compared with the RMONCNT counter value."]
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
    #[doc = "The register value is not compared with the RMONCNT counter value."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Enb::_0
    }
    #[doc = "The register value is compared with the RMONCNT counter value."]
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
    #[doc = "The register value is not compared with the RMONCNT counter value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_0)
    }
    #[doc = "The register value is compared with the RMONCNT counter value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Enb::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - 1 Month Value for the ones place of months"]
    #[inline(always)]
    pub fn mon1(&self) -> Mon1R {
        Mon1R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - 10 Months Value for the tens place of months"]
    #[inline(always)]
    pub fn mon10(&self) -> Mon10R {
        Mon10R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMONAR")
            .field("enb", &self.enb())
            .field("mon10", &self.mon10())
            .field("mon1", &self.mon1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - 1 Month Value for the ones place of months"]
    #[inline(always)]
    pub fn mon1(&mut self) -> Mon1W<RmonarSpec> {
        Mon1W::new(self, 0)
    }
    #[doc = "Bit 4 - 10 Months Value for the tens place of months"]
    #[inline(always)]
    pub fn mon10(&mut self) -> Mon10W<RmonarSpec> {
        Mon10W::new(self, 4)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&mut self) -> EnbW<RmonarSpec> {
        EnbW::new(self, 7)
    }
}
#[doc = "Month Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmonar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmonar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmonarSpec;
impl crate::RegisterSpec for RmonarSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rmonar::R`](R) reader structure"]
impl crate::Readable for RmonarSpec {}
#[doc = "`write(|w| ..)` method takes [`rmonar::W`](W) writer structure"]
impl crate::Writable for RmonarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RMONAR to value 0"]
impl crate::Resettable for RmonarSpec {}
