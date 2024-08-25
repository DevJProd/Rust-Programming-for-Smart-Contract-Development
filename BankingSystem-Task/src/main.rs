
trait _Account {

    fn deposit(&mut self, amount: f64);

    fn withdraw(&mut self, amount: f64);

    fn balance(&mut self) -> f64;
}

struct _BankAccount {
    _account_number: i32,
    _holder_name: String,
    _balance: f64
}

impl _Account for _BankAccount{

    fn deposit(&mut self, amount: f64) {
        self._balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        self._balance -= amount;
    }

    fn balance(&mut self) -> f64{

        self._balance
    }
}

fn main() {
    
    
    let mut _b1 = _BankAccount { _account_number: 1, _holder_name: String::from("Juan"), _balance: 0.00};
    let mut _b2 = _BankAccount { _account_number: 2, _holder_name: String::from("Javier"), _balance: 1000.00};

    _b1.deposit(10000.0);
    
    _b2.withdraw(100.0);
    
    println!("{}",_b1.balance());

    println!("{}",_b2.balance());
}
