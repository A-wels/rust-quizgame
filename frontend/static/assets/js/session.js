// Read the username from local storage
var username = localStorage.getItem('name');

// Read the session cookie if it is set, prevent undefined errors
var session = document.cookie
try{
session = session.split('session=')[1];
}catch(e){

// If the session cookie is not set, redirect to index.html
    window.location.href = "/";
}

// set the name in the h2 element
document.getElementById('playername').innerHTML = "Spiele als: " + username;