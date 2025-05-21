use user_account_states::UserAccount;


fn main() {
    let mut account = UserAccount::new();
    account.login();
    account.suspend();
    account.login();
    account.ban();
    account.login();
  
}
