use std::fs::read_to_string;
use mysql::*;
use mysql::prelude::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {

    let file: Vec<_> = (read_to_string("config.cfg")?).lines().map(|x| x.split(": ")).collect();


    //let url  = "mysql:://" + *cnf.user + cnf.password + cnf.addr + cnf.port + cnf.db;
    
    //let url = "mysql://user:user@127.0.0.1:3306/checkin";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    conn.query_drop(
	r"
CREATE TABLE IF NOT EXISTS aulas (
aluno_id int not null,
total_att int not null,
nome text
)")?;

    println!("sucess!");
    Ok(())
}


/*
use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root:password@localhost:3307/db_name";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    // Let's create a table for payments.
    conn.query_drop(
        r"CREATE TEMPORARY TABLE payment (
            customer_id int not null,
            amount int not null,
            account_name text
        )")?;

    let payments = vec![
        Payment { customer_id: 1, amount: 2, account_name: None },
        Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
        Payment { customer_id: 5, amount: 6, account_name: None },
        Payment { customer_id: 7, amount: 8, account_name: None },
        Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    ];

    // Now let's insert payments to the database
    conn.exec_batch(
        r"INSERT INTO payment (customer_id, amount, account_name)
          VALUES (:customer_id, :amount, :account_name)",
        payments.iter().map(|p| params! {
            "customer_id" => p.customer_id,
            "amount" => p.amount,
            "account_name" => &p.account_name,
        })
    )?;

    // Let's select payments from database. Type inference should do the trick here.
    let selected_payments = conn
        .query_map(
            "SELECT customer_id, amount, account_name from payment",
            |(customer_id, amount, account_name)| {
                Payment { customer_id, amount, account_name }
            },
        )?;

    // Let's make sure, that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows
    // without `ORDER BY`, so assume we are lucky.
    assert_eq!(payments, selected_payments);
    println!("Yay!");

    Ok(())
}
*/
