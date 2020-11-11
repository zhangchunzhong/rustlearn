use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use serde_json;

fn main() {
    let sql = "select x, sum(q) from T1, T2, T3 where T1.x=5 and T2.y>1 and T3.z<3 and T1.a=T3.b and T2.c=T3.d group by x;";

    let dialect = GenericDialect {};

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    for i in &ast {
        println!("{}", serde_json::to_string_pretty(&i).unwrap());
    }
}
