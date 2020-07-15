import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Question } from './@types/question';
import { GlobalEventService } from './services/global-event.service';
import { FormControl, FormGroup } from '@angular/forms';
import { v4 as uuidv4 } from 'uuid';
import { Quiz } from './@types/quiz';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})

export class AppComponent {

  public questions: Question[] = [];

  quizForm: FormGroup = new FormGroup({
    title: new FormControl(''),
    owner: new FormControl(''),
    dateOpen: new FormControl(''),
    timeOpen: new FormControl(''),
    duration: new FormControl(''),
  });

  private today(): string {
    return new Date().toJSON().slice(0, 10);
  }

  constructor(events: GlobalEventService, private readonly http: HttpClient) {
    events.deleteOption$.subscribe(id => this.deleteOption(id));
    events.deleteQuestion$.subscribe(id => this.deleteQuestion(id));

    this.quizForm.get('dateOpen').setValue(this.today());
    this.quizForm.get('duration').setValue(1800);
  }

  public createNewQuestion(): void {
    this.questions.push({
      data: '',
      options: [],
      id: uuidv4(),
    });
  }

  public deleteOption(id: string): void {
    for (const question of this.questions) {
      question.options = question.options.filter(option => option.id !== id);
    }
  }

  public deleteQuestion(id: string): void {
    this.questions = this.questions.filter(question => question.id !== id);
  }

  public generateApiRequest(): object {
    const title = this.quizForm.get('title').value;
    const dateOpen = this.quizForm.get('dateOpen').value;
    const timeOpen = this.quizForm.get('timeOpen').value;
    const duration = this.quizForm.get('duration').value;
    const owner = this.quizForm.get('owner').value;

    const questions = this.questions.map((q) => {
      const options = q.options.map((op) => {
        return `{optionData: \"${op.data}\", isCorrect: ${op.isCorrect}, optionType: \"${op.type}\"}`;
      }).join(',');
      return `{questionData: \"${q.data}\", options: [${options}]}`;
    });

    const quizInput = `{name: \"${title}\", owner: \"${owner}\", openDate: \"${dateOpen} ${timeOpen}.000Z\", duration: \"${duration}\", questions: [${questions}]}`;

    const query = {
      operationName: null,
      query: `mutation {createQuizWithQuestions(quizInput: ${quizInput}){quizId shortcode}}`,
      variables: {}
    };

    return query;

  }

  public submitQuiz(): void {
    const request = this.generateApiRequest();
    const response = this.http.post('http://localhost:8001/graphql', request).subscribe((response: any) => {
      window.location.href = `http://localhost:8002/quiz/${response.data.createQuizWithQuestions.shortcode}`;
    });
  }
}
