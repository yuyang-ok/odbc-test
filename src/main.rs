use odbc::{Statement, create_environment_v3};

fn main() {
    let env = create_environment_v3().map_err(|e| e.unwrap()).unwrap();

    let conn=   env.connect_with_connection_string("Driver={Driver_IBM_DB2};Hostname=127.0.0.1;Port=50000;Protocol=TCPIP;UID=db2inst1;PWD=db2inst1-pwd;CodePage=1208;Database=TSKDB;") .unwrap() ;

    let stmt = Statement::with_parent(&conn).unwrap();

    let r = stmt
        .exec_direct("select tbspace from syscat.tablespaces")
        .unwrap();

    match r {
        odbc::ResultSetState::Data(mut stmt) => {
            let cols = stmt.num_result_cols().unwrap();
            while let Some(mut cursor) = stmt.fetch().unwrap() {
                for i in 1..(cols + 1) {
                    match cursor.get_data::<&str>(i as u16).unwrap() {
                        Some(val) => print!(" {}", val),
                        None => print!(" NULL"),
                    }
                }
                println!("");
            }
        }
        odbc::ResultSetState::NoData(..) => println!("no data"),
    }
}
