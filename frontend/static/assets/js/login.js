var url = getHostIp();
const socket = new WebSocket("ws://" + url + ":8001/ws");

// close socket on page unload
window.addEventListener("beforeunload", function (event) {
  socket.close();
});

socket.addEventListener("message", (event) => {
  if (event.data.startsWith("loginSuccess|")) {
    session = event.data.split("|")[1];
    document.cookie = "session=" + session;
    window.location.href = "qr";
  } else if (event.data.startsWith("loginFailed")) {
    // create and append error message to div error if it does not exist
    if (document.getElementById("error") == null) {
      var error = document.createElement("p");
      error.innerHTML = "Falsches Passwort";
      error.style.color = "red";
      error.id = "error"
      document.getElementById("login").appendChild(error);
    }
  }
});
// add eventhandler to btn-login
document.getElementById("btn-login").addEventListener("click", function () {
  var password = document.getElementById("password").value;
  socket.send("login|" + password);
});

window.addEventListener('keydown', function (event) {
  if (event.key === 'Enter') {
    document.getElementById("btn-login").click();
  }
})
