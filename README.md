# rust-quizgame

A simple quiz game consisting of a HTML+CSS+Javascript frontend and a rust backend.

Players can select a username to play with. They then answer multiple rounds of questions with pauses in between. During the pauses, the correct answer, as well as a statistic how players have answered, are shown.

## How to run
- Put the executable for the backend, config.yml and questions.csv into a folder
- Edit the contents of questions.csv
- Edit config.yml to your liking
- Run the executable for the backend
- Run the executable for the frontend


Navigate to {localip}/index.html on the user clients and to {localip}/login.html on the admin device.

## Warning
Only host this on a local network with devices you trust. There was **no** emphasis on securing communication, since this is meant for the use in a group with kids.