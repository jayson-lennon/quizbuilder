# Infrastructure
The service as a whole is comprised of three microservices:
* Quiz Home/Management
* Quiz Host
* Quiz API

## Quiz API
The Quiz API acts as a communications layer between the services and the database. All quiz submissions, quiz templates, and user data is accessed through the API.

## Quiz Home/Management
The Quiz Home/Management service is simply the web host for home page of the application and for managers of quizzes. The quiz manager can create & edit quizzes, as well as view results of quizzes via this service (which obtains this data from the API).

## Quiz Host
The Quiz Host service is a web host that serves up quizzes for users to complete. It is completely stateless and is designed to be horizontally scalable in order to serve quizzes as efficiently as possible.

In addition, the Quiz Host is responsible for the intake of quiz results and passing these results to the API for storage in the database.
