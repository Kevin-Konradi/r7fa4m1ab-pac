#[doc = "Register `CRCSAR` reader"]
pub type R = crate::R<CrcsarSpec>;
#[doc = "Register `CRCSAR` writer"]
pub type W = crate::W<CrcsarSpec>;
#[doc = "snoop address bit Set the I/O register address to snoop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Crcsa {
    #[doc = "3: SCI0.TDR"]
    _0x0003 = 3,
    #[doc = "5: SCI0.RDR"]
    _0x0005 = 5,
    #[doc = "35: SCI1.TDR"]
    _0x0023 = 35,
    #[doc = "37: SCI1.RDR"]
    _0x0025 = 37,
    #[doc = "67: SCI2.TDR"]
    _0x0043 = 67,
    #[doc = "69: SCI2.RDR"]
    _0x0045 = 69,
    #[doc = "99: SCI3.TDR"]
    _0x0063 = 99,
    #[doc = "101: SCI3.RDR"]
    _0x0065 = 101,
    #[doc = "131: SCI4.TDR"]
    _0x0083 = 131,
    #[doc = "133: SCI4.RDR"]
    _0x0085 = 133,
    #[doc = "291: SCI9.TDR"]
    _0x0123 = 291,
    #[doc = "293: SCI9.RDR"]
    _0x0125 = 293,
    #[doc = "0: Settings other than above are prohibited."]
    Others = 0,
}
impl From<Crcsa> for u16 {
    #[inline(always)]
    fn from(variant: Crcsa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crcsa {
    type Ux = u16;
}
impl crate::IsEnum for Crcsa {}
#[doc = "Field `CRCSA` reader - snoop address bit Set the I/O register address to snoop"]
pub type CrcsaR = crate::FieldReader<Crcsa>;
impl CrcsaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcsa {
        match self.bits {
            3 => Crcsa::_0x0003,
            5 => Crcsa::_0x0005,
            35 => Crcsa::_0x0023,
            37 => Crcsa::_0x0025,
            67 => Crcsa::_0x0043,
            69 => Crcsa::_0x0045,
            99 => Crcsa::_0x0063,
            101 => Crcsa::_0x0065,
            131 => Crcsa::_0x0083,
            133 => Crcsa::_0x0085,
            291 => Crcsa::_0x0123,
            293 => Crcsa::_0x0125,
            _ => Crcsa::Others,
        }
    }
    #[doc = "SCI0.TDR"]
    #[inline(always)]
    pub fn is_0x0003(&self) -> bool {
        *self == Crcsa::_0x0003
    }
    #[doc = "SCI0.RDR"]
    #[inline(always)]
    pub fn is_0x0005(&self) -> bool {
        *self == Crcsa::_0x0005
    }
    #[doc = "SCI1.TDR"]
    #[inline(always)]
    pub fn is_0x0023(&self) -> bool {
        *self == Crcsa::_0x0023
    }
    #[doc = "SCI1.RDR"]
    #[inline(always)]
    pub fn is_0x0025(&self) -> bool {
        *self == Crcsa::_0x0025
    }
    #[doc = "SCI2.TDR"]
    #[inline(always)]
    pub fn is_0x0043(&self) -> bool {
        *self == Crcsa::_0x0043
    }
    #[doc = "SCI2.RDR"]
    #[inline(always)]
    pub fn is_0x0045(&self) -> bool {
        *self == Crcsa::_0x0045
    }
    #[doc = "SCI3.TDR"]
    #[inline(always)]
    pub fn is_0x0063(&self) -> bool {
        *self == Crcsa::_0x0063
    }
    #[doc = "SCI3.RDR"]
    #[inline(always)]
    pub fn is_0x0065(&self) -> bool {
        *self == Crcsa::_0x0065
    }
    #[doc = "SCI4.TDR"]
    #[inline(always)]
    pub fn is_0x0083(&self) -> bool {
        *self == Crcsa::_0x0083
    }
    #[doc = "SCI4.RDR"]
    #[inline(always)]
    pub fn is_0x0085(&self) -> bool {
        *self == Crcsa::_0x0085
    }
    #[doc = "SCI9.TDR"]
    #[inline(always)]
    pub fn is_0x0123(&self) -> bool {
        *self == Crcsa::_0x0123
    }
    #[doc = "SCI9.RDR"]
    #[inline(always)]
    pub fn is_0x0125(&self) -> bool {
        *self == Crcsa::_0x0125
    }
    #[doc = "Settings other than above are prohibited."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Crcsa::Others)
    }
}
#[doc = "Field `CRCSA` writer - snoop address bit Set the I/O register address to snoop"]
pub type CrcsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, Crcsa, crate::Safe>;
impl<'a, REG> CrcsaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "SCI0.TDR"]
    #[inline(always)]
    pub fn _0x0003(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsa::_0x0003)
    }
    #[doc = "SCI0.RDR"]
    #[inline(always)]
    pub fn _0x0005(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsa::_0x0005)
    }
    #[doc = "SCI1.TDR"]
    #[inline(always)]
    pub fn _0x0023(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsa::_0x0023)
    }
    #[doc = "SCI1.RDR"]
    #[inline(always)]
    pub fn _0x0025(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsa::_0x0025)
    }
    #[doc = "SCI2.TDR"]
    #[inline(always)]
    pub fn _0x0043(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsa::_0x0043)
    }
    #[doc = "SCI2.RDR"]
    #[inline(always)]
    pub fn _0x0045(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsa::_0x0045)
    }
    #[doc = "SCI3.TDR"]
    #[inline(always)]
    pub fn _0x0063(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsa::_0x0063)
    }
    #[doc = "SCI3.RDR"]
    #[inline(always)]
    pub fn _0x0065(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsa::_0x0065)
    }
    #[doc = "SCI4.TDR"]
    #[inline(always)]
    pub fn _0x0083(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsa::_0x0083)
    }
    #[doc = "SCI4.RDR"]
    #[inline(always)]
    pub fn _0x0085(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsa::_0x0085)
    }
    #[doc = "SCI9.TDR"]
    #[inline(always)]
    pub fn _0x0123(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsa::_0x0123)
    }
    #[doc = "SCI9.RDR"]
    #[inline(always)]
    pub fn _0x0125(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsa::_0x0125)
    }
    #[doc = "Settings other than above are prohibited."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Crcsa::Others)
    }
}
impl R {
    #[doc = "Bits 0:13 - snoop address bit Set the I/O register address to snoop"]
    #[inline(always)]
    pub fn crcsa(&self) -> CrcsaR {
        CrcsaR::new(self.bits & 0x3fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCSAR")
            .field("crcsa", &self.crcsa())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - snoop address bit Set the I/O register address to snoop"]
    #[inline(always)]
    pub fn crcsa(&mut self) -> CrcsaW<CrcsarSpec> {
        CrcsaW::new(self, 0)
    }
}
#[doc = "Snoop Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcsarSpec;
impl crate::RegisterSpec for CrcsarSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crcsar::R`](R) reader structure"]
impl crate::Readable for CrcsarSpec {}
#[doc = "`write(|w| ..)` method takes [`crcsar::W`](W) writer structure"]
impl crate::Writable for CrcsarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCSAR to value 0"]
impl crate::Resettable for CrcsarSpec {}
