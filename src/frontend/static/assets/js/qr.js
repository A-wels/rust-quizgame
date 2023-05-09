var url = getHostIp();
const socket = new WebSocket('ws://' + url + ':8001/ws');

// try to get session from cookie
var session = document.cookie.split('; ')
if (session[0] == "") {
    window.location.href = "login";
}
else {
    session = session[0].split('=')[1];
}

// wait for socket and send "getQR" to backend
socket.addEventListener('open', event => {
    socket.send("getQR|session=" + session)
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

    }else if (event.data.startsWith("gameID|")){
        var gameID = event.data.split("|")[1];
        document.getElementById("gameID").innerHTML = gameID;
    }
});


