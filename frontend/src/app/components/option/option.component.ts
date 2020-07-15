import { Component, OnInit, Input, ViewChild, ElementRef } from '@angular/core';
import { Option } from '../../@types/option';
import { GlobalEventService } from '../../services/global-event.service';

@Component({
  selector: 'app-option',
  templateUrl: './option.component.html',
  styleUrls: ['./option.component.scss']
})
export class OptionComponent implements OnInit {

  @Input() public option: Option;
  @ViewChild('optionInput') optionInput: ElementRef;

  constructor(private eventService: GlobalEventService) { }

  ngOnInit(): void {
    this.option.type = 'SingleChoice';
  }

  ngAfterViewInit() {
    this.optionInput.nativeElement.focus();
  }

  public deleteOption(): void {
    this.eventService.deleteOption(this.option.id);
  }

}
