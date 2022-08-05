use ang::Angle;
use melcome::expression::{Intention, Expression, Url, ShowOnHeap, Expressing, Digging, Crawling};

fn main() {

    let url1 = Box::new(Url("andyjado.com".to_string())) as ShowOnHeap;
    let url3 = Url("andyjado.com".to_string());
    let cta = Angle::Degrees(30.9);

    let mut epr1 = Expression::from_obe(url1);
    epr1.judge(cta);

    let itn1 = Intention::new();
    let mut expr2 = Expression::from_itn(itn1);
    expr2.express(url3.clone());
    epr1.express(url3);
    let key1 = epr1.key();
    let key2 = Crawling::key(&expr2);

    let m1 = epr1.recover();

    println!("{m1}\n");

    println!("{key1:?}\n\n\n\n{key2}\n");
    println!("{expr2}\n\n\n\n{epr1}\n");

}
