# quizbuilder
An online quiz generator and server that allows instructors to easily create quizzes for their students.

---

Creating online quizzes should be as straightforward as possible, so focus can be spent on creating quality quiz content. In order to make this a reality, this project aims to reduce friction when creating quizzes by streamlining the quiz creation process on the web. In addition, it should be possible to upload a quiz file and have the quiz service automatically configure and rollout a quiz. This will allow instructors to concentrate on quiz content instead of wrestling with a complicated interface.

Quiz participants should also have a great experience. Using responsive web design and clearly marked buttons to navigate the quiz will help facilitate this. The amount of time remaining to take a quiz is important, so timing information should be made clearly available to participants. They should also get notifications when their quiz is about to end, and the service should automatically submit an incomplete quiz. This way, a user's work is not lost and they will at least be able to achieve credit for the work that they have completed.

## Tech Stack
* Rust
* Actix-web
* PostgreSQL
* Angular / Typescript
* Sass
* Kubernetes

## Planned Features
* Time blocks for a quiz
* Timed quiz
  * The timer will adhere to any time blocks supplied
* Quiz creation
  * Web UI
  * Text configuration
* Quiz types
  * Multiple choice
  * Write-in
* Results view
  * List score of each participant
  * Aggregate scores
* When taking a quiz:
  * Mark for review
  * Flag for correction
  * Move between questions
  * Display timer (if applicable)
  * Notify if some questions are unanswered when submission is attempted

## User Stories
### Instructor
* Should be able to create a quiz
* Should be able to set a time limit for a quiz
* Should be able to set a time block to take the quiz
* Should be able to view answers given by a specific individual
* Should be able to view total score for a specific individual
* Should be able to view aggregate score

### Quiz Taker
* Should be able to move between questions
* Should be able to mark questions for review
* Should be notified if submission is attempted when questions are left unanswered
* Should be made aware of the time remaining to complete the quiz
* Should be notified when the time remaining has almost elapsed
* Should have the quiz auto-submitted once the time remaining has elapsed

## Long-term Goals
* Code blocks in questions and answers
* LaTeX in questions and answers
