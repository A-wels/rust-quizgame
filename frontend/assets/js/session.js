// Read the username from local storage
var username = localStorage.getItem('name');

// Read the session cookie if it is set, prevent undefined errors
var session = document.cookie.split('; ').find(row => row.startsWith('session')).split('=')[1];
// If the session cookie is not set, redirect to index.html
if (!session) {
    window.location.href = "index.html";
}
// set the name in the h2 element
document.getElementById('playername').innerHTML = "Spiele als: " + username;
