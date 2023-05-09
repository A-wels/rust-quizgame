var url = getHostIp();
const socket = new WebSocket('ws://' + url + ':8001/ws');
// close socket on page unload
window.addEventListener("beforeunload", function (event) {
  socket.close();
});

var session = document.cookie;
var answerTimer = 0

try {
  session = session.split("session=")[1];
} catch (e) {
  window.location.href = "/";
}

var showingStats = false

socket.addEventListener("open", () => {
 // socket.send("register|" + session);
  socket.send("getPhase|session=" + session)
});
var quiz = [];
var stats = [];
socket.addEventListener("message", (event) => {
  if (event.data.startsWith('{"question":')) {
    quiz = JSON.parse(event.data);
    displayQuestion();
  } else if (event.data === "endOfRound") {
    endOfRound();
  } else if (event.data === "endOfGame") {
    endGame();
  } else if (event.data.startsWith("phase")) {
    handlePhaseLoad(event.data.split("|")[1]);
  }
});

function toggleQuestionVisibility(visible) {
  if (visible == true) {
    document.getElementById("answers").style.display = "inline";
    document.getElementById("progress").style.display = "flex"

  } else {
    document.getElementById("answers").style.display = "none";
    document.getElementById("progress").style.display = "none"
    document.getElementById("progress-bar").style.width = "100%"

  }
}

function handlePhaseLoad(phase) {
  if (phase == "Question") {
    getQuestion();
  } else if (phase == "Stats") {
    requestStats();
  }
}
function displayQuestion() {
  // access
  clearInterval(updateInterval);
  toggleQuestionVisibility(true);
  showingStats = false;
  // remove all children from div with id "results"
  var results = document.getElementById("results");
  while (results.firstChild) {
    results.removeChild(results.firstChild);
  }
  document.getElementById("question").innerHTML = quiz.question;
  document.getElementById("answer1").innerHTML = quiz.answer1;
  document.getElementById("answer2").innerHTML = quiz.answer2;
  document.getElementById("answer3").innerHTML = quiz.answer3;
  document.getElementById("answer4").innerHTML = quiz.answer4;

  // start the timer
  startTimer()
}

function startTimer() {
  // start timer for div progress-bar that increases aria-valuenow from 0 to 100
  var timeleft = 20;
  answerTimer = setInterval(function () {
    if (timeleft < 0) {
      sendAnswer(5);
    }
    timeleft -= 1;
    var widthPercentage = ((timeleft / 20) * 100).toString() + "%"
    document.getElementById("progress-bar").style.width = widthPercentage
    console.log(timeleft / 20.)
    console.log("timeleft: " + timeleft)
    console.log("widthPercentage: " + widthPercentage)
    console.log()


  }, 1000)
}

function sendAnswer(answer) {
  document.getElementById("progress-bar").style.width = "100%"
  // send answer to backend
  // log answer
  socket.send("answer|" + answer + "|session=" + session);
  socket.send("getQuestion|session=" + session);
  clearInterval(answerTimer);
}

let updateInterval = 0;
function endOfRound() {
  document.getElementById("question").innerHTML =
    "Bitte warte auf die nächste Runde.";
  // make answers invisible
  toggleQuestionVisibility(false);
  updateInterval = setInterval(function () {
    requestStats();
  }, 3000);
}

function getQuestion() {
  socket.send("getQuestion|session=" + session);
}

function endGame() {
  document.getElementById("question").innerHTML =
    "Das wars! Danke fürs spielen";
  // make answers invisible
  clearInterval(updateInterval);
  toggleQuestionVisibility(false);
  // remove all children from div with id "results"
  var results = document.getElementById("results");
  while (results.firstChild) {
    results.removeChild(results.firstChild);
  }
  clearInterval(answerTimer)
}
function requestStats() {
  // send request to backend
  socket.send("getStats|session=" + session);
}

// add event listeners to the buttons
document.getElementById("answer1").addEventListener("click", function () {
  sendAnswer(0);
});
document.getElementById("answer2").addEventListener("click", function () {
  sendAnswer(1);
});
document.getElementById("answer3").addEventListener("click", function () {
  sendAnswer(2);
});
document.getElementById("answer4").addEventListener("click", function () {
  sendAnswer(3);
});
