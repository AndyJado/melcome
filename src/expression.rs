use std::rc::Rc;
use std::fmt::{Debug, Display};
use uuid::Uuid;

use ang::Angle;

pub type ShowOnHeap = Box<dyn Observable>;

#[derive(Debug)]
pub struct Expression {
    intention:Option<Intention>,
    distortion: Vec<Option<Angle>>,
    expressed: Vec<ShowOnHeap>,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub trait Expressing:Display { 

    fn express(&mut self, obe: impl Observable + 'static); 
    fn key(&self) -> Option<String>;
}

impl Expressing for Expression {

    fn express(&mut self, obe: impl Observable + 'static) {
        let obes = &mut self.expressed;
        self.distortion.push(None);
        obes.push(Box::new(obe))
    }

    fn key(&self) -> Option<String> {
        let ref int = self.intention;        
        match int {
            Some(orin) => match orin.key() {
                Some(k) => Some(k.to_string()),
                None => None,
            },
            None => None,
        }
    }

}

pub trait Digging:Expressing where Self: Sized {
    fn judge(&mut self, deg:Angle) {}
    fn recover(&self) -> &Expression;
}

pub trait Crawling:Expressing where Self: Sized {
    fn key(&self) -> String;
    fn recover(&self) -> &Expression;
}

impl Expression {
    pub fn from_obe(obe:ShowOnHeap) -> impl Digging {
        Expression { intention: None, distortion: vec![None,], expressed: vec![obe,] }
    }
    
    pub fn from_itn(itn:Intention) -> impl Crawling {
        Expression { intention: Some(Intention::new_from(itn)), distortion: vec![], expressed: vec![] }
    }
    
}

impl Digging for Expression {
    fn judge(&mut self, deg:Angle) {
        let ctas = &mut self.distortion;
        let oops = ctas.pop();
        match oops {
            Some(p) => println!("current measure on the expression against intention is {p:?}\n "),
            None => panic!(),
        }
        ctas.push(Some(deg))
    }

    fn recover(&self) -> &Expression { self }

}

impl Crawling for Expression {
    fn key(&self) ->String {
        Expressing::key(self).unwrap()
    }

    fn recover(&self) -> &Expression { self }
}

#[derive(Default, Debug, PartialEq, PartialOrd, Clone)]
pub struct Intention {
    id:Option<Uuid>,
    origin: Option<Rc<Intention>>
}

impl Intention {

    pub fn new() -> Self {
        Intention { id: Some(Uuid::new_v4()), origin: None }
    }
    
    pub fn new_from(self) -> Intention {
        Intention { 
            id: None,
            origin: Some(Rc::new(self)), 
        }  
    }
    
    pub fn another(&self) -> Self {
        Intention {
            id: Some(Uuid::new_v4()), 
            origin: Some(Rc::clone(&self.origin.as_ref().unwrap())) 
        }
    }
    
    pub fn release(&mut self) {
        self.id = Some(Uuid::new_v4());
        self.origin = None
    }
    
    fn key(&self) -> Option<Uuid> {
        let orin = self.origin.as_ref();
        match orin {
            Some(itn) => itn.key(),
            None => self.id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Url(pub String);

impl Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}" )
    }
}

impl Observable for Url { }

pub trait Observable:Debug + Display { }






