mod tests {
    trait Binary {
        const ZERO: Self;
        const ONE: Self;
    }

    impl Binary for f64 {
        const ZERO: f64 = 0.0;
        const ONE: f64 = 1.0;
    }

    impl Binary for i32 {
        const ZERO: i32 = 0;
        const ONE: i32 = 1;
    }
}
