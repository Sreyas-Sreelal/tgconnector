macro_rules! execute {
    ($amx_list:ident,$name:tt,$botid:ident;$($args:tt)*) => {
        let mut executed: bool = false;
        for amx in $amx_list {
            if let Some(amx) = samp::amx::get(*amx) {
                let botid: usize = $botid;
                let _= exec_public!(amx,$name,botid,$($args)*);
                executed = true;
            }
        }
        if !executed {
            error!("**[TGConnector] Error executing callback {}",$name);
        }
    };
}

macro_rules! cache_get {
    ($cache_list:ident,$dest:ident,$size:ident) => {
        if $cache_list.front() != None {
            match encode_replace(&$cache_list.front().unwrap()) {
                Ok(encoded) => {
                    let mut dest = $dest.into_sized_buffer($size);
                    let _ = samp::cell::string::put_in_buffer(&mut dest, &encoded);
                    Ok(1)
                }
                Err(err) => {
                    error!(
                        "**[TGConnector] Failed encoding {:?} \n {:?}",
                        $cache_list.front().unwrap(),
                        err
                    );
                    Ok(0)
                }
            }
        } else {
            Ok(0)
        }
    };
}
