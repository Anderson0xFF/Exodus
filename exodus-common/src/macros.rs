
#[macro_export]
macro_rules! if_some {
    ($x:expr, $y:expr) => {
        if let Some(val) = $x {
            $y(val)
        }
    };
}

#[macro_export]
macro_rules! if_ok {
    ($x:expr, $y:expr) => {
        if let Ok(val) = $x {
            $y(val)
        }
    };
}

#[macro_export]
macro_rules! if_err {
    ($x:expr, $y:expr) => {
        if let Err(val) = $x {
            $y(val)
        }
    };
}

#[macro_export]
macro_rules! if_ {
    ($x:expr, $y:expr) => {
        if $x {
            $y
        }
    };
}