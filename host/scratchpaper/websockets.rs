    // loop {
    //     std::thread::sleep_ms(60000);
    // }
    //////////////////////////////////////////////////////////////////////

    // let server = Server::bind("127.0.0.1:2794").unwrap();
    // let x=5;
    // let connection = server.next();
    // for connection in server {
    //
    //     let b = rx.clone();
	// 	let request = connection.unwrap().read_request().unwrap(); // Get the request
	// 	let headers = request.headers.clone(); // Keep the headers so we can check them
	// 	request.validate().unwrap(); // Validate the request
    //
	// 	let mut response = request.accept(); // Form a response
    //
	// 	if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
	// 		if protocols.contains(&("rust-websocket".to_string())) {
	// 			// We have a protocol we want to use
	// 			response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
	// 		}
	// 	}
    //
	// 	let mut client = response.send().unwrap(); // Send the response
    //
	// 	let ip = client.get_mut_sender()
	// 		.get_mut()
	// 		.peer_addr()
	// 		.unwrap();
    //
	// 	println!("Connection from {}", ip);
    //
	// 	let message: Message = Message::text("Hello".to_string()+&x.to_string());
	// 	client.send_message(&message).unwrap();
    //
	// 	let (mut sender, mut receiver) = client.split();
    //
    //     thread::spawn(move|| {
    //         loop {
    //             // std::thread::sleep_ms(2000);
    //             let bla = b.lock().unwrap().recv().unwrap();
    //             println!("rx.recv().unwrap(): {:x}",bla);
    //             let message: Message = Message::text(bla.to_string());
    // 		    sender.send_message(&message).unwrap();
    //         }
    //     });
    //
	// 	for message in receiver.incoming_messages() {
	// 		let message: Message = message.unwrap();
    //
	// 		match message.opcode {
	// 			Type::Close => {
	// 				let message = Message::close();
	// 				// sender.send_message(&message).unwrap();
	// 				println!("Client {} disconnected", ip);
	// 				return;
	// 			},
	// 			// _ => sender.send_message(&message).unwrap(),
    //             _ => println!("Message received"),
	// 		}
	// 	}
    //     println!("yesssolo" );
    //
    //
    // }
    
