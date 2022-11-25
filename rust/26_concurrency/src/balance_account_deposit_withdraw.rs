#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    struct Account {
        balance: f64,
    }

    impl Account {
        fn deposit(&mut self, amt: f64) -> bool {
            self.balance += amt;
            true
        }
        fn withdraw(&mut self, amt: f64) -> bool {
            // accept negative account balance
            self.balance -= amt;
            true
        }
    }

    #[test]
    fn test_mutex_account() {
        let account = Account { balance: 100.0 };
        let l_account = Mutex::new(account);

        std::thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..1000 {
                    l_account.lock().unwrap().deposit(100.0);
                }
            });
            s.spawn(|| {
                for _ in 0..1000 {
                    l_account.lock().unwrap().withdraw(100.0);
                }
            });
        });

        assert_eq!(100.0, l_account.lock().unwrap().balance);
    }

    /* Test case */
    struct AccountSync {
        balance: Mutex<f64>,
    }

    impl AccountSync {
        fn deposit(&self, amt: f64) -> bool {
            let mut b = self.balance.lock().unwrap();
            *b += amt;
            true
        }
        fn withdraw(&self, amt: f64) -> bool {
            // accept negative account balance
            let mut b = self.balance.lock().unwrap();
            *b -= amt;
            true
        }
    }

    #[test]
    fn test_account_mutex_balance() {
        let account = AccountSync { balance: Mutex::new(100.0) };
        let ref_account = &account;

        std::thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..1000 {
                    ref_account.deposit(100.0);
                }
            });
            s.spawn(|| {
                for _ in 0..1000 {
                    ref_account.withdraw(100.0);
                }
            });
        });

        assert_eq!(100.0, account.balance.lock().unwrap().to_owned());
    }
}
