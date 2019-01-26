#[macro_export]
macro_rules! exec_callback {
	($amx:ident, $name:tt; $($args:tt)*) => {
		{
			$amx.find_public(&$name)
				.and_then(|index| exec!($amx, index; $($args)*))
		}
	};
}

macro_rules! cast_amx {
    ($amx:ident) => {
        AMX::new(*$amx as *mut _)
    };
}
