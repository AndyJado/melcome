use ang::Angle;
use melcome::expression::{Intention, Expression,Crawling,Digging, Expressing, Url};

fn main() {
    let itn = Intention::new();
    let mut expr = Expression::from_itn(itn);
    let url = Box::new(Url("a thought experiment#0000".to_string()));
    expr.express(url);
    let cta = Angle::Degrees(24.0);
    expr.judge(cta);
    println!("{expr}\n");
    let url2 = Box::new(Url("Example#0000".to_string()));
    let mut expr2 = Expression::from_obe(url2);
    expr2.join(&expr);
    println!("{expr2}\n")
}