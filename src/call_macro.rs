macro_rules! call_string {
        ($name:expr) => {
            $name.to_owned().into_string().unwrap_or_default()
        };
    }


macro_rules! call_vec {
        ($name:expr) => {
            $name.to_owned().into_iter().map(|x| x.into_string().unwrap_or_default()).collect::<Vec<String>>()
        };
    }

macro_rules! call_i64 {
        ($name:expr) => {
            $name.to_owned().as_i64().unwrap_or_default()
        };
    }