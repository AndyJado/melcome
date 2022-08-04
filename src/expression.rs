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

impl Expression {
    
    pub fn from_itn(itn:Intention) -> Expression {
        Expression { intention: Some(Intention::new_from(itn)), distortion: vec![], expressed: vec![] }
    }
    
    pub fn express(&mut self,sth: ShowOnHeap) {
        self.distortion.push(None);
        self.expressed.push(sth);
    }
    
    pub fn biased(&mut self, deg: Angle) {
        let last = self.distortion.pop();
        match last {
            Some(cta) => {
                let new = cta.unwrap_or_default();
                self.distortion.push(Some(new + deg));
            },
            None => println!("nothing expressed!"),
        };
    }
    
    pub fn key(&self) -> Option<String> {
        let ref int = self.intention;        

        match int {
            Some(k) => Some(k.key().to_string()),
            None => None,
        }

    }
}

impl From<ShowOnHeap> for Expression {
    fn from(val: ShowOnHeap) -> Self {
        Expression { intention: None, distortion: vec![None,], expressed: vec![val,] }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let itn = match &self.intention {
            Some(itn) => itn.key().to_string(),
            None => "No intention yet".to_string(),
        };

        let curve = &self.distortion.iter()
                                    .flat_map(|cta| 
                                        cta
                                        .as_ref()
                                        // .unwrap()
                                        .unwrap_or_else(|| &Angle::Degrees(0.0))
                                        .in_degrees()
                                        .to_string()
                                        .chars()
                                        .chain("° against the intention, ".chars())
                                        .collect::<Vec<_>>())
                                    .collect::<String>();
        
        let latest = self.expressed.last().unwrap();
        write!(f,"the intention under was evolved from: {itn:?}.\n\nIntention was expressed a few times, each time {curve}.\n\nthe latest expression be {latest}" )
    }
}

#[derive(Default, Debug, PartialEq, PartialOrd)]
pub struct Intention {
    id:Uuid,
    origin: Option<Rc<Intention>>
}

impl Intention {
    
    pub fn new() -> Self {
        Intention { id: Uuid::new_v4(), origin: None }
    }
    
    pub fn new_from(self) -> Self {
        let new_id = Uuid::new_v4();
        
        Intention { 
            id: new_id,
            origin: Some(Rc::new(self)), 
        }  
    }
    
    pub fn another(&self) -> Self {
        Intention {
            id: Uuid::new_v4(), 
            origin: Some(Rc::clone(&self.origin.as_ref().unwrap())) 
        }
    }
    
    pub fn forget(&mut self) {
        self.origin = None
    }
    
    fn key(&self) -> Uuid {
        let ori = self.origin.as_ref();
        match ori {
            Some(itn) => itn.key(),
            None => self.id,
        }
    }
}

#[derive(Debug)]
pub struct Url(pub String);

impl Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}" )
    }
}

impl Observable for Url { }

pub trait Observable:Debug + Display { }





