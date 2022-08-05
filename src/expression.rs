use std::rc::Rc;
use std::fmt::{Debug, Display};
use uuid::Uuid;

use ang::Angle;

#[derive(Debug)]
pub struct Expression {
    intention:Intention,
    distortion: Vec<Option<Angle>>,
    expressed: Vec<Box<dyn Observable>>,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not finished yet:\n{self:?}")
    }
}

// stateless, the supertrait
pub trait Expressing:Display { 
    fn express(&mut self, obe: Box<dyn Observable>); 
    fn key(&self) -> Option<String>;
}

impl Expressing for Expression {
    // push a pair
    fn express(&mut self, obe: Box<dyn Observable>) {
        let obes = &mut self.expressed;
        self.distortion.push(None);
        obes.push(obe)
    }

    fn key(&self) -> Option<String> {
        let ref int = self.intention;        

        match int.key().as_ref() {
            Some(id) => Some(id.to_string()),
            None => None,
        }
    }

}

// digging the intention of the expressed 
pub trait Digging:Expressing where Self: Sized {
    fn recover(&self) -> &Expression;
    fn sniff(&mut self, obe: Box<dyn Observable>) -> Expression;
}

// express an intention
pub trait Crawling:Expressing where Self: Sized {
    fn key(&self) -> String;
    fn judge(&mut self, deg:Angle);
    fn recover(&self) -> &Expression;
}

impl Expression {
    pub fn from_obe(obe: Box<dyn Observable>) -> impl Digging {
        Expression { intention: Intention::null(), distortion: vec![None,], expressed: vec![obe,] }
    }

    pub fn from_itn(itn:Intention) -> impl Crawling {
        Expression { intention: Intention::new_from(itn), distortion: vec![], expressed: vec![] }
    }
}

impl Digging for Expression {

    fn recover(&self) -> &Expression { self }

    fn sniff(&mut self, obe: Box<dyn Observable>) -> Expression {
        // 2 obes shares a single inten
        let sniffed = Intention::new();
        // update self
        let itn = &mut self.intention;
        itn.originated_from(sniffed);

        let itn2:Intention = self.intention.another();
        println!("sniff self: {self:?}\n");
        Expression { intention: itn2, distortion: vec![None,], expressed: vec![obe,] }
    }

}

impl Crawling for Expression {
    fn key(&self) ->String {
        Expressing::key(self).unwrap()
    }

    fn recover(&self) -> &Expression { self }

    fn judge(&mut self, deg:Angle) {
        let ctas = &mut self.distortion;
        let oops = ctas.pop();
        match oops {
            Some(p) => println!("last measure on the expression against intention is {p:?}\n "),
            None => return,
        }
        ctas.push(Some(deg));
        println!("{deg:?}")
    }
}

// recurrsive type!! no fix size
#[derive(Default, Debug, PartialEq, PartialOrd, Clone)]
pub struct Intention {
    id:Option<Uuid>,
    origin: Option<Rc<Intention>>
}

impl Intention {

    pub fn null() -> Self {
        Intention { id: None, origin: None }
    }

    pub fn new() -> Self {
        Intention { id: Some(Uuid::new_v4()), origin: None }
    }

    pub fn originated_from(&mut self,itn: Self) {
        let orin = &mut self.origin;
        match orin {
            Some(c) => panic!(),
            None => {
                *orin = Some(Rc::new(itn))
            },
        }
        println!("originated_from: {self:?}")
    }
    
    pub fn new_from(self) -> Intention {
        Intention { 
            id: None,
            origin: Some(Rc::new(self)), 
        }  
    }
    
    pub fn another(&self) -> Self {
        let orin = &self.origin;
        Intention {
            id: None, 
            origin: match orin {
                Some(itn) => {
                    Some(Rc::clone(itn))
                },
                None => None,
            }
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