var url = getHostIp();
const socket = new WebSocket('ws://' + url + ':8001/ws');
// close socket on page unload
window.addEventListener("beforeunload", function (event) {
  socket.close();
});

var session = document.cookie.split('; ')
if (session[0] == "") {
  window.location.href = "login.html";
}
else {
  session = session[0].split('=')[1];
}

socket.addEventListener("open", () => {
  socket.send("getPhase");
});

var stats = []
socket.addEventListener("message", (event) => {
  if (event.data.startsWith('{"questions"')) {
    stats = JSON.parse(event.data);
    displayStats();
  } else if (event.data === "endOfGame") {
    endGame();
  } else if (event.data.startsWith("phase")) {
    handlePhaseLoad(event.data.split("|")[1]);
    
  }
});

function handlePhaseLoad(phase) {
  if (phase == "Question") {
    nextPhaseIsStats = true
  } else if (phase == "Stats") {
    nextPhaseIsStats = false
    socket.send("getStats|" + session);
    document.getElementById("btn-next").innerHTML = "Nächste Runde";
    
  }
}

var nextPhaseIsStats = true;
// onclick handler for btn-next
document.getElementById("btn-next").addEventListener("click", function () {
  socket.send("next|" + session);
  if (nextPhaseIsStats) {
    socket.send("getStats|" + session);
    nextPhaseIsStats = false;
    // set text of btn-next to "next"
    document.getElementById("btn-next").innerHTML = "Nächste Runde";
  } else {
    // delete all children from div with id "results"
    var results = document.getElementById("results");
    while (results.firstChild) {
      results.removeChild(results.firstChild);
    }
    nextPhaseIsStats = true;
    document.getElementById("btn-next").innerHTML = "Zeige Statistik";
  }
})

function endGame() {
  var btn = document.getElementById("btn-next")
  btn.remove()
  var endMessage = document.getElementById("card-end")
  endMessage.style.display = "flex"

}
function displayStats() {
  showingStats = true;

  // append child to div with id "results"
  var results = document.getElementById("results");

  for (var i = 0; i < stats.questions.length; i++) {
    var d = document.createElement("canvas");
    d.id = "result" + i;
    d.style = "margin-bottom: 20px;";
    results.appendChild(d);
    const ctx = document.getElementById(d.id);
    var sumOfAnswers = stats.answers.answers[i].reduce((x, y) => x + y, 0)
    var bgcolors = ["#ff0000", "#ff0000", "#ff0000", "#ff0000"]
    bgcolors[stats.questions[i].correct_answer] = "#00ff00"
    new Chart(ctx, {
      type: "bar",
      data: {
        labels: [
          split_long_labels(stats.questions[i].answer1),
          split_long_labels(stats.questions[i].answer2),
          split_long_labels(stats.questions[i].answer3),
          split_long_labels(stats.questions[i].answer4),
        ],
        datasets: [
          {
            label: stats.questions[i].question,
            data: stats.answers.answers[i],
            borderWidth: 1,
            backgroundColor: bgcolors,
          },
        ],
      },
      options: {
        indexAxis: "y",
        scales: {
          x: {
            max: sumOfAnswers,
            ticks: {
              stepSize: 1,
            },
          },
        },
        plugins: {
          legend: {
            display: false,
          },
          title: {
            display: true,
            text: stats.questions[i].question,
            font: {
              size: 18,
            }
          }
        },
      },
    });
  }
}

function split_long_labels(label) {
  let words = label.split(" ")
  let result = []
  let currentLine = ""
  let max_length = 40

  for (let i = 0; i < words.length; i++) {
    if (currentLine.length + words[i].length <= max_length) {
      currentLine += words[i] + " "
    } else {
      result.push(currentLine.trim())
      currentLine = words[i] + " "
    }
  }
  if (currentLine.trim().length > 0) {
    result.push(currentLine.trim());
  }
  return result
}