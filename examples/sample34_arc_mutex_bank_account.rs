#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

#![allow(unused)]
fn main1() {
    struct BankAccount {
        balance: i64,
    }

    impl BankAccount {
        fn new() -> Self {
            BankAccount { balance: 0 }
        }

        fn balance(&self) -> i64 {
            if self.balance < 0 {
                panic!("** Oh no, gnoe overdrawn: {}", self.balance);
            }
            self.balance
        }

        fn deposit(&mut self, amount: i64) {
            self.balance += amount;
        }

        fn withdraw(&mut self, amount: i64) -> bool {
            if self.balance < amount {
                return false;
            }
            self.balance -= amount;
            true
        }
    }

    fn pay_in(account: &mut BankAccount) {
        loop {
            if account.balance() < 200 {
                println!("[A] Running low, deposit 400");
                account.deposit(400);
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }

    fn take_out(account: &mut BankAccount) {
        loop {
            if account.withdraw(100) {
                println!("[B] Withdrew 100, balance is now {}", account.balance());
            } else {
                println!("[B] Failed to withdraw 100");
            }
        }
    }

    let mut account = BankAccount::new();

    // ✖ 可変参照が重複しているため、コンパイラはエラーを出力します。
    // let _taker = std::thread::spawn(move || {
    //    take_out(&mut account);
    // });
    // let _payer = std::thread::spawn(move || {
    //     pay_in(&mut account);
    // });
}

#[allow(unused)]
fn main2() {
    struct BankAccount {
        balance: std::sync::Mutex<i64>,
    }

    impl BankAccount {
        fn new() -> Self {
            BankAccount {
                balance: std::sync::Mutex::new(0),
            }
        }

        fn balance(&self) -> i64 {
            let balance = *self.balance.lock().unwrap();
            if balance < 0 {
                panic!("** Oh no, gnoe overdrawn: {}", balance);
            }
            balance
        }

        fn deposit(&self, amount: i64) {
            *self.balance.lock().unwrap() += amount;
        }

        fn withdraw(&self, amount: i64) -> bool {
            let mut balance = self.balance.lock().unwrap();
            if *balance < amount {
                return false;
            }
            *balance -= amount;
            true
        }
    }

    fn pay_in(account: &BankAccount) {
        loop {
            if account.balance() < 200 {
                println!("[A] Running low, deposit 400");
                account.deposit(400);
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }

    fn take_out(account: &BankAccount) {
        loop {
            if account.withdraw(100) {
                println!("[B] Withdrew 100, balance is now {}", account.balance());
            } else {
                println!("[B] Failed to withdraw 100");
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }

    // let account = BankAccount::new();
    // ✖ accountはブロックの末尾でdropされるが、これらのスレッドは実行し続けるかもしれない
    // let _taker = std::thread::spawn(move || {
    //     take_out(&account);
    // });
    // let _payer = std::thread::spawn(move || {
    //     pay_in(&account);
    // });

    let account = std::sync::Arc::new(BankAccount::new());
    let _taker = {
        let account = account.clone();
        std::thread::spawn(move || {
            take_out(&account);
        })
    };
    let _payer = {
        let account = account.clone();
        std::thread::spawn(move || {
            pay_in(&account);
        })
    };

    // スレッドが終了するのを待つ
    _taker.join().unwrap();
    _payer.join().unwrap();
    println!("Final balance: {}", account.balance());
}

fn main() {
    // main1();
    main2();
}
