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
    // create a hash of the name + czrrent time
    var hash = hashCode(username + Date.now());
    // store the hash as session cookie
    document.cookie = "session=" + hash;
    
    // Redirect to quiz.html
    window.location.href = "quiz.html";

})
