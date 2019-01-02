use super::Message;
use actor::Actor;
use address::Addr;
use context::Context;
use handler::Handler;

use interact::Access;
use interact::{Deser, ImmutAccess, MutAccess, Reflect, ReflectIndirect, ReflectMut};

struct MsgAccess {
    callback: Box<FnMut(&dyn Access) + Send>,
}
struct MsgAccessMut {
    callback: Box<FnMut(&mut dyn Access) + Send>,
}

impl Message for MsgAccess {
    type Result = ();
}
impl Message for MsgAccessMut {
    type Result = ();
}

impl<A> Handler<MsgAccess> for A
where
    A: Access + Actor,
{
    type Result = ();

    fn handle(&mut self, mut msg: MsgAccess, _: &mut A::Context) -> Self::Result {
        (msg.callback)(self);
    }
}

impl<A> Handler<MsgAccessMut> for A
where
    A: Access + Actor,
{
    type Result = ();

    fn handle(&mut self, mut msg: MsgAccessMut, _: &mut A::Context) -> Self::Result {
        (msg.callback)(self);
    }
}

impl<A> ReflectIndirect for Addr<A>
where
    A: Access + Actor<Context = Context<A>>,
{
    fn indirect(&self, callback: Box<FnMut(&dyn Access) + Send>) {
        self.do_send(MsgAccess { callback });
    }
    fn indirect_mut(&mut self, callback: Box<FnMut(&mut dyn Access) + Send>) {
        self.do_send(MsgAccessMut { callback });
    }
}

impl<A> Access for Addr<A>
where
    A: Access + Actor<Context = Context<A>>,
{
    fn immut_access(&self) -> ImmutAccess {
        ImmutAccess::no_funcs(Reflect::Indirect(self))
    }

    fn mut_access(&mut self) -> MutAccess {
        MutAccess::no_funcs(ReflectMut::Indirect(self))
    }
}

impl<A> Deser for Addr<A> where A: Actor<Context = Context<A>> {}
