mod bank_account;
use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new(1000.0);
    println!("Your balance is: ${}", account.balance());

    account.deposit(100.0);
    println!("You have deposited $100. Your balance is now: ${}", account.balance());

    account.deposit(-100.0);
    println!("You have tried to deposit -$100. Your balance is: ${}", account.balance());

    account.withdraw(1000.0);
    println!("You have withdrawn $1000. Your balance is now: ${}", account.balance());

    account.withdraw(-1000.0);
    println!("You have tried to withdraw -$1000. Your balance is: ${}", account.balance());

    account.withdraw(200.0);
    println!("You have tried to withdraw $200. Your balance is ${}", account.balance());
}