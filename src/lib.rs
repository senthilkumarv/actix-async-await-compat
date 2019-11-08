use futures::Future;
use futures3::{Future as Future3, FutureExt, TryFutureExt};
use actix_web::{error::Error};

macro_rules! define_compat {
($name:ident($($arg:ident),+: $($ty:ident),+)) => (
#[inline]
pub fn $name<F, T, Ok, Error, $($ty,)*>(f: F) -> impl Fn($($ty,)*) -> Box<dyn Future<Item = Ok, Error = Error>> + Clone + 'static
where
      Ok: 'static,
      Error: 'static,
      F: Fn($($ty,)*) -> T + Clone + 'static,
      T: Future3<Output = Result<Ok, Error>> + 'static, {
      move |$($arg,)*| {
      let fut1 = f($($arg,)*).boxed_local().compat();
    Box::new(fut1)
      }
      }
);
}

define_compat!(compat(arg1: Arg1));
define_compat!(compat2(arg1, arg2: Arg1, Arg2));
define_compat!(compat3(arg1, arg2, arg3: Arg1, Arg2, Arg3));
define_compat!(compat4(arg1, arg2, arg3, arg4: Arg1, Arg2, Arg3, Arg4));
define_compat!(compat5(arg1, arg2, arg3, arg4, arg5: Arg1, Arg2, Arg3, Arg4, Arg5));
define_compat!(compat6(arg1, arg2, arg3, arg4, arg5, arg6: Arg1, Arg2, Arg3, Arg4, Arg5, Arg6));
