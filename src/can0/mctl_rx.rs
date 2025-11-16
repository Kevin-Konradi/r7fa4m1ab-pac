#[doc = "Register `MCTL_RX[%s]` reader"]
pub type R = crate::R<MctlRxSpec>;
#[doc = "Register `MCTL_RX[%s]` writer"]
pub type W = crate::W<MctlRxSpec>;
#[doc = "Reception Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Newdata {
    #[doc = "0: No data has been received or 0 is written to the NEWDATA bit"]
    _0 = 0,
    #[doc = "1: A new message is being stored or has been stored to the mailbox"]
    _1 = 1,
}
impl From<Newdata> for bool {
    #[inline(always)]
    fn from(variant: Newdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEWDATA` reader - Reception Complete Flag"]
pub type NewdataR = crate::BitReader<Newdata>;
impl NewdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Newdata {
        match self.bits {
            false => Newdata::_0,
            true => Newdata::_1,
        }
    }
    #[doc = "No data has been received or 0 is written to the NEWDATA bit"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Newdata::_0
    }
    #[doc = "A new message is being stored or has been stored to the mailbox"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Newdata::_1
    }
}
#[doc = "Field `NEWDATA` writer - Reception Complete Flag"]
pub type NewdataW<'a, REG> = crate::BitWriter<'a, REG, Newdata>;
impl<'a, REG> NewdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No data has been received or 0 is written to the NEWDATA bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Newdata::_0)
    }
    #[doc = "A new message is being stored or has been stored to the mailbox"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Newdata::_1)
    }
}
#[doc = "Reception-in-Progress Status Flag (Receive mailbox setting enabled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invaldata {
    #[doc = "0: Message valid"]
    _0 = 0,
    #[doc = "1: Message being updated"]
    _1 = 1,
}
impl From<Invaldata> for bool {
    #[inline(always)]
    fn from(variant: Invaldata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVALDATA` reader - Reception-in-Progress Status Flag (Receive mailbox setting enabled)"]
pub type InvaldataR = crate::BitReader<Invaldata>;
impl InvaldataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Invaldata {
        match self.bits {
            false => Invaldata::_0,
            true => Invaldata::_1,
        }
    }
    #[doc = "Message valid"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Invaldata::_0
    }
    #[doc = "Message being updated"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Invaldata::_1
    }
}
#[doc = "Message Lost Flag (Receive mailbox setting enabled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msglost {
    #[doc = "0: Message is not overwritten or overrun"]
    _0 = 0,
    #[doc = "1: Message is overwritten or overrun"]
    _1 = 1,
}
impl From<Msglost> for bool {
    #[inline(always)]
    fn from(variant: Msglost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSGLOST` reader - Message Lost Flag (Receive mailbox setting enabled)"]
pub type MsglostR = crate::BitReader<Msglost>;
impl MsglostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msglost {
        match self.bits {
            false => Msglost::_0,
            true => Msglost::_1,
        }
    }
    #[doc = "Message is not overwritten or overrun"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Msglost::_0
    }
    #[doc = "Message is overwritten or overrun"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Msglost::_1
    }
}
#[doc = "Field `MSGLOST` writer - Message Lost Flag (Receive mailbox setting enabled)"]
pub type MsglostW<'a, REG> = crate::BitWriter<'a, REG, Msglost>;
impl<'a, REG> MsglostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Message is not overwritten or overrun"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Msglost::_0)
    }
    #[doc = "Message is overwritten or overrun"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Msglost::_1)
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
    #[doc = "Bit 0 - Reception Complete Flag"]
    #[inline(always)]
    pub fn newdata(&self) -> NewdataR {
        NewdataR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reception-in-Progress Status Flag (Receive mailbox setting enabled)"]
    #[inline(always)]
    pub fn invaldata(&self) -> InvaldataR {
        InvaldataR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Message Lost Flag (Receive mailbox setting enabled)"]
    #[inline(always)]
    pub fn msglost(&self) -> MsglostR {
        MsglostR::new(((self.bits >> 2) & 1) != 0)
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
        f.debug_struct("MCTL_RX")
            .field("trmreq", &self.trmreq())
            .field("recreq", &self.recreq())
            .field("oneshot", &self.oneshot())
            .field("msglost", &self.msglost())
            .field("invaldata", &self.invaldata())
            .field("newdata", &self.newdata())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reception Complete Flag"]
    #[inline(always)]
    pub fn newdata(&mut self) -> NewdataW<MctlRxSpec> {
        NewdataW::new(self, 0)
    }
    #[doc = "Bit 2 - Message Lost Flag (Receive mailbox setting enabled)"]
    #[inline(always)]
    pub fn msglost(&mut self) -> MsglostW<MctlRxSpec> {
        MsglostW::new(self, 2)
    }
    #[doc = "Bit 4 - One-Shot Enable"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> OneshotW<MctlRxSpec> {
        OneshotW::new(self, 4)
    }
    #[doc = "Bit 6 - Receive Mailbox Request"]
    #[inline(always)]
    pub fn recreq(&mut self) -> RecreqW<MctlRxSpec> {
        RecreqW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit Mailbox Request"]
    #[inline(always)]
    pub fn trmreq(&mut self) -> TrmreqW<MctlRxSpec> {
        TrmreqW::new(self, 7)
    }
}
#[doc = "Message Control Register for Receive\n\nYou can [`read`](crate::Reg::read) this register and get [`mctl_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctl_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MctlRxSpec;
impl crate::RegisterSpec for MctlRxSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mctl_rx::R`](R) reader structure"]
impl crate::Readable for MctlRxSpec {}
#[doc = "`write(|w| ..)` method takes [`mctl_rx::W`](W) writer structure"]
impl crate::Writable for MctlRxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCTL_RX[%s] to value 0"]
impl crate::Resettable for MctlRxSpec {}
