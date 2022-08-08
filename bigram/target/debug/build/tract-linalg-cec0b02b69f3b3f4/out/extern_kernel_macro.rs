
    macro_rules! extern_kernel {
        (fn $name: ident($($par_name:ident : $par_type: ty ),*) -> $rv: ty) => {
            paste! {
                extern "C" { fn [<$name _ 0_15_8>]($(par_name: $par_type),*) -> $rv; }
                use [<$name _ 0_15_8>] as $name;
            }
        }
    }