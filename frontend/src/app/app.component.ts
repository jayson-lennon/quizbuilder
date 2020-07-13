import { Component } from '@angular/core';
import { Question } from './@types/question';
import { GlobalEventService } from './services/global-event.service';
import { FormControl, FormGroup } from '@angular/forms';
import { v4 as uuidv4 } from 'uuid';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})

export class AppComponent {

  public questions: Question[] = [];

  quizForm: FormGroup = new FormGroup({
    title: new FormControl(''),
    dateOpen: new FormControl(''),
    duration: new FormControl(''),
  });

  private today(): string {
    return new Date().toJSON().slice(0, 10);
  }

  constructor(events: GlobalEventService) {
    events.deleteOption$.subscribe(id => this.deleteOption(id));
    events.deleteQuestion$.subscribe(id => this.deleteQuestion(id));

    console.log('set date to ' + this.today());
    this.quizForm.get('dateOpen').setValue(this.today());
    this.quizForm.get('duration').setValue(1800);
  }

  public createNewQuestion(): void {
    this.questions.push({
      data: '',
      options: [],
      id: uuidv4(),
    });
    console.log('there are ' + this.questions.length + ' questions');
  }

  public deleteOption(id: string): void {
    for (const question of this.questions) {
      question.options = question.options.filter(option => option.id !== id);
    }
  }

  public deleteQuestion(id: string): void {
    this.questions = this.questions.filter(question => question.id !== id);
  }

  public generateApiRequest(): void {
    const title = this.quizForm.get('title').value;
    const dateOpen = this.quizForm.get('dateOpen').value;
    const duration = this.quizForm.get('duration').value;

  }

  public submitQuiz(): void {
    this.generateApiRequest();
    console.log('struct:');
    console.log(this.questions);
  }
}
