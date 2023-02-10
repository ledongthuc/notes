#![allow(dead_code)]

use std::fs::File;
use std::io::Write;

struct MobileTopupConnection {
    bank: BankConnection,
    carrier: MobileCarrier,
}

impl MobileTopupConnection {
    fn new() ->  Self {
        MobileTopupConnection { 
            bank: BankConnection::new(),
            carrier: MobileCarrier::new(),
        }
    }
    fn connect(&mut self) {
        self.bank.connect();
        self.carrier.connect();
    }
}

impl Drop for MobileTopupConnection {
    fn drop(&mut self) {
        self.bank.send_close_signal();
        self.bank.status = ConnectionStatus::Closed;

        self.carrier.send_close_signal();
        self.carrier.status = ConnectionStatus::Closed;
    }
}

enum ConnectionStatus {
    Closed,
    Active,
}

struct BankConnection {
    status: ConnectionStatus,
}

impl BankConnection {
    fn new() ->  Self {
        BankConnection{
            status: ConnectionStatus::Closed
        }
    }
    fn connect(&mut self) {
        self.status = ConnectionStatus::Active
    }
    fn send_close_signal(&self) {
        let path = "bank.sock";
        let mut output = File::create(path).unwrap();
        write!(output, "Close bank conn").expect("can't write to bank conn");
    }
}

struct MobileCarrier {
    status: ConnectionStatus,
}

impl MobileCarrier {
    fn new() ->  Self {
        MobileCarrier { 
            status: ConnectionStatus::Closed
        }
    }
    fn connect(&mut self) {
        self.status = ConnectionStatus::Active
    }
    fn send_close_signal(&self) {
        let path = "carrier.sock";
        let mut output = File::create(path).unwrap();
        write!(output, "Close mobile carrier conn").expect("can't write to mobiel carrier conn");
    }
}


#[cfg(test)]
mod tests {
    use crate::MobileTopupConnection;

    #[test]
    fn test() {
        {
            let mut conn = MobileTopupConnection::new();
            conn.connect();
        }
    }
}
