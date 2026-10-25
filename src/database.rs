use sea_orm::sea_query::Expr;
use sea_orm::*;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let url_db = "mysql://root:@127.0.0.1:3306/testdb";
    let mut opt = ConnectOptions::new(url_db.to_string());
    opt.connect_timeout(Duration::from_secs(3));
    let db = match Database::connect(opt).await {
        Ok(conn) => {
            println!("db connected");
            conn
        }
        Err(err) => {
            println!("db connected error: {:?}", err);
            return Err(err);
        }
    };

    // 1. INSERT
    let user = ActiveModel {
        name: Set("alice".to_string()),
        email: Set("alice@test.com".to_string()),
        ..Default::default()
    };
    let uid = match user.insert(&db).await {
        Ok(u) => {
            println!("inserted uid = {}", u.id);
            u.id
        }
        Err(err) => {
            println!("insert error: {:?}", err);
            return Err(err);
        }
    };

    // 2. SELECT
    match Entity::find_by_id(uid).one(&db).await {
        Ok(res) => match res {
            Some(u) => {
                println!("find by id: {:?}", u)
            }
            None => {
                println!("not Found")
            }
        },
        Err(err) => {
            println!("find error: {:?}", err);
            return Err(err);
        }
    };
    println!("=====");

    match Entity::find()
        .filter(Column::Name.like("alic_"))
        .order_by_desc(Column::Id)
        .limit(10)
        .all(&db)
        .await
    {
        Ok(list) => {
            if !list.is_empty() {
                for v in list {
                    println!("find list: {:?} {:?} {:?}", v.id, v.name, v.email);
                }
            } else {
                println!("not Found")
            }
        }
        Err(err) => {
            println!("find error: {:?}", err);
            return Err(err);
        }
    }
    println!("=====");

    // 3. UPDATE
    let user = ActiveModel {
        id: Set(uid),
        email: Set("x@qq.com".to_string()),
        ..Default::default()
    };
    match user.update(&db).await {
        Ok(res) => {
            println!("updated: {:?}", res);
        }
        Err(err) => {
            println!("updated error: {:?}", err);
            return Err(err);
        }
    }

    match Entity::update_many()
        .col_expr(Column::Name, Expr::value("test"))
        .filter(Column::Name.eq("alice"))
        .exec(&db)
        .await
    {
        Ok(res) => {
            println!("updated count: {:?}", res.rows_affected)
        }
        Err(err) => {
            println!("updated error: {:?}", err);
            return Err(err);
        }
    }

    // 4. DELETE
    //     match Entity::delete_by_id(uid).exec(&db).await {
    //         Ok(res) => {
    //             println!("delete rows affected: {:?}", res.rows_affected)
    //         }
    //         Err(err) => {
    //             println!("deleted error: {:?}", err);
    //             return Err(err);
    //         }
    //     }
    //
    //     match Entity::delete_many()
    //         .filter(Column::Name.eq("test"))
    //         .exec(&db)
    //         .await
    //     {
    //         Ok(res) => {
    //             println!("delete rows affected count: {:?}", res.rows_affected)
    //         }
    //         Err(err) => {
    //             println!("deleted error: {:?}", err);
    //             return Err(err);
    //         }
    //     }

    //raw SQL
    let rows = db
        .query_all(Statement::from_sql_and_values(
            DbBackend::MySql,
            r#"
            SELECT id, name, email FROM `user` WHERE id = ?
            "#,
            [1.into()],
        ))
        .await?;

    for row in rows {
        let id: i32 = row.try_get("", "id")?;
        let name: String = row.try_get("", "name")?;
        let email: String = row.try_get("", "email")?;
        println!("id: {}, name: {}, email: {}", id, name, email);
    }

    #[derive(FromQueryResult)]
    struct MyUser {
        id: i32,
        name: String,
        email: String,
        content: String,
    }

    let rows: Vec<MyUser> = MyUser::find_by_statement(Statement::from_sql_and_values(
        DbBackend::MySql,
        r#"
        SELECT a.id, a.name, a.email, b.content FROM user as a, t1 as b
        WHERE a.id = ? and a.name = ? and a.id=b.id
        "#,
        [38.into(), String::from("test").into()],
    ))
    .all(&db)
    .await?;

    for row in rows {
        println!(
            "FromQueryResult: {} {} {} {}",
            row.id, row.name, row.email, row.content
        );
    }

    Ok(())
}
