window.onload = function() {

  var but = document.getElementById('buttonsend');
  but.addEventListener('click', but_send, false);

  // var socket = new WebSocket('ws://echo.websocket.org');
  var socket = new WebSocket('ws://127.0.0.1:2794');
  socket.onopen = function(event) {
    console.log("conntection established ...")
    socket.send("hoitaus");
  };
  socket.onmessage = function(event) {
    var message = event.data;
    $( "#slider" ).slider( "value", message );
    console.log(message);
  };

  function but_send() {
    socket.send("hoitaus");
    console.log("clicked");
  }

  $( function() {
    $( "#slider" ).slider({
      max: 1,
      step: 0.001,
      slide: function( event, ui ) {
        console.log("sending: "+ui.value);
        socket.send(ui.value);
      }
    });
  } );
};
