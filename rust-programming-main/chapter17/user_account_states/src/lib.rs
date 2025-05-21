pub struct UserAccount {
    state: Option<Box<dyn UserState>>
}

impl UserAccount {
    pub fn new() -> UserAccount {
        UserAccount {
            state: Some(Box::new(Active{}))
        }
    }
    pub fn login(&mut self){
        if let Some(s) = self.state.take() {
            self.state = Some(s.login())
        }
    }
    pub fn suspend(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.suspend())
        }
    }
    pub fn ban(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.ban())
        }
    }
}
trait UserState {
    fn login(self: Box<Self>) -> Box<dyn UserState>;
    fn suspend(self: Box<Self>) -> Box<dyn UserState>;
    fn ban(self: Box<Self>) -> Box<dyn UserState>;
}

struct Suspended{}
impl UserState for Suspended {
    fn login(self: Box<Self>) -> Box<dyn UserState> {
        println!("Account is suspended. Cannot log in.");
        self
    }

    fn suspend(self: Box<Self>) -> Box<dyn UserState> {
        println!("Account is already suspended.");
        self
    }

    fn ban(self: Box<Self>) -> Box<dyn UserState> {
        Box::new(Banned{})
    }
}
struct Active{}
impl UserState for Active {
    fn login(self: Box<Self>) -> Box<dyn UserState> {
        println!("User logged in successfully.");
        self
    }

    fn suspend(self: Box<Self>) -> Box<dyn UserState> {
        Box::new(Suspended{})
    }

    fn ban(self: Box<Self>) -> Box<dyn UserState> {
        Box::new(Banned{})
    }
}
struct Banned{}
impl UserState for Banned {
    fn login(self: Box<Self>) -> Box<dyn UserState> {
        println!("Account is banned. Cannot log in.");
        self
    }

    fn suspend(self: Box<Self>) -> Box<dyn UserState> {
        Box::new(Suspended{})
    }

    fn ban(self: Box<Self>) -> Box<dyn UserState> {
        println!("Account is already banned.");
        self
    }
}