#[macro_export]

macro_rules! execute {
    ($amx_list:ident,$name:tt,$botid:ident;$($args:tt)*) => {
        let mut executed: bool = false;
        for amx in $amx_list {
            let amx = cast_amx!(amx);
            let botid: usize = *$botid;
            match exec_callback!(amx,$name;botid,$($args)*) {
                Ok(_) => {
                    executed = true;
                }
                Err(_err) => {
                    continue;
                }
            }
        }
        if !executed {
            log!("**[TGConnector] Error executing callback {}",$name);
        }
    };
}

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
