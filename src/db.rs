use tokio_postgres::{Client, Statement, connect, NoTls};
use actix::{Actor, Context, Addr, Message, Handler, ResponseFuture};
use std::io;

pub struct DB {
    client: Client,
    get: Statement,
}

impl Actor for DB {
    type Context = Context<Self>;
}

impl DB {
    pub async fn connect(db_url: &str) -> Result<Addr<DB>, io::Error> {
        let (client, connection) = connect(db_url, NoTls)
            .await
            .expect("can not connect to postgresql");
        actix_rt::spawn(async move {
            connection.await.expect("connection error")
        });
        let get = client
            .prepare("SELECT 'hello';").await
            .expect("Cannot prepare get statement");
        Ok(DB::create(move |_| DB {
            client,
            get,
        }))
    }
}

pub struct GetMessage {}

impl Message for GetMessage {
    type Result = io::Result<String>;
}

impl Handler<GetMessage> for DB {
    type Result = ResponseFuture<io::Result<String>>;

    fn handle(&mut self, _msg: GetMessage, _ctx: &mut Self::Context) -> Self::Result {
        let fut = self.client.query_one(&self.get, &[]);
        Box::pin(async move {
            let row = fut
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;
            Ok(row.get(0))
        })
    }
}
