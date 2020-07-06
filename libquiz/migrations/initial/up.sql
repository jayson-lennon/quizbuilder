CREATE TABLE IF NOT EXISTS users (
  user_id        uuid          PRIMARY KEY,
  login          text          UNIQUE NOT NULL,
  date_created   timestamptz   NOT NULL,
  password       text          ,
  name           text          ,
  last_login     timestamptz   NOT NULL
);

CREATE TABLE IF NOT EXISTS quizzes (
  quiz_id        uuid          PRIMARY KEY,
  owner          uuid          REFERENCES users (user_id) NOT NULL,
  name           text          ,
  date_created   timestamptz   NOT NULL,
  open_date      timestamptz   NOT NULL,
  close_date     timestamptz   ,
  duration_sec   integer       ,
  shortcode      text          NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS quiz_questions (
  quiz_question_id   uuid       PRIMARY KEY,
  quiz_id            uuid       REFERENCES quizzes (quiz_id) NOT NULL,
  question_data      text       NOT NULL,
  position           integer    CHECK (position > 0)
);

CREATE TABLE IF NOT EXISTS quiz_options (
  quiz_option_id     uuid       PRIMARY KEY,
  quiz_question_id   uuid       REFERENCES quiz_questions (quiz_question_id) NOT NULL,
  option_data        text       NOT NULL,
  is_correct         boolean    NOT NULL,
  position           integer    CHECK (position > 0),
  option_type        integer    NOT NULL
);

CREATE TABLE IF NOT EXISTS quiz_submissions (
  quiz_submission_id  uuid          PRIMARY KEY,
  identity            text          NOT NULL,
  quiz_id             uuid          REFERENCES quizzes (quiz_id) NOT NULL,
  start_date          timestamptz   NOT NULL,
  finish_date         timestamptz
);

CREATE TABLE IF NOT EXISTS quiz_answers (
  quiz_submission_id   uuid     REFERENCES quiz_submissions (quiz_submission_id) NOT NULL,
  quiz_question_id     uuid     REFERENCES quiz_questions (quiz_question_id) NOT NULL,
  quiz_option_id       uuid     REFERENCES quiz_options (quiz_option_id) NOT NULL,
  PRIMARY KEY (quiz_submission_id, quiz_question_id, quiz_option_id)
);

CREATE TABLE IF NOT EXISTS app_config (
  app_key       text   NOT NULL,
  app_value     text   NOT NULL
);

INSERT INTO app_config VALUES ('db_version',      '1');
INSERT INTO app_config VALUES ('shortcode_len',   '8');
INSERT INTO app_config VALUES ('shortcode_chars', 'abcdefghjkmnprtuwxyABCEFHJKLMNR234679');

