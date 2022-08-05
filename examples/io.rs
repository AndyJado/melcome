use ang::Angle;
use melcome::expression::{Intention, Expression, Url, Expressing, Digging, Crawling};

fn main() {

    let url1 = Box::new(Url("andyjado.com.1".to_string()));
    let url3 = Url("andyjado.com.3".to_string());
    let url4 = Box::new(Url("andyjado.com.4".to_string()));
    let cta = Angle::Degrees(30.9);

    let mut epr1 = Expression::from_obe(url1.clone());
    let itn1 = Intention::new();
    let mut expr2 = Expression::from_itn(itn1);
    expr2.judge(cta);
    expr2.express(url1);
    epr1.express(Box::new(url3));
    let mut expr3 = epr1.sniff(url4);

    println!("{epr1}\n\n{expr2}\n\n{expr3}\n");

}
