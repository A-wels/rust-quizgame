const socket = new WebSocket("ws://localhost:8001/ws");
var session = document.cookie
  .split("; ")
  .find((row) => row.startsWith("session"))
  .split("=")[1];
// if session is not set, redirect to login page
if (session == undefined) {
  window.location.href = "index.html";
}
var showingStats = false;

socket.addEventListener("open", () => {
  socket.send("register|" + session);
  socket.send("getPhase");
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
    document.getElementById("answers").style.visibility = "visible";
  } else {
    document.getElementById("answers").style.visibility = "hidden";
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
}

function sendAnswer(answer) {
  // send answer to backend
  // log answer
  socket.send("answer|" + answer + "|" + session);
  socket.send("getQuestion|" + session);
  displayQuestion();
}
let updateInterval = 0;
function endOfRound() {
  document.getElementById("question").innerHTML =
    "Du hast alle Fragen der Runde beantwortet! Bitte warte auf die nächste Runde.";
  // make answers invisible
  toggleQuestionVisibility(false);
  updateInterval = setInterval(function () {
    requestStats();
  }, 3000);
}

function getQuestion() {
  socket.send("getQuestion|" + session);
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
}
function requestStats() {
  // send request to backend
  socket.send("getStats|" + session);
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
