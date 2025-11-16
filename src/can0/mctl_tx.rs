#[doc = "Register `MCTL_TX[%s]` reader"]
pub type R = crate::R<MctlTxSpec>;
#[doc = "Register `MCTL_TX[%s]` writer"]
pub type W = crate::W<MctlTxSpec>;
#[doc = "Transmission Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sentdata {
    #[doc = "0: Transmission is not completed"]
    _0 = 0,
    #[doc = "1: Transmission is completed"]
    _1 = 1,
}
impl From<Sentdata> for bool {
    #[inline(always)]
    fn from(variant: Sentdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SENTDATA` reader - Transmission Complete Flag"]
pub type SentdataR = crate::BitReader<Sentdata>;
impl SentdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sentdata {
        match self.bits {
            false => Sentdata::_0,
            true => Sentdata::_1,
        }
    }
    #[doc = "Transmission is not completed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sentdata::_0
    }
    #[doc = "Transmission is completed"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sentdata::_1
    }
}
#[doc = "Field `SENTDATA` writer - Transmission Complete Flag"]
pub type SentdataW<'a, REG> = crate::BitWriter<'a, REG, Sentdata>;
impl<'a, REG> SentdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission is not completed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sentdata::_0)
    }
    #[doc = "Transmission is completed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sentdata::_1)
    }
}
#[doc = "Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trmactive {
    #[doc = "0: Transmission is pending or transmission is not requested"]
    _0 = 0,
    #[doc = "1: From acceptance of transmission request to completion of transmission, or error/arbitration-lost"]
    _1 = 1,
}
impl From<Trmactive> for bool {
    #[inline(always)]
    fn from(variant: Trmactive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRMACTIVE` reader - Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)"]
pub type TrmactiveR = crate::BitReader<Trmactive>;
impl TrmactiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trmactive {
        match self.bits {
            false => Trmactive::_0,
            true => Trmactive::_1,
        }
    }
    #[doc = "Transmission is pending or transmission is not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trmactive::_0
    }
    #[doc = "From acceptance of transmission request to completion of transmission, or error/arbitration-lost"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trmactive::_1
    }
}
#[doc = "Transmission Abort Complete Flag (Transmit mailbox setting enabled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trmabt {
    #[doc = "0: Transmission has started, transmission abort failed because transmission is completed, or transmission abort is not requested"]
    _0 = 0,
    #[doc = "1: Transmission abort is completed"]
    _1 = 1,
}
impl From<Trmabt> for bool {
    #[inline(always)]
    fn from(variant: Trmabt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRMABT` reader - Transmission Abort Complete Flag (Transmit mailbox setting enabled)"]
pub type TrmabtR = crate::BitReader<Trmabt>;
impl TrmabtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trmabt {
        match self.bits {
            false => Trmabt::_0,
            true => Trmabt::_1,
        }
    }
    #[doc = "Transmission has started, transmission abort failed because transmission is completed, or transmission abort is not requested"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trmabt::_0
    }
    #[doc = "Transmission abort is completed"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trmabt::_1
    }
}
#[doc = "Field `TRMABT` writer - Transmission Abort Complete Flag (Transmit mailbox setting enabled)"]
pub type TrmabtW<'a, REG> = crate::BitWriter<'a, REG, Trmabt>;
impl<'a, REG> TrmabtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmission has started, transmission abort failed because transmission is completed, or transmission abort is not requested"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Trmabt::_0)
    }
    #[doc = "Transmission abort is completed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Trmabt::_1)
    }
}
#[doc = "One-Shot Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oneshot {
    #[doc = "0: One-shot reception or one-shot transmission disabled"]
    _0 = 0,
    #[doc = "1: One-shot reception or one-shot transmission enabled"]
    _1 = 1,
}
impl From<Oneshot> for bool {
    #[inline(always)]
    fn from(variant: Oneshot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONESHOT` reader - One-Shot Enable"]
pub type OneshotR = crate::BitReader<Oneshot>;
impl OneshotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oneshot {
        match self.bits {
            false => Oneshot::_0,
            true => Oneshot::_1,
        }
    }
    #[doc = "One-shot reception or one-shot transmission disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oneshot::_0
    }
    #[doc = "One-shot reception or one-shot transmission enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oneshot::_1
    }
}
#[doc = "Field `ONESHOT` writer - One-Shot Enable"]
pub type OneshotW<'a, REG> = crate::BitWriter<'a, REG, Oneshot>;
impl<'a, REG> OneshotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One-shot reception or one-shot transmission disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshot::_0)
    }
    #[doc = "One-shot reception or one-shot transmission enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshot::_1)
    }
}
#[doc = "Receive Mailbox Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recreq {
    #[doc = "0: Not configured for reception"]
    _0 = 0,
    #[doc = "1: Configured for reception"]
    _1 = 1,
}
impl From<Recreq> for bool {
    #[inline(always)]
    fn from(variant: Recreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECREQ` reader - Receive Mailbox Request"]
pub type RecreqR = crate::BitReader<Recreq>;
impl RecreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Recreq {
        match self.bits {
            false => Recreq::_0,
            true => Recreq::_1,
        }
    }
    #[doc = "Not configured for reception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Recreq::_0
    }
    #[doc = "Configured for reception"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Recreq::_1
    }
}
#[doc = "Field `RECREQ` writer - Receive Mailbox Request"]
pub type RecreqW<'a, REG> = crate::BitWriter<'a, REG, Recreq>;
impl<'a, REG> RecreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not configured for reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Recreq::_0)
    }
    #[doc = "Configured for reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Recreq::_1)
    }
}
#[doc = "Transmit Mailbox Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trmreq {
    #[doc = "0: Not configured for transmission"]
    _0 = 0,
    #[doc = "1: Configured for transmission"]
    _1 = 1,
}
impl From<Trmreq> for bool {
    #[inline(always)]
    fn from(variant: Trmreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRMREQ` reader - Transmit Mailbox Request"]
pub type TrmreqR = crate::BitReader<Trmreq>;
impl TrmreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trmreq {
        match self.bits {
            false => Trmreq::_0,
            true => Trmreq::_1,
        }
    }
    #[doc = "Not configured for transmission"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trmreq::_0
    }
    #[doc = "Configured for transmission"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trmreq::_1
    }
}
#[doc = "Field `TRMREQ` writer - Transmit Mailbox Request"]
pub type TrmreqW<'a, REG> = crate::BitWriter<'a, REG, Trmreq>;
impl<'a, REG> TrmreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not configured for transmission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Trmreq::_0)
    }
    #[doc = "Configured for transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Trmreq::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Transmission Complete Flag"]
    #[inline(always)]
    pub fn sentdata(&self) -> SentdataR {
        SentdataR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)"]
    #[inline(always)]
    pub fn trmactive(&self) -> TrmactiveR {
        TrmactiveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Abort Complete Flag (Transmit mailbox setting enabled)"]
    #[inline(always)]
    pub fn trmabt(&self) -> TrmabtR {
        TrmabtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - One-Shot Enable"]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Mailbox Request"]
    #[inline(always)]
    pub fn recreq(&self) -> RecreqR {
        RecreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Mailbox Request"]
    #[inline(always)]
    pub fn trmreq(&self) -> TrmreqR {
        TrmreqR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCTL_TX")
            .field("trmreq", &self.trmreq())
            .field("recreq", &self.recreq())
            .field("oneshot", &self.oneshot())
            .field("trmabt", &self.trmabt())
            .field("trmactive", &self.trmactive())
            .field("sentdata", &self.sentdata())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmission Complete Flag"]
    #[inline(always)]
    pub fn sentdata(&mut self) -> SentdataW<MctlTxSpec> {
        SentdataW::new(self, 0)
    }
    #[doc = "Bit 2 - Transmission Abort Complete Flag (Transmit mailbox setting enabled)"]
    #[inline(always)]
    pub fn trmabt(&mut self) -> TrmabtW<MctlTxSpec> {
        TrmabtW::new(self, 2)
    }
    #[doc = "Bit 4 - One-Shot Enable"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> OneshotW<MctlTxSpec> {
        OneshotW::new(self, 4)
    }
    #[doc = "Bit 6 - Receive Mailbox Request"]
    #[inline(always)]
    pub fn recreq(&mut self) -> RecreqW<MctlTxSpec> {
        RecreqW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Mailbox Request"]
    #[inline(always)]
    pub fn trmreq(&mut self) -> TrmreqW<MctlTxSpec> {
        TrmreqW::new(self, 7)
    }
}
#[doc = "Message Control Register for Transmit\n\nYou can [`read`](crate::Reg::read) this register and get [`mctl_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctl_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MctlTxSpec;
impl crate::RegisterSpec for MctlTxSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mctl_tx::R`](R) reader structure"]
impl crate::Readable for MctlTxSpec {}
#[doc = "`write(|w| ..)` method takes [`mctl_tx::W`](W) writer structure"]
impl crate::Writable for MctlTxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCTL_TX[%s] to value 0"]
impl crate::Resettable for MctlTxSpec {}
