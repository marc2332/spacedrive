use crate::sys::get_volumes;

use super::{Router, RouterBuilder};

pub(crate) fn mount() -> RouterBuilder {
	<Router>::new().query("get", |_, _: ()| get_volumes().unwrap())
}
