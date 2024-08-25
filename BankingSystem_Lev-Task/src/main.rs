
trait _Account {

    fn deposit(&mut self, amount: f64) -> Result<(), String>;

    fn withdraw(&mut self, amount: f64) -> Result<(), String>;

    fn balance(&mut self) -> f64;
}

struct _BankAccount {
    _account_number: i32,
    _holder_name: String,
    _balance: f64
}

impl _Account for _BankAccount{

    fn deposit(&mut self, amount: f64) -> Result<(), String> {

        if amount <= 0.0 {
            return Err("Deposit amount must be positive".to_string())
        }
        else {
            self._balance += amount;
            return Ok(())
        }
        
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {

        if amount <= 0.0 {

            return Err("Withdrawal amount must be positive".to_string())
        }
        else if amount > self._balance {

            return Err("Insufficient balance".to_string())
        }
        else{
            self._balance -= amount;
            return Ok(())
        }
        
    }

    fn balance(&mut self) -> f64{

        self._balance
    }
}

fn main() {
    
    
    let mut _b1 = _BankAccount { _account_number: 1, _holder_name: String::from("Juan"), _balance: 0.00};
    let mut _b2 = _BankAccount { _account_number: 2, _holder_name: String::from("Javier"), _balance: 10.00};

    match _b1.deposit(10000.0){

        Ok(())=> println!("OK"),
        Err(e) => println!("Deposit Failed {}", e),
    }
    
    match _b2.withdraw(100.0){

        Ok(()) => println!("Withdraw Ok"),
        Err(e) => println!("Withdraw Failed {}", e),
    }
    
    println!("{}",_b1.balance());

    println!("{}",_b2.balance());
}
