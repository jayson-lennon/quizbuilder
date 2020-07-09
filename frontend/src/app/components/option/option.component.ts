import { Component, OnInit, Input } from '@angular/core';
import { Option } from '../../@types/option';
import { GlobalEventService } from '../../services/global-event.service';

@Component({
  selector: 'app-option',
  templateUrl: './option.component.html',
  styleUrls: ['./option.component.scss']
})
export class OptionComponent implements OnInit {

  @Input() public option: Option;

  constructor(private eventService: GlobalEventService) { }

  ngOnInit(): void {
  }

  public deleteOption(): void {
    this.eventService.deleteOption(this.option.id);
  }

}
