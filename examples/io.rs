use ang::Angle;
use melcome::expression::{Intention, Expression, Url, Observable, ShowOnHeap};

fn main() {
    let url = Box::new(Url("andyjado.com".to_string()));
    let url1 = Box::new(Url("andyjado.com".to_string())) as ShowOnHeap;
    let url2 = Box::new(Url("andyjado.com".to_string()));

    let mut epr1:Expression = (url as ShowOnHeap).into();

    let itn1 = Intention::new();
    let mut expr2 = Expression::from_itn(itn1);
    expr2.express(url1);
    expr2.express(url2);
    println!("{epr1}\n\n\n\n{expr2}")
}
