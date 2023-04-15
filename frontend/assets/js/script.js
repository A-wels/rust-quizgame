var session = document.cookie.split('; ').find(row => row.startsWith('session')).split('=')[1];
if (session!= undefined) {
    window.location.href = "quiz.html";
  }
