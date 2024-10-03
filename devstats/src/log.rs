pub fn fatal_no_log<T, E: std::fmt::Debug>(res: &Result<T, E>) {
    match res {
        Ok(_) => {}
        Err(e) => panic!("error({:?}): {:?}", std::any::type_name::<T>(), e),
    }
}

fn fatal_no_log_str(res: Result<(), String>) {
    match res {
        Ok(_) => {}
        Err(_) => fatal_no_log(&res),
    }
}
