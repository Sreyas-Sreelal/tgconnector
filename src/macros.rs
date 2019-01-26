#[macro_export]
macro_rules! exec_callback {
	($amx:ident, $name:ident; $($args:tt)*) => {
		{
			$amx.find_public(&$name)
				.and_then(|index| exec!($amx, index; $($args)*))
		}
	};
	($amx:ident, $name:expr; $($args:tt)*) => {
        {
            $amx.find_public($name)
                .and_then(|index| exec!($amx, index; $($args)*))
        }
    };
}

macro_rules! cast_amx {
    ($amx:ident) => {
        AMX::new(*$amx as *mut _)
    };
}
