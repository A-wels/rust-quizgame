var stats = []
socket.addEventListener("message", (event) => {
    if (event.data.startsWith('{"questions"')) {
        stats = JSON.parse(event.data);
        displayStats();
    }
});
// close socket on page unload
window.addEventListener("beforeunload", function (event) {
  socket.close();
});


function displayStats() {
  showingStats = true;
  document.getElementById("question").innerHTML =
    "Die Runde ist vorbei! Hier sind die Ergebnisse:";
  // make answers invisible
  toggleQuestionVisibility(false);
  clearInterval(updateInterval);
  updateInterval = setInterval(function () {
    getQuestion();
  }, 3000);

  // append child to div with id "results"
  var results = document.getElementById("results");

  for (var i = 0; i < stats.questions.length; i++) {
    var d = document.createElement("canvas");
    d.id = "result" + i;
    d.style = "margin-bottom: 20px;";
    results.appendChild(d);
    const ctx = document.getElementById(d.id);
    var sumOfAnswers = stats.answers.answers[i].reduce((x,y) => x+y, 0)

    var bgcolors = ["#ff0000", "#ff0000", "#ff0000", "#ff0000"]
    bgcolors[stats.questions[i].correct_answer] = "#00ff00"
    new Chart(ctx, {
      type: "bar",
      data: {
        labels: [
          stats.questions[i].answer1,
          stats.questions[i].answer2,
          stats.questions[i].answer3,
          stats.questions[i].answer4,
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