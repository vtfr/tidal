#[macro_export]
macro_rules! all_tuples {
    ($name:ident) => {
        $name!(A1);
        $name!(A1, A2);
        $name!(A1, A2, A3);
        $name!(A1, A2, A3, A4);
        $name!(A1, A2, A3, A4, A5);
        $name!(A1, A2, A3, A4, A5, A6);
        $name!(A1, A2, A3, A4, A5, A6, A7);
        $name!(A1, A2, A3, A4, A5, A6, A7, A8);
        $name!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
        $name!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
        $name!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11);
        $name!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12);
        $name!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13);
        $name!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14);
        $name!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15);
    };
}
