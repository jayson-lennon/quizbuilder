import { Injectable, EventEmitter } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class GlobalEventService {

  public deleteOption$: EventEmitter<string>;
  public deleteQuestion$: EventEmitter<string>;

  constructor() {
    this.deleteOption$ = new EventEmitter();
    this.deleteQuestion$ = new EventEmitter();
  }

  public deleteOption(id: string): void {
    this.deleteOption$.emit(id);
  }

  public deleteQuestion(id: string): void {
    this.deleteQuestion$.emit(id);
  }
}
