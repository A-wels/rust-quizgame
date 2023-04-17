var url = getHostIp();
const socket = new WebSocket('ws://' + url + ':8001/ws');
// close socket on page unload
window.addEventListener("beforeunload", function (event) {
  socket.close();
});

var session = document.cookie.split('; ')
if( session[0] == ""){
    window.location.href = "login.html";
}
else{
    session = session[0].split('=')[1];
}

var stats = []
socket.addEventListener("message", (event) => {
    if (event.data.startsWith('{"questions"')) {
        stats = JSON.parse(event.data);
        displayStats();
    }
});

var nextPhaseIsStats = true;
// onclick handler for btn-next
document.getElementById("btn-next").addEventListener("click", function(){
    socket.send("next|"+session);
    if(nextPhaseIsStats){
        socket.send("getStats|" + session);
        nextPhaseIsStats = false;
        // set text of btn-next to "next"
        document.getElementById("btn-next").innerHTML = "Nächste Runde";
    }else{
        // delete all children from div with id "results"
        var results = document.getElementById("results");
        while (results.firstChild) {
            results.removeChild(results.firstChild);
        }
        nextPhaseIsStats = true;
        document.getElementById("btn-next").innerHTML = "Zeige Statistik";
        }
})


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
            }
          },
        },
      });
    }
  }