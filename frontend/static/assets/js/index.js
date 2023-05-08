var cookie = document.cookie;
try {
  cookie = session
    .split("session=")[1];
  window.location.href = "quiz";

} catch (e) {
}
const queryString = window.location.search;
const urlParams = new URLSearchParams(queryString);
const gameID = urlParams.get('gameID')
if (gameID != null) {
  document.getElementById('gameid').value = gameID
}

var url = getHostIp();
const socket = new WebSocket('ws://' + url + ':8001/ws');
// close socket on page unload
window.addEventListener("beforeunload", function (event) {
  socket.close();
});

socket.addEventListener("message", (event) => {
  console.log(event.data)
  if (event.data.startsWith('addPlayerSuccess|')) {  
    var session = event.data.split("|")[1];
    document.cookie = "session=" + session;
    // Redirect to quiz.html
    console.log("REdirected")

    window.location.href = "quiz";
  } else if(event.data.startsWith("invalidGameID")){
    alert("Falsche Spiel ID")
    document.getElementById('gameid').value = ""
  }
})




// get element with id name
var nameBtn = document.getElementById('select-name');
// function to create a hash of a string
hashCode = s => s.split('').reduce((a,b)=>{a=((a<<5)-a)+b.charCodeAt(0);return a&a},0)

// add an event listener to the button
nameBtn.addEventListener('click', function() {
    // Read the value of the input field with id=name
    var username = document.getElementById('name').value;
    // store the name in local storage
    localStorage.setItem('name', username);
    // request a session id
    var gameID = document.getElementById('gameid').value;
    socket.send("register|"+gameID)

})
