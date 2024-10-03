fn env_is_set(var_name: &str) -> bool {
    match env::var(var_name) {
        Ok(_) => true,
        _ => false,
    }
}

fn env_is_empty(var_name: &str) -> bool {
    match env::var(var_name) {
        Ok(val) => val.trim() == "",
        _ => true,
    }
}

fn env_or_default(var_name: &str, default_value: String) -> String {
    match env::var(var_name) {
        Ok(val) if val.trim() != "" => val,
        Ok(val) if val.trim() == "" => default_value,
        _ => default_value,
    }
}

fn env_number<T: FromStr>(var_name: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    match env::var(var_name) {
        Ok(val) => string_to_num_must::<T>(&val),
        _ => {
            fatal_no_log::<T, String>(&Err(format!(
                "cannot convert env variable {:?} ({:?} no value) to string",
                var_name,
                std::any::type_name::<T>(),
            )));
            // Never gets there, but rust needs this
            string_to_num::<T>("0").ok().unwrap()
        }
    }
}
