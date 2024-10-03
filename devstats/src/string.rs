fn string_to_num<T: FromStr>(s: &str) -> Result<T, T::Err> {
    s.parse::<T>()
}

fn string_to_num_must<T: FromStr>(s: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    // let r = string_to_num::<T>(s);
    let r: Result<T, T::Err> = s.parse::<T>();
    match r {
        Ok(s) => s,
        Err(e) => {
            fatal_no_log::<T, String>(&Err(format!(
                "cannot convert {:?} to integer, error: '{:?}'",
                s, e
            )));
            // Never gets there, but rust needs this
            string_to_num::<T>("0").ok().unwrap()
        }
    }
}