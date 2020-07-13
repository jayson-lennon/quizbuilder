import { Component, OnInit, Input } from '@angular/core';
import { Question } from '../../@types/question';
import { v4 as uuidv4 } from 'uuid';
import { GlobalEventService } from 'src/app/services/global-event.service';

@Component({
  selector: 'app-question',
  templateUrl: './question.component.html',
  styleUrls: ['./question.component.scss']
})
export class QuestionComponent implements OnInit {

  @Input() public question: Question;

  constructor(private eventService: GlobalEventService) { }

  ngOnInit(): void {
    console.log('question=' + this.question);
  }

  public deleteQuestion(): void {
    this.eventService.deleteQuestion(this.question.id);
  }

  public addOption(): void {
    if (this.question.options === undefined) {
      this.question.options = [];
    }

    this.question.options.push({
      data: '',
      type: 'Radio',
      id: uuidv4(),
      isCorrect: false,
    });
  }

}
