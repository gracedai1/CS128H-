// imports
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::io::BufReader;
use tokio::io::BufWriter;
use tokio::net::tcp::WriteHalf;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;
use tokio::net::TcpStream;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use std::io::{self, Write}; 


pub type IOResult<T> = std::io::Result<T>;  //define custom Result type alias for i/o 

#[derive(Debug, Clone)] //struct to represent the type of msgs exchanged bt the client and server
pub struct Message {
    pub socket_address: SocketAddr,
    pub msg_text: String,
    pub user_name: String,
}

#[tokio::main]
pub async fn main() -> IOResult<()> {
    let address = "127.0.0.1:5678";

    femme::start(); //logging w femme crate for async fns

    // create TCP listener on specified address
    let tcp_listener = TcpListener::bind(address).await?; //bind listener to address + await result
    log::info!("server is ready on {}", address);

    let (sender, _) = broadcast::channel::<Message>(10); //create channel to share msgs w/ all clients, w/ buffer size of 10

    // server loop to accept incoming connections
    loop {
        let (tcp_stream, socket_address) = tcp_listener.accept().await?;

        let sender = sender.clone();
        tokio::spawn(async move { // spawn new async task to handle client connection
            let result = client_connection_handler(tcp_stream, sender, socket_address).await;
            match result {
                Ok(_) => { // if task sucessful
                    log::info!("client_connection_handler() terminated gracefully")
                }
                Err(error) => log::error!("client_connection_handler() encountered error: {}", error), //if error during execution
            }
        });
    }
}

//function for processing each client connection
async fn client_connection_handler(
    mut tcp_stream: TcpStream,
    sender: Sender<Message>,
    socket_address: SocketAddr,
) -> IOResult<()> {

    print!("please enter your name: "); //get user's name 
    let mut name = String::new();  
    io::stdout().flush()?;  
    io::stdin().read_line(&mut name)?;  
    let name = name.trim();

    let mut receiver = sender.subscribe(); // to receive messages from other clients

    // split tcp stream into buf reader and writer
    let (bufreader, bufwriter) = tcp_stream.split();
    let mut bufreader = BufReader::new(bufreader);
    let mut bufwriter = BufWriter::new(bufwriter);

   
    let client_first_msg = &format!("user: {}\nBegin chatting!\n\n", name); //first message, w/ user's id name
    bufwriter.write(client_first_msg.as_bytes()).await?;
    bufwriter.flush().await?; //so msg is sent immediately

    let mut incoming_msg = String::new(); //to store incoming messages

    loop { //for communication w/ client
        let sender = sender.clone();
        //let name = name.clone();
        tokio::select! {
            result = receiver.recv() => {
                broadcast_message_handler(result, socket_address, &mut bufwriter).await?; //read from channel
            }

            network_read_result = bufreader.read_line(&mut incoming_msg) => { //read from socket
                let bytes_read: usize = network_read_result?;
                if bytes_read == 0 { //checking for end of file, break if reached
                    break;
                }
                client_message_handler(&name, &incoming_msg, &mut bufwriter, sender, socket_address).await?;
                incoming_msg.clear(); //clear buffer for next msg
            }
        }
    }

    Ok(()) //successful
}

//function for handling msgs from broadcast channel
async fn broadcast_message_handler( 
    result: Result<Message, RecvError>,
    socket_address: SocketAddr,
    bufwriter: &mut BufWriter<WriteHalf<'_>>,
    //name: &str,
) -> IOResult<()> {
    match result {
        Ok(msg) => { //if successful msg
            if msg.socket_address != socket_address { //checking if msg is from the same client
                bufwriter.write_all(format!("[{}]: {}\n", msg.user_name, msg.msg_text).as_bytes()).await?;
                bufwriter.flush().await?;
            }
        }
        Err(error) => { 
            log::error!("{:?}", error);
        }
    }

    Ok(()) //done
}

//function for handling msgs received from client socket
async fn client_message_handler(
    //bytes_read: usize,
    name: &str,
    incoming_msg: &str,
    bufwriter: &mut BufWriter<WriteHalf<'_>>,
    sender: Sender<Message>,
    socket_address: SocketAddr,
) -> IOResult<()> {

    let outgoing_msg = format!("[{}]: {}\n", name, incoming_msg.trim()); //process incoming msg to create outgoing msg

    bufwriter.write(outgoing_msg.as_bytes()).await?; //write outgoing to client
    bufwriter.flush().await?;

    let _ = sender.send(Message { //broadcast incoming msg to all clients besides sender
        socket_address,
        msg_text: incoming_msg.to_string(),
        user_name: name.to_string(),
    });

    Ok(()) //done
}
