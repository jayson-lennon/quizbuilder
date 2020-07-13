import { Component } from '@angular/core';
import { Question } from './@types/question';
import { GlobalEventService } from './services/global-event.service';
import { v4 as uuidv4 } from 'uuid';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})

export class AppComponent {
  title = 'frontend';

  public questions: Question[] = [];

  constructor(events: GlobalEventService) {
    events.deleteOption$.subscribe(id => this.deleteOption(id));
    events.deleteQuestion$.subscribe(id => this.deleteQuestion(id));
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

  public submitQuiz(): void {

  }
}
