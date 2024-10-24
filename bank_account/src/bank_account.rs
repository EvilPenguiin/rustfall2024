#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        let balance = if initial_balance >= 0.0 { initial_balance }
        else { 0.0 };
        BankAccount { balance }
        }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(1000.0);
        assert_eq!(account.balance(), 1000.0);

        let negative_balance_account = BankAccount::new(-1000.0);
        assert_eq!(negative_balance_account.balance(), 0.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(1000.0);
        assert_eq!(account.balance(), 1000.0);

        account.deposit(100.0);
        assert!((account.balance() - 1100.0).abs() < EPSILON);

        account.deposit(-100.0);
        assert!((account.balance() - 1100.0).abs() < EPSILON);

    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(1000.0);

        account.withdraw(100.0);
        assert!((account.balance() - 900.0).abs() < EPSILON);

        account.withdraw(-100.0);
        assert!((account.balance() - 900.0).abs() < EPSILON);

    }

    // Add more tests here

    #[test]
    fn test_other() {
        let mut account = BankAccount::new(100.0);

        account.withdraw(101.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }
}