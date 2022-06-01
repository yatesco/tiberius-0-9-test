use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::main]
async fn main() {

    // NOTE: it makes no difference what credentials are used here - it still fails to connect
    let url = "server=tcp:127.0.0.1,1433;encrypt=false;TrustServerCertificate=true;user=XXX;password=YYY;database=AAA;";
    let config = tiberius::Config::from_ado_string(url).unwrap();

    let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
    tcp.set_nodelay(true).unwrap();

    println!("Connecting...");
    let mut conn = Client::connect(config, tcp.compat_write()).await.unwrap();
    conn.simple_query("").await.unwrap().into_row().await.unwrap();
    println!("Connected");
}
