#[cfg(test)]
mod tests {
    use std::sync::{Mutex,Arc};

    struct Account {
        balance: f64,
    }

    impl Account {
        fn deposit(&mut self, amt: f64) -> bool {
            self.balance += amt;
            true
        }
        fn withdraw(&mut self, amt: f64) -> bool {
            if self.balance >= amt {
                self.balance -= amt;
                true
            } else {
                false
            }
        }
    }

    #[test]
    fn it_works() {
        let mut account = Account { balance: 100.0};
        let arc1 = Arc::new(Mutex::new(&mut account));
        let arc2 = arc1.clone();

        std::thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..1000 {
                    arc1.lock().unwrap().deposit(100.);
                }
            });
            s.spawn(|| {
                for _ in 0..1000 {
                    arc2.lock().unwrap().withdraw(100.);
                }
            });
        });

        assert_eq!(100.0, account.balance);
    }
}
