const socket = new WebSocket('ws://localhost:8001/ws');
var session = document.cookie.split('; ')
if (session[0] != ""){
    session = session[0].split('=')[1];
}


socket.addEventListener('message', event => {
    if (event.data.startsWith('loginSuccess|')) {
        session = event.data.split("|")[1];
        document.cookie = "session="+session;
        window.location.href = "admin.html";

    }else if (event.data.startsWith("loginFailed")){
        // create error message
        var error = document.createElement("p");
        error.innerHTML = "Falsches Passwort";
        error.style.color = "red";
        document.getElementById("login").appendChild(error);


    }

  });
// add eventhandler to btn-login
document.getElementById("btn-login").addEventListener("click", function(){
    var password = document.getElementById("password").value;
    sendRequest(password);
});

function sendRequest(password) {
    // send request to backend
    // log to console
    socket.send("login|"+password);
  }