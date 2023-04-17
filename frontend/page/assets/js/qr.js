var url = getHostIp();
const socket = new WebSocket('ws://' + url + ':8001/ws');

// wait for socket and send "getQR" to backend
socket.addEventListener('open', event => {
    socket.send("getQR");
});



// add listener to socket
socket.addEventListener('message', event => {
    // check if event.data starts with "qr|"
    if (event.data.startsWith('<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n')) {
        var qr = event.data
       // get svg element 
         var svg = document.getElementById("qr");
        // set svg innerHTML to event.data
        svg.innerHTML = qr

    }
});


