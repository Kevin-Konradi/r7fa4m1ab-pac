#[doc = "Register `MSMR` reader"]
pub type R = crate::R<MsmrSpec>;
#[doc = "Register `MSMR` writer"]
pub type W = crate::W<MsmrSpec>;
#[doc = "Mailbox Search Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mbsm {
    #[doc = "0: Receive mailbox search mode"]
    _00 = 0,
    #[doc = "1: Transmit mailbox search mode"]
    _01 = 1,
    #[doc = "2: Message lost search mode"]
    _10 = 2,
    #[doc = "3: Channel search mode"]
    _11 = 3,
}
impl From<Mbsm> for u8 {
    #[inline(always)]
    fn from(variant: Mbsm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mbsm {
    type Ux = u8;
}
impl crate::IsEnum for Mbsm {}
#[doc = "Field `MBSM` reader - Mailbox Search Mode Select"]
pub type MbsmR = crate::FieldReader<Mbsm>;
impl MbsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbsm {
        match self.bits {
            0 => Mbsm::_00,
            1 => Mbsm::_01,
            2 => Mbsm::_10,
            3 => Mbsm::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Receive mailbox search mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Mbsm::_00
    }
    #[doc = "Transmit mailbox search mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Mbsm::_01
    }
    #[doc = "Message lost search mode"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Mbsm::_10
    }
    #[doc = "Channel search mode"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Mbsm::_11
    }
}
#[doc = "Field `MBSM` writer - Mailbox Search Mode Select"]
pub type MbsmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mbsm, crate::Safe>;
impl<'a, REG> MbsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive mailbox search mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Mbsm::_00)
    }
    #[doc = "Transmit mailbox search mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Mbsm::_01)
    }
    #[doc = "Message lost search mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Mbsm::_10)
    }
    #[doc = "Channel search mode"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Mbsm::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Mailbox Search Mode Select"]
    #[inline(always)]
    pub fn mbsm(&self) -> MbsmR {
        MbsmR::new(self.bits & 3)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSMR").field("mbsm", &self.mbsm()).finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Mailbox Search Mode Select"]
    #[inline(always)]
    pub fn mbsm(&mut self) -> MbsmW<MsmrSpec> {
        MbsmW::new(self, 0)
    }
}
#[doc = "Mailbox Search Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsmrSpec;
impl crate::RegisterSpec for MsmrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`msmr::R`](R) reader structure"]
impl crate::Readable for MsmrSpec {}
#[doc = "`write(|w| ..)` method takes [`msmr::W`](W) writer structure"]
impl crate::Writable for MsmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSMR to value 0"]
impl crate::Resettable for MsmrSpec {}
