# CS128H Final Project

### Group Name: 
GD

### Names and NetIDs
Grace Dai, graced4

### Project Introduction
My project aims to implement a simple chat server/client application using Rust and asynchronous networking with the Tokio crate. The goal of this project is to facilitate real-time communication between multiple clients connected to a central server. I chose this project because I wanted to explore network programming concepts in Rust and gain hands-on experience with asynchronous I/O.

### Overview
Server Component:
- The server listens for incoming connections from clients using a TCP listener. Upon connection, it accepts client connections and creates a separate asynchronous task to handle each client. It'll manage client messages by broadcasting received messages to all connected clients except the sender.

Client Component:
- The client connects to the server using a TCP stream. It allows users to input their name and starts a chat session. Client can send messages to the server and display received messages from other clients.

Checkpoint Goals:
- Checkpoint 1: Implement server and TCP listener. Accept client connections and handle basic message exchange.
- Checkpoint 2: Implement client connection to the server. Enable message sending and receiving between clients and server. Implement message broadcasting to connected clients.

### Possible Challenges
- learning asynchronous programming with Rust and Tokio and managing concurrent tasks efficiently
- managing multiple client connections concurrently while ensuring thread safety
- understanding networking conceptslike TCP, sockets, and client-server communication protocols
- learning new libraries and crates

### References
Tokio documentation: https://tokio.rs/
Rust official documentation: https://doc.rust-lang.org/
Inspiration from similar projects available on GitHub and online tutorials.
